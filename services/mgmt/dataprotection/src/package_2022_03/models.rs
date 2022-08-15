#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Delete option with duration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbsoluteDeleteOption {
    #[serde(flatten)]
    pub delete_option: DeleteOption,
}
impl AbsoluteDeleteOption {
    pub fn new(delete_option: DeleteOption) -> Self {
        Self { delete_option }
    }
}
#[doc = "Adhoc backup rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdHocBackupRuleOptions {
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[doc = "Adhoc backup trigger option"]
    #[serde(rename = "triggerOption")]
    pub trigger_option: AdhocBackupTriggerOption,
}
impl AdHocBackupRuleOptions {
    pub fn new(rule_name: String, trigger_option: AdhocBackupTriggerOption) -> Self {
        Self { rule_name, trigger_option }
    }
}
#[doc = "Adhoc backup trigger option"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdhocBackupTriggerOption {
    #[serde(rename = "retentionTagOverride", default, skip_serializing_if = "Option::is_none")]
    pub retention_tag_override: Option<String>,
}
impl AdhocBackupTriggerOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Adhoc backup tagging criteria"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdhocBasedTaggingCriteria {
    #[doc = "Retention tag"]
    #[serde(rename = "tagInfo", default, skip_serializing_if = "Option::is_none")]
    pub tag_info: Option<RetentionTag>,
}
impl AdhocBasedTaggingCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Adhoc trigger context"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdhocBasedTriggerContext {
    #[serde(flatten)]
    pub trigger_context: TriggerContext,
    #[doc = "Adhoc backup tagging criteria"]
    #[serde(rename = "taggingCriteria")]
    pub tagging_criteria: AdhocBasedTaggingCriteria,
}
impl AdhocBasedTriggerContext {
    pub fn new(trigger_context: TriggerContext, tagging_criteria: AdhocBasedTaggingCriteria) -> Self {
        Self {
            trigger_context,
            tagging_criteria,
        }
    }
}
#[doc = "Base class for different types of authentication credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthCredentials {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl AuthCredentials {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Azure backup discrete RecoveryPoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupDiscreteRecoveryPoint {
    #[serde(flatten)]
    pub azure_backup_recovery_point: AzureBackupRecoveryPoint,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "recoveryPointDataStoresDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_point_data_stores_details: Vec<RecoveryPointDataStoreDetails>,
    #[serde(rename = "recoveryPointTime", with = "azure_core::date::rfc3339")]
    pub recovery_point_time: time::OffsetDateTime,
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<String>,
    #[serde(rename = "retentionTagName", default, skip_serializing_if = "Option::is_none")]
    pub retention_tag_name: Option<String>,
    #[serde(rename = "retentionTagVersion", default, skip_serializing_if = "Option::is_none")]
    pub retention_tag_version: Option<String>,
}
impl AzureBackupDiscreteRecoveryPoint {
    pub fn new(azure_backup_recovery_point: AzureBackupRecoveryPoint, recovery_point_time: time::OffsetDateTime) -> Self {
        Self {
            azure_backup_recovery_point,
            friendly_name: None,
            recovery_point_data_stores_details: Vec::new(),
            recovery_point_time,
            policy_name: None,
            policy_version: None,
            recovery_point_id: None,
            recovery_point_type: None,
            retention_tag_name: None,
            retention_tag_version: None,
        }
    }
}
#[doc = "List Restore Ranges Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupFindRestorableTimeRangesRequest {
    #[doc = "Gets or sets the type of the source data store."]
    #[serde(rename = "sourceDataStoreType")]
    pub source_data_store_type: azure_backup_find_restorable_time_ranges_request::SourceDataStoreType,
    #[doc = "Start time for the List Restore Ranges request. ISO 8601 format."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time for the List Restore Ranges request. ISO 8601 format."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
