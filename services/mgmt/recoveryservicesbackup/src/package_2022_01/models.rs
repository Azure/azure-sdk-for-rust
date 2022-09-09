#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure backup goal feature specific request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupGoalFeatureSupportRequest {
    #[serde(flatten)]
    pub feature_support_request: FeatureSupportRequest,
}
impl AzureBackupGoalFeatureSupportRequest {
    pub fn new(feature_support_request: FeatureSupportRequest) -> Self {
        Self { feature_support_request }
    }
}
#[doc = "AzureBackupServer (DPMVenus) workload-specific protection container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupServerContainer {
    #[serde(flatten)]
    pub dpm_container: DpmContainer,
}
impl AzureBackupServerContainer {
    pub fn new(dpm_container: DpmContainer) -> Self {
        Self { dpm_container }
    }
}
#[doc = "Backup engine type when Azure Backup Server is used to manage the backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupServerEngine {
    #[serde(flatten)]
    pub backup_engine_base: BackupEngineBase,
}
impl AzureBackupServerEngine {
    pub fn new(backup_engine_base: BackupEngineBase) -> Self {
        Self { backup_engine_base }
    }
}
#[doc = "AzureFileShare workload-specific backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareBackupRequest {
    #[serde(flatten)]
    pub backup_request: BackupRequest,
    #[doc = "Backup copy will expire after the time specified (UTC)."]
    #[serde(rename = "recoveryPointExpiryTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_expiry_time_in_utc: Option<time::OffsetDateTime>,
}
impl AzureFileShareBackupRequest {
    pub fn new(backup_request: BackupRequest) -> Self {
        Self {
            backup_request,
            recovery_point_expiry_time_in_utc: None,
        }
    }
}
#[doc = "Protectable item for Azure Fileshare workloads."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareProtectableItem {
    #[serde(flatten)]
    pub workload_protectable_item: WorkloadProtectableItem,
    #[doc = "Full Fabric ID of container to which this protectable item belongs. For example, ARM ID."]
    #[serde(rename = "parentContainerFabricId", default, skip_serializing_if = "Option::is_none")]
    pub parent_container_fabric_id: Option<String>,
    #[doc = "Friendly name of container to which this protectable item belongs."]
    #[serde(rename = "parentContainerFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub parent_container_friendly_name: Option<String>,
    #[doc = "File Share type XSync or XSMB."]
    #[serde(rename = "azureFileShareType", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_type: Option<azure_file_share_protectable_item::AzureFileShareType>,
}
impl AzureFileShareProtectableItem {
    pub fn new(workload_protectable_item: WorkloadProtectableItem) -> Self {
        Self {
            workload_protectable_item,
            parent_container_fabric_id: None,
            parent_container_friendly_name: None,
            azure_file_share_type: None,
        }
    }
}
pub mod azure_file_share_protectable_item {
    use super::*;
    #[doc = "File Share type XSync or XSMB."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureFileShareType")]
    pub enum AzureFileShareType {
        Invalid,
        #[serde(rename = "XSMB")]
        Xsmb,
        XSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureFileShareType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureFileShareType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureFileShareType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("AzureFileShareType", 0u32, "Invalid"),
                Self::Xsmb => serializer.serialize_unit_variant("AzureFileShareType", 1u32, "XSMB"),
                Self::XSync => serializer.serialize_unit_variant("AzureFileShareType", 2u32, "XSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "AzureStorage backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareProtectionPolicy {
    #[serde(flatten)]
    pub protection_policy: ProtectionPolicy,
    #[doc = "Type of workload for the backup management"]
    #[serde(rename = "workLoadType", default, skip_serializing_if = "Option::is_none")]
    pub work_load_type: Option<azure_file_share_protection_policy::WorkLoadType>,
    #[doc = "Base class for backup schedule."]
    #[serde(rename = "schedulePolicy", default, skip_serializing_if = "Option::is_none")]
    pub schedule_policy: Option<SchedulePolicy>,
    #[doc = "Base class for retention policy."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
    #[doc = "TimeZone optional input as string. For example: TimeZone = \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl AzureFileShareProtectionPolicy {
    pub fn new(protection_policy: ProtectionPolicy) -> Self {
        Self {
            protection_policy,
            work_load_type: None,
            schedule_policy: None,
            retention_policy: None,
            time_zone: None,
        }
    }
}
pub mod azure_file_share_protection_policy {
    use super::*;
    #[doc = "Type of workload for the backup management"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkLoadType")]
    pub enum WorkLoadType {
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
    impl FromStr for WorkLoadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkLoadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkLoadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("WorkLoadType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("WorkLoadType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("WorkLoadType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("WorkLoadType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("WorkLoadType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("WorkLoadType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("WorkLoadType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("WorkLoadType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("WorkLoadType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("WorkLoadType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("WorkLoadType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("WorkLoadType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("WorkLoadType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("WorkLoadType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("WorkLoadType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update snapshot Uri with the correct friendly Name of the source Azure file share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileShareProvisionIlrRequest {
    #[serde(flatten)]
    pub ilr_request: IlrRequest,
    #[doc = "Recovery point ID."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "Source Storage account ARM Id"]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
}
impl AzureFileShareProvisionIlrRequest {
    pub fn new(ilr_request: IlrRequest) -> Self {
        Self {
            ilr_request,
            recovery_point_id: None,
            source_resource_id: None,
        }
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
#[doc = "IaaS VM workload-specific backup item representing a classic virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSClassicComputeVmContainer {
    #[serde(flatten)]
    pub iaa_svm_container: IaaSvmContainer,
}
impl AzureIaaSClassicComputeVmContainer {
    pub fn new(iaa_svm_container: IaaSvmContainer) -> Self {
        Self { iaa_svm_container }
    }
}
#[doc = "IaaS VM workload-specific backup item representing the Classic Compute VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSClassicComputeVmProtectableItem {
    #[serde(flatten)]
    pub iaa_svm_protectable_item: IaaSvmProtectableItem,
}
impl AzureIaaSClassicComputeVmProtectableItem {
    pub fn new(iaa_svm_protectable_item: IaaSvmProtectableItem) -> Self {
        Self { iaa_svm_protectable_item }
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
#[doc = "IaaS VM workload-specific backup item representing an Azure Resource Manager virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSComputeVmContainer {
    #[serde(flatten)]
    pub iaa_svm_container: IaaSvmContainer,
}
impl AzureIaaSComputeVmContainer {
    pub fn new(iaa_svm_container: IaaSvmContainer) -> Self {
        Self { iaa_svm_container }
    }
}
#[doc = "IaaS VM workload-specific backup item representing the Azure Resource Manager VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSComputeVmProtectableItem {
    #[serde(flatten)]
    pub iaa_svm_protectable_item: IaaSvmProtectableItem,
}
impl AzureIaaSComputeVmProtectableItem {
    pub fn new(iaa_svm_protectable_item: IaaSvmProtectableItem) -> Self {
        Self { iaa_svm_protectable_item }
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
    #[doc = "Container name of the entity on which the current job is executing."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Indicated that whether the job is adhoc(true) or scheduled(false)"]
    #[serde(rename = "isUserTriggered", default, skip_serializing_if = "Option::is_none")]
    pub is_user_triggered: Option<bool>,
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
            container_name: None,
            is_user_triggered: None,
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
#[doc = "Azure IaaS VM workload-specific job object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmJobV2 {
    #[serde(flatten)]
    pub job: Job,
    #[doc = "Gets or sets the state/actions applicable on this job like cancel/retry."]
    #[serde(rename = "actionsInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[doc = "Container name of the entity on which the current job is executing."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Time elapsed during the execution of this job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
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
impl AzureIaaSvmJobV2 {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            actions_info: Vec::new(),
            container_name: None,
            duration: None,
            error_details: Vec::new(),
            virtual_machine_version: None,
            extended_info: None,
        }
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
#[doc = "IaaS VM workload-specific backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmProtectionPolicy {
    #[serde(flatten)]
    pub protection_policy: ProtectionPolicy,
    #[serde(rename = "instantRPDetails", default, skip_serializing_if = "Option::is_none")]
    pub instant_rp_details: Option<InstantRpAdditionalDetails>,
    #[doc = "Base class for backup schedule."]
    #[serde(rename = "schedulePolicy", default, skip_serializing_if = "Option::is_none")]
    pub schedule_policy: Option<SchedulePolicy>,
    #[doc = "Base class for retention policy."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
    #[doc = "Instant RP retention policy range in days"]
    #[serde(rename = "instantRpRetentionRangeInDays", default, skip_serializing_if = "Option::is_none")]
    pub instant_rp_retention_range_in_days: Option<i32>,
    #[doc = "TimeZone optional input as string. For example: TimeZone = \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<azure_iaa_svm_protection_policy::PolicyType>,
}
impl AzureIaaSvmProtectionPolicy {
    pub fn new(protection_policy: ProtectionPolicy) -> Self {
        Self {
            protection_policy,
            instant_rp_details: None,
            schedule_policy: None,
            retention_policy: None,
            instant_rp_retention_range_in_days: None,
            time_zone: None,
            policy_type: None,
        }
    }
}
pub mod azure_iaa_svm_protection_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PolicyType")]
    pub enum PolicyType {
        Invalid,
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PolicyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PolicyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PolicyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("PolicyType", 0u32, "Invalid"),
                Self::V1 => serializer.serialize_unit_variant("PolicyType", 1u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("PolicyType", 2u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Recovery Services Vault specific protection intent item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureRecoveryServiceVaultProtectionIntent {
    #[serde(flatten)]
    pub protection_intent: ProtectionIntent,
}
impl AzureRecoveryServiceVaultProtectionIntent {
    pub fn new(protection_intent: ProtectionIntent) -> Self {
        Self { protection_intent }
    }
}
#[doc = "IaaS VM specific backup protection intent item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceProtectionIntent {
    #[serde(flatten)]
    pub protection_intent: ProtectionIntent,
    #[doc = "Friendly name of the VM represented by this backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl AzureResourceProtectionIntent {
    pub fn new(protection_intent: ProtectionIntent) -> Self {
        Self {
            protection_intent,
            friendly_name: None,
        }
    }
}
#[doc = "Container for SQL workloads under SQL Availability Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlagWorkloadContainerProtectionContainer {
    #[serde(flatten)]
    pub azure_workload_container: AzureWorkloadContainer,
}
impl AzureSqlagWorkloadContainerProtectionContainer {
    pub fn new(azure_workload_container: AzureWorkloadContainer) -> Self {
        Self { azure_workload_container }
    }
}
#[doc = "Azure Sql workload-specific container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
}
impl AzureSqlContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self { protection_container }
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
#[doc = "Azure SQL workload-specific backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlProtectionPolicy {
    #[serde(flatten)]
    pub protection_policy: ProtectionPolicy,
    #[doc = "Base class for retention policy."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl AzureSqlProtectionPolicy {
    pub fn new(protection_policy: ProtectionPolicy) -> Self {
        Self {
            protection_policy,
            retention_policy: None,
        }
    }
}
#[doc = "Azure Storage Account workload-specific container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
    #[doc = "Fully qualified ARM url."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "Storage account version."]
    #[serde(rename = "storageAccountVersion", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_version: Option<String>,
    #[doc = "Resource group name of Recovery Services Vault."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Number of items backed up in this container."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i64>,
    #[doc = "Whether storage account lock is to be acquired for this container or not."]
    #[serde(rename = "acquireStorageAccountLock", default, skip_serializing_if = "Option::is_none")]
    pub acquire_storage_account_lock: Option<azure_storage_container::AcquireStorageAccountLock>,
}
impl AzureStorageContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self {
            protection_container,
            source_resource_id: None,
            storage_account_version: None,
            resource_group: None,
            protected_item_count: None,
            acquire_storage_account_lock: None,
        }
    }
}
pub mod azure_storage_container {
    use super::*;
    #[doc = "Whether storage account lock is to be acquired for this container or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AcquireStorageAccountLock")]
    pub enum AcquireStorageAccountLock {
        Acquire,
        NotAcquire,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AcquireStorageAccountLock {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AcquireStorageAccountLock {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AcquireStorageAccountLock {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Acquire => serializer.serialize_unit_variant("AcquireStorageAccountLock", 0u32, "Acquire"),
                Self::NotAcquire => serializer.serialize_unit_variant("AcquireStorageAccountLock", 1u32, "NotAcquire"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Indicated that whether the job is adhoc(true) or scheduled(false)"]
    #[serde(rename = "isUserTriggered", default, skip_serializing_if = "Option::is_none")]
    pub is_user_triggered: Option<bool>,
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
            is_user_triggered: None,
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
#[doc = "Azure Storage-specific protectable containers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageProtectableContainer {
    #[serde(flatten)]
    pub protectable_container: ProtectableContainer,
}
impl AzureStorageProtectableContainer {
    pub fn new(protectable_container: ProtectableContainer) -> Self {
        Self { protectable_container }
    }
}
#[doc = "Azure workload-specific container"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmAppContainerProtectableContainer {
    #[serde(flatten)]
    pub protectable_container: ProtectableContainer,
}
impl AzureVmAppContainerProtectableContainer {
    pub fn new(protectable_container: ProtectableContainer) -> Self {
        Self { protectable_container }
    }
}
#[doc = "Container for SQL workloads under Azure Virtual Machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmAppContainerProtectionContainer {
    #[serde(flatten)]
    pub azure_workload_container: AzureWorkloadContainer,
}
impl AzureVmAppContainerProtectionContainer {
    pub fn new(azure_workload_container: AzureWorkloadContainer) -> Self {
        Self { azure_workload_container }
    }
}
#[doc = "AzureResource(IaaS VM) Specific feature support request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmResourceFeatureSupportRequest {
    #[serde(flatten)]
    pub feature_support_request: FeatureSupportRequest,
    #[doc = "Size of the resource: VM size(A/D series etc) in case of IaasVM"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "SKUs (Premium/Managed etc) in case of IaasVM"]
    #[serde(rename = "vmSku", default, skip_serializing_if = "Option::is_none")]
    pub vm_sku: Option<String>,
}
impl AzureVmResourceFeatureSupportRequest {
    pub fn new(feature_support_request: FeatureSupportRequest) -> Self {
        Self {
            feature_support_request,
            vm_size: None,
            vm_sku: None,
        }
    }
}
#[doc = "Response for feature support requests for Azure IaasVm"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureVmResourceFeatureSupportResponse {
    #[doc = "Support status of feature"]
    #[serde(rename = "supportStatus", default, skip_serializing_if = "Option::is_none")]
    pub support_status: Option<azure_vm_resource_feature_support_response::SupportStatus>,
}
impl AzureVmResourceFeatureSupportResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_vm_resource_feature_support_response {
    use super::*;
    #[doc = "Support status of feature"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportStatus")]
    pub enum SupportStatus {
        Invalid,
        Supported,
        #[serde(rename = "DefaultOFF")]
        DefaultOff,
        #[serde(rename = "DefaultON")]
        DefaultOn,
        NotSupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SupportStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SupportStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SupportStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("SupportStatus", 0u32, "Invalid"),
                Self::Supported => serializer.serialize_unit_variant("SupportStatus", 1u32, "Supported"),
                Self::DefaultOff => serializer.serialize_unit_variant("SupportStatus", 2u32, "DefaultOFF"),
                Self::DefaultOn => serializer.serialize_unit_variant("SupportStatus", 3u32, "DefaultON"),
                Self::NotSupported => serializer.serialize_unit_variant("SupportStatus", 4u32, "NotSupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure VM workload-specific workload item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadItem {
    #[serde(flatten)]
    pub workload_item: WorkloadItem,
    #[doc = "Name for instance or AG"]
    #[serde(rename = "parentName", default, skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[doc = "Host/Cluster Name for instance or AG"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Indicates if workload item is auto-protectable"]
    #[serde(rename = "isAutoProtectable", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_protectable: Option<bool>,
    #[doc = "For instance or AG, indicates number of DB's present"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subinquireditemcount: Option<i32>,
    #[doc = "For instance or AG, indicates number of DB's to be protected"]
    #[serde(rename = "subWorkloadItemCount", default, skip_serializing_if = "Option::is_none")]
    pub sub_workload_item_count: Option<i32>,
}
impl AzureVmWorkloadItem {
    pub fn new(workload_item: WorkloadItem) -> Self {
        Self {
            workload_item,
            parent_name: None,
            server_name: None,
            is_auto_protectable: None,
            subinquireditemcount: None,
            sub_workload_item_count: None,
        }
    }
}
#[doc = "Azure VM workload-specific protectable item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadProtectableItem {
    #[serde(flatten)]
    pub workload_protectable_item: WorkloadProtectableItem,
    #[doc = "Name for instance or AG"]
    #[serde(rename = "parentName", default, skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[doc = "Parent Unique Name is added to provide the service formatted URI Name of the Parent\r\nOnly Applicable for data bases where the parent would be either Instance or a SQL AG."]
    #[serde(rename = "parentUniqueName", default, skip_serializing_if = "Option::is_none")]
    pub parent_unique_name: Option<String>,
    #[doc = "Host/Cluster Name for instance or AG"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Indicates if protectable item is auto-protectable"]
    #[serde(rename = "isAutoProtectable", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_protectable: Option<bool>,
    #[doc = "Indicates if protectable item is auto-protected"]
    #[serde(rename = "isAutoProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_protected: Option<bool>,
    #[doc = "For instance or AG, indicates number of DB's present"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subinquireditemcount: Option<i32>,
    #[doc = "For instance or AG, indicates number of DB's to be protected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subprotectableitemcount: Option<i32>,
    #[doc = "Pre-backup validation for Azure VM Workload provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebackupvalidation: Option<PreBackupValidation>,
}
impl AzureVmWorkloadProtectableItem {
    pub fn new(workload_protectable_item: WorkloadProtectableItem) -> Self {
        Self {
            workload_protectable_item,
            parent_name: None,
            parent_unique_name: None,
            server_name: None,
            is_auto_protectable: None,
            is_auto_protected: None,
            subinquireditemcount: None,
            subprotectableitemcount: None,
            prebackupvalidation: None,
        }
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
    #[doc = "Indicates consistency of policy object and policy applied to this backup item."]
    #[serde(rename = "recoveryModel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_model: Option<String>,
}
impl AzureVmWorkloadProtectedItemExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure VM (Mercury) workload-specific backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadProtectionPolicy {
    #[serde(flatten)]
    pub protection_policy: ProtectionPolicy,
    #[doc = "Type of workload for the backup management"]
    #[serde(rename = "workLoadType", default, skip_serializing_if = "Option::is_none")]
    pub work_load_type: Option<azure_vm_workload_protection_policy::WorkLoadType>,
    #[doc = "Common settings field for backup management"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    #[doc = "List of sub-protection policies which includes schedule and retention"]
    #[serde(rename = "subProtectionPolicy", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_protection_policy: Vec<SubProtectionPolicy>,
    #[doc = "Fix the policy inconsistency"]
    #[serde(rename = "makePolicyConsistent", default, skip_serializing_if = "Option::is_none")]
    pub make_policy_consistent: Option<bool>,
}
impl AzureVmWorkloadProtectionPolicy {
    pub fn new(protection_policy: ProtectionPolicy) -> Self {
        Self {
            protection_policy,
            work_load_type: None,
            settings: None,
            sub_protection_policy: Vec::new(),
            make_policy_consistent: None,
        }
    }
}
pub mod azure_vm_workload_protection_policy {
    use super::*;
    #[doc = "Type of workload for the backup management"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkLoadType")]
    pub enum WorkLoadType {
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
    impl FromStr for WorkLoadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkLoadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkLoadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("WorkLoadType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("WorkLoadType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("WorkLoadType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("WorkLoadType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("WorkLoadType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("WorkLoadType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("WorkLoadType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("WorkLoadType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("WorkLoadType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("WorkLoadType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("WorkLoadType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("WorkLoadType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("WorkLoadType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("WorkLoadType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("WorkLoadType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Azure VM workload-specific workload item representing SAP ASE Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapAseDatabaseWorkloadItem {
    #[serde(flatten)]
    pub azure_vm_workload_item: AzureVmWorkloadItem,
}
impl AzureVmWorkloadSapAseDatabaseWorkloadItem {
    pub fn new(azure_vm_workload_item: AzureVmWorkloadItem) -> Self {
        Self { azure_vm_workload_item }
    }
}
#[doc = "Azure VM workload-specific protectable item representing SAP ASE System."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapAseSystemProtectableItem {
    #[serde(flatten)]
    pub azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem,
}
impl AzureVmWorkloadSapAseSystemProtectableItem {
    pub fn new(azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem) -> Self {
        Self {
            azure_vm_workload_protectable_item,
        }
    }
}
#[doc = "Azure VM workload-specific workload item representing SAP ASE System."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapAseSystemWorkloadItem {
    #[serde(flatten)]
    pub azure_vm_workload_item: AzureVmWorkloadItem,
}
impl AzureVmWorkloadSapAseSystemWorkloadItem {
    pub fn new(azure_vm_workload_item: AzureVmWorkloadItem) -> Self {
        Self { azure_vm_workload_item }
    }
}
#[doc = "Azure VM workload-specific protectable item representing SAP HANA Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapHanaDatabaseProtectableItem {
    #[serde(flatten)]
    pub azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem,
}
impl AzureVmWorkloadSapHanaDatabaseProtectableItem {
    pub fn new(azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem) -> Self {
        Self {
            azure_vm_workload_protectable_item,
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
#[doc = "Azure VM workload-specific workload item representing SAP HANA Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapHanaDatabaseWorkloadItem {
    #[serde(flatten)]
    pub azure_vm_workload_item: AzureVmWorkloadItem,
}
impl AzureVmWorkloadSapHanaDatabaseWorkloadItem {
    pub fn new(azure_vm_workload_item: AzureVmWorkloadItem) -> Self {
        Self { azure_vm_workload_item }
    }
}
#[doc = "Azure VM workload-specific protectable item representing SAP HANA System."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapHanaSystemProtectableItem {
    #[serde(flatten)]
    pub azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem,
}
impl AzureVmWorkloadSapHanaSystemProtectableItem {
    pub fn new(azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem) -> Self {
        Self {
            azure_vm_workload_protectable_item,
        }
    }
}
#[doc = "Azure VM workload-specific workload item representing SAP HANA System."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSapHanaSystemWorkloadItem {
    #[serde(flatten)]
    pub azure_vm_workload_item: AzureVmWorkloadItem,
}
impl AzureVmWorkloadSapHanaSystemWorkloadItem {
    pub fn new(azure_vm_workload_item: AzureVmWorkloadItem) -> Self {
        Self { azure_vm_workload_item }
    }
}
#[doc = "Azure VM workload-specific protectable item representing SQL Availability Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSqlAvailabilityGroupProtectableItem {
    #[serde(flatten)]
    pub azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem,
}
impl AzureVmWorkloadSqlAvailabilityGroupProtectableItem {
    pub fn new(azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem) -> Self {
        Self {
            azure_vm_workload_protectable_item,
        }
    }
}
#[doc = "Azure VM workload-specific protectable item representing SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSqlDatabaseProtectableItem {
    #[serde(flatten)]
    pub azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem,
}
impl AzureVmWorkloadSqlDatabaseProtectableItem {
    pub fn new(azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem) -> Self {
        Self {
            azure_vm_workload_protectable_item,
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
#[doc = "Azure VM workload-specific workload item representing SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSqlDatabaseWorkloadItem {
    #[serde(flatten)]
    pub azure_vm_workload_item: AzureVmWorkloadItem,
}
impl AzureVmWorkloadSqlDatabaseWorkloadItem {
    pub fn new(azure_vm_workload_item: AzureVmWorkloadItem) -> Self {
        Self { azure_vm_workload_item }
    }
}
#[doc = "Azure VM workload-specific protectable item representing SQL Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSqlInstanceProtectableItem {
    #[serde(flatten)]
    pub azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem,
}
impl AzureVmWorkloadSqlInstanceProtectableItem {
    pub fn new(azure_vm_workload_protectable_item: AzureVmWorkloadProtectableItem) -> Self {
        Self {
            azure_vm_workload_protectable_item,
        }
    }
}
#[doc = "Azure VM workload-specific workload item representing SQL Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmWorkloadSqlInstanceWorkloadItem {
    #[serde(flatten)]
    pub azure_vm_workload_item: AzureVmWorkloadItem,
    #[doc = "Data Directory Paths for default directories"]
    #[serde(rename = "dataDirectoryPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub data_directory_paths: Vec<SqlDataDirectory>,
}
impl AzureVmWorkloadSqlInstanceWorkloadItem {
    pub fn new(azure_vm_workload_item: AzureVmWorkloadItem) -> Self {
        Self {
            azure_vm_workload_item,
            data_directory_paths: Vec::new(),
        }
    }
}
#[doc = "Azure Recovery Services Vault specific protection intent item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadAutoProtectionIntent {
    #[serde(flatten)]
    pub azure_recovery_service_vault_protection_intent: AzureRecoveryServiceVaultProtectionIntent,
}
impl AzureWorkloadAutoProtectionIntent {
    pub fn new(azure_recovery_service_vault_protection_intent: AzureRecoveryServiceVaultProtectionIntent) -> Self {
        Self {
            azure_recovery_service_vault_protection_intent,
        }
    }
}
#[doc = "AzureWorkload workload-specific backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadBackupRequest {
    #[serde(flatten)]
    pub backup_request: BackupRequest,
    #[doc = "Type of backup, viz. Full, Differential, Log or CopyOnlyFull"]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<azure_workload_backup_request::BackupType>,
    #[doc = "Bool for Compression setting"]
    #[serde(rename = "enableCompression", default, skip_serializing_if = "Option::is_none")]
    pub enable_compression: Option<bool>,
    #[doc = "Backup copy will expire after the time specified (UTC)."]
    #[serde(rename = "recoveryPointExpiryTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_expiry_time_in_utc: Option<time::OffsetDateTime>,
}
impl AzureWorkloadBackupRequest {
    pub fn new(backup_request: BackupRequest) -> Self {
        Self {
            backup_request,
            backup_type: None,
            enable_compression: None,
            recovery_point_expiry_time_in_utc: None,
        }
    }
}
pub mod azure_workload_backup_request {
    use super::*;
    #[doc = "Type of backup, viz. Full, Differential, Log or CopyOnlyFull"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupType")]
    pub enum BackupType {
        Invalid,
        Full,
        Differential,
        Log,
        CopyOnlyFull,
        Incremental,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupType", 0u32, "Invalid"),
                Self::Full => serializer.serialize_unit_variant("BackupType", 1u32, "Full"),
                Self::Differential => serializer.serialize_unit_variant("BackupType", 2u32, "Differential"),
                Self::Log => serializer.serialize_unit_variant("BackupType", 3u32, "Log"),
                Self::CopyOnlyFull => serializer.serialize_unit_variant("BackupType", 4u32, "CopyOnlyFull"),
                Self::Incremental => serializer.serialize_unit_variant("BackupType", 5u32, "Incremental"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Container for the workloads running inside Azure Compute or Classic Compute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
    #[doc = "ARM ID of the virtual machine represented by this Azure Workload Container"]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "Time stamp when this container was updated."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "Extended information of the container."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureWorkloadContainerExtendedInfo>,
    #[doc = "Workload type for which registration was sent."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<azure_workload_container::WorkloadType>,
    #[doc = "Re-Do Operation"]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<azure_workload_container::OperationType>,
}
impl AzureWorkloadContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self {
            protection_container,
            source_resource_id: None,
            last_updated_time: None,
            extended_info: None,
            workload_type: None,
            operation_type: None,
        }
    }
}
pub mod azure_workload_container {
    use super::*;
    #[doc = "Workload type for which registration was sent."]
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
    #[doc = "Re-Do Operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        Invalid,
        Register,
        Reregister,
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
                Self::Invalid => serializer.serialize_unit_variant("OperationType", 0u32, "Invalid"),
                Self::Register => serializer.serialize_unit_variant("OperationType", 1u32, "Register"),
                Self::Reregister => serializer.serialize_unit_variant("OperationType", 2u32, "Reregister"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure workload specific protection intent item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadContainerAutoProtectionIntent {
    #[serde(flatten)]
    pub protection_intent: ProtectionIntent,
}
impl AzureWorkloadContainerAutoProtectionIntent {
    pub fn new(protection_intent: ProtectionIntent) -> Self {
        Self { protection_intent }
    }
}
#[doc = "Extended information of the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureWorkloadContainerExtendedInfo {
    #[doc = "Host Os Name in case of Stand Alone and Cluster Name in case of distributed container."]
    #[serde(rename = "hostServerName", default, skip_serializing_if = "Option::is_none")]
    pub host_server_name: Option<String>,
    #[doc = "Details about inquired protectable items under a given container."]
    #[serde(rename = "inquiryInfo", default, skip_serializing_if = "Option::is_none")]
    pub inquiry_info: Option<InquiryInfo>,
    #[doc = "List of the nodes in case of distributed container."]
    #[serde(rename = "nodesList", default, skip_serializing_if = "Vec::is_empty")]
    pub nodes_list: Vec<DistributedNodesInfo>,
}
impl AzureWorkloadContainerExtendedInfo {
    pub fn new() -> Self {
        Self::default()
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
    pub recovery_point_tier_details: Vec<RecoveryPointTierInformationV2>,
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
#[doc = "AzureWorkload SAP Hana-specific restore with integrated rehydration of recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSapHanaPointInTimeRestoreWithRehydrateRequest {
    #[serde(flatten)]
    pub azure_workload_sap_hana_point_in_time_restore_request: AzureWorkloadSapHanaPointInTimeRestoreRequest,
    #[doc = "RP Rehydration Info"]
    #[serde(rename = "recoveryPointRehydrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_rehydration_info: Option<RecoveryPointRehydrationInfo>,
}
impl AzureWorkloadSapHanaPointInTimeRestoreWithRehydrateRequest {
    pub fn new(azure_workload_sap_hana_point_in_time_restore_request: AzureWorkloadSapHanaPointInTimeRestoreRequest) -> Self {
        Self {
            azure_workload_sap_hana_point_in_time_restore_request,
            recovery_point_rehydration_info: None,
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
#[doc = "AzureWorkload SAP Hana-specific restore with integrated rehydration of recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSapHanaRestoreWithRehydrateRequest {
    #[serde(flatten)]
    pub azure_workload_sap_hana_restore_request: AzureWorkloadSapHanaRestoreRequest,
    #[doc = "RP Rehydration Info"]
    #[serde(rename = "recoveryPointRehydrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_rehydration_info: Option<RecoveryPointRehydrationInfo>,
}
impl AzureWorkloadSapHanaRestoreWithRehydrateRequest {
    pub fn new(azure_workload_sap_hana_restore_request: AzureWorkloadSapHanaRestoreRequest) -> Self {
        Self {
            azure_workload_sap_hana_restore_request,
            recovery_point_rehydration_info: None,
        }
    }
}
#[doc = "Azure Workload SQL Auto Protection intent item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlAutoProtectionIntent {
    #[serde(flatten)]
    pub azure_workload_auto_protection_intent: AzureWorkloadAutoProtectionIntent,
    #[doc = "Workload item type of the item for which intent is to be set"]
    #[serde(rename = "workloadItemType", default, skip_serializing_if = "Option::is_none")]
    pub workload_item_type: Option<azure_workload_sql_auto_protection_intent::WorkloadItemType>,
}
impl AzureWorkloadSqlAutoProtectionIntent {
    pub fn new(azure_workload_auto_protection_intent: AzureWorkloadAutoProtectionIntent) -> Self {
        Self {
            azure_workload_auto_protection_intent,
            workload_item_type: None,
        }
    }
}
pub mod azure_workload_sql_auto_protection_intent {
    use super::*;
    #[doc = "Workload item type of the item for which intent is to be set"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadItemType")]
    pub enum WorkloadItemType {
        Invalid,
        #[serde(rename = "SQLInstance")]
        SqlInstance,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        #[serde(rename = "SAPHanaSystem")]
        SapHanaSystem,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseSystem")]
        SapAseSystem,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadItemType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadItemType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadItemType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("WorkloadItemType", 0u32, "Invalid"),
                Self::SqlInstance => serializer.serialize_unit_variant("WorkloadItemType", 1u32, "SQLInstance"),
                Self::SqlDataBase => serializer.serialize_unit_variant("WorkloadItemType", 2u32, "SQLDataBase"),
                Self::SapHanaSystem => serializer.serialize_unit_variant("WorkloadItemType", 3u32, "SAPHanaSystem"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("WorkloadItemType", 4u32, "SAPHanaDatabase"),
                Self::SapAseSystem => serializer.serialize_unit_variant("WorkloadItemType", 5u32, "SAPAseSystem"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("WorkloadItemType", 6u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "AzureWorkload SQL-specific restore with integrated rehydration of recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlPointInTimeRestoreWithRehydrateRequest {
    #[serde(flatten)]
    pub azure_workload_sql_point_in_time_restore_request: AzureWorkloadSqlPointInTimeRestoreRequest,
    #[doc = "RP Rehydration Info"]
    #[serde(rename = "recoveryPointRehydrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_rehydration_info: Option<RecoveryPointRehydrationInfo>,
}
impl AzureWorkloadSqlPointInTimeRestoreWithRehydrateRequest {
    pub fn new(azure_workload_sql_point_in_time_restore_request: AzureWorkloadSqlPointInTimeRestoreRequest) -> Self {
        Self {
            azure_workload_sql_point_in_time_restore_request,
            recovery_point_rehydration_info: None,
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
#[doc = "AzureWorkload SQL-specific restore with integrated rehydration of recovery point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlRestoreWithRehydrateRequest {
    #[serde(flatten)]
    pub azure_workload_sql_restore_request: AzureWorkloadSqlRestoreRequest,
    #[doc = "RP Rehydration Info"]
    #[serde(rename = "recoveryPointRehydrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_rehydration_info: Option<RecoveryPointRehydrationInfo>,
}
impl AzureWorkloadSqlRestoreWithRehydrateRequest {
    pub fn new(azure_workload_sql_restore_request: AzureWorkloadSqlRestoreRequest) -> Self {
        Self {
            azure_workload_sql_restore_request,
            recovery_point_rehydration_info: None,
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
#[doc = "Query parameters to fetch list of backup engines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsBackupEngineQueryObject {
    #[doc = "attribute to add extended info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
}
impl BmsBackupEngineQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameters to fetch list of backup engines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsBackupEnginesQueryObject {
    #[doc = "Backup management type for the backup engine."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<bms_backup_engines_query_object::BackupManagementType>,
    #[doc = "Friendly name of the backup engine."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Attribute to add extended info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
}
impl BmsBackupEnginesQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bms_backup_engines_query_object {
    use super::*;
    #[doc = "Backup management type for the backup engine."]
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
#[doc = "The query filters that can be used with the list containers API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BmsContainerQueryObject {
    #[doc = "Backup management type for this container."]
    #[serde(rename = "backupManagementType")]
    pub backup_management_type: bms_container_query_object::BackupManagementType,
    #[doc = "Type of container for filter"]
    #[serde(rename = "containerType", default, skip_serializing_if = "Option::is_none")]
    pub container_type: Option<bms_container_query_object::ContainerType>,
    #[doc = "Backup engine name"]
    #[serde(rename = "backupEngineName", default, skip_serializing_if = "Option::is_none")]
    pub backup_engine_name: Option<String>,
    #[doc = "Fabric name for filter"]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "Status of registration of this container with the Recovery Services Vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Friendly name of this container."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl BmsContainerQueryObject {
    pub fn new(backup_management_type: bms_container_query_object::BackupManagementType) -> Self {
        Self {
            backup_management_type,
            container_type: None,
            backup_engine_name: None,
            fabric_name: None,
            status: None,
            friendly_name: None,
        }
    }
}
pub mod bms_container_query_object {
    use super::*;
    #[doc = "Backup management type for this container."]
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
    #[doc = "Type of container for filter"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContainerType")]
    pub enum ContainerType {
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
    impl FromStr for ContainerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContainerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContainerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ContainerType", 0u32, "Invalid"),
                Self::Unknown => serializer.serialize_unit_variant("ContainerType", 1u32, "Unknown"),
                Self::IaasVmContainer => serializer.serialize_unit_variant("ContainerType", 2u32, "IaasVMContainer"),
                Self::IaasVmServiceContainer => serializer.serialize_unit_variant("ContainerType", 3u32, "IaasVMServiceContainer"),
                Self::DpmContainer => serializer.serialize_unit_variant("ContainerType", 4u32, "DPMContainer"),
                Self::AzureBackupServerContainer => serializer.serialize_unit_variant("ContainerType", 5u32, "AzureBackupServerContainer"),
                Self::MabContainer => serializer.serialize_unit_variant("ContainerType", 6u32, "MABContainer"),
                Self::Cluster => serializer.serialize_unit_variant("ContainerType", 7u32, "Cluster"),
                Self::AzureSqlContainer => serializer.serialize_unit_variant("ContainerType", 8u32, "AzureSqlContainer"),
                Self::Windows => serializer.serialize_unit_variant("ContainerType", 9u32, "Windows"),
                Self::VCenter => serializer.serialize_unit_variant("ContainerType", 10u32, "VCenter"),
                Self::VmAppContainer => serializer.serialize_unit_variant("ContainerType", 11u32, "VMAppContainer"),
                Self::SqlagWorkLoadContainer => serializer.serialize_unit_variant("ContainerType", 12u32, "SQLAGWorkLoadContainer"),
                Self::StorageContainer => serializer.serialize_unit_variant("ContainerType", 13u32, "StorageContainer"),
                Self::GenericContainer => serializer.serialize_unit_variant("ContainerType", 14u32, "GenericContainer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The query filters that can be used with the inquire container API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsContainersInquiryQueryObject {
    #[doc = "Backup management type for this container."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<bms_containers_inquiry_query_object::BackupManagementType>,
    #[doc = "Workload type for this container."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<bms_containers_inquiry_query_object::WorkloadType>,
}
impl BmsContainersInquiryQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bms_containers_inquiry_query_object {
    use super::*;
    #[doc = "Backup management type for this container."]
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
    #[doc = "Workload type for this container."]
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
#[doc = "Filters to list items that can be backed up."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmspoQueryObject {
    #[doc = "Backup management type."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<bmspo_query_object::BackupManagementType>,
    #[doc = "Workload type"]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<bmspo_query_object::WorkloadType>,
    #[doc = "Full name of the container whose Protectable Objects should be returned."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Backup status query parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl BmspoQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bmspo_query_object {
    use super::*;
    #[doc = "Backup management type."]
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
        AzureSql,
        AzureBackupServer,
        AzureWorkload,
        AzureStorage,
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
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureSql"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureBackupServer"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureWorkload"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureStorage"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Workload type"]
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
#[doc = "The query filters that can be used with the refresh container API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsRefreshContainersQueryObject {
    #[doc = "Backup management type for this container."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<bms_refresh_containers_query_object::BackupManagementType>,
}
impl BmsRefreshContainersQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bms_refresh_containers_query_object {
    use super::*;
    #[doc = "Backup management type for this container."]
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
#[doc = "Filters to list items that can be backed up."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmsWorkloadItemQueryObject {
    #[doc = "Backup management type."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<bms_workload_item_query_object::BackupManagementType>,
    #[doc = "Workload Item type"]
    #[serde(rename = "workloadItemType", default, skip_serializing_if = "Option::is_none")]
    pub workload_item_type: Option<bms_workload_item_query_object::WorkloadItemType>,
    #[doc = "Workload type"]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<bms_workload_item_query_object::WorkloadType>,
    #[doc = "Backup status query parameter."]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<bms_workload_item_query_object::ProtectionStatus>,
}
impl BmsWorkloadItemQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bms_workload_item_query_object {
    use super::*;
    #[doc = "Backup management type."]
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
        AzureSql,
        AzureBackupServer,
        AzureWorkload,
        AzureStorage,
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
                Self::AzureSql => serializer.serialize_unit_variant("BackupManagementType", 4u32, "AzureSql"),
                Self::AzureBackupServer => serializer.serialize_unit_variant("BackupManagementType", 5u32, "AzureBackupServer"),
                Self::AzureWorkload => serializer.serialize_unit_variant("BackupManagementType", 6u32, "AzureWorkload"),
                Self::AzureStorage => serializer.serialize_unit_variant("BackupManagementType", 7u32, "AzureStorage"),
                Self::DefaultBackup => serializer.serialize_unit_variant("BackupManagementType", 8u32, "DefaultBackup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Workload Item type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadItemType")]
    pub enum WorkloadItemType {
        Invalid,
        #[serde(rename = "SQLInstance")]
        SqlInstance,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        #[serde(rename = "SAPHanaSystem")]
        SapHanaSystem,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseSystem")]
        SapAseSystem,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadItemType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadItemType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadItemType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("WorkloadItemType", 0u32, "Invalid"),
                Self::SqlInstance => serializer.serialize_unit_variant("WorkloadItemType", 1u32, "SQLInstance"),
                Self::SqlDataBase => serializer.serialize_unit_variant("WorkloadItemType", 2u32, "SQLDataBase"),
                Self::SapHanaSystem => serializer.serialize_unit_variant("WorkloadItemType", 3u32, "SAPHanaSystem"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("WorkloadItemType", 4u32, "SAPHanaDatabase"),
                Self::SapAseSystem => serializer.serialize_unit_variant("WorkloadItemType", 5u32, "SAPAseSystem"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("WorkloadItemType", 6u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Workload type"]
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
    #[doc = "Backup status query parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionStatus")]
    pub enum ProtectionStatus {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionStatus", 0u32, "Invalid"),
                Self::NotProtected => serializer.serialize_unit_variant("ProtectionStatus", 1u32, "NotProtected"),
                Self::Protecting => serializer.serialize_unit_variant("ProtectionStatus", 2u32, "Protecting"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionStatus", 3u32, "Protected"),
                Self::ProtectionFailed => serializer.serialize_unit_variant("ProtectionStatus", 4u32, "ProtectionFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The base backup engine class. All workload specific backup engines derive from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupEngineBase {
    #[doc = "Friendly name of the backup engine."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Type of backup management for the backup engine."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<backup_engine_base::BackupManagementType>,
    #[doc = "Registration status of the backup engine with the Recovery Services Vault."]
    #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
    #[doc = "Status of the backup engine with the Recovery Services Vault. = {Active/Deleting/DeleteFailed}"]
    #[serde(rename = "backupEngineState", default, skip_serializing_if = "Option::is_none")]
    pub backup_engine_state: Option<String>,
    #[doc = "Backup status of the backup engine."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[doc = "Type of the backup engine."]
    #[serde(rename = "backupEngineType")]
    pub backup_engine_type: backup_engine_base::BackupEngineType,
    #[doc = "Flag indicating if the backup engine be registered, once already registered."]
    #[serde(rename = "canReRegister", default, skip_serializing_if = "Option::is_none")]
    pub can_re_register: Option<bool>,
    #[doc = "ID of the backup engine."]
    #[serde(rename = "backupEngineId", default, skip_serializing_if = "Option::is_none")]
    pub backup_engine_id: Option<String>,
    #[doc = "Backup engine version"]
    #[serde(rename = "dpmVersion", default, skip_serializing_if = "Option::is_none")]
    pub dpm_version: Option<String>,
    #[doc = "Backup agent version"]
    #[serde(rename = "azureBackupAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub azure_backup_agent_version: Option<String>,
    #[doc = "To check if backup agent upgrade available"]
    #[serde(rename = "isAzureBackupAgentUpgradeAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_backup_agent_upgrade_available: Option<bool>,
    #[doc = "To check if backup engine upgrade available"]
    #[serde(rename = "isDpmUpgradeAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_dpm_upgrade_available: Option<bool>,
    #[doc = "Additional information on backup engine."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<BackupEngineExtendedInfo>,
}
impl BackupEngineBase {
    pub fn new(backup_engine_type: backup_engine_base::BackupEngineType) -> Self {
        Self {
            friendly_name: None,
            backup_management_type: None,
            registration_status: None,
            backup_engine_state: None,
            health_status: None,
            backup_engine_type,
            can_re_register: None,
            backup_engine_id: None,
            dpm_version: None,
            azure_backup_agent_version: None,
            is_azure_backup_agent_upgrade_available: None,
            is_dpm_upgrade_available: None,
            extended_info: None,
        }
    }
}
pub mod backup_engine_base {
    use super::*;
    #[doc = "Type of backup management for the backup engine."]
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
    #[doc = "Type of the backup engine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupEngineType")]
    pub enum BackupEngineType {
        Invalid,
        DpmBackupEngine,
        AzureBackupServerEngine,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupEngineType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupEngineType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupEngineType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupEngineType", 0u32, "Invalid"),
                Self::DpmBackupEngine => serializer.serialize_unit_variant("BackupEngineType", 1u32, "DpmBackupEngine"),
                Self::AzureBackupServerEngine => serializer.serialize_unit_variant("BackupEngineType", 2u32, "AzureBackupServerEngine"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The base backup engine class. All workload specific backup engines derive from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupEngineBaseResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The base backup engine class. All workload specific backup engines derive from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupEngineBase>,
}
impl BackupEngineBaseResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of BackupEngineBase resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupEngineBaseResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupEngineBaseResource>,
}
impl azure_core::Continuable for BackupEngineBaseResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupEngineBaseResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional information on backup engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupEngineExtendedInfo {
    #[doc = "Database name of backup engine."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Number of protected items in the backup engine."]
    #[serde(rename = "protectedItemsCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_items_count: Option<i32>,
    #[doc = "Number of protected servers in the backup engine."]
    #[serde(rename = "protectedServersCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_servers_count: Option<i32>,
    #[doc = "Number of disks in the backup engine."]
    #[serde(rename = "diskCount", default, skip_serializing_if = "Option::is_none")]
    pub disk_count: Option<i32>,
    #[doc = "Disk space used in the backup engine."]
    #[serde(rename = "usedDiskSpace", default, skip_serializing_if = "Option::is_none")]
    pub used_disk_space: Option<f64>,
    #[doc = "Disk space currently available in the backup engine."]
    #[serde(rename = "availableDiskSpace", default, skip_serializing_if = "Option::is_none")]
    pub available_disk_space: Option<f64>,
    #[doc = "Last refresh time in the backup engine."]
    #[serde(rename = "refreshedAt", default, with = "azure_core::date::rfc3339::option")]
    pub refreshed_at: Option<time::OffsetDateTime>,
    #[doc = "Protected instances in the backup engine."]
    #[serde(rename = "azureProtectedInstances", default, skip_serializing_if = "Option::is_none")]
    pub azure_protected_instances: Option<i32>,
}
impl BackupEngineExtendedInfo {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Base class for backup request. Workload-specific backup requests are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupRequest {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BackupRequest {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Base class for backup request. Workload-specific backup requests are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupRequestResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup request. Workload-specific backup requests are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupRequest>,
}
impl BackupRequestResource {
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
    #[doc = "Vault Dedup state"]
    #[serde(rename = "dedupState", default, skip_serializing_if = "Option::is_none")]
    pub dedup_state: Option<backup_resource_config::DedupState>,
    #[doc = "Vault x-cool state"]
    #[serde(rename = "xcoolState", default, skip_serializing_if = "Option::is_none")]
    pub xcool_state: Option<backup_resource_config::XcoolState>,
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
    #[doc = "Vault Dedup state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DedupState")]
    pub enum DedupState {
        Invalid,
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DedupState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DedupState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DedupState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("DedupState", 0u32, "Invalid"),
                Self::Enabled => serializer.serialize_unit_variant("DedupState", 1u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("DedupState", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Vault x-cool state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "XcoolState")]
    pub enum XcoolState {
        Invalid,
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for XcoolState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for XcoolState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for XcoolState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("XcoolState", 0u32, "Invalid"),
                Self::Enabled => serializer.serialize_unit_variant("XcoolState", 1u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("XcoolState", 2u32, "Disabled"),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceEncryptionConfig {
    #[doc = "Encryption At Rest Type"]
    #[serde(rename = "encryptionAtRestType", default, skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_type: Option<backup_resource_encryption_config::EncryptionAtRestType>,
    #[doc = "Key Vault Key URI"]
    #[serde(rename = "keyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_uri: Option<String>,
    #[doc = "Key Vault Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "lastUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<backup_resource_encryption_config::LastUpdateStatus>,
    #[serde(rename = "infrastructureEncryptionState", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_encryption_state: Option<backup_resource_encryption_config::InfrastructureEncryptionState>,
}
impl BackupResourceEncryptionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_resource_encryption_config {
    use super::*;
    #[doc = "Encryption At Rest Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EncryptionAtRestType")]
    pub enum EncryptionAtRestType {
        Invalid,
        MicrosoftManaged,
        CustomerManaged,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EncryptionAtRestType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EncryptionAtRestType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EncryptionAtRestType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("EncryptionAtRestType", 0u32, "Invalid"),
                Self::MicrosoftManaged => serializer.serialize_unit_variant("EncryptionAtRestType", 1u32, "MicrosoftManaged"),
                Self::CustomerManaged => serializer.serialize_unit_variant("EncryptionAtRestType", 2u32, "CustomerManaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastUpdateStatus")]
    pub enum LastUpdateStatus {
        Invalid,
        NotEnabled,
        PartiallySucceeded,
        PartiallyFailed,
        Failed,
        Succeeded,
        Initialized,
        FirstInitialization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("LastUpdateStatus", 0u32, "Invalid"),
                Self::NotEnabled => serializer.serialize_unit_variant("LastUpdateStatus", 1u32, "NotEnabled"),
                Self::PartiallySucceeded => serializer.serialize_unit_variant("LastUpdateStatus", 2u32, "PartiallySucceeded"),
                Self::PartiallyFailed => serializer.serialize_unit_variant("LastUpdateStatus", 3u32, "PartiallyFailed"),
                Self::Failed => serializer.serialize_unit_variant("LastUpdateStatus", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("LastUpdateStatus", 5u32, "Succeeded"),
                Self::Initialized => serializer.serialize_unit_variant("LastUpdateStatus", 6u32, "Initialized"),
                Self::FirstInitialization => serializer.serialize_unit_variant("LastUpdateStatus", 7u32, "FirstInitialization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InfrastructureEncryptionState")]
    pub enum InfrastructureEncryptionState {
        Invalid,
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InfrastructureEncryptionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InfrastructureEncryptionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InfrastructureEncryptionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("InfrastructureEncryptionState", 0u32, "Invalid"),
                Self::Disabled => serializer.serialize_unit_variant("InfrastructureEncryptionState", 1u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("InfrastructureEncryptionState", 2u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceEncryptionConfigExtended {
    #[serde(flatten)]
    pub backup_resource_encryption_config: BackupResourceEncryptionConfig,
    #[doc = "User Assigned Identity Id"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
    #[doc = "bool to indicate whether to use system Assigned Identity or not"]
    #[serde(rename = "useSystemAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub use_system_assigned_identity: Option<bool>,
}
impl BackupResourceEncryptionConfigExtended {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceEncryptionConfigExtendedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupResourceEncryptionConfigExtended>,
}
impl BackupResourceEncryptionConfigExtendedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceEncryptionConfigResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupResourceEncryptionConfig>,
}
impl BackupResourceEncryptionConfigResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backup resource vault config details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceVaultConfig {
    #[doc = "Storage type."]
    #[serde(rename = "storageModelType", default, skip_serializing_if = "Option::is_none")]
    pub storage_model_type: Option<backup_resource_vault_config::StorageModelType>,
    #[doc = "Storage type."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<backup_resource_vault_config::StorageType>,
    #[doc = "Locked or Unlocked. Once a machine is registered against a resource, the storageTypeState is always Locked."]
    #[serde(rename = "storageTypeState", default, skip_serializing_if = "Option::is_none")]
    pub storage_type_state: Option<backup_resource_vault_config::StorageTypeState>,
    #[doc = "Enabled or Disabled."]
    #[serde(rename = "enhancedSecurityState", default, skip_serializing_if = "Option::is_none")]
    pub enhanced_security_state: Option<backup_resource_vault_config::EnhancedSecurityState>,
    #[doc = "Soft Delete feature state"]
    #[serde(rename = "softDeleteFeatureState", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_feature_state: Option<backup_resource_vault_config::SoftDeleteFeatureState>,
    #[doc = "ResourceGuard Operation Requests"]
    #[serde(rename = "resourceGuardOperationRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operation_requests: Vec<String>,
    #[doc = "Is soft delete feature state editable"]
    #[serde(rename = "isSoftDeleteFeatureStateEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_soft_delete_feature_state_editable: Option<bool>,
}
impl BackupResourceVaultConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_resource_vault_config {
    use super::*;
    #[doc = "Storage type."]
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
    #[doc = "Enabled or Disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnhancedSecurityState")]
    pub enum EnhancedSecurityState {
        Invalid,
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnhancedSecurityState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnhancedSecurityState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnhancedSecurityState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("EnhancedSecurityState", 0u32, "Invalid"),
                Self::Enabled => serializer.serialize_unit_variant("EnhancedSecurityState", 1u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("EnhancedSecurityState", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Soft Delete feature state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SoftDeleteFeatureState")]
    pub enum SoftDeleteFeatureState {
        Invalid,
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SoftDeleteFeatureState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SoftDeleteFeatureState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SoftDeleteFeatureState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("SoftDeleteFeatureState", 0u32, "Invalid"),
                Self::Enabled => serializer.serialize_unit_variant("SoftDeleteFeatureState", 1u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SoftDeleteFeatureState", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backup resource vault config details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResourceVaultConfigResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Backup resource vault config details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupResourceVaultConfig>,
}
impl BackupResourceVaultConfigResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BackupStatus request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupStatusRequest {
    #[doc = "Container Type - VM, SQLPaaS, DPM, AzureFileShare..."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<backup_status_request::ResourceType>,
    #[doc = "Entire ARM resource id of the resource"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Protectable Item Logical Name"]
    #[serde(rename = "poLogicalName", default, skip_serializing_if = "Option::is_none")]
    pub po_logical_name: Option<String>,
}
impl BackupStatusRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_status_request {
    use super::*;
    #[doc = "Container Type - VM, SQLPaaS, DPM, AzureFileShare..."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceType")]
    pub enum ResourceType {
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
    impl FromStr for ResourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ResourceType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("ResourceType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("ResourceType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("ResourceType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("ResourceType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("ResourceType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("ResourceType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("ResourceType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("ResourceType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("ResourceType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("ResourceType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("ResourceType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("ResourceType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("ResourceType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("ResourceType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "BackupStatus response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupStatusResponse {
    #[doc = "Specifies whether the container is registered or not"]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<backup_status_response::ProtectionStatus>,
    #[doc = "Specifies the arm resource id of the vault"]
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[doc = "Specifies the fabric name - Azure or AD"]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<backup_status_response::FabricName>,
    #[doc = "Specifies the product specific container name. E.g. iaasvmcontainer;iaasvmcontainer;csname;vmname."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Specifies the product specific ds name. E.g. vm;iaasvmcontainer;csname;vmname."]
    #[serde(rename = "protectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
    #[doc = "ErrorCode in case of intent failed"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "ErrorMessage in case of intent failed."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Specifies the policy name which is used for protection"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Container registration status"]
    #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
}
impl BackupStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_status_response {
    use super::*;
    #[doc = "Specifies whether the container is registered or not"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionStatus")]
    pub enum ProtectionStatus {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionStatus", 0u32, "Invalid"),
                Self::NotProtected => serializer.serialize_unit_variant("ProtectionStatus", 1u32, "NotProtected"),
                Self::Protecting => serializer.serialize_unit_variant("ProtectionStatus", 2u32, "Protecting"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionStatus", 3u32, "Protected"),
                Self::ProtectionFailed => serializer.serialize_unit_variant("ProtectionStatus", 4u32, "ProtectionFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the fabric name - Azure or AD"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FabricName")]
    pub enum FabricName {
        Invalid,
        Azure,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FabricName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FabricName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FabricName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("FabricName", 0u32, "Invalid"),
                Self::Azure => serializer.serialize_unit_variant("FabricName", 1u32, "Azure"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Localized display information of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryDisplay {
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "ResourceType for which this Operation can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operations Name itself."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation having details of what operation is about."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ClientDiscoveryDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox log specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForLogSpecification {
    #[doc = "Name for shoebox log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "blob duration of shoebox log specification"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl ClientDiscoveryForLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox properties in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForProperties {
    #[doc = "Class to represent shoebox service specification in json client discovery."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ClientDiscoveryForServiceSpecification>,
}
impl ClientDiscoveryForProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox service specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForServiceSpecification {
    #[doc = "List of log specifications of this operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<ClientDiscoveryForLogSpecification>,
}
impl ClientDiscoveryForServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations List response which contains list of available APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryResponse {
    #[doc = "List of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientDiscoveryValueForSingleApi>,
    #[doc = "Link to the next chunk of Response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClientDiscoveryResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClientDiscoveryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryValueForSingleApi {
    #[doc = "Name of the Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display information of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClientDiscoveryDisplay>,
    #[doc = "The intended executor of the operation;governs the display of the operation in the RBAC UX and the audit logs UX"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Class to represent shoebox properties in json client discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClientDiscoveryForProperties>,
}
impl ClientDiscoveryValueForSingleApi {
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
#[doc = "An error response from the Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Container Instance service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
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
#[doc = "An error response from the Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container identity information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerIdentityInfo {
    #[doc = "Unique name of the container"]
    #[serde(rename = "uniqueName", default, skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    #[doc = "Protection container identity - AAD Tenant"]
    #[serde(rename = "aadTenantId", default, skip_serializing_if = "Option::is_none")]
    pub aad_tenant_id: Option<String>,
    #[doc = "Protection container identity - AAD Service Principal"]
    #[serde(rename = "servicePrincipalClientId", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_client_id: Option<String>,
    #[doc = "Protection container identity - Audience"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}
impl ContainerIdentityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional information of the DPMContainer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DpmContainerExtendedInfo {
    #[doc = "Last refresh time of the DPMContainer."]
    #[serde(rename = "lastRefreshedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_refreshed_at: Option<time::OffsetDateTime>,
}
impl DpmContainerExtendedInfo {
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
#[doc = "Daily retention format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DailyRetentionFormat {
    #[doc = "List of days of the month."]
    #[serde(rename = "daysOfTheMonth", default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_the_month: Vec<Day>,
}
impl DailyRetentionFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Daily retention schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DailyRetentionSchedule {
    #[doc = "Retention times of retention policy."]
    #[serde(rename = "retentionTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub retention_times: Vec<time::OffsetDateTime>,
    #[doc = "Retention duration."]
    #[serde(rename = "retentionDuration", default, skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<RetentionDuration>,
}
impl DailyRetentionSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DailySchedule {
    #[doc = "List of times of day this schedule has to be run."]
    #[serde(rename = "scheduleRunTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_run_times: Vec<time::OffsetDateTime>,
}
impl DailySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Day of the week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Day {
    #[doc = "Date of the month"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i32>,
    #[doc = "Whether Date is last date of month"]
    #[serde(rename = "isLast", default, skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
}
impl Day {
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
#[doc = "This is used to represent the various nodes of the distributed container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedNodesInfo {
    #[doc = "Name of the node under a distributed container."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Status of this Node.\r\nFailed | Succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error Detail class which encapsulates Code, Message and Recommendations."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
}
impl DistributedNodesInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Protection Manager (DPM) specific backup engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmBackupEngine {
    #[serde(flatten)]
    pub backup_engine_base: BackupEngineBase,
}
impl DpmBackupEngine {
    pub fn new(backup_engine_base: BackupEngineBase) -> Self {
        Self { backup_engine_base }
    }
}
#[doc = "DPM workload-specific protection container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
    #[doc = "Specifies whether the container is re-registrable."]
    #[serde(rename = "canReRegister", default, skip_serializing_if = "Option::is_none")]
    pub can_re_register: Option<bool>,
    #[doc = "ID of container."]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[doc = "Number of protected items in the BackupEngine"]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i64>,
    #[doc = "Backup engine Agent version"]
    #[serde(rename = "dpmAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub dpm_agent_version: Option<String>,
    #[doc = "List of BackupEngines protecting the container"]
    #[serde(rename = "dpmServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dpm_servers: Vec<String>,
    #[doc = "To check if upgrade available"]
    #[serde(rename = "upgradeAvailable", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_available: Option<bool>,
    #[doc = "Protection status of the container."]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<String>,
    #[doc = "Additional information of the DPMContainer."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<DpmContainerExtendedInfo>,
}
impl DpmContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self {
            protection_container,
            can_re_register: None,
            container_id: None,
            protected_item_count: None,
            dpm_agent_version: None,
            dpm_servers: Vec::new(),
            upgrade_available: None,
            protection_status: None,
            extended_info: None,
        }
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
#[doc = "This class is used to send blob details after exporting jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportJobsOperationResultInfo {
    #[serde(flatten)]
    pub operation_result_info_base: OperationResultInfoBase,
    #[doc = "URL of the blob into which the serialized string of list of jobs is exported."]
    #[serde(rename = "blobUrl", default, skip_serializing_if = "Option::is_none")]
    pub blob_url: Option<String>,
    #[doc = "SAS key to access the blob. It expires in 15 mins."]
    #[serde(rename = "blobSasKey", default, skip_serializing_if = "Option::is_none")]
    pub blob_sas_key: Option<String>,
    #[doc = "URL of the blob into which the ExcelFile is uploaded."]
    #[serde(rename = "excelFileBlobUrl", default, skip_serializing_if = "Option::is_none")]
    pub excel_file_blob_url: Option<String>,
    #[doc = "SAS key to access the blob. It expires in 15 mins."]
    #[serde(rename = "excelFileBlobSasKey", default, skip_serializing_if = "Option::is_none")]
    pub excel_file_blob_sas_key: Option<String>,
}
impl ExportJobsOperationResultInfo {
    pub fn new(operation_result_info_base: OperationResultInfoBase) -> Self {
        Self {
            operation_result_info_base,
            blob_url: None,
            blob_sas_key: None,
            excel_file_blob_url: None,
            excel_file_blob_sas_key: None,
        }
    }
}
#[doc = "Extended Properties for Azure IaasVM Backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedProperties {
    #[serde(rename = "diskExclusionProperties", default, skip_serializing_if = "Option::is_none")]
    pub disk_exclusion_properties: Option<DiskExclusionProperties>,
    #[doc = "Linux VM name"]
    #[serde(rename = "linuxVmApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub linux_vm_application_name: Option<String>,
}
impl ExtendedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for feature request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureSupportRequest {
    #[doc = "backup support feature type."]
    #[serde(rename = "featureType")]
    pub feature_type: String,
}
impl FeatureSupportRequest {
    pub fn new(feature_type: String) -> Self {
        Self { feature_type }
    }
}
#[doc = "Base class for generic container of backup items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
    #[doc = "Name of the container's fabric"]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "Container extended information"]
    #[serde(rename = "extendedInformation", default, skip_serializing_if = "Option::is_none")]
    pub extended_information: Option<GenericContainerExtendedInfo>,
}
impl GenericContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self {
            protection_container,
            fabric_name: None,
            extended_information: None,
        }
    }
}
#[doc = "Container extended information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericContainerExtendedInfo {
    #[doc = "Public key of container cert"]
    #[serde(rename = "rawCertData", default, skip_serializing_if = "Option::is_none")]
    pub raw_cert_data: Option<String>,
    #[doc = "Container identity information"]
    #[serde(rename = "containerIdentityInfo", default, skip_serializing_if = "Option::is_none")]
    pub container_identity_info: Option<ContainerIdentityInfo>,
    #[doc = "Azure Backup Service Endpoints for the container"]
    #[serde(rename = "serviceEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoints: Option<serde_json::Value>,
}
impl GenericContainerExtendedInfo {
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
#[doc = "Azure VM (Mercury) workload-specific backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericProtectionPolicy {
    #[serde(flatten)]
    pub protection_policy: ProtectionPolicy,
    #[doc = "List of sub-protection policies which includes schedule and retention"]
    #[serde(rename = "subProtectionPolicy", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_protection_policy: Vec<SubProtectionPolicy>,
    #[doc = "TimeZone optional input as string. For example: TimeZone = \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Name of this policy's fabric."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
}
impl GenericProtectionPolicy {
    pub fn new(protection_policy: ProtectionPolicy) -> Self {
        Self {
            protection_policy,
            sub_protection_policy: Vec::new(),
            time_zone: None,
            fabric_name: None,
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
#[doc = "Filters to list backup items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetProtectedItemQueryObject {
    #[doc = "Specifies if the additional information should be provided for this item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
}
impl GetProtectedItemQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HourlySchedule {
    #[doc = "Interval at which backup needs to be triggered. For hourly the value\r\n can be 4/6/8/12"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "To specify start time of the backup window"]
    #[serde(rename = "scheduleWindowStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub schedule_window_start_time: Option<time::OffsetDateTime>,
    #[doc = "To specify duration of the backup window"]
    #[serde(rename = "scheduleWindowDuration", default, skip_serializing_if = "Option::is_none")]
    pub schedule_window_duration: Option<i32>,
}
impl HourlySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to Provision ILR API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IlrRequest {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl IlrRequest {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Parameters to Provision ILR API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IlrRequestResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Parameters to Provision ILR API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IlrRequest>,
}
impl IlrRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IaaS VM workload-specific container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaaSvmContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
    #[doc = "Fully qualified ARM url of the virtual machine represented by this Azure IaaS VM container."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Specifies whether the container represents a Classic or an Azure Resource Manager VM."]
    #[serde(rename = "virtualMachineVersion", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_version: Option<String>,
    #[doc = "Resource group name of Recovery Services Vault."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl IaaSvmContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self {
            protection_container,
            virtual_machine_id: None,
            virtual_machine_version: None,
            resource_group: None,
        }
    }
}
#[doc = "IaaS VM workload-specific backup item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaaSvmProtectableItem {
    #[serde(flatten)]
    pub workload_protectable_item: WorkloadProtectableItem,
    #[doc = "Fully qualified ARM ID of the virtual machine."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "Specifies whether the container represents a Classic or an Azure Resource Manager VM."]
    #[serde(rename = "virtualMachineVersion", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_version: Option<String>,
    #[doc = "Resource group name of Recovery Services Vault."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl IaaSvmProtectableItem {
    pub fn new(workload_protectable_item: WorkloadProtectableItem) -> Self {
        Self {
            workload_protectable_item,
            virtual_machine_id: None,
            virtual_machine_version: None,
            resource_group: None,
        }
    }
}
#[doc = "IaaS VM workload-specific backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaasVmBackupRequest {
    #[serde(flatten)]
    pub backup_request: BackupRequest,
    #[doc = "Backup copy will expire after the time specified (UTC)."]
    #[serde(rename = "recoveryPointExpiryTimeInUTC", default, with = "azure_core::date::rfc3339::option")]
    pub recovery_point_expiry_time_in_utc: Option<time::OffsetDateTime>,
}
impl IaasVmBackupRequest {
    pub fn new(backup_request: BackupRequest) -> Self {
        Self {
            backup_request,
            recovery_point_expiry_time_in_utc: None,
        }
    }
}
#[doc = "Restore files/folders from a backup copy of IaaS VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaasVmilrRegistrationRequest {
    #[serde(flatten)]
    pub ilr_request: IlrRequest,
    #[doc = "ID of the IaaS VM backup copy from where the files/folders have to be restored."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "Fully qualified ARM ID of the virtual machine whose the files / folders have to be restored."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
    #[doc = "iSCSI initiator name."]
    #[serde(rename = "initiatorName", default, skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    #[doc = "Whether to renew existing registration with the iSCSI server."]
    #[serde(rename = "renewExistingRegistration", default, skip_serializing_if = "Option::is_none")]
    pub renew_existing_registration: Option<bool>,
}
impl IaasVmilrRegistrationRequest {
    pub fn new(ilr_request: IlrRequest) -> Self {
        Self {
            ilr_request,
            recovery_point_id: None,
            virtual_machine_id: None,
            initiator_name: None,
            renew_existing_registration: None,
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
    pub recovery_point_tier_details: Vec<RecoveryPointTierInformationV2>,
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
#[doc = "IaaS VM workload-specific restore with integrated rehydration of recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IaasVmRestoreWithRehydrationRequest {
    #[serde(flatten)]
    pub iaas_vm_restore_request: IaasVmRestoreRequest,
    #[doc = "RP Rehydration Info"]
    #[serde(rename = "recoveryPointRehydrationInfo", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_rehydration_info: Option<RecoveryPointRehydrationInfo>,
}
impl IaasVmRestoreWithRehydrationRequest {
    pub fn new(iaas_vm_restore_request: IaasVmRestoreRequest) -> Self {
        Self {
            iaas_vm_restore_request,
            recovery_point_rehydration_info: None,
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
#[doc = "Details about inquired protectable items under a given container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InquiryInfo {
    #[doc = "Inquiry Status for this container such as\r\nInProgress | Failed | Succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error Detail class which encapsulates Code, Message and Recommendations."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[doc = "Inquiry Details which will have workload specific details.\r\nFor e.g. - For SQL and oracle this will contain different details."]
    #[serde(rename = "inquiryDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub inquiry_details: Vec<WorkloadInquiryDetails>,
}
impl InquiryInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Validation for inquired protectable items under a given container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InquiryValidation {
    #[doc = "Status for the Inquiry Validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error Detail class which encapsulates Code, Message and Recommendations."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[doc = "Error Additional Detail in case the status is non-success."]
    #[serde(rename = "additionalDetail", default, skip_serializing_if = "Option::is_none")]
    pub additional_detail: Option<String>,
}
impl InquiryValidation {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstantRpAdditionalDetails {
    #[serde(rename = "azureBackupRGNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub azure_backup_rg_name_prefix: Option<String>,
    #[serde(rename = "azureBackupRGNameSuffix", default, skip_serializing_if = "Option::is_none")]
    pub azure_backup_rg_name_suffix: Option<String>,
}
impl InstantRpAdditionalDetails {
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
#[doc = "ListRecoveryPointsRecommendedForMoveRequest Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListRecoveryPointsRecommendedForMoveRequest {
    #[doc = "Gets the class type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "List of Recovery Points excluded from Move"]
    #[serde(rename = "excludedRPList", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_rp_list: Vec<String>,
}
impl ListRecoveryPointsRecommendedForMoveRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log policy schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSchedulePolicy {
    #[serde(flatten)]
    pub schedule_policy: SchedulePolicy,
    #[doc = "Frequency of the log schedule operation of this policy in minutes."]
    #[serde(rename = "scheduleFrequencyInMins", default, skip_serializing_if = "Option::is_none")]
    pub schedule_frequency_in_mins: Option<i32>,
}
impl LogSchedulePolicy {
    pub fn new(schedule_policy: SchedulePolicy) -> Self {
        Self {
            schedule_policy,
            schedule_frequency_in_mins: None,
        }
    }
}
#[doc = "Long term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LongTermRetentionPolicy {
    #[serde(flatten)]
    pub retention_policy: RetentionPolicy,
    #[doc = "Daily retention schedule."]
    #[serde(rename = "dailySchedule", default, skip_serializing_if = "Option::is_none")]
    pub daily_schedule: Option<DailyRetentionSchedule>,
    #[doc = "Weekly retention schedule."]
    #[serde(rename = "weeklySchedule", default, skip_serializing_if = "Option::is_none")]
    pub weekly_schedule: Option<WeeklyRetentionSchedule>,
    #[doc = "Monthly retention schedule."]
    #[serde(rename = "monthlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub monthly_schedule: Option<MonthlyRetentionSchedule>,
    #[doc = "Yearly retention schedule."]
    #[serde(rename = "yearlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub yearly_schedule: Option<YearlyRetentionSchedule>,
}
impl LongTermRetentionPolicy {
    pub fn new(retention_policy: RetentionPolicy) -> Self {
        Self {
            retention_policy,
            daily_schedule: None,
            weekly_schedule: None,
            monthly_schedule: None,
            yearly_schedule: None,
        }
    }
}
#[doc = "Long term policy schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LongTermSchedulePolicy {
    #[serde(flatten)]
    pub schedule_policy: SchedulePolicy,
}
impl LongTermSchedulePolicy {
    pub fn new(schedule_policy: SchedulePolicy) -> Self {
        Self { schedule_policy }
    }
}
#[doc = "MAB workload-specific Health Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MabContainerHealthDetails {
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
impl MabContainerHealthDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container with items backed up using MAB backup engine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabContainer {
    #[serde(flatten)]
    pub protection_container: ProtectionContainer,
    #[doc = "Can the container be registered one more time."]
    #[serde(rename = "canReRegister", default, skip_serializing_if = "Option::is_none")]
    pub can_re_register: Option<bool>,
    #[doc = "ContainerID represents the container."]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<i64>,
    #[doc = "Number of items backed up in this container."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i64>,
    #[doc = "Agent version of this container."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Additional information of the container."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<MabContainerExtendedInfo>,
    #[doc = "Health details on this mab container."]
    #[serde(rename = "mabContainerHealthDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub mab_container_health_details: Vec<MabContainerHealthDetails>,
    #[doc = "Health state of mab container."]
    #[serde(rename = "containerHealthState", default, skip_serializing_if = "Option::is_none")]
    pub container_health_state: Option<String>,
}
impl MabContainer {
    pub fn new(protection_container: ProtectionContainer) -> Self {
        Self {
            protection_container,
            can_re_register: None,
            container_id: None,
            protected_item_count: None,
            agent_version: None,
            extended_info: None,
            mab_container_health_details: Vec::new(),
            container_health_state: None,
        }
    }
}
#[doc = "Additional information of the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MabContainerExtendedInfo {
    #[doc = "Time stamp when this container was refreshed."]
    #[serde(rename = "lastRefreshedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_refreshed_at: Option<time::OffsetDateTime>,
    #[doc = "Type of backup items associated with this container."]
    #[serde(rename = "backupItemType", default, skip_serializing_if = "Option::is_none")]
    pub backup_item_type: Option<mab_container_extended_info::BackupItemType>,
    #[doc = "List of backup items associated with this container."]
    #[serde(rename = "backupItems", default, skip_serializing_if = "Vec::is_empty")]
    pub backup_items: Vec<String>,
    #[doc = "Backup policy associated with this container."]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Latest backup status of this container."]
    #[serde(rename = "lastBackupStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_status: Option<String>,
}
impl MabContainerExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mab_container_extended_info {
    use super::*;
    #[doc = "Type of backup items associated with this container."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupItemType")]
    pub enum BackupItemType {
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
    impl FromStr for BackupItemType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupItemType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupItemType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("BackupItemType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("BackupItemType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("BackupItemType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("BackupItemType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("BackupItemType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("BackupItemType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("BackupItemType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("BackupItemType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("BackupItemType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("BackupItemType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("BackupItemType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("BackupItemType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("BackupItemType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("BackupItemType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("BackupItemType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Mab container-specific backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabProtectionPolicy {
    #[serde(flatten)]
    pub protection_policy: ProtectionPolicy,
    #[doc = "Base class for backup schedule."]
    #[serde(rename = "schedulePolicy", default, skip_serializing_if = "Option::is_none")]
    pub schedule_policy: Option<SchedulePolicy>,
    #[doc = "Base class for retention policy."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl MabProtectionPolicy {
    pub fn new(protection_policy: ProtectionPolicy) -> Self {
        Self {
            protection_policy,
            schedule_policy: None,
            retention_policy: None,
        }
    }
}
#[doc = "Monthly retention schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonthlyRetentionSchedule {
    #[doc = "Retention schedule format type for monthly retention policy."]
    #[serde(rename = "retentionScheduleFormatType", default, skip_serializing_if = "Option::is_none")]
    pub retention_schedule_format_type: Option<monthly_retention_schedule::RetentionScheduleFormatType>,
    #[doc = "Daily retention format."]
    #[serde(rename = "retentionScheduleDaily", default, skip_serializing_if = "Option::is_none")]
    pub retention_schedule_daily: Option<DailyRetentionFormat>,
    #[doc = "Weekly retention format."]
    #[serde(rename = "retentionScheduleWeekly", default, skip_serializing_if = "Option::is_none")]
    pub retention_schedule_weekly: Option<WeeklyRetentionFormat>,
    #[doc = "Retention times of retention policy."]
    #[serde(rename = "retentionTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub retention_times: Vec<time::OffsetDateTime>,
    #[doc = "Retention duration."]
    #[serde(rename = "retentionDuration", default, skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<RetentionDuration>,
}
impl MonthlyRetentionSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monthly_retention_schedule {
    use super::*;
    #[doc = "Retention schedule format type for monthly retention policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RetentionScheduleFormatType")]
    pub enum RetentionScheduleFormatType {
        Invalid,
        Daily,
        Weekly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RetentionScheduleFormatType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RetentionScheduleFormatType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RetentionScheduleFormatType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RetentionScheduleFormatType", 0u32, "Invalid"),
                Self::Daily => serializer.serialize_unit_variant("RetentionScheduleFormatType", 1u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("RetentionScheduleFormatType", 2u32, "Weekly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveRpAcrossTiersRequest {
    #[doc = "Gets the class type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "Source tier from where RP needs to be moved"]
    #[serde(rename = "sourceTierType", default, skip_serializing_if = "Option::is_none")]
    pub source_tier_type: Option<move_rp_across_tiers_request::SourceTierType>,
    #[doc = "Target tier where RP needs to be moved"]
    #[serde(rename = "targetTierType", default, skip_serializing_if = "Option::is_none")]
    pub target_tier_type: Option<move_rp_across_tiers_request::TargetTierType>,
}
impl MoveRpAcrossTiersRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod move_rp_across_tiers_request {
    use super::*;
    #[doc = "Source tier from where RP needs to be moved"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceTierType {
        Invalid,
        #[serde(rename = "InstantRP")]
        InstantRp,
        #[serde(rename = "HardenedRP")]
        HardenedRp,
        #[serde(rename = "ArchivedRP")]
        ArchivedRp,
    }
    #[doc = "Target tier where RP needs to be moved"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetTierType {
        Invalid,
        #[serde(rename = "InstantRP")]
        InstantRp,
        #[serde(rename = "HardenedRP")]
        HardenedRp,
        #[serde(rename = "ArchivedRP")]
        ArchivedRp,
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
#[doc = "Operation result info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultInfo {
    #[serde(flatten)]
    pub operation_result_info_base: OperationResultInfoBase,
    #[doc = "List of jobs created by this operation."]
    #[serde(rename = "jobList", default, skip_serializing_if = "Vec::is_empty")]
    pub job_list: Vec<String>,
}
impl OperationResultInfo {
    pub fn new(operation_result_info_base: OperationResultInfoBase) -> Self {
        Self {
            operation_result_info_base,
            job_list: Vec::new(),
        }
    }
}
#[doc = "Base class for operation result info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultInfoBase {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl OperationResultInfoBase {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Base class for operation result info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultInfoBaseResource {
    #[serde(flatten)]
    pub operation_worker_response: OperationWorkerResponse,
    #[doc = "Base class for operation result info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationResultInfoBase>,
}
impl OperationResultInfoBaseResource {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Operation status extended info for ValidateOperation action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusValidateOperationExtendedInfo {
    #[serde(flatten)]
    pub operation_status_extended_info: OperationStatusExtendedInfo,
    #[doc = "Base class for validate operation response."]
    #[serde(rename = "validateOperationResponse", default, skip_serializing_if = "Option::is_none")]
    pub validate_operation_response: Option<ValidateOperationResponse>,
}
impl OperationStatusValidateOperationExtendedInfo {
    pub fn new(operation_status_extended_info: OperationStatusExtendedInfo) -> Self {
        Self {
            operation_status_extended_info,
            validate_operation_response: None,
        }
    }
}
#[doc = "This is the base class for operation result responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationWorkerResponse {
    #[doc = "HTTP Status Code of the operation."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<operation_worker_response::StatusCode>,
    #[doc = "HTTP headers associated with this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}
impl OperationWorkerResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_worker_response {
    use super::*;
    #[doc = "HTTP Status Code of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusCode {
        Continue,
        SwitchingProtocols,
        #[serde(rename = "OK")]
        Ok,
        Created,
        Accepted,
        NonAuthoritativeInformation,
        NoContent,
        ResetContent,
        PartialContent,
        MultipleChoices,
        Ambiguous,
        MovedPermanently,
        Moved,
        Found,
        Redirect,
        SeeOther,
        RedirectMethod,
        NotModified,
        UseProxy,
        Unused,
        TemporaryRedirect,
        RedirectKeepVerb,
        BadRequest,
        Unauthorized,
        PaymentRequired,
        Forbidden,
        NotFound,
        MethodNotAllowed,
        NotAcceptable,
        ProxyAuthenticationRequired,
        RequestTimeout,
        Conflict,
        Gone,
        LengthRequired,
        PreconditionFailed,
        RequestEntityTooLarge,
        RequestUriTooLong,
        UnsupportedMediaType,
        RequestedRangeNotSatisfiable,
        ExpectationFailed,
        UpgradeRequired,
        InternalServerError,
        NotImplemented,
        BadGateway,
        ServiceUnavailable,
        GatewayTimeout,
        HttpVersionNotSupported,
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
#[doc = "Pre-backup validation for Azure VM Workload provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreBackupValidation {
    #[doc = "Status of protectable item, i.e. InProgress,Succeeded,Failed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<pre_backup_validation::Status>,
    #[doc = "Error code of protectable item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Message corresponding to the error code for the protectable item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl PreBackupValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pre_backup_validation {
    use super::*;
    #[doc = "Status of protectable item, i.e. InProgress,Succeeded,Failed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        Success,
        Failed,
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
                Self::Success => serializer.serialize_unit_variant("Status", 1u32, "Success"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contract to validate if backup can be enabled on the given resource in a given vault and given configuration.\r\nIt will validate followings\r\n1. Vault capacity\r\n2. VM is already protected\r\n3. Any VM related configuration passed in properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreValidateEnableBackupRequest {
    #[doc = "ProtectedItem Type- VM, SqlDataBase, AzureFileShare etc"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<pre_validate_enable_backup_request::ResourceType>,
    #[doc = "ARM Virtual Machine Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "ARM id of the Recovery Services Vault"]
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[doc = "Configuration of VM if any needs to be validated like OS type etc"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}
impl PreValidateEnableBackupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pre_validate_enable_backup_request {
    use super::*;
    #[doc = "ProtectedItem Type- VM, SqlDataBase, AzureFileShare etc"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceType")]
    pub enum ResourceType {
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
    impl FromStr for ResourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ResourceType", 0u32, "Invalid"),
                Self::Vm => serializer.serialize_unit_variant("ResourceType", 1u32, "VM"),
                Self::FileFolder => serializer.serialize_unit_variant("ResourceType", 2u32, "FileFolder"),
                Self::AzureSqlDb => serializer.serialize_unit_variant("ResourceType", 3u32, "AzureSqlDb"),
                Self::Sqldb => serializer.serialize_unit_variant("ResourceType", 4u32, "SQLDB"),
                Self::Exchange => serializer.serialize_unit_variant("ResourceType", 5u32, "Exchange"),
                Self::Sharepoint => serializer.serialize_unit_variant("ResourceType", 6u32, "Sharepoint"),
                Self::VMwareVm => serializer.serialize_unit_variant("ResourceType", 7u32, "VMwareVM"),
                Self::SystemState => serializer.serialize_unit_variant("ResourceType", 8u32, "SystemState"),
                Self::Client => serializer.serialize_unit_variant("ResourceType", 9u32, "Client"),
                Self::GenericDataSource => serializer.serialize_unit_variant("ResourceType", 10u32, "GenericDataSource"),
                Self::SqlDataBase => serializer.serialize_unit_variant("ResourceType", 11u32, "SQLDataBase"),
                Self::AzureFileShare => serializer.serialize_unit_variant("ResourceType", 12u32, "AzureFileShare"),
                Self::SapHanaDatabase => serializer.serialize_unit_variant("ResourceType", 13u32, "SAPHanaDatabase"),
                Self::SapAseDatabase => serializer.serialize_unit_variant("ResourceType", 14u32, "SAPAseDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Response contract for enable backup validation request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreValidateEnableBackupResponse {
    #[doc = "Validation Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<pre_validate_enable_backup_response::Status>,
    #[doc = "Response error code"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Response error message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Recommended action for user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[doc = "Specifies the product specific container name. E.g. iaasvmcontainer;iaasvmcontainer;rgname;vmname. This is required\r\nfor portal"]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "Specifies the product specific ds name. E.g. vm;iaasvmcontainer;rgname;vmname. This is required for portal"]
    #[serde(rename = "protectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
}
impl PreValidateEnableBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pre_validate_enable_backup_response {
    use super::*;
    #[doc = "Validation Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        Succeeded,
        Failed,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Prepare DataMove Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareDataMoveRequest {
    #[doc = "ARM Id of target vault"]
    #[serde(rename = "targetResourceId")]
    pub target_resource_id: String,
    #[doc = "Target Region"]
    #[serde(rename = "targetRegion")]
    pub target_region: String,
    #[doc = "DataMove Level"]
    #[serde(rename = "dataMoveLevel")]
    pub data_move_level: prepare_data_move_request::DataMoveLevel,
    #[doc = "Source Container ArmIds\r\nThis needs to be populated only if DataMoveLevel is set to container"]
    #[serde(rename = "sourceContainerArmIds", default, skip_serializing_if = "Vec::is_empty")]
    pub source_container_arm_ids: Vec<String>,
    #[doc = "Ignore the artifacts which are already moved."]
    #[serde(rename = "ignoreMoved", default, skip_serializing_if = "Option::is_none")]
    pub ignore_moved: Option<bool>,
}
impl PrepareDataMoveRequest {
    pub fn new(target_resource_id: String, target_region: String, data_move_level: prepare_data_move_request::DataMoveLevel) -> Self {
        Self {
            target_resource_id,
            target_region,
            data_move_level,
            source_container_arm_ids: Vec::new(),
            ignore_moved: None,
        }
    }
}
pub mod prepare_data_move_request {
    use super::*;
    #[doc = "DataMove Level"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataMoveLevel")]
    pub enum DataMoveLevel {
        Invalid,
        Vault,
        Container,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataMoveLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataMoveLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataMoveLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("DataMoveLevel", 0u32, "Invalid"),
                Self::Vault => serializer.serialize_unit_variant("DataMoveLevel", 1u32, "Vault"),
                Self::Container => serializer.serialize_unit_variant("DataMoveLevel", 2u32, "Container"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Prepare DataMove Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareDataMoveResponse {
    #[serde(flatten)]
    pub vault_storage_config_operation_result_response: VaultStorageConfigOperationResultResponse,
    #[doc = "Co-relationId for move operation"]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Source Vault Properties"]
    #[serde(rename = "sourceVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub source_vault_properties: Option<serde_json::Value>,
}
impl PrepareDataMoveResponse {
    pub fn new(vault_storage_config_operation_result_response: VaultStorageConfigOperationResultResponse) -> Self {
        Self {
            vault_storage_config_operation_result_response,
            correlation_id: None,
            source_vault_properties: None,
        }
    }
}
#[doc = "The Private Endpoint network resource that is linked to the Private Endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Gets or sets id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Endpoint Connection Response Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[doc = "Gets or sets provisioning state of the private endpoint connection"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection::ProvisioningState>,
    #[doc = "The Private Endpoint network resource that is linked to the Private Endpoint connection"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "Private Link Service Connection State"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection {
    use super::*;
    #[doc = "Gets or sets provisioning state of the private endpoint connection"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        Failed,
        Pending,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Private Endpoint Connection Response Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Private Endpoint Connection Response Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Link Service Connection State"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "Gets or sets the status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<private_link_service_connection_state::Status>,
    #[doc = "Gets or sets description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets actions required"]
    #[serde(rename = "actionRequired", default, skip_serializing_if = "Option::is_none")]
    pub action_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "Gets or sets the status"]
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
#[doc = "Protectable Container Class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectableContainer {
    #[doc = "Friendly name of the container."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Type of backup management for the container."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protectable_container::BackupManagementType>,
    #[doc = "Type of the container. The value of this property for\r\n1. Compute Azure VM is Microsoft.Compute/virtualMachines\r\n2. Classic Compute Azure VM is Microsoft.ClassicCompute/virtualMachines"]
    #[serde(rename = "protectableContainerType")]
    pub protectable_container_type: protectable_container::ProtectableContainerType,
    #[doc = "Status of health of the container."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[doc = "Fabric Id of the container such as ARM Id."]
    #[serde(rename = "containerId", default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
}
impl ProtectableContainer {
    pub fn new(protectable_container_type: protectable_container::ProtectableContainerType) -> Self {
        Self {
            friendly_name: None,
            backup_management_type: None,
            protectable_container_type,
            health_status: None,
            container_id: None,
        }
    }
}
pub mod protectable_container {
    use super::*;
    #[doc = "Type of backup management for the container."]
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
    #[doc = "Type of the container. The value of this property for\r\n1. Compute Azure VM is Microsoft.Compute/virtualMachines\r\n2. Classic Compute Azure VM is Microsoft.ClassicCompute/virtualMachines"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtectableContainerType {
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
        #[serde(rename = "Microsoft.ClassicCompute/virtualMachines")]
        MicrosoftClassicComputeVirtualMachines,
        #[serde(rename = "Microsoft.Compute/virtualMachines")]
        MicrosoftComputeVirtualMachines,
        AzureWorkloadContainer,
    }
}
#[doc = "Protectable Container Class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectableContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Protectable Container Class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectableContainer>,
}
impl ProtectableContainerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ProtectableContainer resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectableContainerResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectableContainerResource>,
}
impl azure_core::Continuable for ProtectableContainerResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProtectableContainerResourceList {
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
    #[doc = "Flag to identify whether datasource is protected in archive"]
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
    #[doc = "Name of the policy used for protection"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
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
            is_archive_enabled: None,
            policy_name: None,
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
#[doc = "Base class for container with backup items. Containers with specific workloads are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionContainer {
    #[doc = "Friendly name of the container."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Type of backup management for the container."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protection_container::BackupManagementType>,
    #[doc = "Status of registration of the container with the Recovery Services Vault."]
    #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
    #[doc = "Status of health of the container."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[doc = "Type of the container. The value of this property for: 1. Compute Azure VM is Microsoft.Compute/virtualMachines 2.\r\nClassic Compute Azure VM is Microsoft.ClassicCompute/virtualMachines 3. Windows machines (like MAB, DPM etc) is\r\nWindows 4. Azure SQL instance is AzureSqlContainer. 5. Storage containers is StorageContainer. 6. Azure workload\r\nBackup is VMAppContainer"]
    #[serde(rename = "containerType")]
    pub container_type: protection_container::ContainerType,
    #[doc = "Type of the protectable object associated with this container"]
    #[serde(rename = "protectableObjectType", default, skip_serializing_if = "Option::is_none")]
    pub protectable_object_type: Option<String>,
}
impl ProtectionContainer {
    pub fn new(container_type: protection_container::ContainerType) -> Self {
        Self {
            friendly_name: None,
            backup_management_type: None,
            registration_status: None,
            health_status: None,
            container_type,
            protectable_object_type: None,
        }
    }
}
pub mod protection_container {
    use super::*;
    #[doc = "Type of backup management for the container."]
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
    #[doc = "Type of the container. The value of this property for: 1. Compute Azure VM is Microsoft.Compute/virtualMachines 2.\r\nClassic Compute Azure VM is Microsoft.ClassicCompute/virtualMachines 3. Windows machines (like MAB, DPM etc) is\r\nWindows 4. Azure SQL instance is AzureSqlContainer. 5. Storage containers is StorageContainer. 6. Azure workload\r\nBackup is VMAppContainer"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContainerType")]
    pub enum ContainerType {
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
        #[serde(rename = "Microsoft.ClassicCompute/virtualMachines")]
        MicrosoftClassicComputeVirtualMachines,
        #[serde(rename = "Microsoft.Compute/virtualMachines")]
        MicrosoftComputeVirtualMachines,
        AzureWorkloadContainer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ContainerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContainerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContainerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ContainerType", 0u32, "Invalid"),
                Self::Unknown => serializer.serialize_unit_variant("ContainerType", 1u32, "Unknown"),
                Self::IaasVmContainer => serializer.serialize_unit_variant("ContainerType", 2u32, "IaasVMContainer"),
                Self::IaasVmServiceContainer => serializer.serialize_unit_variant("ContainerType", 3u32, "IaasVMServiceContainer"),
                Self::DpmContainer => serializer.serialize_unit_variant("ContainerType", 4u32, "DPMContainer"),
                Self::AzureBackupServerContainer => serializer.serialize_unit_variant("ContainerType", 5u32, "AzureBackupServerContainer"),
                Self::MabContainer => serializer.serialize_unit_variant("ContainerType", 6u32, "MABContainer"),
                Self::Cluster => serializer.serialize_unit_variant("ContainerType", 7u32, "Cluster"),
                Self::AzureSqlContainer => serializer.serialize_unit_variant("ContainerType", 8u32, "AzureSqlContainer"),
                Self::Windows => serializer.serialize_unit_variant("ContainerType", 9u32, "Windows"),
                Self::VCenter => serializer.serialize_unit_variant("ContainerType", 10u32, "VCenter"),
                Self::VmAppContainer => serializer.serialize_unit_variant("ContainerType", 11u32, "VMAppContainer"),
                Self::SqlagWorkLoadContainer => serializer.serialize_unit_variant("ContainerType", 12u32, "SQLAGWorkLoadContainer"),
                Self::StorageContainer => serializer.serialize_unit_variant("ContainerType", 13u32, "StorageContainer"),
                Self::GenericContainer => serializer.serialize_unit_variant("ContainerType", 14u32, "GenericContainer"),
                Self::MicrosoftClassicComputeVirtualMachines => {
                    serializer.serialize_unit_variant("ContainerType", 15u32, "Microsoft.ClassicCompute/virtualMachines")
                }
                Self::MicrosoftComputeVirtualMachines => {
                    serializer.serialize_unit_variant("ContainerType", 16u32, "Microsoft.Compute/virtualMachines")
                }
                Self::AzureWorkloadContainer => serializer.serialize_unit_variant("ContainerType", 17u32, "AzureWorkloadContainer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for container with backup items. Containers with specific workloads are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for container with backup items. Containers with specific workloads are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectionContainer>,
}
impl ProtectionContainerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ProtectionContainer resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectionContainerResource>,
}
impl azure_core::Continuable for ProtectionContainerResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProtectionContainerResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup ProtectionIntent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionIntent {
    #[doc = "backup protectionIntent type."]
    #[serde(rename = "protectionIntentItemType")]
    pub protection_intent_item_type: protection_intent::ProtectionIntentItemType,
    #[doc = "Type of backup management for the backed up item."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protection_intent::BackupManagementType>,
    #[doc = "ARM ID of the resource to be backed up."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "ID of the item which is getting protected, In case of Azure Vm , it is ProtectedItemId"]
    #[serde(rename = "itemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[doc = "ID of the backup policy with which this item is backed up."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Backup state of this backup item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<protection_intent::ProtectionState>,
}
impl ProtectionIntent {
    pub fn new(protection_intent_item_type: protection_intent::ProtectionIntentItemType) -> Self {
        Self {
            protection_intent_item_type,
            backup_management_type: None,
            source_resource_id: None,
            item_id: None,
            policy_id: None,
            protection_state: None,
        }
    }
}
pub mod protection_intent {
    use super::*;
    #[doc = "backup protectionIntent type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionIntentItemType")]
    pub enum ProtectionIntentItemType {
        Invalid,
        AzureResourceItem,
        RecoveryServiceVaultItem,
        AzureWorkloadContainerAutoProtectionIntent,
        AzureWorkloadAutoProtectionIntent,
        #[serde(rename = "AzureWorkloadSQLAutoProtectionIntent")]
        AzureWorkloadSqlAutoProtectionIntent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProtectionIntentItemType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProtectionIntentItemType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProtectionIntentItemType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ProtectionIntentItemType", 0u32, "Invalid"),
                Self::AzureResourceItem => serializer.serialize_unit_variant("ProtectionIntentItemType", 1u32, "AzureResourceItem"),
                Self::RecoveryServiceVaultItem => {
                    serializer.serialize_unit_variant("ProtectionIntentItemType", 2u32, "RecoveryServiceVaultItem")
                }
                Self::AzureWorkloadContainerAutoProtectionIntent => {
                    serializer.serialize_unit_variant("ProtectionIntentItemType", 3u32, "AzureWorkloadContainerAutoProtectionIntent")
                }
                Self::AzureWorkloadAutoProtectionIntent => {
                    serializer.serialize_unit_variant("ProtectionIntentItemType", 4u32, "AzureWorkloadAutoProtectionIntent")
                }
                Self::AzureWorkloadSqlAutoProtectionIntent => {
                    serializer.serialize_unit_variant("ProtectionIntentItemType", 5u32, "AzureWorkloadSQLAutoProtectionIntent")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
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
    #[doc = "Backup state of this backup item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
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
                Self::NotProtected => serializer.serialize_unit_variant("ProtectionState", 1u32, "NotProtected"),
                Self::Protecting => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protecting"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 3u32, "Protected"),
                Self::ProtectionFailed => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Filters to list protection intent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionIntentQueryObject {
    #[doc = "Backup management type for the backed up item"]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protection_intent_query_object::BackupManagementType>,
    #[doc = "Type of workload this item represents"]
    #[serde(rename = "itemType", default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<protection_intent_query_object::ItemType>,
    #[doc = "Parent name of the intent"]
    #[serde(rename = "parentName", default, skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[doc = "Item name of the intent"]
    #[serde(rename = "itemName", default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
}
impl ProtectionIntentQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod protection_intent_query_object {
    use super::*;
    #[doc = "Backup management type for the backed up item"]
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
    #[doc = "Type of workload this item represents"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ItemType")]
    pub enum ItemType {
        Invalid,
        #[serde(rename = "SQLInstance")]
        SqlInstance,
        #[serde(rename = "SQLAvailabilityGroupContainer")]
        SqlAvailabilityGroupContainer,
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
                Self::SqlInstance => serializer.serialize_unit_variant("ItemType", 1u32, "SQLInstance"),
                Self::SqlAvailabilityGroupContainer => serializer.serialize_unit_variant("ItemType", 2u32, "SQLAvailabilityGroupContainer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for backup ProtectionIntent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionIntentResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup ProtectionIntent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectionIntent>,
}
impl ProtectionIntentResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ProtectionIntent resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionIntentResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectionIntentResource>,
}
impl azure_core::Continuable for ProtectionIntentResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProtectionIntentResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup policy. Workload-specific backup policies are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionPolicy {
    #[doc = "Number of items associated with this policy."]
    #[serde(rename = "protectedItemsCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_items_count: Option<i32>,
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "backupManagementType")]
    pub backup_management_type: String,
    #[doc = "ResourceGuard Operation Requests"]
    #[serde(rename = "resourceGuardOperationRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operation_requests: Vec<String>,
}
impl ProtectionPolicy {
    pub fn new(backup_management_type: String) -> Self {
        Self {
            protected_items_count: None,
            backup_management_type,
            resource_guard_operation_requests: Vec::new(),
        }
    }
}
#[doc = "Filters the list backup policies API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionPolicyQueryObject {
    #[doc = "Backup management type for the backup policy."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protection_policy_query_object::BackupManagementType>,
    #[doc = "Fabric name for filter"]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "Workload type for the backup policy."]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<protection_policy_query_object::WorkloadType>,
}
impl ProtectionPolicyQueryObject {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod protection_policy_query_object {
    use super::*;
    #[doc = "Backup management type for the backup policy."]
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
    #[doc = "Workload type for the backup policy."]
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
#[doc = "Base class for backup policy. Workload-specific backup policies are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionPolicyResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup policy. Workload-specific backup policies are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectionPolicy>,
}
impl ProtectionPolicyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ProtectionPolicy resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionPolicyResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectionPolicyResource>,
}
impl azure_core::Continuable for ProtectionPolicyResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProtectionPolicyResourceList {
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
#[doc = "RP Rehydration Info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointRehydrationInfo {
    #[doc = "How long the rehydrated RP should be kept\r\nShould be ISO8601 Duration format e.g. \"P7D\""]
    #[serde(rename = "rehydrationRetentionDuration", default, skip_serializing_if = "Option::is_none")]
    pub rehydration_retention_duration: Option<String>,
    #[doc = "Rehydration Priority"]
    #[serde(rename = "rehydrationPriority", default, skip_serializing_if = "Option::is_none")]
    pub rehydration_priority: Option<recovery_point_rehydration_info::RehydrationPriority>,
}
impl RecoveryPointRehydrationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recovery_point_rehydration_info {
    use super::*;
    #[doc = "Rehydration Priority"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RehydrationPriority")]
    pub enum RehydrationPriority {
        Standard,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RehydrationPriority {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RehydrationPriority {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RehydrationPriority {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("RehydrationPriority", 0u32, "Standard"),
                Self::High => serializer.serialize_unit_variant("RehydrationPriority", 1u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "RecoveryPoint Tier Information V2"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointTierInformationV2 {
    #[serde(flatten)]
    pub recovery_point_tier_information: RecoveryPointTierInformation,
    #[doc = "Recovery point tier type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<recovery_point_tier_information_v2::Type>,
    #[doc = "Recovery point tier status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<recovery_point_tier_information_v2::Status>,
}
impl RecoveryPointTierInformationV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recovery_point_tier_information_v2 {
    use super::*;
    #[doc = "Recovery point tier type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Invalid,
        #[serde(rename = "InstantRP")]
        InstantRp,
        #[serde(rename = "HardenedRP")]
        HardenedRp,
        #[serde(rename = "ArchivedRP")]
        ArchivedRp,
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
                Self::InstantRp => serializer.serialize_unit_variant("Type", 1u32, "InstantRP"),
                Self::HardenedRp => serializer.serialize_unit_variant("Type", 2u32, "HardenedRP"),
                Self::ArchivedRp => serializer.serialize_unit_variant("Type", 3u32, "ArchivedRP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Recovery point tier status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Invalid,
        Valid,
        Disabled,
        Deleted,
        Rehydrated,
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
                Self::Valid => serializer.serialize_unit_variant("Status", 1u32, "Valid"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 2u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 3u32, "Deleted"),
                Self::Rehydrated => serializer.serialize_unit_variant("Status", 4u32, "Rehydrated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardOperationDetail {
    #[serde(rename = "vaultCriticalOperation", default, skip_serializing_if = "Option::is_none")]
    pub vault_critical_operation: Option<String>,
    #[serde(rename = "defaultResourceRequest", default, skip_serializing_if = "Option::is_none")]
    pub default_resource_request: Option<String>,
}
impl ResourceGuardOperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardProxyBase {
    #[serde(rename = "resourceGuardResourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_guard_resource_id: Option<String>,
    #[serde(rename = "resourceGuardOperationDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operation_details: Vec<ResourceGuardOperationDetail>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceGuardProxyBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardProxyBaseResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGuardProxyBase>,
}
impl ResourceGuardProxyBaseResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ResourceGuardProxyBase resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardProxyBaseResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceGuardProxyBaseResource>,
}
impl azure_core::Continuable for ResourceGuardProxyBaseResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ResourceGuardProxyBaseResourceList {
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
#[doc = "Base class for restore request. Workload-specific restore requests are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreRequestResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for restore request. Workload-specific restore requests are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestoreRequest>,
}
impl RestoreRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Retention duration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionDuration {
    #[doc = "Count of duration types. Retention duration is obtained by the counting the duration type Count times.\r\nFor example, when Count = 3 and DurationType = Weeks, retention duration will be three weeks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Retention duration type of retention policy."]
    #[serde(rename = "durationType", default, skip_serializing_if = "Option::is_none")]
    pub duration_type: Option<retention_duration::DurationType>,
}
impl RetentionDuration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod retention_duration {
    use super::*;
    #[doc = "Retention duration type of retention policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DurationType")]
    pub enum DurationType {
        Invalid,
        Days,
        Weeks,
        Months,
        Years,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DurationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DurationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DurationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("DurationType", 0u32, "Invalid"),
                Self::Days => serializer.serialize_unit_variant("DurationType", 1u32, "Days"),
                Self::Weeks => serializer.serialize_unit_variant("DurationType", 2u32, "Weeks"),
                Self::Months => serializer.serialize_unit_variant("DurationType", 3u32, "Months"),
                Self::Years => serializer.serialize_unit_variant("DurationType", 4u32, "Years"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "retentionPolicyType")]
    pub retention_policy_type: String,
}
impl RetentionPolicy {
    pub fn new(retention_policy_type: String) -> Self {
        Self { retention_policy_type }
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
#[doc = "Base class for backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchedulePolicy {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "schedulePolicyType")]
    pub schedule_policy_type: String,
}
impl SchedulePolicy {
    pub fn new(schedule_policy_type: String) -> Self {
        Self { schedule_policy_type }
    }
}
#[doc = "Base class for get security pin request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityPinBase {
    #[doc = "ResourceGuard Operation Requests"]
    #[serde(rename = "resourceGuardOperationRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operation_requests: Vec<String>,
}
impl SecurityPinBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common settings field for backup management"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Settings {
    #[doc = "TimeZone optional input as string. For example: TimeZone = \"Pacific Standard Time\"."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "SQL compression flag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issqlcompression: Option<bool>,
    #[doc = "Workload compression flag. This has been added so that 'isSqlCompression'\r\nwill be deprecated once clients upgrade to consider this flag."]
    #[serde(rename = "isCompression", default, skip_serializing_if = "Option::is_none")]
    pub is_compression: Option<bool>,
}
impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Simple policy retention."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleRetentionPolicy {
    #[serde(flatten)]
    pub retention_policy: RetentionPolicy,
    #[doc = "Retention duration."]
    #[serde(rename = "retentionDuration", default, skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<RetentionDuration>,
}
impl SimpleRetentionPolicy {
    pub fn new(retention_policy: RetentionPolicy) -> Self {
        Self {
            retention_policy,
            retention_duration: None,
        }
    }
}
#[doc = "Simple policy schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleSchedulePolicy {
    #[serde(flatten)]
    pub schedule_policy: SchedulePolicy,
    #[doc = "Frequency of the schedule operation of this policy."]
    #[serde(rename = "scheduleRunFrequency", default, skip_serializing_if = "Option::is_none")]
    pub schedule_run_frequency: Option<simple_schedule_policy::ScheduleRunFrequency>,
    #[doc = "List of days of week this schedule has to be run."]
    #[serde(rename = "scheduleRunDays", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_run_days: Vec<String>,
    #[doc = "List of times of day this schedule has to be run."]
    #[serde(rename = "scheduleRunTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_run_times: Vec<time::OffsetDateTime>,
    #[serde(rename = "hourlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub hourly_schedule: Option<HourlySchedule>,
    #[doc = "At every number weeks this schedule has to be run."]
    #[serde(rename = "scheduleWeeklyFrequency", default, skip_serializing_if = "Option::is_none")]
    pub schedule_weekly_frequency: Option<i32>,
}
impl SimpleSchedulePolicy {
    pub fn new(schedule_policy: SchedulePolicy) -> Self {
        Self {
            schedule_policy,
            schedule_run_frequency: None,
            schedule_run_days: Vec::new(),
            schedule_run_times: Vec::new(),
            hourly_schedule: None,
            schedule_weekly_frequency: None,
        }
    }
}
pub mod simple_schedule_policy {
    use super::*;
    #[doc = "Frequency of the schedule operation of this policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScheduleRunFrequency")]
    pub enum ScheduleRunFrequency {
        Invalid,
        Daily,
        Weekly,
        Hourly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScheduleRunFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScheduleRunFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScheduleRunFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ScheduleRunFrequency", 0u32, "Invalid"),
                Self::Daily => serializer.serialize_unit_variant("ScheduleRunFrequency", 1u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("ScheduleRunFrequency", 2u32, "Weekly"),
                Self::Hourly => serializer.serialize_unit_variant("ScheduleRunFrequency", 3u32, "Hourly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The V2 policy schedule for IaaS that supports hourly backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleSchedulePolicyV2 {
    #[serde(flatten)]
    pub schedule_policy: SchedulePolicy,
    #[doc = "Frequency of the schedule operation of this policy."]
    #[serde(rename = "scheduleRunFrequency", default, skip_serializing_if = "Option::is_none")]
    pub schedule_run_frequency: Option<simple_schedule_policy_v2::ScheduleRunFrequency>,
    #[serde(rename = "hourlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub hourly_schedule: Option<HourlySchedule>,
    #[serde(rename = "dailySchedule", default, skip_serializing_if = "Option::is_none")]
    pub daily_schedule: Option<DailySchedule>,
    #[serde(rename = "weeklySchedule", default, skip_serializing_if = "Option::is_none")]
    pub weekly_schedule: Option<WeeklySchedule>,
}
impl SimpleSchedulePolicyV2 {
    pub fn new(schedule_policy: SchedulePolicy) -> Self {
        Self {
            schedule_policy,
            schedule_run_frequency: None,
            hourly_schedule: None,
            daily_schedule: None,
            weekly_schedule: None,
        }
    }
}
pub mod simple_schedule_policy_v2 {
    use super::*;
    #[doc = "Frequency of the schedule operation of this policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScheduleRunFrequency")]
    pub enum ScheduleRunFrequency {
        Invalid,
        Daily,
        Weekly,
        Hourly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScheduleRunFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScheduleRunFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScheduleRunFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("ScheduleRunFrequency", 0u32, "Invalid"),
                Self::Daily => serializer.serialize_unit_variant("ScheduleRunFrequency", 1u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("ScheduleRunFrequency", 2u32, "Weekly"),
                Self::Hourly => serializer.serialize_unit_variant("ScheduleRunFrequency", 3u32, "Hourly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Sub-protection policy which includes schedule and retention"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubProtectionPolicy {
    #[doc = "Type of backup policy type"]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<sub_protection_policy::PolicyType>,
    #[doc = "Base class for backup schedule."]
    #[serde(rename = "schedulePolicy", default, skip_serializing_if = "Option::is_none")]
    pub schedule_policy: Option<SchedulePolicy>,
    #[doc = "Base class for retention policy."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl SubProtectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sub_protection_policy {
    use super::*;
    #[doc = "Type of backup policy type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PolicyType")]
    pub enum PolicyType {
        Invalid,
        Full,
        Differential,
        Log,
        CopyOnlyFull,
        Incremental,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PolicyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PolicyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PolicyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("PolicyType", 0u32, "Invalid"),
                Self::Full => serializer.serialize_unit_variant("PolicyType", 1u32, "Full"),
                Self::Differential => serializer.serialize_unit_variant("PolicyType", 2u32, "Differential"),
                Self::Log => serializer.serialize_unit_variant("PolicyType", 3u32, "Log"),
                Self::CopyOnlyFull => serializer.serialize_unit_variant("PolicyType", 4u32, "CopyOnlyFull"),
                Self::Incremental => serializer.serialize_unit_variant("PolicyType", 5u32, "Incremental"),
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
#[doc = "The token information details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenInformation {
    #[doc = "Token value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Expiry time of token."]
    #[serde(rename = "expiryTimeInUtcTicks", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time_in_utc_ticks: Option<i64>,
    #[doc = "Security PIN"]
    #[serde(rename = "securityPIN", default, skip_serializing_if = "Option::is_none")]
    pub security_pin: Option<String>,
}
impl TokenInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger DataMove Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerDataMoveRequest {
    #[doc = "ARM Id of source vault"]
    #[serde(rename = "sourceResourceId")]
    pub source_resource_id: String,
    #[doc = "Source Region"]
    #[serde(rename = "sourceRegion")]
    pub source_region: String,
    #[doc = "DataMove Level"]
    #[serde(rename = "dataMoveLevel")]
    pub data_move_level: trigger_data_move_request::DataMoveLevel,
    #[doc = "Correlation Id"]
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[doc = "Source Container ArmIds"]
    #[serde(rename = "sourceContainerArmIds", default, skip_serializing_if = "Vec::is_empty")]
    pub source_container_arm_ids: Vec<String>,
    #[doc = "Pause GC"]
    #[serde(rename = "pauseGC", default, skip_serializing_if = "Option::is_none")]
    pub pause_gc: Option<bool>,
}
impl TriggerDataMoveRequest {
    pub fn new(
        source_resource_id: String,
        source_region: String,
        data_move_level: trigger_data_move_request::DataMoveLevel,
        correlation_id: String,
    ) -> Self {
        Self {
            source_resource_id,
            source_region,
            data_move_level,
            correlation_id,
            source_container_arm_ids: Vec::new(),
            pause_gc: None,
        }
    }
}
pub mod trigger_data_move_request {
    use super::*;
    #[doc = "DataMove Level"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataMoveLevel")]
    pub enum DataMoveLevel {
        Invalid,
        Vault,
        Container,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataMoveLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataMoveLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataMoveLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("DataMoveLevel", 0u32, "Invalid"),
                Self::Vault => serializer.serialize_unit_variant("DataMoveLevel", 1u32, "Vault"),
                Self::Container => serializer.serialize_unit_variant("DataMoveLevel", 2u32, "Container"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request body of unlock delete API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnlockDeleteRequest {
    #[serde(rename = "resourceGuardOperationRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operation_requests: Vec<String>,
    #[serde(rename = "resourceToBeDeleted", default, skip_serializing_if = "Option::is_none")]
    pub resource_to_be_deleted: Option<String>,
}
impl UnlockDeleteRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of Unlock Delete API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnlockDeleteResponse {
    #[doc = "This is the time when unlock delete privileges will get expired."]
    #[serde(rename = "unlockDeleteExpiryTime", default, skip_serializing_if = "Option::is_none")]
    pub unlock_delete_expiry_time: Option<String>,
}
impl UnlockDeleteResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureRestoreValidation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateIaasVmRestoreOperationRequest {
    #[serde(flatten)]
    pub validate_restore_operation_request: ValidateRestoreOperationRequest,
}
impl ValidateIaasVmRestoreOperationRequest {
    pub fn new(validate_restore_operation_request: ValidateRestoreOperationRequest) -> Self {
        Self {
            validate_restore_operation_request,
        }
    }
}
#[doc = "Base class for validate operation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateOperationRequest {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl ValidateOperationRequest {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Base class for validate operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateOperationResponse {
    #[doc = "Gets the validation result"]
    #[serde(rename = "validationResults", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_results: Vec<ErrorDetail>,
}
impl ValidateOperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateOperationsResponse {
    #[doc = "Base class for validate operation response."]
    #[serde(rename = "validateOperationResponse", default, skip_serializing_if = "Option::is_none")]
    pub validate_operation_response: Option<ValidateOperationResponse>,
}
impl ValidateOperationsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureRestoreValidation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateRestoreOperationRequest {
    #[serde(flatten)]
    pub validate_operation_request: ValidateOperationRequest,
    #[doc = "Base class for restore request. Workload-specific restore requests are derived from this class."]
    #[serde(rename = "restoreRequest", default, skip_serializing_if = "Option::is_none")]
    pub restore_request: Option<RestoreRequest>,
}
impl ValidateRestoreOperationRequest {
    pub fn new(validate_operation_request: ValidateOperationRequest) -> Self {
        Self {
            validate_operation_request,
            restore_request: None,
        }
    }
}
#[doc = "Vault level Job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultJob {
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
    pub error_details: Vec<VaultJobErrorInfo>,
    #[doc = "Vault Job for CMK - has CMK specific info."]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<VaultJobExtendedInfo>,
}
impl VaultJob {
    pub fn new(job: Job) -> Self {
        Self {
            job,
            duration: None,
            actions_info: Vec::new(),
            error_details: Vec::new(),
            extended_info: None,
        }
    }
}
#[doc = "Vault Job specific error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultJobErrorInfo {
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
impl VaultJobErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault Job for CMK - has CMK specific info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultJobExtendedInfo {
    #[doc = "Job properties."]
    #[serde(rename = "propertyBag", default, skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
}
impl VaultJobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation result response for Vault Storage Config"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultStorageConfigOperationResultResponse {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl VaultStorageConfigOperationResultResponse {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Weekly retention format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeeklyRetentionFormat {
    #[doc = "List of days of the week."]
    #[serde(rename = "daysOfTheWeek", default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_the_week: Vec<String>,
    #[doc = "List of weeks of month."]
    #[serde(rename = "weeksOfTheMonth", default, skip_serializing_if = "Vec::is_empty")]
    pub weeks_of_the_month: Vec<String>,
}
impl WeeklyRetentionFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Weekly retention schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeeklyRetentionSchedule {
    #[doc = "List of days of week for weekly retention policy."]
    #[serde(rename = "daysOfTheWeek", default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_the_week: Vec<String>,
    #[doc = "Retention times of retention policy."]
    #[serde(rename = "retentionTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub retention_times: Vec<time::OffsetDateTime>,
    #[doc = "Retention duration."]
    #[serde(rename = "retentionDuration", default, skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<RetentionDuration>,
}
impl WeeklyRetentionSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeeklySchedule {
    #[serde(rename = "scheduleRunDays", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_run_days: Vec<String>,
    #[doc = "List of times of day this schedule has to be run."]
    #[serde(rename = "scheduleRunTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_run_times: Vec<time::OffsetDateTime>,
}
impl WeeklySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of an inquired protectable item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadInquiryDetails {
    #[doc = "Type of the Workload such as SQL, Oracle etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Contains the protectable item Count inside this Container."]
    #[serde(rename = "itemCount", default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[doc = "Validation for inquired protectable items under a given container."]
    #[serde(rename = "inquiryValidation", default, skip_serializing_if = "Option::is_none")]
    pub inquiry_validation: Option<InquiryValidation>,
}
impl WorkloadInquiryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup item. Workload-specific backup items are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadItem {
    #[doc = "Type of backup management to backup an item."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<String>,
    #[doc = "Type of workload for the backup management"]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[doc = "Type of the backup item."]
    #[serde(rename = "workloadItemType")]
    pub workload_item_type: String,
    #[doc = "Friendly name of the backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "State of the back up item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<workload_item::ProtectionState>,
}
impl WorkloadItem {
    pub fn new(workload_item_type: String) -> Self {
        Self {
            backup_management_type: None,
            workload_type: None,
            workload_item_type,
            friendly_name: None,
            protection_state: None,
        }
    }
}
pub mod workload_item {
    use super::*;
    #[doc = "State of the back up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
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
                Self::NotProtected => serializer.serialize_unit_variant("ProtectionState", 1u32, "NotProtected"),
                Self::Protecting => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protecting"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 3u32, "Protected"),
                Self::ProtectionFailed => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for backup item. Workload-specific backup items are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadItemResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup item. Workload-specific backup items are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadItem>,
}
impl WorkloadItemResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of WorkloadItem resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadItemResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadItemResource>,
}
impl azure_core::Continuable for WorkloadItemResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkloadItemResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for backup item. Workload-specific backup items are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadProtectableItem {
    #[doc = "Type of backup management to backup an item."]
    #[serde(rename = "backupManagementType", default, skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<String>,
    #[doc = "Type of workload for the backup management"]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[doc = "Type of the backup item."]
    #[serde(rename = "protectableItemType")]
    pub protectable_item_type: String,
    #[doc = "Friendly name of the backup item."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "State of the back up item."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<workload_protectable_item::ProtectionState>,
}
impl WorkloadProtectableItem {
    pub fn new(protectable_item_type: String) -> Self {
        Self {
            backup_management_type: None,
            workload_type: None,
            protectable_item_type,
            friendly_name: None,
            protection_state: None,
        }
    }
}
pub mod workload_protectable_item {
    use super::*;
    #[doc = "State of the back up item."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProtectionState")]
    pub enum ProtectionState {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
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
                Self::NotProtected => serializer.serialize_unit_variant("ProtectionState", 1u32, "NotProtected"),
                Self::Protecting => serializer.serialize_unit_variant("ProtectionState", 2u32, "Protecting"),
                Self::Protected => serializer.serialize_unit_variant("ProtectionState", 3u32, "Protected"),
                Self::ProtectionFailed => serializer.serialize_unit_variant("ProtectionState", 4u32, "ProtectionFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for backup item. Workload-specific backup items are derived from this class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadProtectableItemResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Base class for backup item. Workload-specific backup items are derived from this class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadProtectableItem>,
}
impl WorkloadProtectableItemResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of WorkloadProtectableItem resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadProtectableItemResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadProtectableItemResource>,
}
impl azure_core::Continuable for WorkloadProtectableItemResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkloadProtectableItemResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Yearly retention schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct YearlyRetentionSchedule {
    #[doc = "Retention schedule format for yearly retention policy."]
    #[serde(rename = "retentionScheduleFormatType", default, skip_serializing_if = "Option::is_none")]
    pub retention_schedule_format_type: Option<yearly_retention_schedule::RetentionScheduleFormatType>,
    #[doc = "List of months of year of yearly retention policy."]
    #[serde(rename = "monthsOfYear", default, skip_serializing_if = "Vec::is_empty")]
    pub months_of_year: Vec<String>,
    #[doc = "Daily retention format."]
    #[serde(rename = "retentionScheduleDaily", default, skip_serializing_if = "Option::is_none")]
    pub retention_schedule_daily: Option<DailyRetentionFormat>,
    #[doc = "Weekly retention format."]
    #[serde(rename = "retentionScheduleWeekly", default, skip_serializing_if = "Option::is_none")]
    pub retention_schedule_weekly: Option<WeeklyRetentionFormat>,
    #[doc = "Retention times of retention policy."]
    #[serde(rename = "retentionTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub retention_times: Vec<time::OffsetDateTime>,
    #[doc = "Retention duration."]
    #[serde(rename = "retentionDuration", default, skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<RetentionDuration>,
}
impl YearlyRetentionSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod yearly_retention_schedule {
    use super::*;
    #[doc = "Retention schedule format for yearly retention policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RetentionScheduleFormatType")]
    pub enum RetentionScheduleFormatType {
        Invalid,
        Daily,
        Weekly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RetentionScheduleFormatType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RetentionScheduleFormatType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RetentionScheduleFormatType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RetentionScheduleFormatType", 0u32, "Invalid"),
                Self::Daily => serializer.serialize_unit_variant("RetentionScheduleFormatType", 1u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("RetentionScheduleFormatType", 2u32, "Weekly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
