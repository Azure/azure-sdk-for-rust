#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadProperties {
    #[serde(rename = "servicePrincipalClientId", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_client_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "servicePrincipalObjectId", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_object_id: Option<String>,
}
impl AadProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadPropertiesResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadProperties>,
}
impl AadPropertiesResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure File Share workload specific backup copy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareRecoveryPoint {
    #[serde(flatten)]
    pub recovery_point: RecoveryPoint,
    #[doc = "Type of the backup copy. Specifies whether it is a crash consistent backup or app consistent."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<String>,
    #[doc = "Time at which this backup copy was created."]
    #[serde(rename = "recoveryPointTime", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
    #[doc = "Contains Url to the snapshot of fileshare, if applicable"]
    #[serde(rename = "fileShareSnapshotUri", default, skip_serializing_if = "Option::is_none")]
    pub file_share_snapshot_uri: Option<String>,
    #[doc = "Contains recovery point size"]
    #[serde(rename = "recoveryPointSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_size_in_gb: Option<i32>,
}
impl AzureFileShareRecoveryPoint {
    pub fn new(recovery_point: RecoveryPoint) -> Self {
        Self {
            recovery_point,
            recovery_point_type: None,
            recovery_point_time: None,
            file_share_snapshot_uri: None,
            recovery_point_size_in_gb: None,
        }
    }
}
#[doc = "AzureFileShare Restore Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareRestoreRequest {
    #[serde(flatten)]
    pub restore_request: RestoreRequest,
    #[doc = "Type of this recovery."]
    #[serde(rename = "recoveryType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_type: Option<azure_file_share_restore_request::RecoveryType>,
    #[doc = "Source storage account ARM Id"]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "Options to resolve copy conflicts."]
    #[serde(rename = "copyOptions", default, skip_serializing_if = "Option::is_none")]
    pub copy_options: Option<azure_file_share_restore_request::CopyOptions>,
    #[doc = "Restore Type (FullShareRestore or ItemLevelRestore)"]
    #[serde(rename = "restoreRequestType", default, skip_serializing_if = "Option::is_none")]
    pub restore_request_type: Option<azure_file_share_restore_request::RestoreRequestType>,
    #[doc = "List of Source Files/Folders(which need to recover) and TargetFolderPath details"]
    #[serde(rename = "restoreFileSpecs", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_specs: Vec<RestoreFileSpecs>,
    #[doc = "Target Azure File Share Info."]
    #[serde(rename = "targetDetails", default, skip_serializing_if = "Option::is_none")]
    pub target_details: Option<TargetAfsRestoreInfo>,
}
impl AzureFileShareRestoreRequest {
    pub fn new(restore_request: RestoreRequest) -> Self {
        Self {
            restore_request,
            recovery_type: None,
            source_resource_id: None,
            copy_options: None,
            restore_request_type: None,
            restore_file_specs: Vec::new(),
            target_details: None,
        }
    }
}
pub mod azure_file_share_restore_request {
    use super::*;
    #[doc = "Type of this recovery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryType")]
    pub enum RecoveryType {
        Invalid,
        OriginalLocation,
        AlternateLocation,
        RestoreDisks,
        Offline,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RecoveryType", 0u32, "Invalid"),
                Self::OriginalLocation => serializer.serialize_unit_variant("RecoveryType", 1u32, "OriginalLocation"),
                Self::AlternateLocation => serializer.serialize_unit_variant("RecoveryType", 2u32, "AlternateLocation"),
                Self::RestoreDisks => serializer.serialize_unit_variant("RecoveryType", 3u32, "RestoreDisks"),
                Self::Offline => serializer.serialize_unit_variant("RecoveryType", 4u32, "Offline"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Options to resolve copy conflicts."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CopyOptions")]
    pub enum CopyOptions {
        Invalid,
        CreateCopy,
        Skip,
        Overwrite,
        FailOnConflict,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CopyOptions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CopyOptions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CopyOptions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("CopyOptions", 0u32, "Invalid"),
                Self::CreateCopy => serializer.serialize_unit_variant("CopyOptions", 1u32, "CreateCopy"),
                Self::Skip => serializer.serialize_unit_variant("CopyOptions", 2u32, "Skip"),
                Self::Overwrite => serializer.serialize_unit_variant("CopyOptions", 3u32, "Overwrite"),
                Self::FailOnConflict => serializer.serialize_unit_variant("CopyOptions", 4u32, "FailOnConflict"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Restore Type (FullShareRestore or ItemLevelRestore)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RestoreRequestType")]
    pub enum RestoreRequestType {
        Invalid,
        FullShareRestore,
        ItemLevelRestore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RestoreRequestType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RestoreRequestType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RestoreRequestType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RestoreRequestType", 0u32, "Invalid"),
                Self::FullShareRestore => serializer.serialize_unit_variant("RestoreRequestType", 1u32, "FullShareRestore"),
                Self::ItemLevelRestore => serializer.serialize_unit_variant("RestoreRequestType", 2u32, "ItemLevelRestore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure File Share workload-specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileshareProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Friendly name of the fileshare represented by this backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Backup status of this backup item."]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<String>,
    #[doc = "Backup state of this backup item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<azure_fileshare_protected_item::ProtectionState>,
    #[doc = "backups running status for this backup item."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<azure_fileshare_protected_item::HealthStatus>,
    #[doc = "Last backup operation status. Possible values: Healthy, Unhealthy."]
    #[serde(rename = "lastBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_status: Option<String>,
    #[doc = "Timestamp of the last backup operation on this backup item."]
    #[serde(rename = "lastBackupTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_backup_time: Option<time::OffsetDateTime>,
    #[doc = "Health details of different KPIs"]
    #[serde(rename = "kpisHealths", default, skip_serializing_if = "Option::is_none")]
    pub kpis_healths: Option<serde_json::Value>,
    #[doc = "Additional information about Azure File Share backup item."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureFileshareProtectedItemExtendedInfo>,
}
impl AzureFileshareProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            friendly_name: None,
            protection_status: None,
            protection_state: None,
            health_status: None,
            last_backup_status: None,
            last_backup_time: None,
            kpis_healths: None,
            extended_info: None,
        }
    }
}
pub mod azure_fileshare_protected_item {
    use super::*;
    #[doc = "Backup state of this backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        #[serde(rename = "IRPending")]
        IrPending,
        Protected,
        ProtectionError,
        ProtectionStopped,
        ProtectionPaused,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionState", 0u32, "Invalid"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectionState", 1u32, "IRPending"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protected"),
                Self::ProtectionError => serializer.serialize_unit_variant("ProtectionState", 3u32, "ProtectionError"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionStopped"),
                Self::ProtectionPaused => serializer.serialize_unit_variant("ProtectionState", 5u32, "ProtectionPaused"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "backups running status for this backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthStatus")]
    pub enum HealthStatus {
        Passed,
        ActionRequired,
        ActionSuggested,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Passed => serializer.serialize_unit_variant("HealthStatus", 0u32, "Passed"),
                Self::ActionRequired => serializer.serialize_unit_variant("HealthStatus", 1u32, "ActionRequired"),
                Self::ActionSuggested => serializer.serialize_unit_variant("HealthStatus", 2u32, "ActionSuggested"),
                Self::Invalid => serializer.serialize_unit_variant("HealthStatus", 3u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional information about Azure File Share backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileshareProtectedItemExtendedInfo {
    #[doc = "The oldest backup copy available for this item in the service."]
    #[serde(rename = "oldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "Number of available backup copies associated with this backup item."]
    #[serde(rename = "recoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_count: Option<i32>,
    #[doc = "Indicates consistency of policy object and policy applied to this backup item."]
    #[serde(rename = "policyState", default, skip_serializing_if = "Option::is_none")]
    pub policy_state: Option<String>,
    #[doc = "Indicates the state of this resource. Possible values are from enum ResourceState {Invalid, Active, SoftDeleted, Deleted}"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<String>,
    #[doc = "The resource state sync time for this backup item."]
    #[serde(rename = "resourceStateSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub resource_state_sync_time: Option<time::OffsetDateTime>,
}
impl AzureFileshareProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IaaS VM workload-specific backup item representing the Classic Compute VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSClassicComputeVmProtectedItem {
    #[serde(flatten)]
    pub azure_iaa_svm_protected_item: AzureIaaSvmProtectedItem,
}
impl AzureIaaSClassicComputeVmProtectedItem {
    pub fn new(azure_iaa_svm_protected_item: AzureIaaSvmProtectedItem) -> Self {
        Self {
            azure_iaa_svm_protected_item,
        }
    }
}
#[doc = "IaaS VM workload-specific backup item representing the Azure Resource Manager VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSComputeVmProtectedItem {
    #[serde(flatten)]
    pub azure_iaa_svm_protected_item: AzureIaaSvmProtectedItem,
}
impl AzureIaaSComputeVmProtectedItem {
    pub fn new(azure_iaa_svm_protected_item: AzureIaaSvmProtectedItem) -> Self {
        Self {
            azure_iaa_svm_protected_item,
        }
    }
}
#[doc = "Azure IaaS VM workload-specific error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureIaaSvmErrorInfo {
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Title: Typically, the entity that the error pertains to."]
    #[serde(rename = "errorTitle", default, skip_serializing_if = "Option::is_none")]
    pub error_title: Option<String>,
    #[doc = "Localized error string."]
    #[serde(rename = "errorString", default, skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[doc = "List of localized recommendations for above error code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
impl AzureIaaSvmErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure IaaS VM workload-specific Health Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureIaaSvmHealthDetails {
    #[serde(flatten)]
    pub resource_health_details: ResourceHealthDetails,
}
impl AzureIaaSvmHealthDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure IaaS VM workload-specific job object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmJob {
    #[serde(flatten)]
    pub job: Job,
    #[doc = "Time elapsed during the execution of this job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Gets or sets the state/actions applicable on this job like cancel/retry."]
    #[serde(rename = "actionsInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[doc = "Error details on execution of this job."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<AzureIaaSvmErrorInfo>,
    #[doc = "Specifies whether the backup item is a Classic or an Azure Resource Manager VM."]
    #[serde(rename = "virtualMachineVersion", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_version: Option<String>,
    #[doc = "Azure IaaS VM workload-specific additional information for job."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureIaaSvmJobExtendedInfo>,
}
impl AzureIaaSvmJob {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            duration: None,
            actions_info: Vec::new(),
            error_details: Vec::new(),
            virtual_machine_version: None,
            extended_info: None,
        }
    }
}
#[doc = "Azure IaaS VM workload-specific additional information for job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureIaaSvmJobExtendedInfo {
    #[doc = "List of tasks associated with this job."]
    #[serde(rename = "tasksList", default, skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<AzureIaaSvmJobTaskDetails>,
    #[doc = "Job properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[doc = "Job internal properties."]
    #[serde(rename = "internalPropertyBag", default, skip_serializing_if = "Option::is_none")]
    pub internal_property_bag: Option<serde_json::Value>,
    #[doc = "Indicates progress of the job. Null if it has not started or completed."]
    #[serde(rename = "progressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
    #[doc = "Time remaining for execution of this job."]
    #[serde(rename = "estimatedRemainingDuration", default, skip_serializing_if = "Option::is_none")]
    pub estimated_remaining_duration: Option<String>,
    #[doc = "Non localized error message on job execution."]
    #[serde(rename = "dynamicErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