impl AzureBackupFindRestorableTimeRangesRequest {
    pub fn new(source_data_store_type: azure_backup_find_restorable_time_ranges_request::SourceDataStoreType) -> Self {
        Self {
            source_data_store_type,
            start_time: None,
            end_time: None,
        }
    }
}
pub mod azure_backup_find_restorable_time_ranges_request {
    use super::*;
    #[doc = "Gets or sets the type of the source data store."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceDataStoreType")]
    pub enum SourceDataStoreType {
        OperationalStore,
        VaultStore,
        ArchiveStore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceDataStoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceDataStoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceDataStoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OperationalStore => serializer.serialize_unit_variant("SourceDataStoreType", 0u32, "OperationalStore"),
                Self::VaultStore => serializer.serialize_unit_variant("SourceDataStoreType", 1u32, "VaultStore"),
                Self::ArchiveStore => serializer.serialize_unit_variant("SourceDataStoreType", 2u32, "ArchiveStore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List Restore Ranges Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupFindRestorableTimeRangesRequestResource {
    #[serde(flatten)]
    pub dpp_worker_request: DppWorkerRequest,
    #[doc = "List Restore Ranges Request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<AzureBackupFindRestorableTimeRangesRequest>,
}
impl AzureBackupFindRestorableTimeRangesRequestResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List Restore Ranges Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupFindRestorableTimeRangesResponse {
    #[doc = "Returns the Restore Ranges available on the Backup Instance."]
    #[serde(rename = "restorableTimeRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub restorable_time_ranges: Vec<RestorableTimeRange>,
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}
impl AzureBackupFindRestorableTimeRangesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List Restore Ranges Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupFindRestorableTimeRangesResponseResource {
    #[serde(flatten)]
    pub dpp_resource: DppResource,
    #[doc = "List Restore Ranges Response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureBackupFindRestorableTimeRangesResponse>,
}
impl AzureBackupFindRestorableTimeRangesResponseResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureBackup Job Class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupJob {
    #[doc = "Job Activity Id"]
    #[serde(rename = "activityID")]
    pub activity_id: String,
    #[doc = "Name of the Backup Instance"]
    #[serde(rename = "backupInstanceFriendlyName")]
    pub backup_instance_friendly_name: String,
    #[doc = "ARM ID of the Backup Instance"]
    #[serde(rename = "backupInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub backup_instance_id: Option<String>,
    #[doc = "ARM ID of the DataSource"]
    #[serde(rename = "dataSourceId")]
    pub data_source_id: String,
    #[doc = "Location of the DataSource"]
    #[serde(rename = "dataSourceLocation")]
    pub data_source_location: String,
    #[doc = "User Friendly Name of the DataSource"]
    #[serde(rename = "dataSourceName")]
    pub data_source_name: String,
    #[doc = "Data Source Set Name of the DataSource"]
    #[serde(rename = "dataSourceSetName", default, skip_serializing_if = "Option::is_none")]
    pub data_source_set_name: Option<String>,
    #[doc = "Type of DataSource"]
    #[serde(rename = "dataSourceType")]
    pub data_source_type: String,
    #[doc = "Total run time of the job. ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "EndTime of the job(in UTC)"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "A List, detailing the errors related to the job"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<UserFacingError>,
    #[doc = "Extended Information about the job"]
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<JobExtendedInfo>,
    #[doc = "Indicated that whether the job is adhoc(true) or scheduled(false)"]
    #[serde(rename = "isUserTriggered")]
    pub is_user_triggered: bool,
    #[doc = "It indicates the type of Job i.e. Backup:full/log/diff ;Restore:ALR/OLR; Tiering:Backup/Archive ; Management:ConfigureProtection/UnConfigure"]
    pub operation: String,
    #[doc = "It indicates the type of Job i.e. Backup/Restore/Tiering/Management"]
    #[serde(rename = "operationCategory")]
    pub operation_category: String,
    #[doc = "ARM ID of the policy"]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Name of the policy"]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "Indicated whether progress is enabled for the job"]
    #[serde(rename = "progressEnabled")]
    pub progress_enabled: bool,
    #[doc = "Url which contains job's progress"]
    #[serde(rename = "progressUrl", default, skip_serializing_if = "Option::is_none")]
    pub progress_url: Option<String>,
    #[doc = "It indicates the sub type of operation i.e. in case of Restore it can be ALR/OLR"]
    #[serde(rename = "restoreType", default, skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    #[doc = "Resource Group Name of the Datasource"]
    #[serde(rename = "sourceResourceGroup")]
    pub source_resource_group: String,
    #[doc = "SubscriptionId corresponding to the DataSource"]
    #[serde(rename = "sourceSubscriptionID")]
    pub source_subscription_id: String,
    #[doc = "StartTime of the job(in UTC)"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Status of the job like InProgress/Success/Failed/Cancelled/SuccessWithWarning"]
    pub status: String,
    #[doc = "Subscription Id of the corresponding backup vault"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[doc = "List of supported actions"]
    #[serde(rename = "supportedActions")]
    pub supported_actions: Vec<String>,
    #[doc = "Name of the vault"]
    #[serde(rename = "vaultName")]
    pub vault_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "sourceDataStoreName", default, skip_serializing_if = "Option::is_none")]
    pub source_data_store_name: Option<String>,
    #[serde(rename = "destinationDataStoreName", default, skip_serializing_if = "Option::is_none")]
    pub destination_data_store_name: Option<String>,
}
impl AzureBackupJob {
    pub fn new(
        activity_id: String,
        backup_instance_friendly_name: String,
        data_source_id: String,
        data_source_location: String,
        data_source_name: String,
        data_source_type: String,
        is_user_triggered: bool,
        operation: String,
        operation_category: String,
        progress_enabled: bool,
        source_resource_group: String,
        source_subscription_id: String,
        start_time: time::OffsetDateTime,
        status: String,
        subscription_id: String,
        supported_actions: Vec<String>,
        vault_name: String,
    ) -> Self {
        Self {
            activity_id,
            backup_instance_friendly_name,
            backup_instance_id: None,
            data_source_id,
            data_source_location,
            data_source_name,
            data_source_set_name: None,
            data_source_type,
            duration: None,
            end_time: None,
            error_details: Vec::new(),
            extended_info: None,
            is_user_triggered,
            operation,
            operation_category,
            policy_id: None,
            policy_name: None,
            progress_enabled,
            progress_url: None,
            restore_type: None,
            source_resource_group,
            source_subscription_id,
            start_time,
            status,
            subscription_id,
            supported_actions,
            vault_name,
            etag: None,
            source_data_store_name: None,
            destination_data_store_name: None,
        }
    }
}
#[doc = "AzureBackup Job Resource Class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupJobResource {
    #[serde(flatten)]
    pub dpp_resource: DppResource,
    #[doc = "AzureBackup Job Class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureBackupJob>,
}
impl AzureBackupJobResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of AzureBackup Job resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupJobResourceList {
    #[serde(flatten)]
    pub dpp_resource_list: DppResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureBackupJobResource>,
}
impl azure_core::Continuable for AzureBackupJobResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureBackupJobResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure backup parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupParams {
    #[serde(flatten)]
    pub backup_parameters: BackupParameters,
    #[doc = "BackupType ; Full/Incremental etc"]
    #[serde(rename = "backupType")]
    pub backup_type: String,
}
impl AzureBackupParams {
    pub fn new(backup_parameters: BackupParameters, backup_type: String) -> Self {
        Self {
            backup_parameters,
            backup_type,
        }
    }
}
#[doc = "Azure backup recoveryPoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRecoveryPoint {
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl AzureBackupRecoveryPoint {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Azure backup recoveryPoint based restore request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRecoveryPointBasedRestoreRequest {
    #[serde(flatten)]
    pub azure_backup_restore_request: AzureBackupRestoreRequest,
    #[serde(rename = "recoveryPointId")]
    pub recovery_point_id: String,
}
impl AzureBackupRecoveryPointBasedRestoreRequest {
    pub fn new(azure_backup_restore_request: AzureBackupRestoreRequest, recovery_point_id: String) -> Self {
        Self {
            azure_backup_restore_request,
            recovery_point_id,
        }
    }
}
#[doc = "Azure backup recoveryPoint resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupRecoveryPointResource {
    #[serde(flatten)]
    pub dpp_resource: DppResource,
    #[doc = "Azure backup recoveryPoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureBackupRecoveryPoint>,
}
impl AzureBackupRecoveryPointResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure backup recoveryPoint resource list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBackupRecoveryPointResourceList {
    #[serde(flatten)]
    pub dpp_resource_list: DppResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureBackupRecoveryPointResource>,
}
impl azure_core::Continuable for AzureBackupRecoveryPointResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureBackupRecoveryPointResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureBackup RecoveryPointTime Based Restore Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRecoveryTimeBasedRestoreRequest {
    #[serde(flatten)]
    pub azure_backup_restore_request: AzureBackupRestoreRequest,
    #[doc = "The recovery time in ISO 8601 format example - 2020-08-14T17:30:00.0000000Z."]
    #[serde(rename = "recoveryPointTime")]
    pub recovery_point_time: String,
}
impl AzureBackupRecoveryTimeBasedRestoreRequest {
    pub fn new(azure_backup_restore_request: AzureBackupRestoreRequest, recovery_point_time: String) -> Self {
        Self {
            azure_backup_restore_request,
            recovery_point_time,
        }
    }
}
#[doc = "Azure Backup Rehydrate Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRehydrationRequest {
    #[doc = "Id of the recovery point to be recovered"]
    #[serde(rename = "recoveryPointId")]
    pub recovery_point_id: String,
    #[doc = "Priority to be used for rehydration. Values High or Standard"]
    #[serde(rename = "rehydrationPriority", default, skip_serializing_if = "Option::is_none")]
    pub rehydration_priority: Option<RehydrationPriority>,
    #[doc = "Retention duration in ISO 8601 format i.e P10D ."]
    #[serde(rename = "rehydrationRetentionDuration")]
    pub rehydration_retention_duration: String,
}
impl AzureBackupRehydrationRequest {
    pub fn new(recovery_point_id: String, rehydration_retention_duration: String) -> Self {
        Self {
            recovery_point_id,
            rehydration_priority: None,
            rehydration_retention_duration,
        }
    }
}
#[doc = "Azure backup restore request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRestoreRequest {
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[doc = "Base class common to RestoreTargetInfo and RestoreFilesTargetInfo"]
    #[serde(rename = "restoreTargetInfo")]
    pub restore_target_info: RestoreTargetInfoBase,
    #[doc = "Gets or sets the type of the source data store."]
    #[serde(rename = "sourceDataStoreType")]
    pub source_data_store_type: azure_backup_restore_request::SourceDataStoreType,
    #[doc = "Fully qualified Azure Resource Manager ID of the datasource which is being recovered."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
}
impl AzureBackupRestoreRequest {
    pub fn new(
        object_type: String,
        restore_target_info: RestoreTargetInfoBase,
        source_data_store_type: azure_backup_restore_request::SourceDataStoreType,
    ) -> Self {
        Self {
            object_type,
            restore_target_info,
            source_data_store_type,
            source_resource_id: None,
        }
    }
}
pub mod azure_backup_restore_request {
    use super::*;
    #[doc = "Gets or sets the type of the source data store."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceDataStoreType")]
    pub enum SourceDataStoreType {
        ArchiveStore,
        SnapshotStore,
        VaultStore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceDataStoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceDataStoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceDataStoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ArchiveStore => serializer.serialize_unit_variant("SourceDataStoreType", 0u32, "ArchiveStore"),
                Self::SnapshotStore => serializer.serialize_unit_variant("SourceDataStoreType", 1u32, "SnapshotStore"),
                Self::VaultStore => serializer.serialize_unit_variant("SourceDataStoreType", 2u32, "VaultStore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "AzureBackup Restore with Rehydration Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRestoreWithRehydrationRequest {
    #[serde(flatten)]
    pub azure_backup_recovery_point_based_restore_request: AzureBackupRecoveryPointBasedRestoreRequest,
    #[doc = "Priority to be used for rehydration. Values High or Standard"]
    #[serde(rename = "rehydrationPriority")]
    pub rehydration_priority: RehydrationPriority,
    #[doc = "Retention duration in ISO 8601 format i.e P10D ."]
    #[serde(rename = "rehydrationRetentionDuration")]
    pub rehydration_retention_duration: String,
}
impl AzureBackupRestoreWithRehydrationRequest {
    pub fn new(
        azure_backup_recovery_point_based_restore_request: AzureBackupRecoveryPointBasedRestoreRequest,
        rehydration_priority: RehydrationPriority,
        rehydration_retention_duration: String,
    ) -> Self {
        Self {
            azure_backup_recovery_point_based_restore_request,
            rehydration_priority,
            rehydration_retention_duration,
        }
    }
}
#[doc = "Azure backup rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupRule {
    #[serde(flatten)]
    pub base_policy_rule: BasePolicyRule,
    #[doc = "BackupParameters base"]
    #[serde(rename = "backupParameters", default, skip_serializing_if = "Option::is_none")]
    pub backup_parameters: Option<BackupParameters>,
    #[doc = "DataStoreInfo base"]
    #[serde(rename = "dataStore")]
    pub data_store: DataStoreInfoBase,
    #[doc = "Trigger context"]
    pub trigger: TriggerContext,
}
impl AzureBackupRule {
    pub fn new(base_policy_rule: BasePolicyRule, data_store: DataStoreInfoBase, trigger: TriggerContext) -> Self {
        Self {
            base_policy_rule,
            backup_parameters: None,
            data_store,
            trigger,
        }
    }
}
#[doc = "Parameters for Operational-Tier DataStore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOperationalStoreParameters {
    #[serde(flatten)]
    pub data_store_parameters: DataStoreParameters,
    #[doc = "Gets or sets the Snapshot Resource Group Uri."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}
impl AzureOperationalStoreParameters {
    pub fn new(data_store_parameters: DataStoreParameters) -> Self {
        Self {
            data_store_parameters,
            resource_group_id: None,
        }
    }
}
#[doc = "Azure retention rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureRetentionRule {
    #[serde(flatten)]
    pub base_policy_rule: BasePolicyRule,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    pub lifecycles: Vec<SourceLifeCycle>,
}
impl AzureRetentionRule {
    pub fn new(base_policy_rule: BasePolicyRule, lifecycles: Vec<SourceLifeCycle>) -> Self {
        Self {
            base_policy_rule,
            is_default: None,
            lifecycles,
        }
    }
}
#[doc = "BackupCriteria base class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupCriteria {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BackupCriteria {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Backup Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupInstance {
    #[doc = "Gets or sets the Backup Instance friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Datasource to be backed up"]
    #[serde(rename = "dataSourceInfo")]
    pub data_source_info: Datasource,
    #[doc = "DatasourceSet details of datasource to be backed up"]
    #[serde(rename = "dataSourceSetInfo", default, skip_serializing_if = "Option::is_none")]
    pub data_source_set_info: Option<DatasourceSet>,
    #[doc = "Policy Info in backupInstance"]
    #[serde(rename = "policyInfo")]
    pub policy_info: PolicyInfo,
    #[doc = "Protection status details"]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<ProtectionStatusDetails>,
    #[doc = "Specifies the current protection state of the resource"]
    #[serde(rename = "currentProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub current_protection_state: Option<backup_instance::CurrentProtectionState>,
    #[doc = "Error object used by layers that have access to localized content, and propagate that to user"]
    #[serde(rename = "protectionErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub protection_error_details: Option<UserFacingError>,
    #[doc = "Specifies the provisioning state of the resource i.e. provisioning/updating/Succeeded/Failed"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Base class for different types of authentication credentials."]
    #[serde(rename = "datasourceAuthCredentials", default, skip_serializing_if = "Option::is_none")]
    pub datasource_auth_credentials: Option<AuthCredentials>,
    #[doc = "Specifies the type of validation. In case of DeepValidation, all validations from /validateForBackup API will run again."]
    #[serde(rename = "validationType", default, skip_serializing_if = "Option::is_none")]
    pub validation_type: Option<backup_instance::ValidationType>,
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BackupInstance {
    pub fn new(data_source_info: Datasource, policy_info: PolicyInfo, object_type: String) -> Self {
        Self {
            friendly_name: None,
            data_source_info,
            data_source_set_info: None,
            policy_info,
            protection_status: None,
            current_protection_state: None,
            protection_error_details: None,
            provisioning_state: None,
            datasource_auth_credentials: None,
            validation_type: None,
            object_type,
        }
    }
}
pub mod backup_instance {
    use super::*;
    #[doc = "Specifies the current protection state of the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentProtectionState")]
    pub enum CurrentProtectionState {
        Invalid,
        NotProtected,
        ConfiguringProtection,
        ProtectionConfigured,
        BackupSchedulesSuspended,
        RetentionSchedulesSuspended,
        ProtectionStopped,
        ProtectionError,
        ConfiguringProtectionFailed,
        SoftDeleting,
        SoftDeleted,
        UpdatingProtection,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentProtectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentProtectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentProtectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("CurrentProtectionState", 0u32, "Invalid"),
                Self::NotProtected => serializer.serialize_unit_variant("CurrentProtectionState", 1u32, "NotProtected"),
                Self::ConfiguringProtection => serializer.serialize_unit_variant("CurrentProtectionState", 2u32, "ConfiguringProtection"),
                Self::ProtectionConfigured => serializer.serialize_unit_variant("CurrentProtectionState", 3u32, "ProtectionConfigured"),
                Self::BackupSchedulesSuspended => {
                    serializer.serialize_unit_variant("CurrentProtectionState", 4u32, "BackupSchedulesSuspended")
                }
                Self::RetentionSchedulesSuspended => {
                    serializer.serialize_unit_variant("CurrentProtectionState", 5u32, "RetentionSchedulesSuspended")
                }
                Self::ProtectionStopped => serializer.serialize_unit_variant("CurrentProtectionState", 6u32, "ProtectionStopped"),
                Self::ProtectionError => serializer.serialize_unit_variant("CurrentProtectionState", 7u32, "ProtectionError"),
                Self::ConfiguringProtectionFailed => {
                    serializer.serialize_unit_variant("CurrentProtectionState", 8u32, "ConfiguringProtectionFailed")
                }
                Self::SoftDeleting => serializer.serialize_unit_variant("CurrentProtectionState", 9u32, "SoftDeleting"),
                Self::SoftDeleted => serializer.serialize_unit_variant("CurrentProtectionState", 10u32, "SoftDeleted"),
                Self::UpdatingProtection => serializer.serialize_unit_variant("CurrentProtectionState", 11u32, "UpdatingProtection"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the type of validation. In case of DeepValidation, all validations from /validateForBackup API will run again."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationType")]
    pub enum ValidationType {
        ShallowValidation,
        DeepValidation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ShallowValidation => serializer.serialize_unit_variant("ValidationType", 0u32, "ShallowValidation"),
                Self::DeepValidation => serializer.serialize_unit_variant("ValidationType", 1u32, "DeepValidation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "BackupInstance Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupInstanceResource {
    #[serde(flatten)]
    pub dpp_resource: DppResource,
    #[doc = "Backup Instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupInstance>,
}
impl BackupInstanceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BackupInstance Resource list response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupInstanceResourceList {
    #[serde(flatten)]
    pub dpp_resource_list: DppResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupInstanceResource>,
}
impl azure_core::Continuable for BackupInstanceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupInstanceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BackupParameters base"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupParameters {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BackupParameters {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Rule based backup policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicy {
    #[serde(flatten)]
    pub base_backup_policy: BaseBackupPolicy,
    #[doc = "Policy rule dictionary that contains rules for each backuptype i.e Full/Incremental/Logs etc"]
    #[serde(rename = "policyRules")]
    pub policy_rules: Vec<BasePolicyRule>,
}
impl BackupPolicy {
    pub fn new(base_backup_policy: BaseBackupPolicy, policy_rules: Vec<BasePolicyRule>) -> Self {
        Self {
            base_backup_policy,
            policy_rules,
        }
    }
}
#[doc = "Schedule for backup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupSchedule {
    #[doc = "ISO 8601 repeating time interval format"]
    #[serde(rename = "repeatingTimeIntervals")]
    pub repeating_time_intervals: Vec<String>,
    #[doc = "Time zone for a schedule. Example: Pacific Standard Time"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl BackupSchedule {
    pub fn new(repeating_time_intervals: Vec<String>) -> Self {
        Self {
            repeating_time_intervals,
            time_zone: None,
        }
    }
}
#[doc = "Backup Vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupVault {
    #[doc = "Provisioning state of the BackupVault resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<backup_vault::ProvisioningState>,
    #[doc = "Resource move state for backup vault"]
    #[serde(rename = "resourceMoveState", default, skip_serializing_if = "Option::is_none")]
    pub resource_move_state: Option<backup_vault::ResourceMoveState>,
    #[doc = "ResourceMoveDetails will be returned in response to GetResource call from ARM"]
    #[serde(rename = "resourceMoveDetails", default, skip_serializing_if = "Option::is_none")]
    pub resource_move_details: Option<ResourceMoveDetails>,
    #[doc = "Storage Settings"]
    #[serde(rename = "storageSettings")]
    pub storage_settings: Vec<StorageSetting>,
}
impl BackupVault {
    pub fn new(storage_settings: Vec<StorageSetting>) -> Self {
        Self {
            provisioning_state: None,
            resource_move_state: None,
            resource_move_details: None,
            storage_settings,
        }
    }
}
pub mod backup_vault {
    use super::*;
    #[doc = "Provisioning state of the BackupVault resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Failed,
        Provisioning,
        Succeeded,
        Unknown,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Failed"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioning"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Resource move state for backup vault"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceMoveState")]
    pub enum ResourceMoveState {
        Unknown,
        InProgress,
        PrepareFailed,
        CommitFailed,
        Failed,
        PrepareTimedout,
        CommitTimedout,
        CriticalFailure,
        PartialSuccess,
        MoveSucceeded,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceMoveState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceMoveState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceMoveState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ResourceMoveState", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("ResourceMoveState", 1u32, "InProgress"),
                Self::PrepareFailed => serializer.serialize_unit_variant("ResourceMoveState", 2u32, "PrepareFailed"),
                Self::CommitFailed => serializer.serialize_unit_variant("ResourceMoveState", 3u32, "CommitFailed"),
                Self::Failed => serializer.serialize_unit_variant("ResourceMoveState", 4u32, "Failed"),
                Self::PrepareTimedout => serializer.serialize_unit_variant("ResourceMoveState", 5u32, "PrepareTimedout"),
                Self::CommitTimedout => serializer.serialize_unit_variant("ResourceMoveState", 6u32, "CommitTimedout"),
                Self::CriticalFailure => serializer.serialize_unit_variant("ResourceMoveState", 7u32, "CriticalFailure"),
                Self::PartialSuccess => serializer.serialize_unit_variant("ResourceMoveState", 8u32, "PartialSuccess"),
                Self::MoveSucceeded => serializer.serialize_unit_variant("ResourceMoveState", 9u32, "MoveSucceeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backup Vault Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupVaultResource {
    #[serde(flatten)]
    pub dpp_tracked_resource: DppTrackedResource,
    #[doc = "Backup Vault"]
    pub properties: BackupVault,
}
impl BackupVaultResource {
    pub fn new(properties: BackupVault) -> Self {
        Self {
            dpp_tracked_resource: DppTrackedResource::default(),
            properties,
        }
    }
}
#[doc = "List of BackupVault resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupVaultResourceList {
    #[serde(flatten)]
    pub dpp_resource_list: DppResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupVaultResource>,
}
impl azure_core::Continuable for BackupVaultResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BackupVaultResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BackupPolicy base"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseBackupPolicy {
    #[doc = "Type of datasource for the backup management"]
    #[serde(rename = "datasourceTypes")]
    pub datasource_types: Vec<String>,
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BaseBackupPolicy {
    pub fn new(datasource_types: Vec<String>, object_type: String) -> Self {
        Self {
            datasource_types,
            object_type,
        }
    }
}
#[doc = "BaseBackupPolicy resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseBackupPolicyResource {
    #[serde(flatten)]
    pub dpp_resource: DppResource,
    #[doc = "BackupPolicy base"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaseBackupPolicy>,
}
impl BaseBackupPolicyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of BaseBackupPolicy resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseBackupPolicyResourceList {
    #[serde(flatten)]
    pub dpp_resource_list: DppResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BaseBackupPolicyResource>,
}
impl azure_core::Continuable for BaseBackupPolicyResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BaseBackupPolicyResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BasePolicy Rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasePolicyRule {
    pub name: String,
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BasePolicyRule {
    pub fn new(name: String, object_type: String) -> Self {
        Self { name, object_type }
    }
}
#[doc = "CheckNameAvailability Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "Resource name for which availability needs to be checked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the Resource type: Microsoft.DataProtection/BackupVaults"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CheckNameAvailability Result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets or sets the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets a value indicating whether [name available]."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets or sets the reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localized display information of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryDisplay {
    #[doc = "Description of the operation having details of what operation is about."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Operations Name itself."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "ResourceType for which this Operation can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl ClientDiscoveryDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox log specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForLogSpecification {
    #[doc = "blob duration of shoebox log specification"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
    #[doc = "Localized display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Name for shoebox log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[doc = "Link to the next chunk of Response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientDiscoveryValueForSingleApi>,
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
    #[doc = "Localized display information of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClientDiscoveryDisplay>,
    #[doc = "Name of the Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
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
#[doc = "An error response from Azure Backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
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
#[doc = "Copy on Expiry Option"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyOnExpiryOption {
    #[serde(flatten)]
    pub copy_option: CopyOption,
}
impl CopyOnExpiryOption {
    pub fn new(copy_option: CopyOption) -> Self {
        Self { copy_option }
    }
}
#[doc = "Options to copy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyOption {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl CopyOption {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Duration based custom options to copy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomCopyOption {
    #[serde(flatten)]
    pub copy_option: CopyOption,
    #[doc = "Data copied after given timespan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl CustomCopyOption {
    pub fn new(copy_option: CopyOption) -> Self {
        Self {
            copy_option,
            duration: None,
        }
    }
}
#[doc = "DataStoreInfo base"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreInfoBase {
    #[doc = "type of datastore; Operational/Vault/Archive"]
    #[serde(rename = "dataStoreType")]
    pub data_store_type: data_store_info_base::DataStoreType,
    #[doc = "Type of Datasource object, used to initialize the right inherited type"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl DataStoreInfoBase {
    pub fn new(data_store_type: data_store_info_base::DataStoreType, object_type: String) -> Self {
        Self {
            data_store_type,
            object_type,
        }
    }
}
pub mod data_store_info_base {
    use super::*;
    #[doc = "type of datastore; Operational/Vault/Archive"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataStoreType")]
    pub enum DataStoreType {
        OperationalStore,
        VaultStore,
        ArchiveStore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataStoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataStoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataStoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OperationalStore => serializer.serialize_unit_variant("DataStoreType", 0u32, "OperationalStore"),
                Self::VaultStore => serializer.serialize_unit_variant("DataStoreType", 1u32, "VaultStore"),
                Self::ArchiveStore => serializer.serialize_unit_variant("DataStoreType", 2u32, "ArchiveStore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters for DataStore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreParameters {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[doc = "type of datastore; Operational/Vault/Archive"]
    #[serde(rename = "dataStoreType")]
    pub data_store_type: data_store_parameters::DataStoreType,
}
impl DataStoreParameters {
    pub fn new(object_type: String, data_store_type: data_store_parameters::DataStoreType) -> Self {
        Self {
            object_type,
            data_store_type,
        }
    }
}
pub mod data_store_parameters {
    use super::*;
    #[doc = "type of datastore; Operational/Vault/Archive"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataStoreType")]
    pub enum DataStoreType {
        OperationalStore,
        VaultStore,
        ArchiveStore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataStoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataStoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataStoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OperationalStore => serializer.serialize_unit_variant("DataStoreType", 0u32, "OperationalStore"),
                Self::VaultStore => serializer.serialize_unit_variant("DataStoreType", 1u32, "VaultStore"),
                Self::ArchiveStore => serializer.serialize_unit_variant("DataStoreType", 2u32, "ArchiveStore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Datasource to be backed up"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Datasource {
    #[doc = "DatasourceType of the resource."]
    #[serde(rename = "datasourceType", default, skip_serializing_if = "Option::is_none")]
    pub datasource_type: Option<String>,
    #[doc = "Type of Datasource object, used to initialize the right inherited type"]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "Full ARM ID of the resource. For azure resources, this is ARM ID. For non azure resources, this will be the ID created by backup service via Fabric/Vault."]
    #[serde(rename = "resourceID")]
    pub resource_id: String,
    #[doc = "Location of datasource."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Unique identifier of the resource in the context of parent."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Resource Type of Datasource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Uri of the resource."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl Datasource {
    pub fn new(resource_id: String) -> Self {
        Self {
            datasource_type: None,
            object_type: None,
            resource_id,
            resource_location: None,
            resource_name: None,
            resource_type: None,
            resource_uri: None,
        }
    }
}
#[doc = "DatasourceSet details of datasource to be backed up"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasourceSet {
    #[doc = "DatasourceType of the resource."]
    #[serde(rename = "datasourceType", default, skip_serializing_if = "Option::is_none")]
    pub datasource_type: Option<String>,
    #[doc = "Type of Datasource object, used to initialize the right inherited type"]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "Full ARM ID of the resource. For azure resources, this is ARM ID. For non azure resources, this will be the ID created by backup service via Fabric/Vault."]
    #[serde(rename = "resourceID")]
    pub resource_id: String,
    #[doc = "Location of datasource."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Unique identifier of the resource in the context of parent."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Resource Type of Datasource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Uri of the resource."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl DatasourceSet {
    pub fn new(resource_id: String) -> Self {
        Self {
            datasource_type: None,
            object_type: None,
            resource_id,
            resource_location: None,
            resource_name: None,
            resource_type: None,
            resource_uri: None,
        }
    }
}
#[doc = "Day of the week"]
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
#[doc = "Delete Option"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteOption {
    #[doc = "Duration of deletion after given timespan"]
    pub duration: String,
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl DeleteOption {
    pub fn new(duration: String, object_type: String) -> Self {
        Self { duration, object_type }
    }
}
#[doc = "Base resource under Microsoft.DataProtection provider namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppBaseResource {
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DppBaseResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base for all lists of V2 resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppBaseResourceList {
    #[doc = "List of Dpp resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DppBaseResource>,
    #[doc = "The uri to fetch the next page of resources. Call ListNext() fetches next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DppBaseResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DppBaseResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppIdentityDetails {
    #[doc = "The object ID of the service principal object for the managed identity that is used to grant role-based access to an Azure resource."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "A Globally Unique Identifier (GUID) that represents the Azure AD tenant where the resource is now a member."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identityType which can be either SystemAssigned or None"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DppIdentityDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource class"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppResource {
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DppResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ListResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppResourceList {
    #[doc = "The uri to fetch the next page of resources. Call ListNext() fetches next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DppResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppTrackedResource {
    #[doc = "Optional ETag."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Identity details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DppIdentityDetails>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DppTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppTrackedResourceList {
    #[doc = "The uri to fetch the next page of resources. Call ListNext() fetches next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DppTrackedResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DppWorkerRequest {
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(rename = "supportedGroupVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_group_versions: Vec<String>,
    #[serde(rename = "cultureInfo", default, skip_serializing_if = "Option::is_none")]
    pub culture_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "httpMethod", default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
}
impl DppWorkerRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result for export jobs containing blob details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportJobsResult {
    #[doc = "URL of the blob into which the serialized string of list of jobs is exported."]
    #[serde(rename = "blobUrl", default, skip_serializing_if = "Option::is_none")]
    pub blob_url: Option<String>,
    #[doc = "SAS key to access the blob."]
    #[serde(rename = "blobSasKey", default, skip_serializing_if = "Option::is_none")]
    pub blob_sas_key: Option<String>,
    #[doc = "URL of the blob into which the ExcelFile is uploaded."]
    #[serde(rename = "excelFileBlobUrl", default, skip_serializing_if = "Option::is_none")]
    pub excel_file_blob_url: Option<String>,
    #[doc = "SAS key to access the ExcelFile blob."]
    #[serde(rename = "excelFileBlobSasKey", default, skip_serializing_if = "Option::is_none")]
    pub excel_file_blob_sas_key: Option<String>,
}
impl ExportJobsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for feature object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureValidationRequest {
    #[serde(flatten)]
    pub feature_validation_request_base: FeatureValidationRequestBase,
    #[doc = "backup support feature type."]
    #[serde(rename = "featureType", default, skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<feature_validation_request::FeatureType>,
    #[doc = "backup support feature name."]
    #[serde(rename = "featureName", default, skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
}
impl FeatureValidationRequest {
    pub fn new(feature_validation_request_base: FeatureValidationRequestBase) -> Self {
        Self {
            feature_validation_request_base,
            feature_type: None,
            feature_name: None,
        }
    }
}
pub mod feature_validation_request {
    use super::*;
    #[doc = "backup support feature type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FeatureType")]
    pub enum FeatureType {
        Invalid,
        DataSourceType,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FeatureType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FeatureType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FeatureType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("FeatureType", 0u32, "Invalid"),
                Self::DataSourceType => serializer.serialize_unit_variant("FeatureType", 1u32, "DataSourceType"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for Backup Feature support"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureValidationRequestBase {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl FeatureValidationRequestBase {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Feature Validation Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureValidationResponse {
    #[serde(flatten)]
    pub feature_validation_response_base: FeatureValidationResponseBase,
    #[doc = "backup support feature type."]
    #[serde(rename = "featureType", default, skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<feature_validation_response::FeatureType>,
    #[doc = "Response features"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<SupportedFeature>,
}
impl FeatureValidationResponse {
    pub fn new(feature_validation_response_base: FeatureValidationResponseBase) -> Self {
        Self {
            feature_validation_response_base,
            feature_type: None,
            features: Vec::new(),
        }
    }
}
pub mod feature_validation_response {
    use super::*;
    #[doc = "backup support feature type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FeatureType")]
    pub enum FeatureType {
        Invalid,
        DataSourceType,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FeatureType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FeatureType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FeatureType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("FeatureType", 0u32, "Invalid"),
                Self::DataSourceType => serializer.serialize_unit_variant("FeatureType", 1u32, "DataSourceType"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for Backup Feature support"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureValidationResponseBase {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl FeatureValidationResponseBase {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Immediate copy Option"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImmediateCopyOption {
    #[serde(flatten)]
    pub copy_option: CopyOption,
}
impl ImmediateCopyOption {
    pub fn new(copy_option: CopyOption) -> Self {
        Self { copy_option }
    }
}
#[doc = "Inner Error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "Any Key value pairs that can be provided to the client for additional  verbose information."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
    #[doc = "Unique code for this error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Inner Error"]
    #[serde(rename = "embeddedInnerError", default, skip_serializing_if = "Option::is_none")]
    pub embedded_inner_error: Box<Option<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to contain criteria for item level restore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemLevelRestoreCriteria {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl ItemLevelRestoreCriteria {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Restore target info for Item level restore operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemLevelRestoreTargetInfo {
    #[serde(flatten)]
    pub restore_target_info_base: RestoreTargetInfoBase,
    #[doc = "Restore Criteria"]
    #[serde(rename = "restoreCriteria")]
    pub restore_criteria: Vec<ItemLevelRestoreCriteria>,
    #[doc = "Datasource to be backed up"]
    #[serde(rename = "datasourceInfo")]
    pub datasource_info: Datasource,
    #[doc = "DatasourceSet details of datasource to be backed up"]
    #[serde(rename = "datasourceSetInfo", default, skip_serializing_if = "Option::is_none")]
    pub datasource_set_info: Option<DatasourceSet>,
    #[doc = "Base class for different types of authentication credentials."]
    #[serde(rename = "datasourceAuthCredentials", default, skip_serializing_if = "Option::is_none")]
    pub datasource_auth_credentials: Option<AuthCredentials>,
}
impl ItemLevelRestoreTargetInfo {
    pub fn new(
        restore_target_info_base: RestoreTargetInfoBase,
        restore_criteria: Vec<ItemLevelRestoreCriteria>,
        datasource_info: Datasource,
    ) -> Self {
        Self {
            restore_target_info_base,
            restore_criteria,
            datasource_info,
            datasource_set_info: None,
            datasource_auth_credentials: None,
        }
    }
}
#[doc = "Extended Information about the job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExtendedInfo {
    #[doc = "Job's Additional Details"]
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<serde_json::Value>,
    #[doc = "State of the Backup Instance"]
    #[serde(rename = "backupInstanceState", default, skip_serializing_if = "Option::is_none")]
    pub backup_instance_state: Option<String>,
    #[doc = "Number of bytes transferred"]
    #[serde(rename = "dataTransferredInBytes", default, skip_serializing_if = "Option::is_none")]
    pub data_transferred_in_bytes: Option<f64>,
    #[doc = "Destination where restore is done"]
    #[serde(rename = "recoveryDestination", default, skip_serializing_if = "Option::is_none")]
    pub recovery_destination: Option<String>,
    #[serde(rename = "sourceRecoverPoint", default, skip_serializing_if = "Option::is_none")]
    pub source_recover_point: Option<RestoreJobRecoveryPointDetails>,
    #[doc = "List of Sub Tasks of the job"]
    #[serde(rename = "subTasks", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_tasks: Vec<JobSubTask>,
    #[serde(rename = "targetRecoverPoint", default, skip_serializing_if = "Option::is_none")]
    pub target_recover_point: Option<RestoreJobRecoveryPointDetails>,
}
impl JobExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of Job's Sub Task"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSubTask {
    #[doc = "Additional details of Sub Tasks"]
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<serde_json::Value>,
    #[doc = "Task Id of the Sub Task"]
    #[serde(rename = "taskId")]
    pub task_id: i32,
    #[doc = "Name of the Sub Task"]
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[doc = "Progress of the Sub Task"]
    #[serde(rename = "taskProgress", default, skip_serializing_if = "Option::is_none")]
    pub task_progress: Option<String>,
    #[doc = "Status of the Sub Task"]
    #[serde(rename = "taskStatus")]
    pub task_status: String,
}
impl JobSubTask {
    pub fn new(task_id: i32, task_name: String, task_status: String) -> Self {
        Self {
            additional_details: None,
            task_id,
            task_name,
            task_progress: None,
            task_status,
        }
    }
}
#[doc = "Item Level kubernetes persistent volume target info for restore operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesPvRestoreCriteria {
    #[serde(flatten)]
    pub item_level_restore_criteria: ItemLevelRestoreCriteria,
    #[doc = "Selected persistent volume claim name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Selected storage class name for restore operation"]
    #[serde(rename = "storageClassName", default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
}
impl KubernetesPvRestoreCriteria {
    pub fn new(item_level_restore_criteria: ItemLevelRestoreCriteria) -> Self {
        Self {
            item_level_restore_criteria,
            name: None,
            storage_class_name: None,
        }
    }
}
#[doc = "Item Level kubernetes storage class target info for restore operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesStorageClassRestoreCriteria {
    #[serde(flatten)]
    pub item_level_restore_criteria: ItemLevelRestoreCriteria,
    #[doc = "Selected storage class name"]
    #[serde(rename = "selectedStorageClassName", default, skip_serializing_if = "Option::is_none")]
    pub selected_storage_class_name: Option<String>,
    #[doc = "Provisioner of the storage class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioner: Option<String>,
}
impl KubernetesStorageClassRestoreCriteria {
    pub fn new(item_level_restore_criteria: ItemLevelRestoreCriteria) -> Self {
        Self {
            item_level_restore_criteria,
            selected_storage_class_name: None,
            provisioner: None,
        }
    }
}
#[doc = "Operation Extended Info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationExtendedInfo {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl OperationExtendedInfo {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Operation Job Extended Info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationJobExtendedInfo {
    #[serde(flatten)]
    pub operation_extended_info: OperationExtendedInfo,
    #[doc = "Arm Id of the job created for this operation."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}
impl OperationJobExtendedInfo {
    pub fn new(operation_extended_info: OperationExtendedInfo) -> Self {
        Self {
            operation_extended_info,
            job_id: None,
        }
    }
}
#[doc = "Operation Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResource {
    #[doc = "End time of the operation"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The resource management error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "It should match what is used to GET the operation result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "It must match the last segment of the \"id\" field, and will typically be a GUID / system generated value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation Extended Info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationExtendedInfo>,
    #[doc = "Start time of the operation"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl OperationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patch Request content for Microsoft.DataProtection resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchResourceRequestInput {
    #[doc = "Identity details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DppIdentityDetails>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchResourceRequestInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy Info in backupInstance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyInfo {
    #[serde(rename = "policyId")]
    pub policy_id: String,
    #[serde(rename = "policyVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[doc = "Parameters in Policy"]
    #[serde(rename = "policyParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_parameters: Option<PolicyParameters>,
}
impl PolicyInfo {
    pub fn new(policy_id: String) -> Self {
        Self {
            policy_id,
            policy_version: None,
            policy_parameters: None,
        }
    }
}
#[doc = "Parameters in Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyParameters {
    #[doc = "Gets or sets the DataStore Parameters"]
    #[serde(rename = "dataStoreParametersList", default, skip_serializing_if = "Vec::is_empty")]
    pub data_store_parameters_list: Vec<DataStoreParameters>,
}
impl PolicyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection status details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionStatusDetails {
    #[doc = "Error object used by layers that have access to localized content, and propagate that to user"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<UserFacingError>,
    #[doc = "Specifies the protection status of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<protection_status_details::Status>,
}
impl ProtectionStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod protection_status_details {
    use super::*;
    #[doc = "Specifies the protection status of the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        ConfiguringProtection,
        ConfiguringProtectionFailed,
        ProtectionConfigured,
        ProtectionStopped,
        SoftDeleted,
        SoftDeleting,
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
                Self::ConfiguringProtection => serializer.serialize_unit_variant("Status", 0u32, "ConfiguringProtection"),
                Self::ConfiguringProtectionFailed => serializer.serialize_unit_variant("Status", 1u32, "ConfiguringProtectionFailed"),
                Self::ProtectionConfigured => serializer.serialize_unit_variant("Status", 2u32, "ProtectionConfigured"),
                Self::ProtectionStopped => serializer.serialize_unit_variant("Status", 3u32, "ProtectionStopped"),
                Self::SoftDeleted => serializer.serialize_unit_variant("Status", 4u32, "SoftDeleted"),
                Self::SoftDeleting => serializer.serialize_unit_variant("Status", 5u32, "SoftDeleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Item Level target info for restore operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RangeBasedItemLevelRestoreCriteria {
    #[serde(flatten)]
    pub item_level_restore_criteria: ItemLevelRestoreCriteria,
    #[doc = "minimum value for range prefix match"]
    #[serde(rename = "minMatchingValue", default, skip_serializing_if = "Option::is_none")]
    pub min_matching_value: Option<String>,
    #[doc = "maximum value for range prefix match"]
    #[serde(rename = "maxMatchingValue", default, skip_serializing_if = "Option::is_none")]
    pub max_matching_value: Option<String>,
}
impl RangeBasedItemLevelRestoreCriteria {
    pub fn new(item_level_restore_criteria: ItemLevelRestoreCriteria) -> Self {
        Self {
            item_level_restore_criteria,
            min_matching_value: None,
            max_matching_value: None,
        }
    }
}
#[doc = "RecoveryPoint datastore details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointDataStoreDetails {
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metaData", default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "rehydrationExpiryTime", with = "azure_core::date::rfc3339::option")]
    pub rehydration_expiry_time: Option<time::OffsetDateTime>,
    #[serde(rename = "rehydrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub rehydration_status: Option<recovery_point_data_store_details::RehydrationStatus>,
}
impl RecoveryPointDataStoreDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recovery_point_data_store_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RehydrationStatus")]
    pub enum RehydrationStatus {
        #[serde(rename = "CREATE_IN_PROGRESS")]
        CreateInProgress,
        #[serde(rename = "COMPLETED")]
        Completed,
        #[serde(rename = "DELETE_IN_PROGRESS")]
        DeleteInProgress,
        #[serde(rename = "DELETED")]
        Deleted,
        #[serde(rename = "FAILED")]
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RehydrationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RehydrationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RehydrationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CreateInProgress => serializer.serialize_unit_variant("RehydrationStatus", 0u32, "CREATE_IN_PROGRESS"),
                Self::Completed => serializer.serialize_unit_variant("RehydrationStatus", 1u32, "COMPLETED"),
                Self::DeleteInProgress => serializer.serialize_unit_variant("RehydrationStatus", 2u32, "DELETE_IN_PROGRESS"),
                Self::Deleted => serializer.serialize_unit_variant("RehydrationStatus", 3u32, "DELETED"),
                Self::Failed => serializer.serialize_unit_variant("RehydrationStatus", 4u32, "FAILED"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointsFilters {
    #[serde(rename = "restorePointDataStoreId", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_data_store_id: Option<String>,
    #[serde(rename = "isVisible", default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "extendedInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<bool>,
    #[serde(rename = "restorePointState", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_state: Option<String>,
}
impl RecoveryPointsFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Priority to be used for rehydration. Values High or Standard"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RehydrationPriority")]
pub enum RehydrationPriority {
    Invalid,
    High,
    Standard,
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
            Self::Invalid => serializer.serialize_unit_variant("RehydrationPriority", 0u32, "Invalid"),
            Self::High => serializer.serialize_unit_variant("RehydrationPriority", 1u32, "High"),
            Self::Standard => serializer.serialize_unit_variant("RehydrationPriority", 2u32, "Standard"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuard {
    #[doc = "Provisioning state of the BackupVault resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<resource_guard::ProvisioningState>,
    #[doc = "This flag indicates whether auto approval is allowed or not."]
    #[serde(rename = "allowAutoApprovals", default, skip_serializing_if = "Option::is_none")]
    pub allow_auto_approvals: Option<bool>,
    #[doc = "{readonly} List of operation details those are protected by the ResourceGuard resource"]
    #[serde(rename = "resourceGuardOperations", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_guard_operations: Vec<ResourceGuardOperation>,
    #[doc = "List of critical operations which are not protected by this resourceGuard"]
    #[serde(rename = "vaultCriticalOperationExclusionList", default, skip_serializing_if = "Vec::is_empty")]
    pub vault_critical_operation_exclusion_list: Vec<String>,
    #[doc = "Description about the pre-req steps to perform all the critical operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceGuard {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_guard {
    use super::*;
    #[doc = "Provisioning state of the BackupVault resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Failed,
        Provisioning,
        Succeeded,
        Unknown,
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
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Failed"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Provisioning"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Unknown"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This class contains all the details about a critical operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardOperation {
    #[doc = "Name of the critical operation."]
    #[serde(rename = "vaultCriticalOperation", default, skip_serializing_if = "Option::is_none")]
    pub vault_critical_operation: Option<String>,
    #[doc = "Type of resource request."]
    #[serde(rename = "requestResourceType", default, skip_serializing_if = "Option::is_none")]
    pub request_resource_type: Option<String>,
}
impl ResourceGuardOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardResource {
    #[serde(flatten)]
    pub dpp_tracked_resource: DppTrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGuard>,
}
impl ResourceGuardResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ResourceGuard resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGuardResourceList {
    #[serde(flatten)]
    pub dpp_tracked_resource_list: DppTrackedResourceList,
    #[doc = "List of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceGuardResource>,
}
impl azure_core::Continuable for ResourceGuardResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ResourceGuardResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ResourceMoveDetails will be returned in response to GetResource call from ARM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceMoveDetails {
    #[doc = "CorrelationId of latest ResourceMove operation attempted"]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Start time in UTC of latest ResourceMove operation attempted. ISO 8601 format."]
    #[serde(rename = "startTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,
    #[doc = "Completion time in UTC of latest ResourceMove operation attempted. ISO 8601 format."]
    #[serde(rename = "completionTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub completion_time_utc: Option<String>,
    #[doc = "ARM resource path of source resource"]
    #[serde(rename = "sourceResourcePath", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_path: Option<String>,
    #[doc = "ARM resource path of target resource used in latest ResourceMove operation"]
    #[serde(rename = "targetResourcePath", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_path: Option<String>,
}
impl ResourceMoveDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorableTimeRange {
    #[doc = "Start time for the available restore range"]
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[doc = "End time for the available restore range"]
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}
impl RestorableTimeRange {
    pub fn new(start_time: String, end_time: String) -> Self {
        Self {
            start_time,
            end_time,
            object_type: None,
        }
    }
}
#[doc = "Class encapsulating restore as files target parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreFilesTargetInfo {
    #[serde(flatten)]
    pub restore_target_info_base: RestoreTargetInfoBase,
    #[doc = "Class encapsulating target details, used where the destination is not a datasource"]
    #[serde(rename = "targetDetails")]
    pub target_details: TargetDetails,
}
impl RestoreFilesTargetInfo {
    pub fn new(restore_target_info_base: RestoreTargetInfoBase, target_details: TargetDetails) -> Self {
        Self {
            restore_target_info_base,
            target_details,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreJobRecoveryPointDetails {
    #[serde(rename = "recoveryPointID", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[serde(rename = "recoveryPointTime", with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
}
impl RestoreJobRecoveryPointDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class encapsulating restore target parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreTargetInfo {
    #[serde(flatten)]
    pub restore_target_info_base: RestoreTargetInfoBase,
    #[doc = "Datasource to be backed up"]
    #[serde(rename = "datasourceInfo")]
    pub datasource_info: Datasource,
    #[doc = "DatasourceSet details of datasource to be backed up"]
    #[serde(rename = "datasourceSetInfo", default, skip_serializing_if = "Option::is_none")]
    pub datasource_set_info: Option<DatasourceSet>,
    #[doc = "Base class for different types of authentication credentials."]
    #[serde(rename = "datasourceAuthCredentials", default, skip_serializing_if = "Option::is_none")]
    pub datasource_auth_credentials: Option<AuthCredentials>,
}
impl RestoreTargetInfo {
    pub fn new(restore_target_info_base: RestoreTargetInfoBase, datasource_info: Datasource) -> Self {
        Self {
            restore_target_info_base,
            datasource_info,
            datasource_set_info: None,
            datasource_auth_credentials: None,
        }
    }
}
#[doc = "Base class common to RestoreTargetInfo and RestoreFilesTargetInfo"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreTargetInfoBase {
    #[doc = "Type of Datasource object, used to initialize the right inherited type"]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[doc = "Recovery Option"]
    #[serde(rename = "recoveryOption")]
    pub recovery_option: restore_target_info_base::RecoveryOption,
    #[doc = "Target Restore region"]
    #[serde(rename = "restoreLocation", default, skip_serializing_if = "Option::is_none")]
    pub restore_location: Option<String>,
}
impl RestoreTargetInfoBase {
    pub fn new(object_type: String, recovery_option: restore_target_info_base::RecoveryOption) -> Self {
        Self {
            object_type,
            recovery_option,
            restore_location: None,
        }
    }
}
pub mod restore_target_info_base {
    use super::*;
    #[doc = "Recovery Option"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryOption")]
    pub enum RecoveryOption {
        FailIfExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::FailIfExists => serializer.serialize_unit_variant("RecoveryOption", 0u32, "FailIfExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Retention tag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionTag {
    #[doc = "Retention Tag version."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Retention Tag version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Retention Tag Name to relate it to retention rule."]
    #[serde(rename = "tagName")]
    pub tag_name: String,
}
impl RetentionTag {
    pub fn new(tag_name: String) -> Self {
        Self {
            e_tag: None,
            id: None,
            tag_name,
        }
    }
}
#[doc = "Schedule based backup criteria"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleBasedBackupCriteria {
    #[serde(flatten)]
    pub backup_criteria: BackupCriteria,
    #[doc = "it contains absolute values like \"AllBackup\" / \"FirstOfDay\" / \"FirstOfWeek\" / \"FirstOfMonth\"\r\nand should be part of AbsoluteMarker enum"]
    #[serde(rename = "absoluteCriteria", default, skip_serializing_if = "Vec::is_empty")]
    pub absolute_criteria: Vec<String>,
    #[doc = "This is day of the month from 1 to 28 other wise last of month"]
    #[serde(rename = "daysOfMonth", default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_month: Vec<Day>,
    #[doc = "It should be Sunday/Monday/T..../Saturday"]
    #[serde(rename = "daysOfTheWeek", default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_the_week: Vec<String>,
    #[doc = "It should be January/February/....../December"]
    #[serde(rename = "monthsOfYear", default, skip_serializing_if = "Vec::is_empty")]
    pub months_of_year: Vec<String>,
    #[doc = "List of schedule times for backup"]
    #[serde(rename = "scheduleTimes", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_times: Vec<time::OffsetDateTime>,
    #[doc = "It should be First/Second/Third/Fourth/Last"]
    #[serde(rename = "weeksOfTheMonth", default, skip_serializing_if = "Vec::is_empty")]
    pub weeks_of_the_month: Vec<String>,
}
impl ScheduleBasedBackupCriteria {
    pub fn new(backup_criteria: BackupCriteria) -> Self {
        Self {
            backup_criteria,
            absolute_criteria: Vec::new(),
            days_of_month: Vec::new(),
            days_of_the_week: Vec::new(),
            months_of_year: Vec::new(),
            schedule_times: Vec::new(),
            weeks_of_the_month: Vec::new(),
        }
    }
}
#[doc = "Schedule based trigger context"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleBasedTriggerContext {
    #[serde(flatten)]
    pub trigger_context: TriggerContext,
    #[doc = "Schedule for backup"]
    pub schedule: BackupSchedule,
    #[doc = "List of tags that can be applicable for given schedule."]
    #[serde(rename = "taggingCriteria")]
    pub tagging_criteria: Vec<TaggingCriteria>,
}
impl ScheduleBasedTriggerContext {
    pub fn new(trigger_context: TriggerContext, schedule: BackupSchedule, tagging_criteria: Vec<TaggingCriteria>) -> Self {
        Self {
            trigger_context,
            schedule,
            tagging_criteria,
        }
    }
}
#[doc = "Secret store based authentication credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretStoreBasedAuthCredentials {
    #[serde(flatten)]
    pub auth_credentials: AuthCredentials,
    #[doc = "Class representing a secret store resource."]
    #[serde(rename = "secretStoreResource", default, skip_serializing_if = "Option::is_none")]
    pub secret_store_resource: Option<SecretStoreResource>,
}
impl SecretStoreBasedAuthCredentials {
    pub fn new(auth_credentials: AuthCredentials) -> Self {
        Self {
            auth_credentials,
            secret_store_resource: None,
        }
    }
}
#[doc = "Class representing a secret store resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretStoreResource {
    #[doc = "Uri to get to the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Gets or sets the type of secret store"]
    #[serde(rename = "secretStoreType")]
    pub secret_store_type: secret_store_resource::SecretStoreType,
    #[doc = "Gets or sets value stored in secret store resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SecretStoreResource {
    pub fn new(secret_store_type: secret_store_resource::SecretStoreType) -> Self {
        Self {
            uri: None,
            secret_store_type,
            value: None,
        }
    }
}
pub mod secret_store_resource {
    use super::*;
    #[doc = "Gets or sets the type of secret store"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecretStoreType")]
    pub enum SecretStoreType {
        Invalid,
        AzureKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecretStoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecretStoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecretStoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("SecretStoreType", 0u32, "Invalid"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("SecretStoreType", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Source LifeCycle"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceLifeCycle {
    #[doc = "Delete Option"]
    #[serde(rename = "deleteAfter")]
    pub delete_after: DeleteOption,
    #[doc = "DataStoreInfo base"]
    #[serde(rename = "sourceDataStore")]
    pub source_data_store: DataStoreInfoBase,
    #[serde(rename = "targetDataStoreCopySettings", default, skip_serializing_if = "Vec::is_empty")]
    pub target_data_store_copy_settings: Vec<TargetCopySetting>,
}
impl SourceLifeCycle {
    pub fn new(delete_after: DeleteOption, source_data_store: DataStoreInfoBase) -> Self {
        Self {
            delete_after,
            source_data_store,
            target_data_store_copy_settings: Vec::new(),
        }
    }
}
#[doc = "Storage setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSetting {
    #[doc = "Gets or sets the type of the datastore."]
    #[serde(rename = "datastoreType", default, skip_serializing_if = "Option::is_none")]
    pub datastore_type: Option<storage_setting::DatastoreType>,
    #[doc = "Gets or sets the type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<storage_setting::Type>,
}
impl StorageSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_setting {
    use super::*;
    #[doc = "Gets or sets the type of the datastore."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DatastoreType")]
    pub enum DatastoreType {
        ArchiveStore,
        SnapshotStore,
        VaultStore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DatastoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DatastoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DatastoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ArchiveStore => serializer.serialize_unit_variant("DatastoreType", 0u32, "ArchiveStore"),
                Self::SnapshotStore => serializer.serialize_unit_variant("DatastoreType", 1u32, "SnapshotStore"),
                Self::VaultStore => serializer.serialize_unit_variant("DatastoreType", 2u32, "VaultStore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        GeoRedundant,
        LocallyRedundant,
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
                Self::GeoRedundant => serializer.serialize_unit_variant("Type", 0u32, "GeoRedundant"),
                Self::LocallyRedundant => serializer.serialize_unit_variant("Type", 1u32, "LocallyRedundant"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Elements class for feature request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedFeature {
    #[doc = "support feature type."]
    #[serde(rename = "featureName", default, skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[doc = "feature support status"]
    #[serde(rename = "supportStatus", default, skip_serializing_if = "Option::is_none")]
    pub support_status: Option<supported_feature::SupportStatus>,
    #[doc = "support feature type."]
    #[serde(rename = "exposureControlledFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub exposure_controlled_features: Vec<String>,
}
impl SupportedFeature {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod supported_feature {
    use super::*;
    #[doc = "feature support status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportStatus")]
    pub enum SupportStatus {
        Invalid,
        NotSupported,
        AlphaPreview,
        PrivatePreview,
        PublicPreview,
        GenerallyAvailable,
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
                Self::NotSupported => serializer.serialize_unit_variant("SupportStatus", 1u32, "NotSupported"),
                Self::AlphaPreview => serializer.serialize_unit_variant("SupportStatus", 2u32, "AlphaPreview"),
                Self::PrivatePreview => serializer.serialize_unit_variant("SupportStatus", 3u32, "PrivatePreview"),
                Self::PublicPreview => serializer.serialize_unit_variant("SupportStatus", 4u32, "PublicPreview"),
                Self::GenerallyAvailable => serializer.serialize_unit_variant("SupportStatus", 5u32, "GenerallyAvailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Sync BackupInstance Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncBackupInstanceRequest {
    #[doc = "Field indicating sync type e.g. to sync only in case of failure or in all cases"]
    #[serde(rename = "syncType", default, skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<sync_backup_instance_request::SyncType>,
}
impl SyncBackupInstanceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sync_backup_instance_request {
    use super::*;
    #[doc = "Field indicating sync type e.g. to sync only in case of failure or in all cases"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncType")]
    pub enum SyncType {
        Default,
        ForceResync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("SyncType", 0u32, "Default"),
                Self::ForceResync => serializer.serialize_unit_variant("SyncType", 1u32, "ForceResync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Tagging criteria"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggingCriteria {
    #[doc = "Criteria which decides whether the tag can be applied to a triggered backup."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<BackupCriteria>,
    #[doc = "Specifies if tag is default."]
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[doc = "Retention Tag priority."]
    #[serde(rename = "taggingPriority")]
    pub tagging_priority: i64,
    #[doc = "Retention tag"]
    #[serde(rename = "tagInfo")]
    pub tag_info: RetentionTag,
}
impl TaggingCriteria {
    pub fn new(is_default: bool, tagging_priority: i64, tag_info: RetentionTag) -> Self {
        Self {
            criteria: Vec::new(),
            is_default,
            tagging_priority,
            tag_info,
        }
    }
}
#[doc = "Target copy settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetCopySetting {
    #[doc = "Options to copy"]
    #[serde(rename = "copyAfter")]
    pub copy_after: CopyOption,
    #[doc = "DataStoreInfo base"]
    #[serde(rename = "dataStore")]
    pub data_store: DataStoreInfoBase,
}
impl TargetCopySetting {
    pub fn new(copy_after: CopyOption, data_store: DataStoreInfoBase) -> Self {
        Self { copy_after, data_store }
    }
}
#[doc = "Class encapsulating target details, used where the destination is not a datasource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetDetails {
    #[doc = "Restore operation may create multiple files inside location pointed by Url\r\nBelow will be the common prefix for all of them"]
    #[serde(rename = "filePrefix")]
    pub file_prefix: String,
    #[doc = "Denotes the target location where the data will be restored,\r\nstring value for the enum {Microsoft.Internal.AzureBackup.DataProtection.Common.Interface.RestoreTargetLocationType}"]
    #[serde(rename = "restoreTargetLocationType")]
    pub restore_target_location_type: target_details::RestoreTargetLocationType,
    #[doc = "Url denoting the restore destination. It can point to container / file share etc"]
    pub url: String,
}
impl TargetDetails {
    pub fn new(file_prefix: String, restore_target_location_type: target_details::RestoreTargetLocationType, url: String) -> Self {
        Self {
            file_prefix,
            restore_target_location_type,
            url,
        }
    }
}
pub mod target_details {
    use super::*;
    #[doc = "Denotes the target location where the data will be restored,\r\nstring value for the enum {Microsoft.Internal.AzureBackup.DataProtection.Common.Interface.RestoreTargetLocationType}"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RestoreTargetLocationType")]
    pub enum RestoreTargetLocationType {
        Invalid,
        AzureBlobs,
        AzureFiles,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RestoreTargetLocationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RestoreTargetLocationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RestoreTargetLocationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("RestoreTargetLocationType", 0u32, "Invalid"),
                Self::AzureBlobs => serializer.serialize_unit_variant("RestoreTargetLocationType", 1u32, "AzureBlobs"),
                Self::AzureFiles => serializer.serialize_unit_variant("RestoreTargetLocationType", 2u32, "AzureFiles"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Trigger backup request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerBackupRequest {
    #[doc = "Adhoc backup rules"]
    #[serde(rename = "backupRuleOptions")]
    pub backup_rule_options: AdHocBackupRuleOptions,
}
impl TriggerBackupRequest {
    pub fn new(backup_rule_options: AdHocBackupRuleOptions) -> Self {
        Self { backup_rule_options }
    }
}
#[doc = "Trigger context"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerContext {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl TriggerContext {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "Error object used by layers that have access to localized content, and propagate that to user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserFacingError {
    #[doc = "Unique code for this error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Additional related Errors"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<UserFacingError>,
    #[doc = "Inner Error"]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
    #[doc = "Whether the operation will be retryable or not"]
    #[serde(rename = "isRetryable", default, skip_serializing_if = "Option::is_none")]
    pub is_retryable: Option<bool>,
    #[doc = "Whether the operation is due to a user error or service error"]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
    #[doc = "Any key value pairs that can be injected inside error object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "RecommendedAction � localized."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_action: Vec<String>,
    #[doc = "Target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl UserFacingError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Validate for backup request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateForBackupRequest {
    #[doc = "Backup Instance"]
    #[serde(rename = "backupInstance")]
    pub backup_instance: BackupInstance,
}
impl ValidateForBackupRequest {
    pub fn new(backup_instance: BackupInstance) -> Self {
        Self { backup_instance }
    }
}
#[doc = "Validate restore request object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateRestoreRequestObject {
    #[doc = "Azure backup restore request"]
    #[serde(rename = "restoreRequestObject")]
    pub restore_request_object: AzureBackupRestoreRequest,
}
impl ValidateRestoreRequestObject {
    pub fn new(restore_request_object: AzureBackupRestoreRequest) -> Self {
        Self { restore_request_object }
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
    #[doc = "The type of identity that last modified the resource."]
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