impl AzureIaaSvmJobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure IaaS VM workload-specific job task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureIaaSvmJobTaskDetails {
    #[doc = "The task display name."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The instanceId."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Time elapsed for task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Progress of the task."]
    #[serde(rename = "progressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
    #[doc = "Details about execution of the task.\r\neg: number of bytes transferred etc"]
    #[serde(rename = "taskExecutionDetails", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_details: Option<String>,
}
impl AzureIaaSvmJobTaskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IaaS VM workload-specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Friendly name of the VM represented by this backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Fully qualified ARM ID of the virtual machine represented by this item."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Backup status of this backup item."]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<String>,
    #[doc = "Backup state of this backup item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<azure_iaa_svm_protected_item::ProtectionState>,
    #[doc = "Health status of protected item."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<azure_iaa_svm_protected_item::HealthStatus>,
    #[doc = "Health details on this backup item."]
    #[serde(rename = "healthDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub health_details: Vec<AzureIaaSvmHealthDetails>,
    #[doc = "Health details of different KPIs"]
    #[serde(rename = "kpisHealths", default, skip_serializing_if = "Option::is_none")]
    pub kpis_healths: Option<serde_json::Value>,
    #[doc = "Last backup operation status."]
    #[serde(rename = "lastBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_status: Option<String>,
    #[doc = "Timestamp of the last backup operation on this backup item."]
    #[serde(rename = "lastBackupTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_backup_time: Option<time::OffsetDateTime>,
    #[doc = "Data ID of the protected item."]
    #[serde(rename = "protectedItemDataId", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_data_id: Option<String>,
    #[doc = "Additional information on Azure IaaS VM specific backup item."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureIaaSvmProtectedItemExtendedInfo>,
    #[doc = "Extended Properties for Azure IaasVM Backup."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,
}
impl AzureIaaSvmProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            friendly_name: None,
            virtual_machine_id: None,
            protection_status: None,
            protection_state: None,
            health_status: None,
            health_details: Vec::new(),
            kpis_healths: None,
            last_backup_status: None,
            last_backup_time: None,
            protected_item_data_id: None,
            extended_info: None,
            extended_properties: None,
        }
    }
}
pub mod azure_iaa_svm_protected_item {
    use super::*;
    #[doc = "Backup state of this backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        #[serde(rename = "IRPending")]
        IrPending,
        Protected,
        ProtectionError,
        ProtectionStopped,
        ProtectionPaused,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionState", 0u32, "Invalid"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectionState", 1u32, "IRPending"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protected"),
                Self::ProtectionError => serializer.serialize_unit_variant("ProtectionState", 3u32, "ProtectionError"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionStopped"),
                Self::ProtectionPaused => serializer.serialize_unit_variant("ProtectionState", 5u32, "ProtectionPaused"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Health status of protected item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthStatus")]
    pub enum HealthStatus {
        Passed,
        ActionRequired,
        ActionSuggested,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Passed => serializer.serialize_unit_variant("HealthStatus", 0u32, "Passed"),
                Self::ActionRequired => serializer.serialize_unit_variant("HealthStatus", 1u32, "ActionRequired"),
                Self::ActionSuggested => serializer.serialize_unit_variant("HealthStatus", 2u32, "ActionSuggested"),
                Self::Invalid => serializer.serialize_unit_variant("HealthStatus", 3u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional information on Azure IaaS VM specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureIaaSvmProtectedItemExtendedInfo {
    #[doc = "The oldest backup copy available for this backup item."]
    #[serde(rename = "oldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "Number of backup copies available for this backup item."]
    #[serde(rename = "recoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_count: Option<i32>,
    #[doc = "Specifies if backup policy associated with the backup item is inconsistent."]
    #[serde(rename = "policyInconsistent", default, skip_serializing_if = "Option::is_none")]
    pub policy_inconsistent: Option<bool>,
}
impl AzureIaaSvmProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure SQL workload-specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Internal ID of a backup item. Used by Azure SQL Backup engine to contact Recovery Services."]
    #[serde(rename = "protectedItemDataId", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_data_id: Option<String>,
    #[doc = "Backup state of the backed up item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<azure_sql_protected_item::ProtectionState>,
    #[doc = "Additional information on Azure Sql specific protected item."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureSqlProtectedItemExtendedInfo>,
}
impl AzureSqlProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            protected_item_data_id: None,
            protection_state: None,
            extended_info: None,
        }
    }
}
pub mod azure_sql_protected_item {
    use super::*;
    #[doc = "Backup state of the backed up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        #[serde(rename = "IRPending")]
        IrPending,
        Protected,
        ProtectionError,
        ProtectionStopped,
        ProtectionPaused,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionState", 0u32, "Invalid"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectionState", 1u32, "IRPending"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protected"),
                Self::ProtectionError => serializer.serialize_unit_variant("ProtectionState", 3u32, "ProtectionError"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionStopped"),
                Self::ProtectionPaused => serializer.serialize_unit_variant("ProtectionState", 5u32, "ProtectionPaused"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional information on Azure Sql specific protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlProtectedItemExtendedInfo {
    #[doc = "The oldest backup copy available for this item in the service."]
    #[serde(rename = "oldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "Number of available backup copies associated with this backup item."]
    #[serde(rename = "recoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_count: Option<i32>,
    #[doc = "State of the backup policy associated with this backup item."]
    #[serde(rename = "policyState", default, skip_serializing_if = "Option::is_none")]
    pub policy_state: Option<String>,
}
impl AzureSqlProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure storage specific error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageErrorInfo {
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Localized error string."]
    #[serde(rename = "errorString", default, skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[doc = "List of localized recommendations for above error code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
impl AzureStorageErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure storage specific job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageJob {
    #[serde(flatten)]
    pub job: Job,
    #[doc = "Time elapsed during the execution of this job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Gets or sets the state/actions applicable on this job like cancel/retry."]
    #[serde(rename = "actionsInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[doc = "Error details on execution of this job."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<AzureStorageErrorInfo>,
    #[doc = "Specifies friendly name of the storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[doc = "Specifies whether the Storage account is a Classic or an Azure Resource Manager Storage account."]
    #[serde(rename = "storageAccountVersion", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_version: Option<String>,
    #[doc = "Azure Storage workload-specific additional information for job."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureStorageJobExtendedInfo>,
}
impl AzureStorageJob {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            duration: None,
            actions_info: Vec::new(),
            error_details: Vec::new(),
            storage_account_name: None,
            storage_account_version: None,
            extended_info: None,
        }
    }
}
#[doc = "Azure Storage workload-specific additional information for job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageJobExtendedInfo {
    #[doc = "List of tasks for this job"]
    #[serde(rename = "tasksList", default, skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<AzureStorageJobTaskDetails>,
    #[doc = "Job properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[doc = "Non localized error message on job execution."]
    #[serde(rename = "dynamicErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
impl AzureStorageJobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure storage workload specific job task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageJobTaskDetails {
    #[doc = "The task display name."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl AzureStorageJobTaskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure VM workload-specific protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Friendly name of the DB represented by this backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Host/Cluster Name for instance or AG"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Parent name of the DB such as Instance or Availability Group."]
    #[serde(rename = "parentName", default, skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[doc = "Parent type of protected item, example: for a DB, standalone server or distributed"]
    #[serde(rename = "parentType", default, skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
    #[doc = "Backup status of this backup item."]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<String>,
    #[doc = "Backup state of this backup item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<azure_vm_workload_protected_item::ProtectionState>,
    #[doc = "Last backup operation status. Possible values: Healthy, Unhealthy."]
    #[serde(rename = "lastBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_status: Option<azure_vm_workload_protected_item::LastBackupStatus>,
    #[doc = "Timestamp of the last backup operation on this backup item."]
    #[serde(rename = "lastBackupTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_backup_time: Option<time::OffsetDateTime>,
    #[doc = "Error Detail class which encapsulates Code, Message and Recommendations."]
    #[serde(rename = "lastBackupErrorDetail", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_error_detail: Option<ErrorDetail>,
    #[doc = "Data ID of the protected item."]
    #[serde(rename = "protectedItemDataSourceId", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_data_source_id: Option<String>,
    #[doc = "Health status of the backup item, evaluated based on last heartbeat received"]
    #[serde(rename = "protectedItemHealthStatus", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_health_status: Option<azure_vm_workload_protected_item::ProtectedItemHealthStatus>,
    #[doc = "Additional information on Azure Workload for SQL specific backup item."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureVmWorkloadProtectedItemExtendedInfo>,
    #[doc = "Health details of different KPIs"]
    #[serde(rename = "kpisHealths", default, skip_serializing_if = "Option::is_none")]
    pub kpis_healths: Option<serde_json::Value>,
}
impl AzureVmWorkloadProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            friendly_name: None,
            server_name: None,
            parent_name: None,
            parent_type: None,
            protection_status: None,
            protection_state: None,
            last_backup_status: None,
            last_backup_time: None,
            last_backup_error_detail: None,
            protected_item_data_source_id: None,
            protected_item_health_status: None,
            extended_info: None,
            kpis_healths: None,
        }
    }
}
pub mod azure_vm_workload_protected_item {
    use super::*;
    #[doc = "Backup state of this backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        #[serde(rename = "IRPending")]
        IrPending,
        Protected,
        ProtectionError,
        ProtectionStopped,
        ProtectionPaused,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionState", 0u32, "Invalid"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectionState", 1u32, "IRPending"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protected"),
                Self::ProtectionError => serializer.serialize_unit_variant("ProtectionState", 3u32, "ProtectionError"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionStopped"),
                Self::ProtectionPaused => serializer.serialize_unit_variant("ProtectionState", 5u32, "ProtectionPaused"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Last backup operation status. Possible values: Healthy, Unhealthy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastBackupStatus")]
    pub enum LastBackupStatus {
        Invalid,
        Healthy,
        Unhealthy,
        #[serde(rename = "IRPending")]
        IrPending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastBackupStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastBackupStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastBackupStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("LastBackupStatus", 0u32, "Invalid"),
                Self::Healthy => serializer.serialize_unit_variant("LastBackupStatus", 1u32, "Healthy"),
                Self::Unhealthy => serializer.serialize_unit_variant("LastBackupStatus", 2u32, "Unhealthy"),
                Self::IrPending => serializer.serialize_unit_variant("LastBackupStatus", 3u32, "IRPending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Health status of the backup item, evaluated based on last heartbeat received"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectedItemHealthStatus")]
    pub enum ProtectedItemHealthStatus {
        Invalid,
        Healthy,
        Unhealthy,
        NotReachable,
        #[serde(rename = "IRPending")]
        IrPending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectedItemHealthStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectedItemHealthStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectedItemHealthStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectedItemHealthStatus", 0u32, "Invalid"),
                Self::Healthy => serializer.serialize_unit_variant("ProtectedItemHealthStatus", 1u32, "Healthy"),
                Self::Unhealthy => serializer.serialize_unit_variant("ProtectedItemHealthStatus", 2u32, "Unhealthy"),
                Self::NotReachable => serializer.serialize_unit_variant("ProtectedItemHealthStatus", 3u32, "NotReachable"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectedItemHealthStatus", 4u32, "IRPending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional information on Azure Workload for SQL specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureVmWorkloadProtectedItemExtendedInfo {
    #[doc = "The oldest backup copy available for this backup item."]
    #[serde(rename = "oldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "Number of backup copies available for this backup item."]
    #[serde(rename = "recoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_count: Option<i32>,
    #[doc = "Indicates consistency of policy object and policy applied to this backup item."]
    #[serde(rename = "policyState", default, skip_serializing_if = "Option::is_none")]
    pub policy_state: Option<String>,
}
impl AzureVmWorkloadProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure VM workload-specific protected item representing SAP ASE Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapAseDatabaseProtectedItem {
    #[serde(flatten)]
    pub azure_vm_workload_protected_item: AzureVmWorkloadProtectedItem,
}
impl AzureVmWorkloadSapAseDatabaseProtectedItem {
    pub fn new(azure_vm_workload_protected_item: AzureVmWorkloadProtectedItem) -> Self {
        Self {
            azure_vm_workload_protected_item,
        }
    }
}
#[doc = "Azure VM workload-specific protected item representing SAP HANA Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapHanaDatabaseProtectedItem {
    #[serde(flatten)]
    pub azure_vm_workload_protected_item: AzureVmWorkloadProtectedItem,
}
impl AzureVmWorkloadSapHanaDatabaseProtectedItem {
    pub fn new(azure_vm_workload_protected_item: AzureVmWorkloadProtectedItem) -> Self {
        Self {
            azure_vm_workload_protected_item,
        }
    }
}
#[doc = "Azure VM workload-specific protected item representing SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSqlDatabaseProtectedItem {
    #[serde(flatten)]
    pub azure_vm_workload_protected_item: AzureVmWorkloadProtectedItem,
}
impl AzureVmWorkloadSqlDatabaseProtectedItem {
    pub fn new(azure_vm_workload_protected_item: AzureVmWorkloadProtectedItem) -> Self {
        Self {
            azure_vm_workload_protected_item,
        }
    }
}
#[doc = "Azure storage specific error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureWorkloadErrorInfo {
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Localized error string."]
    #[serde(rename = "errorString", default, skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[doc = "Title: Typically, the entity that the error pertains to."]
    #[serde(rename = "errorTitle", default, skip_serializing_if = "Option::is_none")]
    pub error_title: Option<String>,
    #[doc = "List of localized recommendations for above error code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
    #[doc = "Additional details for above error code."]
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}
impl AzureWorkloadErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure storage specific job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadJob {
    #[serde(flatten)]
    pub job: Job,
    #[doc = "Workload type of the job"]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[doc = "Time elapsed during the execution of this job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Gets or sets the state/actions applicable on this job like cancel/retry."]
    #[serde(rename = "actionsInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[doc = "Error details on execution of this job."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<AzureWorkloadErrorInfo>,
    #[doc = "Azure VM workload-specific additional information for job."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureWorkloadJobExtendedInfo>,
}
impl AzureWorkloadJob {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            workload_type: None,
            duration: None,
            actions_info: Vec::new(),
            error_details: Vec::new(),
            extended_info: None,
        }
    }
}
#[doc = "Azure VM workload-specific additional information for job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureWorkloadJobExtendedInfo {
    #[doc = "List of tasks for this job"]
    #[serde(rename = "tasksList", default, skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<AzureWorkloadJobTaskDetails>,
    #[doc = "Job properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[doc = "Non localized error message on job execution."]
    #[serde(rename = "dynamicErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
impl AzureWorkloadJobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure VM workload specific job task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureWorkloadJobTaskDetails {
    #[doc = "The task display name."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl AzureWorkloadJobTaskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery point specific to PointInTime"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadPointInTimeRecoveryPoint {
    #[serde(flatten)]
    pub azure_workload_recovery_point: AzureWorkloadRecoveryPoint,
    #[doc = "List of log ranges"]
    #[serde(rename = "timeRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub time_ranges: Vec<PointInTimeRange>,
}
impl AzureWorkloadPointInTimeRecoveryPoint {
    pub fn new(azure_workload_recovery_point: AzureWorkloadRecoveryPoint) -> Self {
        Self {
            azure_workload_recovery_point,
            time_ranges: Vec::new(),
        }
    }
}
#[doc = "AzureWorkload SAP Hana -specific restore. Specifically for PointInTime/Log restore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadPointInTimeRestoreRequest {
    #[serde(flatten)]
    pub azure_workload_restore_request: AzureWorkloadRestoreRequest,
    #[doc = "PointInTime value"]
    #[serde(rename = "pointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub point_in_time: Option<time::OffsetDateTime>,
}
impl AzureWorkloadPointInTimeRestoreRequest {
    pub fn new(azure_workload_restore_request: AzureWorkloadRestoreRequest) -> Self {
        Self {
            azure_workload_restore_request,
            point_in_time: None,
        }
    }
}
#[doc = "Workload specific recovery point, specifically encapsulates full/diff recovery point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadRecoveryPoint {
    #[serde(flatten)]
    pub recovery_point: RecoveryPoint,
    #[doc = "UTC time at which recovery point was created"]
    #[serde(rename = "recoveryPointTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "Type of restore point"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<azure_workload_recovery_point::Type>,
    #[doc = "Recovery point tier information."]
    #[serde(rename = "recoveryPointTierDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_point_tier_details: Vec<RecoveryPointTierInformation>,
    #[doc = "Eligibility of RP to be moved to another tier"]
    #[serde(rename = "recoveryPointMoveReadinessInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_move_readiness_info: Option<serde_json::Value>,
}
impl AzureWorkloadRecoveryPoint {
    pub fn new(recovery_point: RecoveryPoint) -> Self {
        Self {
            recovery_point,
            recovery_point_time_in_utc: None,
            type_: None,
            recovery_point_tier_details: Vec::new(),
            recovery_point_move_readiness_info: None,
        }
    }
}
pub mod azure_workload_recovery_point {
    use super::*;
    #[doc = "Type of restore point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Invalid,
        Full,
        Log,
        Differential,
        Incremental,
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
                Self::Invalid => serializer.serialize_unit_variant("Type", 0u32, "Invalid"),
                Self::Full => serializer.serialize_unit_variant("Type", 1u32, "Full"),
                Self::Log => serializer.serialize_unit_variant("Type", 2u32, "Log"),
                Self::Differential => serializer.serialize_unit_variant("Type", 3u32, "Differential"),
                Self::Incremental => serializer.serialize_unit_variant("Type", 4u32, "Incremental"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "AzureWorkload-specific restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadRestoreRequest {
    #[serde(flatten)]
    pub restore_request: RestoreRequest,
    #[doc = "Type of this recovery."]
    #[serde(rename = "recoveryType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_type: Option<azure_workload_restore_request::RecoveryType>,
    #[doc = "Fully qualified ARM ID of the VM on which workload that was running is being recovered."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "Workload specific property bag."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[doc = "Details about target workload during restore operation."]
    #[serde(rename = "targetInfo", default, skip_serializing_if = "Option::is_none")]
    pub target_info: Option<TargetRestoreInfo>,
    #[doc = "Defines whether the current recovery mode is file restore or database restore"]
    #[serde(rename = "recoveryMode", default, skip_serializing_if = "Option::is_none")]
    pub recovery_mode: Option<azure_workload_restore_request::RecoveryMode>,
    #[doc = "This is the complete ARM Id of the target VM\r\nFor e.g. /subscriptions/{subId}/resourcegroups/{rg}/provider/Microsoft.Compute/virtualmachines/{vm}"]
    #[serde(rename = "targetVirtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub target_virtual_machine_id: Option<String>,
}
impl AzureWorkloadRestoreRequest {
    pub fn new(restore_request: RestoreRequest) -> Self {
        Self {
            restore_request,
            recovery_type: None,
            source_resource_id: None,
            property_bag: None,
            target_info: None,
            recovery_mode: None,
            target_virtual_machine_id: None,
        }
    }
}
pub mod azure_workload_restore_request {
    use super::*;
    #[doc = "Type of this recovery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryType")]
    pub enum RecoveryType {
        Invalid,
        OriginalLocation,
        AlternateLocation,
        RestoreDisks,
        Offline,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RecoveryType", 0u32, "Invalid"),
                Self::OriginalLocation => serializer.serialize_unit_variant("RecoveryType", 1u32, "OriginalLocation"),
                Self::AlternateLocation => serializer.serialize_unit_variant("RecoveryType", 2u32, "AlternateLocation"),
                Self::RestoreDisks => serializer.serialize_unit_variant("RecoveryType", 3u32, "RestoreDisks"),
                Self::Offline => serializer.serialize_unit_variant("RecoveryType", 4u32, "Offline"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Defines whether the current recovery mode is file restore or database restore"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryMode")]
    pub enum RecoveryMode {
        Invalid,
        FileRecovery,
        WorkloadRecovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RecoveryMode", 0u32, "Invalid"),
                Self::FileRecovery => serializer.serialize_unit_variant("RecoveryMode", 1u32, "FileRecovery"),
                Self::WorkloadRecovery => serializer.serialize_unit_variant("RecoveryMode", 2u32, "WorkloadRecovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery point specific to PointInTime in SAPHana"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSapHanaPointInTimeRecoveryPoint {
    #[serde(flatten)]
    pub azure_workload_point_in_time_recovery_point: AzureWorkloadPointInTimeRecoveryPoint,
}
impl AzureWorkloadSapHanaPointInTimeRecoveryPoint {
    pub fn new(azure_workload_point_in_time_recovery_point: AzureWorkloadPointInTimeRecoveryPoint) -> Self {
        Self {
            azure_workload_point_in_time_recovery_point,
        }
    }
}
#[doc = "AzureWorkload SAP Hana -specific restore. Specifically for PointInTime/Log restore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSapHanaPointInTimeRestoreRequest {
    #[serde(flatten)]
    pub azure_workload_sap_hana_restore_request: AzureWorkloadSapHanaRestoreRequest,
    #[doc = "PointInTime value"]
    #[serde(rename = "pointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub point_in_time: Option<time::OffsetDateTime>,
}
impl AzureWorkloadSapHanaPointInTimeRestoreRequest {
    pub fn new(azure_workload_sap_hana_restore_request: AzureWorkloadSapHanaRestoreRequest) -> Self {
        Self {
            azure_workload_sap_hana_restore_request,
            point_in_time: None,
        }
    }
}
#[doc = "SAPHana specific recoverypoint, specifically encapsulates full/diff recoverypoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSapHanaRecoveryPoint {
    #[serde(flatten)]
    pub azure_workload_recovery_point: AzureWorkloadRecoveryPoint,
}
impl AzureWorkloadSapHanaRecoveryPoint {
    pub fn new(azure_workload_recovery_point: AzureWorkloadRecoveryPoint) -> Self {
        Self {
            azure_workload_recovery_point,
        }
    }
}
#[doc = "AzureWorkload SAP Hana-specific restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSapHanaRestoreRequest {
    #[serde(flatten)]
    pub azure_workload_restore_request: AzureWorkloadRestoreRequest,
}
impl AzureWorkloadSapHanaRestoreRequest {
    pub fn new(azure_workload_restore_request: AzureWorkloadRestoreRequest) -> Self {
        Self {
            azure_workload_restore_request,
        }
    }
}
#[doc = "Recovery point specific to PointInTime"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlPointInTimeRecoveryPoint {
    #[serde(flatten)]
    pub azure_workload_sql_recovery_point: AzureWorkloadSqlRecoveryPoint,
    #[doc = "List of log ranges"]
    #[serde(rename = "timeRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub time_ranges: Vec<PointInTimeRange>,
}
impl AzureWorkloadSqlPointInTimeRecoveryPoint {
    pub fn new(azure_workload_sql_recovery_point: AzureWorkloadSqlRecoveryPoint) -> Self {
        Self {
            azure_workload_sql_recovery_point,
            time_ranges: Vec::new(),
        }
    }
}
#[doc = "AzureWorkload SQL -specific restore. Specifically for PointInTime/Log restore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlPointInTimeRestoreRequest {
    #[serde(flatten)]
    pub azure_workload_sql_restore_request: AzureWorkloadSqlRestoreRequest,
    #[doc = "PointInTime value"]
    #[serde(rename = "pointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub point_in_time: Option<time::OffsetDateTime>,
}
impl AzureWorkloadSqlPointInTimeRestoreRequest {
    pub fn new(azure_workload_sql_restore_request: AzureWorkloadSqlRestoreRequest) -> Self {
        Self {
            azure_workload_sql_restore_request,
            point_in_time: None,
        }
    }
}
#[doc = "SQL specific recoverypoint, specifically encapsulates full/diff recoverypoint along with extended info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlRecoveryPoint {
    #[serde(flatten)]
    pub azure_workload_recovery_point: AzureWorkloadRecoveryPoint,
    #[doc = "Extended info class details"]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureWorkloadSqlRecoveryPointExtendedInfo>,
}
impl AzureWorkloadSqlRecoveryPoint {
    pub fn new(azure_workload_recovery_point: AzureWorkloadRecoveryPoint) -> Self {
        Self {
            azure_workload_recovery_point,
            extended_info: None,
        }
    }
}
#[doc = "Extended info class details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureWorkloadSqlRecoveryPointExtendedInfo {
    #[doc = "UTC time at which data directory info was captured"]
    #[serde(rename = "dataDirectoryTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub data_directory_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "List of data directory paths during restore operation."]
    #[serde(rename = "dataDirectoryPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub data_directory_paths: Vec<SqlDataDirectory>,
}
impl AzureWorkloadSqlRecoveryPointExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureWorkload SQL -specific restore. Specifically for full/diff restore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlRestoreRequest {
    #[serde(flatten)]
    pub azure_workload_restore_request: AzureWorkloadRestoreRequest,
    #[doc = "Default option set to true. If this is set to false, alternate data directory must be provided"]
    #[serde(rename = "shouldUseAlternateTargetLocation", default, skip_serializing_if = "Option::is_none")]
    pub should_use_alternate_target_location: Option<bool>,
    #[doc = "SQL specific property where user can chose to set no-recovery when restore operation is tried"]
    #[serde(rename = "isNonRecoverable", default, skip_serializing_if = "Option::is_none")]
    pub is_non_recoverable: Option<bool>,
    #[doc = "Data directory details"]
    #[serde(rename = "alternateDirectoryPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_directory_paths: Vec<SqlDataDirectoryMapping>,
}
impl AzureWorkloadSqlRestoreRequest {
    pub fn new(azure_workload_restore_request: AzureWorkloadRestoreRequest) -> Self {
        Self {
            azure_workload_restore_request,
            should_use_alternate_target_location: None,
            is_non_recoverable: None,
            alternate_directory_paths: Vec::new(),
        }
    }
}
#[doc = "BEK is bitlocker encryption key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BekDetails {
    #[doc = "Secret is BEK."]
    #[serde(rename = "secretUrl", default, skip_serializing_if = "Option::is_none")]
    pub secret_url: Option<String>,
    #[doc = "ID of the Key Vault where this Secret is stored."]
    #[serde(rename = "secretVaultId", default, skip_serializing_if = "Option::is_none")]
    pub secret_vault_id: Option<String>,
    #[doc = "BEK data."]
    #[serde(rename = "secretData", default, skip_serializing_if = "Option::is_none")]
    pub secret_data: Option<String>,
}
impl BekDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filters to list backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsaadPropertiesQueryObject {
    #[doc = "Backup management type for the backed up item."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<bmsaad_properties_query_object::BackupManagementType>,
}
impl BmsaadPropertiesQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bmsaad_properties_query_object {
    use super::*;
    #[doc = "Backup management type for the backed up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupManagementType")]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupManagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupManagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupManagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupManagementType", 0u32, "Invalid"),
                Self::AzureIaasVm => serializer.serialize_unit_variant("BackupManagementType", 1u32, "AzureIaasVM"),
                Self::Mab => serializer.serialize_unit_variant("BackupManagementType", 2u32, "MAB"),
                Self::Dpm => serializer.serialize_unit_variant("BackupManagementType", 3u32, "DPM"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureBackupServer"),
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureSql"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureStorage"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureWorkload"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Query parameters to fetch backup summaries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsBackupSummariesQueryObject {
    #[doc = "Backup management type for this container."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<bms_backup_summaries_query_object::Type>,
}
impl BmsBackupSummariesQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bms_backup_summaries_query_object {
    use super::*;
    #[doc = "Backup management type for this container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Invalid,
        BackupProtectedItemCountSummary,
        BackupProtectionContainerCountSummary,
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
                Self::Invalid => serializer.serialize_unit_variant("Type", 0u32, "Invalid"),
                Self::BackupProtectedItemCountSummary => serializer.serialize_unit_variant("Type", 1u32, "BackupProtectedItemCountSummary"),
                Self::BackupProtectionContainerCountSummary => {
                    serializer.serialize_unit_variant("Type", 2u32, "BackupProtectionContainerCountSummary")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Filters to list backup copies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsrpQueryObject {
    #[doc = "Backup copies created after this time."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Backup copies created before this time."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "RestorePoint type"]
    #[serde(rename = "restorePointQueryType", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_query_type: Option<bmsrp_query_object::RestorePointQueryType>,
    #[doc = "In Get Recovery Point, it tells whether extended information about recovery point is asked."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<bool>,
    #[doc = "Whether the RP can be moved to another tier"]
    #[serde(rename = "moveReadyRPOnly", default, skip_serializing_if = "Option::is_none")]
    pub move_ready_rp_only: Option<bool>,
}
impl BmsrpQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bmsrp_query_object {
    use super::*;
    #[doc = "RestorePoint type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RestorePointQueryType")]
    pub enum RestorePointQueryType {
        Invalid,
        Full,
        Log,
        Differential,
        FullAndDifferential,
        All,
        Incremental,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RestorePointQueryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RestorePointQueryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RestorePointQueryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RestorePointQueryType", 0u32, "Invalid"),
                Self::Full => serializer.serialize_unit_variant("RestorePointQueryType", 1u32, "Full"),
                Self::Log => serializer.serialize_unit_variant("RestorePointQueryType", 2u32, "Log"),
                Self::Differential => serializer.serialize_unit_variant("RestorePointQueryType", 3u32, "Differential"),
                Self::FullAndDifferential => serializer.serialize_unit_variant("RestorePointQueryType", 4u32, "FullAndDifferential"),
                Self::All => serializer.serialize_unit_variant("RestorePointQueryType", 5u32, "All"),
                Self::Incremental => serializer.serialize_unit_variant("RestorePointQueryType", 6u32, "Incremental"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backup management usages of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupManagementUsage {
    #[doc = "Unit of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<backup_management_usage::Unit>,
    #[doc = "Quota period of usage."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Next reset time of usage."]
    #[serde(rename = "nextResetTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
    #[doc = "Current value of usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "Limit of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The name of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NameInfo>,
}
impl BackupManagementUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_management_usage {
    use super::*;
    #[doc = "Unit of the usage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "Bytes"),
                Self::Seconds => serializer.serialize_unit_variant("Unit", 2u32, "Seconds"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 3u32, "Percent"),
                Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 4u32, "CountPerSecond"),
                Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 5u32, "BytesPerSecond"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backup management usage for vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupManagementUsageList {
    #[doc = "The list of backup management usages for the given vault."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupManagementUsage>,
}
impl azure_core::Continuable for BackupManagementUsageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupManagementUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceConfig {
    #[doc = "Storage type"]
    #[serde(rename = "storageModelType", default, skip_serializing_if = "Option::is_none")]
    pub storage_model_type: Option<backup_resource_config::StorageModelType>,
    #[doc = "Storage type."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<backup_resource_config::StorageType>,
    #[doc = "Locked or Unlocked. Once a machine is registered against a resource, the storageTypeState is always Locked."]
    #[serde(rename = "storageTypeState", default, skip_serializing_if = "Option::is_none")]
    pub storage_type_state: Option<backup_resource_config::StorageTypeState>,
    #[doc = "Opt in details of Cross Region Restore feature."]
    #[serde(rename = "crossRegionRestoreFlag", default, skip_serializing_if = "Option::is_none")]
    pub cross_region_restore_flag: Option<bool>,
}
impl BackupResourceConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_resource_config {
    use super::*;
    #[doc = "Storage type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageModelType")]
    pub enum StorageModelType {
        Invalid,
        GeoRedundant,
        LocallyRedundant,
        ZoneRedundant,
        ReadAccessGeoZoneRedundant,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageModelType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageModelType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageModelType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("StorageModelType", 0u32, "Invalid"),
                Self::GeoRedundant => serializer.serialize_unit_variant("StorageModelType", 1u32, "GeoRedundant"),
                Self::LocallyRedundant => serializer.serialize_unit_variant("StorageModelType", 2u32, "LocallyRedundant"),
                Self::ZoneRedundant => serializer.serialize_unit_variant("StorageModelType", 3u32, "ZoneRedundant"),
                Self::ReadAccessGeoZoneRedundant => {
                    serializer.serialize_unit_variant("StorageModelType", 4u32, "ReadAccessGeoZoneRedundant")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Storage type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageType")]
    pub enum StorageType {
        Invalid,
        GeoRedundant,
        LocallyRedundant,
        ZoneRedundant,
        ReadAccessGeoZoneRedundant,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("StorageType", 0u32, "Invalid"),
                Self::GeoRedundant => serializer.serialize_unit_variant("StorageType", 1u32, "GeoRedundant"),
                Self::LocallyRedundant => serializer.serialize_unit_variant("StorageType", 2u32, "LocallyRedundant"),
                Self::ZoneRedundant => serializer.serialize_unit_variant("StorageType", 3u32, "ZoneRedundant"),
                Self::ReadAccessGeoZoneRedundant => serializer.serialize_unit_variant("StorageType", 4u32, "ReadAccessGeoZoneRedundant"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Locked or Unlocked. Once a machine is registered against a resource, the storageTypeState is always Locked."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageTypeState")]
    pub enum StorageTypeState {
        Invalid,
        Locked,
        Unlocked,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageTypeState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageTypeState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageTypeState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("StorageTypeState", 0u32, "Invalid"),
                Self::Locked => serializer.serialize_unit_variant("StorageTypeState", 1u32, "Locked"),
                Self::Unlocked => serializer.serialize_unit_variant("StorageTypeState", 2u32, "Unlocked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceConfigResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The resource storage details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupResourceConfig>,
}
impl BackupResourceConfigResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client script details for file / folder restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientScriptForConnect {
    #[doc = "File content of the client script for file / folder restore."]
    #[serde(rename = "scriptContent", default, skip_serializing_if = "Option::is_none")]
    pub script_content: Option<String>,
    #[doc = "File extension of the client script for file / folder restore - .ps1 , .sh , etc."]
    #[serde(rename = "scriptExtension", default, skip_serializing_if = "Option::is_none")]
    pub script_extension: Option<String>,
    #[doc = "OS type - Windows, Linux etc. for which this file / folder restore client script works."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "URL of Executable from where to source the content. If this is not null then ScriptContent should not be used"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Mandatory suffix that should be added to the name of script that is given for download to user.\r\nIf its null or empty then , ignore it."]
    #[serde(rename = "scriptNameSuffix", default, skip_serializing_if = "Option::is_none")]
    pub script_name_suffix: Option<String>,
}
impl ClientScriptForConnect {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CrossRegionRestoreRequest {
    #[serde(rename = "crossRegionRestoreAccessDetails", default, skip_serializing_if = "Option::is_none")]
    pub cross_region_restore_access_details: Option<CrrAccessToken>,
    #[doc = "Base class for restore request. Workload-specific restore requests are derived from this class."]
    #[serde(rename = "restoreRequest", default, skip_serializing_if = "Option::is_none")]
    pub restore_request: Option<RestoreRequest>,
}
impl CrossRegionRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CrossRegionRestoreRequestResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CrossRegionRestoreRequest>,
}
impl CrossRegionRestoreRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrrAccessToken {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[doc = "Access token used for authentication"]
    #[serde(rename = "accessTokenString", default, skip_serializing_if = "Option::is_none")]
    pub access_token_string: Option<String>,
    #[doc = "Subscription Id of the source vault"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource Group name of the source vault"]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "Resource Name of the source vault"]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Resource Id of the source vault"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Protected item container id"]
    #[serde(rename = "protectionContainerId", default, skip_serializing_if = "Option::is_none")]
    pub protection_container_id: Option<i64>,
    #[doc = "Recovery Point Id"]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "Recovery Point Time"]
    #[serde(rename = "recoveryPointTime", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_time: Option<String>,
    #[doc = "Container Unique name"]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Container Type"]
    #[serde(rename = "containerType", default, skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[doc = "Backup Management Type"]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<String>,
    #[doc = "Datasource Type"]
    #[serde(rename = "datasourceType", default, skip_serializing_if = "Option::is_none")]
    pub datasource_type: Option<String>,
    #[doc = "Datasource Friendly Name"]
    #[serde(rename = "datasourceName", default, skip_serializing_if = "Option::is_none")]
    pub datasource_name: Option<String>,
    #[doc = "Datasource Id"]
    #[serde(rename = "datasourceId", default, skip_serializing_if = "Option::is_none")]
    pub datasource_id: Option<String>,
    #[doc = "Datasource Container Unique Name"]
    #[serde(rename = "datasourceContainerName", default, skip_serializing_if = "Option::is_none")]
    pub datasource_container_name: Option<String>,
    #[doc = "CoordinatorServiceStampId to be used by BCM in restore call"]
    #[serde(rename = "coordinatorServiceStampId", default, skip_serializing_if = "Option::is_none")]
    pub coordinator_service_stamp_id: Option<String>,
    #[doc = "CoordinatorServiceStampUri to be used by BCM in restore call"]
    #[serde(rename = "coordinatorServiceStampUri", default, skip_serializing_if = "Option::is_none")]
    pub coordinator_service_stamp_uri: Option<String>,
    #[doc = "ProtectionServiceStampId to be used by BCM in restore call"]
    #[serde(rename = "protectionServiceStampId", default, skip_serializing_if = "Option::is_none")]
    pub protection_service_stamp_id: Option<String>,
    #[doc = "ProtectionServiceStampUri to be used by BCM in restore call"]
    #[serde(rename = "protectionServiceStampUri", default, skip_serializing_if = "Option::is_none")]
    pub protection_service_stamp_uri: Option<String>,
    #[doc = "Extended Information about the token like FileSpec etc."]
    #[serde(rename = "tokenExtendedInformation", default, skip_serializing_if = "Option::is_none")]
    pub token_extended_information: Option<String>,
    #[doc = "Recovery point Tier Information"]
    #[serde(rename = "rpTierInformation", default, skip_serializing_if = "Option::is_none")]
    pub rp_tier_information: Option<serde_json::Value>,
    #[doc = "Recovery point information: Original SA option"]
    #[serde(rename = "rpOriginalSAOption", default, skip_serializing_if = "Option::is_none")]
    pub rp_original_sa_option: Option<bool>,
    #[doc = "Recovery point information: Managed virtual machine"]
    #[serde(rename = "rpIsManagedVirtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub rp_is_managed_virtual_machine: Option<bool>,
    #[doc = "Recovery point information: VM size description"]
    #[serde(rename = "rpVMSizeDescription", default, skip_serializing_if = "Option::is_none")]
    pub rp_vm_size_description: Option<String>,
    #[doc = "Active region name of BMS Stamp"]
    #[serde(rename = "bMSActiveRegion", default, skip_serializing_if = "Option::is_none")]
    pub b_ms_active_region: Option<String>,
}
impl CrrAccessToken {
    pub fn new(object_type: String) -> Self {
        Self {
            object_type,
            access_token_string: None,
            subscription_id: None,
            resource_group_name: None,
            resource_name: None,
            resource_id: None,
            protection_container_id: None,
            recovery_point_id: None,
            recovery_point_time: None,
            container_name: None,
            container_type: None,
            backup_management_type: None,
            datasource_type: None,
            datasource_name: None,
            datasource_id: None,
            datasource_container_name: None,
            coordinator_service_stamp_id: None,
            coordinator_service_stamp_uri: None,
            protection_service_stamp_id: None,
            protection_service_stamp_uri: None,
            token_extended_information: None,
            rp_tier_information: None,
            rp_original_sa_option: None,
            rp_is_managed_virtual_machine: None,
            rp_vm_size_description: None,
            b_ms_active_region: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CrrAccessTokenResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CrrAccessToken>,
}
impl CrrAccessTokenResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request object for fetching CRR jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CrrJobRequest {
    #[doc = "Entire ARM resource id of the resource"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Job Name of the job to be fetched"]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}
impl CrrJobRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request object for fetching CRR jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CrrJobRequestResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Request object for fetching CRR jobs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CrrJobRequest>,
}
impl CrrJobRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional information on Backup engine specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Friendly name of the managed item"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Backup Management server protecting this backup item"]
    #[serde(rename = "backupEngineName", default, skip_serializing_if = "Option::is_none")]
    pub backup_engine_name: Option<String>,
    #[doc = "Protection state of the backup engine"]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<dpm_protected_item::ProtectionState>,
    #[doc = "Additional information of DPM Protected item."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<DpmProtectedItemExtendedInfo>,
}
impl DpmProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            friendly_name: None,
            backup_engine_name: None,
            protection_state: None,
            extended_info: None,
        }
    }
}
pub mod dpm_protected_item {
    use super::*;
    #[doc = "Protection state of the backup engine"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        #[serde(rename = "IRPending")]
        IrPending,
        Protected,
        ProtectionError,
        ProtectionStopped,
        ProtectionPaused,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionState", 0u32, "Invalid"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectionState", 1u32, "IRPending"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protected"),
                Self::ProtectionError => serializer.serialize_unit_variant("ProtectionState", 3u32, "ProtectionError"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionStopped"),
                Self::ProtectionPaused => serializer.serialize_unit_variant("ProtectionState", 5u32, "ProtectionPaused"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional information of DPM Protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DpmProtectedItemExtendedInfo {
    #[doc = "Attribute to provide information on various DBs."]
    #[serde(rename = "protectableObjectLoadPath", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_load_path: Option<serde_json::Value>,
    #[doc = "To check if backup item is disk protected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    #[doc = "To check if backup item is cloud protected."]
    #[serde(rename = "isPresentOnCloud", default, skip_serializing_if = "Option::is_none")]
    pub is_present_on_cloud: Option<bool>,
    #[doc = "Last backup status information on backup item."]
    #[serde(rename = "lastBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_status: Option<String>,
    #[doc = "Last refresh time on backup item."]
    #[serde(rename = "lastRefreshedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_refreshed_at: Option<time::OffsetDateTime>,
    #[doc = "Oldest cloud recovery point time."]
    #[serde(rename = "oldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "cloud recovery point count."]
    #[serde(rename = "recoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_count: Option<i32>,
    #[doc = "Oldest disk recovery point time."]
    #[serde(rename = "onPremiseOldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub on_premise_oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "latest disk recovery point time."]
    #[serde(rename = "onPremiseLatestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub on_premise_latest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "disk recovery point count."]
    #[serde(rename = "onPremiseRecoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub on_premise_recovery_point_count: Option<i32>,
    #[doc = "To check if backup item is collocated."]
    #[serde(rename = "isCollocated", default, skip_serializing_if = "Option::is_none")]
    pub is_collocated: Option<bool>,
    #[doc = "Protection group name of the backup item."]
    #[serde(rename = "protectionGroupName", default, skip_serializing_if = "Option::is_none")]
    pub protection_group_name: Option<String>,
    #[doc = "Used Disk storage in bytes."]
    #[serde(rename = "diskStorageUsedInBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_storage_used_in_bytes: Option<String>,
    #[doc = "total Disk storage in bytes."]
    #[serde(rename = "totalDiskStorageSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_disk_storage_size_in_bytes: Option<String>,
}
impl DpmProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskExclusionProperties {
    #[doc = "List of Disks' Logical Unit Numbers (LUN) to be used for VM Protection."]
    #[serde(rename = "diskLunList", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_lun_list: Vec<i32>,
    #[doc = "Flag to indicate whether DiskLunList is to be included/ excluded from backup."]
    #[serde(rename = "isInclusionList", default, skip_serializing_if = "Option::is_none")]
    pub is_inclusion_list: Option<bool>,
}
impl DiskExclusionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DiskInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DPM workload-specific error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DpmErrorInfo {
    #[doc = "Localized error string."]
    #[serde(rename = "errorString", default, skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[doc = "List of localized recommendations for above error code."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
impl DpmErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DPM workload-specific job object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmJob {
    #[serde(flatten)]
    pub job: Job,
    #[doc = "Time elapsed for job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "DPM server name managing the backup item or backup job."]
    #[serde(rename = "dpmServerName", default, skip_serializing_if = "Option::is_none")]
    pub dpm_server_name: Option<String>,
    #[doc = "Name of cluster/server protecting current backup item, if any."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Type of container."]
    #[serde(rename = "containerType", default, skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[doc = "Type of backup item."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[doc = "The state/actions applicable on this job like cancel/retry."]
    #[serde(rename = "actionsInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[doc = "The errors."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<DpmErrorInfo>,
    #[doc = "Additional information on the DPM workload-specific job."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<DpmJobExtendedInfo>,
}
impl DpmJob {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            duration: None,
            dpm_server_name: None,
            container_name: None,
            container_type: None,
            workload_type: None,
            actions_info: Vec::new(),
            error_details: Vec::new(),
            extended_info: None,
        }
    }
}
#[doc = "Additional information on the DPM workload-specific job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DpmJobExtendedInfo {
    #[doc = "List of tasks associated with this job."]
    #[serde(rename = "tasksList", default, skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<DpmJobTaskDetails>,
    #[doc = "The job properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[doc = "Non localized error message on job execution."]
    #[serde(rename = "dynamicErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
impl DpmJobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DPM workload-specific job task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DpmJobTaskDetails {
    #[doc = "The task display name."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Time elapsed for task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl DpmJobTaskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details needed if the VM was encrypted at the time of backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionDetails {
    #[doc = "Identifies whether this backup copy represents an encrypted VM at the time of backup."]
    #[serde(rename = "encryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub encryption_enabled: Option<bool>,
    #[doc = "Key Url."]
    #[serde(rename = "kekUrl", default, skip_serializing_if = "Option::is_none")]
    pub kek_url: Option<String>,
    #[doc = "Secret Url."]
    #[serde(rename = "secretKeyUrl", default, skip_serializing_if = "Option::is_none")]
    pub secret_key_url: Option<String>,
    #[doc = "ID of Key Vault where KEK is stored."]
    #[serde(rename = "kekVaultId", default, skip_serializing_if = "Option::is_none")]
    pub kek_vault_id: Option<String>,
    #[doc = "ID of Key Vault where Secret is stored."]
    #[serde(rename = "secretKeyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub secret_key_vault_id: Option<String>,
}
impl EncryptionDetails {
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
#[doc = "Error Detail class which encapsulates Code, Message and Recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error Message related to the Code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "List of recommendation strings."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extended Properties for Azure IaasVM Backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedProperties {
    #[serde(rename = "diskExclusionProperties", default, skip_serializing_if = "Option::is_none")]
    pub disk_exclusion_properties: Option<DiskExclusionProperties>,
}
impl ExtendedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Friendly name of the container."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Indicates consistency of policy object and policy applied to this backup item."]
    #[serde(rename = "policyState", default, skip_serializing_if = "Option::is_none")]
    pub policy_state: Option<String>,
    #[doc = "Backup state of this backup item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<generic_protected_item::ProtectionState>,
    #[doc = "Data Plane Service ID of the protected item."]
    #[serde(rename = "protectedItemId", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_id: Option<i64>,
    #[doc = "Loosely coupled (type, value) associations (example - parent of a protected item)"]
    #[serde(rename = "sourceAssociations", default, skip_serializing_if = "Option::is_none")]
    pub source_associations: Option<serde_json::Value>,
    #[doc = "Name of this backup item's fabric."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
}
impl GenericProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            friendly_name: None,
            policy_state: None,
            protection_state: None,
            protected_item_id: None,
            source_associations: None,
            fabric_name: None,
        }
    }
}
pub mod generic_protected_item {
    use super::*;
    #[doc = "Backup state of this backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        #[serde(rename = "IRPending")]
        IrPending,
        Protected,
        ProtectionError,
        ProtectionStopped,
        ProtectionPaused,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionState", 0u32, "Invalid"),
                Self::IrPending => serializer.serialize_unit_variant("ProtectionState", 1u32, "IRPending"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protected"),
                Self::ProtectionError => serializer.serialize_unit_variant("ProtectionState", 3u32, "ProtectionError"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionStopped"),
                Self::ProtectionPaused => serializer.serialize_unit_variant("ProtectionState", 5u32, "ProtectionPaused"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Generic backup copy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericRecoveryPoint {
    #[serde(flatten)]
    pub recovery_point: RecoveryPoint,
    #[doc = "Friendly name of the backup copy."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Type of the backup copy."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<String>,
    #[doc = "Time at which this backup copy was created."]
    #[serde(rename = "recoveryPointTime", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
    #[doc = "Additional information associated with this backup copy."]
    #[serde(rename = "recoveryPointAdditionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_additional_info: Option<String>,
}
impl GenericRecoveryPoint {
    pub fn new(recovery_point: RecoveryPoint) -> Self {
        Self {
            recovery_point,
            friendly_name: None,
            recovery_point_type: None,
            recovery_point_time: None,
            recovery_point_additional_info: None,
        }
    }
}
#[doc = "IaaS VM workload specific backup copy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaasVmRecoveryPoint {
    #[serde(flatten)]
    pub recovery_point: RecoveryPoint,
    #[doc = "Type of the backup copy."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<String>,
    #[doc = "Time at which this backup copy was created."]
    #[serde(rename = "recoveryPointTime", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
    #[doc = "Additional information associated with this backup copy."]
    #[serde(rename = "recoveryPointAdditionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_additional_info: Option<String>,
    #[doc = "Storage type of the VM whose backup copy is created."]
    #[serde(rename = "sourceVMStorageType", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_storage_type: Option<String>,
    #[doc = "Identifies whether the VM was encrypted when the backup copy is created."]
    #[serde(rename = "isSourceVMEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_source_vm_encrypted: Option<bool>,
    #[doc = "BEK is bitlocker key.\r\nKEK is encryption key for BEK\r\nIf the VM was encrypted then we will store following details :\r\n1. Secret(BEK) - Url + Backup Data + vaultId.\r\n2. Key(KEK) - Url + Backup Data + vaultId.\r\n3. EncryptionMechanism\r\nBEK and KEK can potentially have different vault ids."]
    #[serde(rename = "keyAndSecret", default, skip_serializing_if = "Option::is_none")]
    pub key_and_secret: Option<KeyAndSecretDetails>,
    #[doc = "Is the session to recover items from this backup copy still active."]
    #[serde(rename = "isInstantIlrSessionActive", default, skip_serializing_if = "Option::is_none")]
    pub is_instant_ilr_session_active: Option<bool>,
    #[doc = "Recovery point tier information."]
    #[serde(rename = "recoveryPointTierDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_point_tier_details: Vec<RecoveryPointTierInformation>,
    #[doc = "Whether VM is with Managed Disks"]
    #[serde(rename = "isManagedVirtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub is_managed_virtual_machine: Option<bool>,
    #[doc = "Virtual Machine Size"]
    #[serde(rename = "virtualMachineSize", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_size: Option<String>,
    #[doc = "Original Storage Account Option"]
    #[serde(rename = "originalStorageAccountOption", default, skip_serializing_if = "Option::is_none")]
    pub original_storage_account_option: Option<bool>,
    #[doc = "OS type"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Disk configuration"]
    #[serde(rename = "recoveryPointDiskConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_disk_configuration: Option<RecoveryPointDiskConfiguration>,
    #[doc = "Identifies the zone of the VM at the time of backup. Applicable only for zone-pinned Vms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Eligibility of RP to be moved to another tier"]
    #[serde(rename = "recoveryPointMoveReadinessInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_move_readiness_info: Option<serde_json::Value>,
}
impl IaasVmRecoveryPoint {
    pub fn new(recovery_point: RecoveryPoint) -> Self {
        Self {
            recovery_point,
            recovery_point_type: None,
            recovery_point_time: None,
            recovery_point_additional_info: None,
            source_vm_storage_type: None,
            is_source_vm_encrypted: None,
            key_and_secret: None,
            is_instant_ilr_session_active: None,
            recovery_point_tier_details: Vec::new(),
            is_managed_virtual_machine: None,
            virtual_machine_size: None,
            original_storage_account_option: None,
            os_type: None,
            recovery_point_disk_configuration: None,
            zones: Vec::new(),
            recovery_point_move_readiness_info: None,
        }
    }
}
#[doc = "IaaS VM workload-specific restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaasVmRestoreRequest {
    #[serde(flatten)]
    pub restore_request: RestoreRequest,
    #[doc = "ID of the backup copy to be recovered."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "Type of this recovery."]
    #[serde(rename = "recoveryType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_type: Option<iaas_vm_restore_request::RecoveryType>,
    #[doc = "Fully qualified ARM ID of the VM which is being recovered."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "This is the complete ARM Id of the VM that will be created.\r\nFor e.g. /subscriptions/{subId}/resourcegroups/{rg}/provider/Microsoft.Compute/virtualmachines/{vm}"]
    #[serde(rename = "targetVirtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub target_virtual_machine_id: Option<String>,
    #[doc = "This is the ARM Id of the resource group that you want to create for this Virtual machine and other artifacts.\r\nFor e.g. /subscriptions/{subId}/resourcegroups/{rg}"]
    #[serde(rename = "targetResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group_id: Option<String>,
    #[doc = "Fully qualified ARM ID of the storage account to which the VM has to be restored."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "This is the virtual network Id of the vnet that will be attached to the virtual machine.\r\nUser will be validated for join action permissions in the linked access."]
    #[serde(rename = "virtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_id: Option<String>,
    #[doc = "Subnet ID, is the subnet ID associated with the to be restored VM. For Classic VMs it would be\r\n{VnetID}/Subnet/{SubnetName} and, for the Azure Resource Manager VMs it would be ARM resource ID used to represent\r\nthe subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Fully qualified ARM ID of the domain name to be associated to the VM being restored. This applies only to Classic\r\nVirtual Machines."]
    #[serde(rename = "targetDomainNameId", default, skip_serializing_if = "Option::is_none")]
    pub target_domain_name_id: Option<String>,
    #[doc = "Region in which the virtual machine is restored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Affinity group associated to VM to be restored. Used only for Classic Compute Virtual Machines."]
    #[serde(rename = "affinityGroup", default, skip_serializing_if = "Option::is_none")]
    pub affinity_group: Option<String>,
    #[doc = "Should a new cloud service be created while restoring the VM. If this is false, VM will be restored to the same\r\ncloud service as it was at the time of backup."]
    #[serde(rename = "createNewCloudService", default, skip_serializing_if = "Option::is_none")]
    pub create_new_cloud_service: Option<bool>,
    #[doc = "Original Storage Account Option"]
    #[serde(rename = "originalStorageAccountOption", default, skip_serializing_if = "Option::is_none")]
    pub original_storage_account_option: Option<bool>,
    #[doc = "Details needed if the VM was encrypted at the time of backup."]
    #[serde(rename = "encryptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<EncryptionDetails>,
    #[doc = "List of Disk LUNs for partial restore"]
    #[serde(rename = "restoreDiskLunList", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_disk_lun_list: Vec<i32>,
    #[doc = "Flag to denote of an Unmanaged disk VM should be restored with Managed disks."]
    #[serde(rename = "restoreWithManagedDisks", default, skip_serializing_if = "Option::is_none")]
    pub restore_with_managed_disks: Option<bool>,
    #[doc = "DiskEncryptionSet's ID - needed if the VM needs to be encrypted at rest during restore with customer managed key."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "Target zone where the VM and its disks should be restored."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Encapsulates Managed Identity related information"]
    #[serde(rename = "identityInfo", default, skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<IdentityInfo>,
    #[doc = "IaaS VM workload specific restore details for restores using managed identity"]
    #[serde(rename = "identityBasedRestoreDetails", default, skip_serializing_if = "Option::is_none")]
    pub identity_based_restore_details: Option<IdentityBasedRestoreDetails>,
}
impl IaasVmRestoreRequest {
    pub fn new(restore_request: RestoreRequest) -> Self {
        Self {
            restore_request,
            recovery_point_id: None,
            recovery_type: None,
            source_resource_id: None,
            target_virtual_machine_id: None,
            target_resource_group_id: None,
            storage_account_id: None,
            virtual_network_id: None,
            subnet_id: None,
            target_domain_name_id: None,
            region: None,
            affinity_group: None,
            create_new_cloud_service: None,
            original_storage_account_option: None,
            encryption_details: None,
            restore_disk_lun_list: Vec::new(),
            restore_with_managed_disks: None,
            disk_encryption_set_id: None,
            zones: Vec::new(),
            identity_info: None,
            identity_based_restore_details: None,
        }
    }
}
pub mod iaas_vm_restore_request {
    use super::*;
    #[doc = "Type of this recovery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryType")]
    pub enum RecoveryType {
        Invalid,
        OriginalLocation,
        AlternateLocation,
        RestoreDisks,
        Offline,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RecoveryType", 0u32, "Invalid"),
                Self::OriginalLocation => serializer.serialize_unit_variant("RecoveryType", 1u32, "OriginalLocation"),
                Self::AlternateLocation => serializer.serialize_unit_variant("RecoveryType", 2u32, "AlternateLocation"),
                Self::RestoreDisks => serializer.serialize_unit_variant("RecoveryType", 3u32, "RestoreDisks"),
                Self::Offline => serializer.serialize_unit_variant("RecoveryType", 4u32, "Offline"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IaaS VM workload specific restore details for restores using managed identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityBasedRestoreDetails {
    #[doc = "Gets the class type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "Fully qualified ARM ID of the target storage account."]
    #[serde(rename = "targetStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_account_id: Option<String>,
}
impl IdentityBasedRestoreDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encapsulates Managed Identity related information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityInfo {
    #[doc = "To differentiate if the managed identity is system assigned or user assigned"]
    #[serde(rename = "isSystemAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub is_system_assigned_identity: Option<bool>,
    #[doc = "Managed Identity Resource Id\r\nOptional: Might not be required in the case of system assigned managed identity"]
    #[serde(rename = "managedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity_resource_id: Option<String>,
}
impl IdentityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target details for file / folder restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstantItemRecoveryTarget {
    #[doc = "List of client scripts."]
    #[serde(rename = "clientScripts", default, skip_serializing_if = "Vec::is_empty")]
    pub client_scripts: Vec<ClientScriptForConnect>,
}
impl InstantItemRecoveryTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines workload agnostic properties for a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[doc = "Friendly name of the entity on which the current job is executing."]
    #[serde(rename = "entityFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub entity_friendly_name: Option<String>,
    #[doc = "Backup management type to execute the current job."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<job::BackupManagementType>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Job status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "ActivityId of job."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "jobType")]
    pub job_type: String,
}
impl Job {
    pub fn new(job_type: String) -> Self {
        Self {
            entity_friendly_name: None,
            backup_management_type: None,
            operation: None,
            status: None,
            start_time: None,
            end_time: None,
            activity_id: None,
            job_type,
        }
    }
}
pub mod job {
    use super::*;
    #[doc = "Backup management type to execute the current job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupManagementType")]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupManagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupManagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupManagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupManagementType", 0u32, "Invalid"),
                Self::AzureIaasVm => serializer.serialize_unit_variant("BackupManagementType", 1u32, "AzureIaasVM"),
                Self::Mab => serializer.serialize_unit_variant("BackupManagementType", 2u32, "MAB"),
                Self::Dpm => serializer.serialize_unit_variant("BackupManagementType", 3u32, "DPM"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureBackupServer"),
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureSql"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureStorage"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureWorkload"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Filters to list the jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobQueryObject {
    #[doc = "Status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_query_object::Status>,
    #[doc = "Type of backup management for the job."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<job_query_object::BackupManagementType>,
    #[doc = "Type of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<job_query_object::Operation>,
    #[doc = "JobID represents the job uniquely."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Job has started at this time. Value is in UTC."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Job has ended at this time. Value is in UTC."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl JobQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_query_object {
    use super::*;
    #[doc = "Status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        InProgress,
        Completed,
        Failed,
        CompletedWithWarnings,
        Cancelled,
        Cancelling,
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
                Self::Invalid => serializer.serialize_unit_variant("Status", 0u32, "Invalid"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 5u32, "Cancelled"),
                Self::Cancelling => serializer.serialize_unit_variant("Status", 6u32, "Cancelling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of backup management for the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupManagementType")]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupManagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupManagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupManagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupManagementType", 0u32, "Invalid"),
                Self::AzureIaasVm => serializer.serialize_unit_variant("BackupManagementType", 1u32, "AzureIaasVM"),
                Self::Mab => serializer.serialize_unit_variant("BackupManagementType", 2u32, "MAB"),
                Self::Dpm => serializer.serialize_unit_variant("BackupManagementType", 3u32, "DPM"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureBackupServer"),
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureSql"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureStorage"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureWorkload"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operation")]
    pub enum Operation {
        Invalid,
        Register,
        UnRegister,
        ConfigureBackup,
        Backup,
        Restore,
        DisableBackup,
        DeleteBackupData,
        CrossRegionRestore,
        Undelete,
        UpdateCustomerManagedKey,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Operation", 0u32, "Invalid"),
                Self::Register => serializer.serialize_unit_variant("Operation", 1u32, "Register"),
                Self::UnRegister => serializer.serialize_unit_variant("Operation", 2u32, "UnRegister"),
                Self::ConfigureBackup => serializer.serialize_unit_variant("Operation", 3u32, "ConfigureBackup"),
                Self::Backup => serializer.serialize_unit_variant("Operation", 4u32, "Backup"),
                Self::Restore => serializer.serialize_unit_variant("Operation", 5u32, "Restore"),
                Self::DisableBackup => serializer.serialize_unit_variant("Operation", 6u32, "DisableBackup"),
                Self::DeleteBackupData => serializer.serialize_unit_variant("Operation", 7u32, "DeleteBackupData"),
                Self::CrossRegionRestore => serializer.serialize_unit_variant("Operation", 8u32, "CrossRegionRestore"),
                Self::Undelete => serializer.serialize_unit_variant("Operation", 9u32, "Undelete"),
                Self::UpdateCustomerManagedKey => serializer.serialize_unit_variant("Operation", 10u32, "UpdateCustomerManagedKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines workload agnostic properties for a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defines workload agnostic properties for a job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Job>,
}
impl JobResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Job resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResource>,
}
impl azure_core::Continuable for JobResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl JobResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KEK is encryption key for BEK."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KekDetails {
    #[doc = "Key is KEK."]
    #[serde(rename = "keyUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_url: Option<String>,
    #[doc = "Key Vault ID where this Key is stored."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "KEK data."]
    #[serde(rename = "keyBackupData", default, skip_serializing_if = "Option::is_none")]
    pub key_backup_data: Option<String>,
}
impl KekDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KPI Resource Health Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KpiResourceHealthDetails {
    #[doc = "Resource Health Status"]
    #[serde(rename = "resourceHealthStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_health_status: Option<kpi_resource_health_details::ResourceHealthStatus>,
    #[doc = "Resource Health Status"]
    #[serde(rename = "resourceHealthDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_health_details: Vec<ResourceHealthDetails>,
}
impl KpiResourceHealthDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kpi_resource_health_details {
    use super::*;
    #[doc = "Resource Health Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceHealthStatus")]
    pub enum ResourceHealthStatus {
        Healthy,
        TransientDegraded,
        PersistentDegraded,
        TransientUnhealthy,
        PersistentUnhealthy,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceHealthStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceHealthStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceHealthStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("ResourceHealthStatus", 0u32, "Healthy"),
                Self::TransientDegraded => serializer.serialize_unit_variant("ResourceHealthStatus", 1u32, "TransientDegraded"),
                Self::PersistentDegraded => serializer.serialize_unit_variant("ResourceHealthStatus", 2u32, "PersistentDegraded"),
                Self::TransientUnhealthy => serializer.serialize_unit_variant("ResourceHealthStatus", 3u32, "TransientUnhealthy"),
                Self::PersistentUnhealthy => serializer.serialize_unit_variant("ResourceHealthStatus", 4u32, "PersistentUnhealthy"),
                Self::Invalid => serializer.serialize_unit_variant("ResourceHealthStatus", 5u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "BEK is bitlocker key.\r\nKEK is encryption key for BEK\r\nIf the VM was encrypted then we will store following details :\r\n1. Secret(BEK) - Url + Backup Data + vaultId.\r\n2. Key(KEK) - Url + Backup Data + vaultId.\r\n3. EncryptionMechanism\r\nBEK and KEK can potentially have different vault ids."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyAndSecretDetails {
    #[doc = "KEK is encryption key for BEK."]
    #[serde(rename = "kekDetails", default, skip_serializing_if = "Option::is_none")]
    pub kek_details: Option<KekDetails>,
    #[doc = "BEK is bitlocker encryption key."]
    #[serde(rename = "bekDetails", default, skip_serializing_if = "Option::is_none")]
    pub bek_details: Option<BekDetails>,
    #[doc = "Encryption mechanism: None/ SinglePass/ DoublePass"]
    #[serde(rename = "encryptionMechanism", default, skip_serializing_if = "Option::is_none")]
    pub encryption_mechanism: Option<String>,
}
impl KeyAndSecretDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MAB workload-specific error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MabErrorInfo {
    #[doc = "Localized error string."]
    #[serde(rename = "errorString", default, skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[doc = "List of localized recommendations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
impl MabErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MAB workload-specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabFileFolderProtectedItem {
    #[serde(flatten)]
    pub protected_item: ProtectedItem,
    #[doc = "Friendly name of this backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Name of the computer associated with this backup item."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "Status of last backup operation."]
    #[serde(rename = "lastBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_status: Option<String>,
    #[doc = "Timestamp of the last backup operation on this backup item."]
    #[serde(rename = "lastBackupTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_backup_time: Option<time::OffsetDateTime>,
    #[doc = "Protected, ProtectionStopped, IRPending or ProtectionError"]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<String>,
    #[doc = "Sync time for deferred deletion in UTC"]
    #[serde(rename = "deferredDeleteSyncTimeInUTC", default, skip_serializing_if = "Option::is_none")]
    pub deferred_delete_sync_time_in_utc: Option<i64>,
    #[doc = "Additional information on the backed up item."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<MabFileFolderProtectedItemExtendedInfo>,
}
impl MabFileFolderProtectedItem {
    pub fn new(protected_item: ProtectedItem) -> Self {
        Self {
            protected_item,
            friendly_name: None,
            computer_name: None,
            last_backup_status: None,
            last_backup_time: None,
            protection_state: None,
            deferred_delete_sync_time_in_utc: None,
            extended_info: None,
        }
    }
}
#[doc = "Additional information on the backed up item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MabFileFolderProtectedItemExtendedInfo {
    #[doc = "Last time when the agent data synced to service."]
    #[serde(rename = "lastRefreshedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_refreshed_at: Option<time::OffsetDateTime>,
    #[doc = "The oldest backup copy available."]
    #[serde(rename = "oldestRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub oldest_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "Number of backup copies associated with the backup item."]
    #[serde(rename = "recoveryPointCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_count: Option<i32>,
}
impl MabFileFolderProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MAB workload-specific job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabJob {
    #[serde(flatten)]
    pub job: Job,
    #[doc = "Time taken by job to run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The state/actions applicable on jobs like cancel/retry."]
    #[serde(rename = "actionsInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[doc = "Name of server protecting the DS."]
    #[serde(rename = "mabServerName", default, skip_serializing_if = "Option::is_none")]
    pub mab_server_name: Option<String>,
    #[doc = "Server type of MAB container."]
    #[serde(rename = "mabServerType", default, skip_serializing_if = "Option::is_none")]
    pub mab_server_type: Option<mab_job::MabServerType>,
    #[doc = "Workload type of backup item."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<mab_job::WorkloadType>,
    #[doc = "The errors."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<MabErrorInfo>,
    #[doc = "Additional information for the MAB workload-specific job."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<MabJobExtendedInfo>,
}
impl MabJob {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            duration: None,
            actions_info: Vec::new(),
            mab_server_name: None,
            mab_server_type: None,
            workload_type: None,
            error_details: Vec::new(),
            extended_info: None,
        }
    }
}
pub mod mab_job {
    use super::*;
    #[doc = "Server type of MAB container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MabServerType")]
    pub enum MabServerType {
        Invalid,
        Unknown,
        #[serde(rename = "IaasVMContainer")]
        IaasVmContainer,
        #[serde(rename = "IaasVMServiceContainer")]
        IaasVmServiceContainer,
        #[serde(rename = "DPMContainer")]
        DpmContainer,
        AzureBackupServerContainer,
        #[serde(rename = "MABContainer")]
        MabContainer,
        Cluster,
        AzureSqlContainer,
        Windows,
        VCenter,
        #[serde(rename = "VMAppContainer")]
        VmAppContainer,
        #[serde(rename = "SQLAGWorkLoadContainer")]
        SqlagWorkLoadContainer,
        StorageContainer,
        GenericContainer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MabServerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MabServerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MabServerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("MabServerType", 0u32, "Invalid"),
                Self::Unknown => serializer.serialize_unit_variant("MabServerType", 1u32, "Unknown"),
                Self::IaasVmContainer => serializer.serialize_unit_variant("MabServerType", 2u32, "IaasVMContainer"),
                Self::IaasVmServiceContainer => serializer.serialize_unit_variant("MabServerType", 3u32, "IaasVMServiceContainer"),
                Self::DpmContainer => serializer.serialize_unit_variant("MabServerType", 4u32, "DPMContainer"),
                Self::AzureBackupServerContainer => serializer.serialize_unit_variant("MabServerType", 5u32, "AzureBackupServerContainer"),
                Self::MabContainer => serializer.serialize_unit_variant("MabServerType", 6u32, "MABContainer"),
                Self::Cluster => serializer.serialize_unit_variant("MabServerType", 7u32, "Cluster"),
                Self::AzureSqlContainer => serializer.serialize_unit_variant("MabServerType", 8u32, "AzureSqlContainer"),
                Self::Windows => serializer.serialize_unit_variant("MabServerType", 9u32, "Windows"),
                Self::VCenter => serializer.serialize_unit_variant("MabServerType", 10u32, "VCenter"),
                Self::VmAppContainer => serializer.serialize_unit_variant("MabServerType", 11u32, "VMAppContainer"),
                Self::SqlagWorkLoadContainer => serializer.serialize_unit_variant("MabServerType", 12u32, "SQLAGWorkLoadContainer"),
                Self::StorageContainer => serializer.serialize_unit_variant("MabServerType", 13u32, "StorageContainer"),
                Self::GenericContainer => serializer.serialize_unit_variant("MabServerType", 14u32, "GenericContainer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Workload type of backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadType")]
    pub enum WorkloadType {
        Invalid,
        #[serde(rename = "VM")]
        Vm,
        FileFolder,
        AzureSqlDb,
        #[serde(rename = "SQLDB")]
        Sqldb,
        Exchange,
        Sharepoint,
        #[serde(rename = "VMwareVM")]
        VMwareVm,
        SystemState,
        Client,
        GenericDataSource,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        AzureFileShare,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("WorkloadType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("WorkloadType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("WorkloadType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("WorkloadType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("WorkloadType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("WorkloadType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("WorkloadType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("WorkloadType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("WorkloadType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("WorkloadType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("WorkloadType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("WorkloadType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("WorkloadType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("WorkloadType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("WorkloadType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional information for the MAB workload-specific job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MabJobExtendedInfo {
    #[doc = "List of tasks for this job."]
    #[serde(rename = "tasksList", default, skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<MabJobTaskDetails>,
    #[doc = "The job properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[doc = "Non localized error message specific to this job."]
    #[serde(rename = "dynamicErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
impl MabJobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MAB workload-specific job task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MabJobTaskDetails {
    #[doc = "The task display name."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Time elapsed for task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl MabJobTaskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameInfo {
    #[doc = "Value of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized value of usage."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl NameInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewErrorResponse {
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<new_error_response::Error>,
}
impl azure_core::Continuable for NewErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl NewErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod new_error_response {
    use super::*;
    #[doc = "The error object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
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
        pub details: Vec<NewErrorResponse>,
        #[doc = "The error additional info."]
        #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
        pub additional_info: Vec<ErrorAdditionalInfo>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "ID of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_status::Status>,
    #[doc = "Operation start time. Format: ISO-8601."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Operation end time. Format: ISO-8601."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Error information associated with operation status call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationStatusError>,
    #[doc = "Base class for additional information of operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationStatusExtendedInfo>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_status {
    use super::*;
    #[doc = "Operation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Invalid => serializer.serialize_unit_variant("Status", 0u32, "Invalid"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error information associated with operation status call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusError {
    #[doc = "Error code of the operation failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message displayed if the operation failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for additional information of operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusExtendedInfo {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl OperationStatusExtendedInfo {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Operation status job extended info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusJobExtendedInfo {
    #[serde(flatten)]
    pub operation_status_extended_info: OperationStatusExtendedInfo,
    #[doc = "ID of the job created for this protected item."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}
impl OperationStatusJobExtendedInfo {
    pub fn new(operation_status_extended_info: OperationStatusExtendedInfo) -> Self {
        Self {
            operation_status_extended_info,
            job_id: None,
        }
    }
}
#[doc = "Operation status extended info for list of jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusJobsExtendedInfo {
    #[serde(flatten)]
    pub operation_status_extended_info: OperationStatusExtendedInfo,
    #[doc = "IDs of the jobs created for the protected item."]
    #[serde(rename = "jobIds", default, skip_serializing_if = "Vec::is_empty")]
    pub job_ids: Vec<String>,
    #[doc = "Stores all the failed jobs along with the corresponding error codes."]
    #[serde(rename = "failedJobsError", default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs_error: Option<serde_json::Value>,
}
impl OperationStatusJobsExtendedInfo {
    pub fn new(operation_status_extended_info: OperationStatusExtendedInfo) -> Self {
        Self {
            operation_status_extended_info,
            job_ids: Vec::new(),
            failed_jobs_error: None,
        }
    }
}
#[doc = "Operation status extended info for ILR provision action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusProvisionIlrExtendedInfo {
    #[serde(flatten)]
    pub operation_status_extended_info: OperationStatusExtendedInfo,
    #[doc = "Target details for file / folder restore."]
    #[serde(rename = "recoveryTarget", default, skip_serializing_if = "Option::is_none")]
    pub recovery_target: Option<InstantItemRecoveryTarget>,
}
impl OperationStatusProvisionIlrExtendedInfo {
    pub fn new(operation_status_extended_info: OperationStatusExtendedInfo) -> Self {
        Self {
            operation_status_extended_info,
            recovery_target: None,
        }
    }
}
#[doc = "Operation status extended info for Updated Recovery Point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusRecoveryPointExtendedInfo {
    #[serde(flatten)]
    pub operation_status_extended_info: OperationStatusExtendedInfo,
    #[doc = "Base class for backup copies. Workload-specific backup copies are derived from this class."]
    #[serde(rename = "updatedRecoveryPoint", default, skip_serializing_if = "Option::is_none")]
    pub updated_recovery_point: Option<RecoveryPoint>,
    #[doc = "In case the share is in soft-deleted state, populate this field with deleted backup item"]
    #[serde(rename = "deletedBackupItemVersion", default, skip_serializing_if = "Option::is_none")]
    pub deleted_backup_item_version: Option<String>,
}
impl OperationStatusRecoveryPointExtendedInfo {
    pub fn new(operation_status_extended_info: OperationStatusExtendedInfo) -> Self {
        Self {
            operation_status_extended_info,
            updated_recovery_point: None,
            deleted_backup_item_version: None,
        }
    }
}
#[doc = "Provides details for log ranges"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PointInTimeRange {
    #[doc = "Start time of the time range for log recovery."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the time range for log recovery."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl PointInTimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectedItem {
    #[doc = "backup item type."]
    #[serde(rename = "protectedItemType")]
    pub protected_item_type: String,
    #[doc = "Type of backup management for the backed up item."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protected_item::BackupManagementType>,
    #[doc = "Type of workload this item represents."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<protected_item::WorkloadType>,
    #[doc = "Unique name of container"]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "ARM ID of the resource to be backed up."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "ID of the backup policy with which this item is backed up."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Timestamp when the last (latest) backup copy was created for this backup item."]
    #[serde(rename = "lastRecoveryPoint", default, with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point: Option<time::OffsetDateTime>,
    #[doc = "Name of the backup set the backup item belongs to"]
    #[serde(rename = "backupSetName", default, skip_serializing_if = "Option::is_none")]
    pub backup_set_name: Option<String>,
    #[doc = "Create mode to indicate recovery of existing soft deleted data source or creation of new data source."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<protected_item::CreateMode>,
    #[doc = "Time for deferred deletion in UTC"]
    #[serde(rename = "deferredDeleteTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub deferred_delete_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "Flag to identify whether the DS is scheduled for deferred delete"]
    #[serde(rename = "isScheduledForDeferredDelete", default, skip_serializing_if = "Option::is_none")]
    pub is_scheduled_for_deferred_delete: Option<bool>,
    #[doc = "Time remaining before the DS marked for deferred delete is permanently deleted"]
    #[serde(rename = "deferredDeleteTimeRemaining", default, skip_serializing_if = "Option::is_none")]
    pub deferred_delete_time_remaining: Option<String>,
    #[doc = "Flag to identify whether the deferred deleted DS is to be purged soon"]
    #[serde(rename = "isDeferredDeleteScheduleUpcoming", default, skip_serializing_if = "Option::is_none")]
    pub is_deferred_delete_schedule_upcoming: Option<bool>,
    #[doc = "Flag to identify that deferred deleted DS is to be moved into Pause state"]
    #[serde(rename = "isRehydrate", default, skip_serializing_if = "Option::is_none")]
    pub is_rehydrate: Option<bool>,
    #[doc = "ResourceGuardOperationRequests on which LAC check will be performed"]
    #[serde(rename = "resourceGuardOperationRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operation_requests: Vec<String>,
}
impl ProtectedItem {
    pub fn new(protected_item_type: String) -> Self {
        Self {
            protected_item_type,
            backup_management_type: None,
            workload_type: None,
            container_name: None,
            source_resource_id: None,
            policy_id: None,
            last_recovery_point: None,
            backup_set_name: None,
            create_mode: None,
            deferred_delete_time_in_utc: None,
            is_scheduled_for_deferred_delete: None,
            deferred_delete_time_remaining: None,
            is_deferred_delete_schedule_upcoming: None,
            is_rehydrate: None,
            resource_guard_operation_requests: Vec::new(),
        }
    }
}
pub mod protected_item {
    use super::*;
    #[doc = "Type of backup management for the backed up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupManagementType")]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupManagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupManagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupManagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupManagementType", 0u32, "Invalid"),
                Self::AzureIaasVm => serializer.serialize_unit_variant("BackupManagementType", 1u32, "AzureIaasVM"),
                Self::Mab => serializer.serialize_unit_variant("BackupManagementType", 2u32, "MAB"),
                Self::Dpm => serializer.serialize_unit_variant("BackupManagementType", 3u32, "DPM"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureBackupServer"),
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureSql"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureStorage"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureWorkload"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of workload this item represents."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadType")]
    pub enum WorkloadType {
        Invalid,
        #[serde(rename = "VM")]
        Vm,
        FileFolder,
        AzureSqlDb,
        #[serde(rename = "SQLDB")]
        Sqldb,
        Exchange,
        Sharepoint,
        #[serde(rename = "VMwareVM")]
        VMwareVm,
        SystemState,
        Client,
        GenericDataSource,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        AzureFileShare,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("WorkloadType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("WorkloadType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("WorkloadType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("WorkloadType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("WorkloadType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("WorkloadType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("WorkloadType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("WorkloadType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("WorkloadType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("WorkloadType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("WorkloadType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("WorkloadType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("WorkloadType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("WorkloadType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("WorkloadType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Create mode to indicate recovery of existing soft deleted data source or creation of new data source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Invalid,
        Default,
        Recover,
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
                Self::Invalid => serializer.serialize_unit_variant("CreateMode", 0u32, "Invalid"),
                Self::Default => serializer.serialize_unit_variant("CreateMode", 1u32, "Default"),
                Self::Recover => serializer.serialize_unit_variant("CreateMode", 2u32, "Recover"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Filters to list backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedItemQueryObject {
    #[doc = "Health State for the backed up item."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<protected_item_query_object::HealthState>,
    #[doc = "Backup management type for the backed up item."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protected_item_query_object::BackupManagementType>,
    #[doc = "Type of workload this item represents."]
    #[serde(rename = "itemType", default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<protected_item_query_object::ItemType>,
    #[doc = "Backup policy name associated with the backup item."]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Name of the container."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Backup Engine name"]
    #[serde(rename = "backupEngineName", default, skip_serializing_if = "Option::is_none")]
    pub backup_engine_name: Option<String>,
    #[doc = "Friendly name of protected item"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Name of the fabric."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "Name of the backup set."]
    #[serde(rename = "backupSetName", default, skip_serializing_if = "Option::is_none")]
    pub backup_set_name: Option<String>,
}
impl ProtectedItemQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod protected_item_query_object {
    use super::*;
    #[doc = "Health State for the backed up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HealthState")]
    pub enum HealthState {
        Passed,
        ActionRequired,
        ActionSuggested,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HealthState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HealthState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HealthState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Passed => serializer.serialize_unit_variant("HealthState", 0u32, "Passed"),
                Self::ActionRequired => serializer.serialize_unit_variant("HealthState", 1u32, "ActionRequired"),
                Self::ActionSuggested => serializer.serialize_unit_variant("HealthState", 2u32, "ActionSuggested"),
                Self::Invalid => serializer.serialize_unit_variant("HealthState", 3u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Backup management type for the backed up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupManagementType")]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupManagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupManagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupManagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupManagementType", 0u32, "Invalid"),
                Self::AzureIaasVm => serializer.serialize_unit_variant("BackupManagementType", 1u32, "AzureIaasVM"),
                Self::Mab => serializer.serialize_unit_variant("BackupManagementType", 2u32, "MAB"),
                Self::Dpm => serializer.serialize_unit_variant("BackupManagementType", 3u32, "DPM"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureBackupServer"),
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureSql"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureStorage"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureWorkload"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of workload this item represents."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ItemType")]
    pub enum ItemType {
        Invalid,
        #[serde(rename = "VM")]
        Vm,
        FileFolder,
        AzureSqlDb,
        #[serde(rename = "SQLDB")]
        Sqldb,
        Exchange,
        Sharepoint,
        #[serde(rename = "VMwareVM")]
        VMwareVm,
        SystemState,
        Client,
        GenericDataSource,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        AzureFileShare,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ItemType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ItemType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ItemType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ItemType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("ItemType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("ItemType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("ItemType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("ItemType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("ItemType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("ItemType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("ItemType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("ItemType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("ItemType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("ItemType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("ItemType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("ItemType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("ItemType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("ItemType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedItemResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectedItem>,
}
impl ProtectedItemResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ProtectedItem resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedItemResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectedItemResource>,
}
impl azure_core::Continuable for ProtectedItemResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProtectedItemResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup copies. Workload-specific backup copies are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPoint {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl RecoveryPoint {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Disk configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointDiskConfiguration {
    #[doc = "Number of disks included in backup"]
    #[serde(rename = "numberOfDisksIncludedInBackup", default, skip_serializing_if = "Option::is_none")]
    pub number_of_disks_included_in_backup: Option<i32>,
    #[doc = "Number of disks attached to the VM"]
    #[serde(rename = "numberOfDisksAttachedToVm", default, skip_serializing_if = "Option::is_none")]
    pub number_of_disks_attached_to_vm: Option<i32>,
    #[doc = "Information of disks included in backup"]
    #[serde(rename = "includedDiskList", default, skip_serializing_if = "Vec::is_empty")]
    pub included_disk_list: Vec<DiskInformation>,
    #[doc = "Information of disks excluded from backup"]
    #[serde(rename = "excludedDiskList", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_disk_list: Vec<DiskInformation>,
}
impl RecoveryPointDiskConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointMoveReadinessInfo {
    #[serde(rename = "isReadyForMove", default, skip_serializing_if = "Option::is_none")]
    pub is_ready_for_move: Option<bool>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}
impl RecoveryPointMoveReadinessInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup copies. Workload-specific backup copies are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup copies. Workload-specific backup copies are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoveryPoint>,
}
impl RecoveryPointResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RecoveryPoint resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecoveryPointResource>,
}
impl azure_core::Continuable for RecoveryPointResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RecoveryPointResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery point tier information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointTierInformation {
    #[doc = "Recovery point tier type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<recovery_point_tier_information::Type>,
    #[doc = "Recovery point tier status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<recovery_point_tier_information::Status>,
    #[doc = "Recovery point tier status."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<serde_json::Value>,
}
impl RecoveryPointTierInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recovery_point_tier_information {
    use super::*;
    #[doc = "Recovery point tier type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Invalid,
        #[serde(rename = "InstantRP")]
        InstantRp,
        #[serde(rename = "HardenedRP")]
        HardenedRp,
        #[serde(rename = "ArchivedRP")]
        ArchivedRp,
    }
    #[doc = "Recovery point tier status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Invalid,
        Valid,
        Disabled,
        Deleted,
        Rehydrated,
    }
}
#[doc = "ARM Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Optional ETag."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health Details for backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceHealthDetails {
    #[doc = "Health Code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[doc = "Health Title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Health Message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Health Recommended Actions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
impl ResourceHealthDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base for all lists of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceList {
    #[doc = "The uri to fetch the next page of resources. Call ListNext() fetches next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restore file specs like file path, type and target folder path info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreFileSpecs {
    #[doc = "Source File/Folder path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Indicates what the Path variable stands for"]
    #[serde(rename = "fileSpecType", default, skip_serializing_if = "Option::is_none")]
    pub file_spec_type: Option<String>,
    #[doc = "Destination folder path in target FileShare"]
    #[serde(rename = "targetFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub target_folder_path: Option<String>,
}
impl RestoreFileSpecs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for restore request. Workload-specific restore requests are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreRequest {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl RestoreRequest {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "SQLDataDirectory info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDataDirectory {
    #[doc = "Type of data directory mapping"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sql_data_directory::Type>,
    #[doc = "File path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Logical name of the file"]
    #[serde(rename = "logicalName", default, skip_serializing_if = "Option::is_none")]
    pub logical_name: Option<String>,
}
impl SqlDataDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_data_directory {
    use super::*;
    #[doc = "Type of data directory mapping"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Invalid,
        Data,
        Log,
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
                Self::Invalid => serializer.serialize_unit_variant("Type", 0u32, "Invalid"),
                Self::Data => serializer.serialize_unit_variant("Type", 1u32, "Data"),
                Self::Log => serializer.serialize_unit_variant("Type", 2u32, "Log"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Encapsulates information regarding data directory"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDataDirectoryMapping {
    #[doc = "Type of data directory mapping"]
    #[serde(rename = "mappingType", default, skip_serializing_if = "Option::is_none")]
    pub mapping_type: Option<sql_data_directory_mapping::MappingType>,
    #[doc = "Restore source logical name path"]
    #[serde(rename = "sourceLogicalName", default, skip_serializing_if = "Option::is_none")]
    pub source_logical_name: Option<String>,
    #[doc = "Restore source path"]
    #[serde(rename = "sourcePath", default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[doc = "Target path"]
    #[serde(rename = "targetPath", default, skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
}
impl SqlDataDirectoryMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_data_directory_mapping {
    use super::*;
    #[doc = "Type of data directory mapping"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MappingType")]
    pub enum MappingType {
        Invalid,
        Data,
        Log,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MappingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MappingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MappingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("MappingType", 0u32, "Invalid"),
                Self::Data => serializer.serialize_unit_variant("MappingType", 1u32, "Data"),
                Self::Log => serializer.serialize_unit_variant("MappingType", 2u32, "Log"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Target Azure File Share Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetAfsRestoreInfo {
    #[doc = "File share name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Target file share resource ARM ID"]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
}
impl TargetAfsRestoreInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about target workload during restore operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetRestoreInfo {
    #[doc = "Can Overwrite if Target DataBase already exists"]
    #[serde(rename = "overwriteOption", default, skip_serializing_if = "Option::is_none")]
    pub overwrite_option: Option<target_restore_info::OverwriteOption>,
    #[doc = "Resource Id name of the container in which Target DataBase resides"]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[doc = "Database name InstanceName/DataBaseName for SQL or System/DbName for SAP Hana"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Target directory location for restore as files."]
    #[serde(rename = "targetDirectoryForFileRestore", default, skip_serializing_if = "Option::is_none")]
    pub target_directory_for_file_restore: Option<String>,
}
impl TargetRestoreInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_restore_info {
    use super::*;
    #[doc = "Can Overwrite if Target DataBase already exists"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OverwriteOption")]
    pub enum OverwriteOption {
        Invalid,
        FailOnConflict,
        Overwrite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OverwriteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OverwriteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OverwriteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("OverwriteOption", 0u32, "Invalid"),
                Self::FailOnConflict => serializer.serialize_unit_variant("OverwriteOption", 1u32, "FailOnConflict"),
                Self::Overwrite => serializer.serialize_unit_variant("OverwriteOption", 2u32, "Overwrite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadCrrAccessToken {
    #[serde(flatten)]
    pub crr_access_token: CrrAccessToken,
    #[serde(rename = "protectableObjectUniqueName", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_unique_name: Option<String>,
    #[serde(rename = "protectableObjectFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_friendly_name: Option<String>,
    #[serde(rename = "protectableObjectWorkloadType", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_workload_type: Option<String>,
    #[serde(rename = "protectableObjectProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_protection_state: Option<String>,
    #[serde(rename = "protectableObjectContainerHostOsName", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_container_host_os_name: Option<String>,
    #[serde(
        rename = "protectableObjectParentLogicalContainerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protectable_object_parent_logical_container_name: Option<String>,
    #[doc = "Container Id"]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[doc = "Policy Name"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Policy Id"]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}
impl WorkloadCrrAccessToken {
    pub fn new(crr_access_token: CrrAccessToken) -> Self {
        Self {
            crr_access_token,
            protectable_object_unique_name: None,
            protectable_object_friendly_name: None,
            protectable_object_workload_type: None,
            protectable_object_protection_state: None,
            protectable_object_container_host_os_name: None,
            protectable_object_parent_logical_container_name: None,
            container_id: None,
            policy_name: None,
            policy_id: None,
        }
    }
}
