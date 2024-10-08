#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Activation Links model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivationLinks {
    #[doc = "New Cloud Account Activation Link"]
    #[serde(rename = "newCloudAccountActivationLink", default, skip_serializing_if = "Option::is_none")]
    pub new_cloud_account_activation_link: Option<String>,
    #[doc = "Existing Cloud Account Activation Link"]
    #[serde(rename = "existingCloudAccountActivationLink", default, skip_serializing_if = "Option::is_none")]
    pub existing_cloud_account_activation_link: Option<String>,
}
impl ActivationLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Add/Remove (Virtual Machine) DbNode model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRemoveDbNode {
    #[doc = "Db servers ocids"]
    #[serde(rename = "dbServers")]
    pub db_servers: Vec<Ocid>,
}
impl AddRemoveDbNode {
    pub fn new(db_servers: Vec<Ocid>) -> Self {
        Self { db_servers }
    }
}
#[doc = "The connection string profile to allow clients to group, filter and select connection string values based on structured metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllConnectionStringType {
    #[doc = "The High database service provides the highest level of resources to each SQL statement resulting in the highest performance, but supports the fewest number of concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[doc = "The Low database service provides the least level of resources to each SQL statement, but supports the most number of concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[doc = "The Medium database service provides a lower level of resources to each SQL statement potentially resulting a lower level of performance, but supports more concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
}
impl AllConnectionStringType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about Oracle APEX Application Development."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApexDetailsType {
    #[doc = "The Oracle APEX Application Development version."]
    #[serde(rename = "apexVersion", default, skip_serializing_if = "Option::is_none")]
    pub apex_version: Option<String>,
    #[doc = "The Oracle REST Data Services (ORDS) version."]
    #[serde(rename = "ordsVersion", default, skip_serializing_if = "Option::is_none")]
    pub ords_version: Option<String>,
}
impl ApexDetailsType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous Database  resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabase {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Autonomous Database base resource model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDatabaseBasePropertiesUnion>,
}
impl AutonomousDatabase {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "AutonomousDatabaseBackup resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseBackup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AutonomousDatabaseBackup resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDatabaseBackupProperties>,
}
impl AutonomousDatabaseBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous database backup lifecycle state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutonomousDatabaseBackupLifecycleState")]
pub enum AutonomousDatabaseBackupLifecycleState {
    Creating,
    Active,
    Deleting,
    Failed,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutonomousDatabaseBackupLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutonomousDatabaseBackupLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutonomousDatabaseBackupLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("AutonomousDatabaseBackupLifecycleState", 0u32, "Creating"),
            Self::Active => serializer.serialize_unit_variant("AutonomousDatabaseBackupLifecycleState", 1u32, "Active"),
            Self::Deleting => serializer.serialize_unit_variant("AutonomousDatabaseBackupLifecycleState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("AutonomousDatabaseBackupLifecycleState", 3u32, "Failed"),
            Self::Updating => serializer.serialize_unit_variant("AutonomousDatabaseBackupLifecycleState", 4u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a AutonomousDatabaseBackup list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseBackupListResult {
    #[doc = "The AutonomousDatabaseBackup items on this page"]
    pub value: Vec<AutonomousDatabaseBackup>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutonomousDatabaseBackupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutonomousDatabaseBackupListResult {
    pub fn new(value: Vec<AutonomousDatabaseBackup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AutonomousDatabaseBackup resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseBackupProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "autonomousDatabaseOcid", default, skip_serializing_if = "Option::is_none")]
    pub autonomous_database_ocid: Option<Ocid>,
    #[doc = "The size of the database in terabytes at the time the backup was taken."]
    #[serde(rename = "databaseSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub database_size_in_tbs: Option<f64>,
    #[doc = "A valid Oracle Database version for Autonomous Database."]
    #[serde(rename = "dbVersion", default, skip_serializing_if = "Option::is_none")]
    pub db_version: Option<String>,
    #[doc = "The user-friendly name for the backup. The name does not have to be unique."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "Indicates whether the backup is user-initiated or automatic."]
    #[serde(rename = "isAutomatic", default, skip_serializing_if = "Option::is_none")]
    pub is_automatic: Option<bool>,
    #[doc = "Indicates whether the backup can be used to restore the associated Autonomous Database."]
    #[serde(rename = "isRestorable", default, skip_serializing_if = "Option::is_none")]
    pub is_restorable: Option<bool>,
    #[doc = "Additional information about the current lifecycle state."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "Autonomous database backup lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<AutonomousDatabaseBackupLifecycleState>,
    #[doc = "Retention period, in days, for long-term backups."]
    #[serde(rename = "retentionPeriodInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
    #[doc = "The backup size in terabytes (TB)."]
    #[serde(rename = "sizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub size_in_tbs: Option<f64>,
    #[doc = "Timestamp until when the backup will be available."]
    #[serde(rename = "timeAvailableTil", default, with = "azure_core::date::rfc3339::option")]
    pub time_available_til: Option<::time::OffsetDateTime>,
    #[doc = "The date and time the backup started."]
    #[serde(rename = "timeStarted", default, skip_serializing_if = "Option::is_none")]
    pub time_started: Option<String>,
    #[doc = "The date and time the backup completed."]
    #[serde(rename = "timeEnded", default, skip_serializing_if = "Option::is_none")]
    pub time_ended: Option<String>,
    #[doc = "Autonomous database backup type enum"]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<AutonomousDatabaseBackupType>,
    #[doc = "Azure Resource Provisioning State enum"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceProvisioningState>,
}
impl AutonomousDatabaseBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous database backup type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutonomousDatabaseBackupType")]
pub enum AutonomousDatabaseBackupType {
    Incremental,
    Full,
    LongTerm,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutonomousDatabaseBackupType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutonomousDatabaseBackupType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutonomousDatabaseBackupType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Incremental => serializer.serialize_unit_variant("AutonomousDatabaseBackupType", 0u32, "Incremental"),
            Self::Full => serializer.serialize_unit_variant("AutonomousDatabaseBackupType", 1u32, "Full"),
            Self::LongTerm => serializer.serialize_unit_variant("AutonomousDatabaseBackupType", 2u32, "LongTerm"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the AutonomousDatabaseBackup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseBackupUpdate {
    #[doc = "The updatable properties of the AutonomousDatabaseBackup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDatabaseBackupUpdateProperties>,
}
impl AutonomousDatabaseBackupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the AutonomousDatabaseBackup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseBackupUpdateProperties {
    #[doc = "Retention period, in days, for long-term backups."]
    #[serde(rename = "retentionPeriodInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
}
impl AutonomousDatabaseBackupUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous Database base resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseBaseProperties {
    #[doc = "Password string."]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<Password>,
    #[doc = "Autonomous database maintenance schedule type enum."]
    #[serde(rename = "autonomousMaintenanceScheduleType", default, skip_serializing_if = "Option::is_none")]
    pub autonomous_maintenance_schedule_type: Option<AutonomousMaintenanceScheduleType>,
    #[doc = "The character set for the autonomous database."]
    #[serde(rename = "characterSet", default, skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
    #[doc = "The compute amount (CPUs) available to the database."]
    #[serde(rename = "computeCount", default, skip_serializing_if = "Option::is_none")]
    pub compute_count: Option<f32>,
    #[doc = "Compute model enum"]
    #[serde(rename = "computeModel", default, skip_serializing_if = "Option::is_none")]
    pub compute_model: Option<ComputeModel>,
    #[doc = "The number of CPU cores to be made available to the database."]
    #[serde(rename = "cpuCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_core_count: Option<i32>,
    #[doc = "Customer Contacts."]
    #[serde(
        rename = "customerContacts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customer_contacts: Vec<CustomerContact>,
    #[doc = "The quantity of data in the database, in terabytes."]
    #[serde(rename = "dataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_tbs: Option<i32>,
    #[doc = "The size, in gigabytes, of the data volume that will be created and attached to the database."]
    #[serde(rename = "dataStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_gbs: Option<i32>,
    #[doc = "A valid Oracle Database version for Autonomous Database."]
    #[serde(rename = "dbVersion", default, skip_serializing_if = "Option::is_none")]
    pub db_version: Option<String>,
    #[doc = "WorkloadType enum"]
    #[serde(rename = "dbWorkload", default, skip_serializing_if = "Option::is_none")]
    pub db_workload: Option<WorkloadType>,
    #[doc = "The user-friendly name for the Autonomous Database."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicates if auto scaling is enabled for the Autonomous Database CPU core count."]
    #[serde(rename = "isAutoScalingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_scaling_enabled: Option<bool>,
    #[doc = "Indicates if auto scaling is enabled for the Autonomous Database storage."]
    #[serde(rename = "isAutoScalingForStorageEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_scaling_for_storage_enabled: Option<bool>,
    #[doc = "The list of [OCIDs](https://docs.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of standby databases located in Autonomous Data Guard remote regions that are associated with the source database. Note that for Autonomous Database Serverless instances, standby databases located in the same region as the source primary database do not have OCIDs."]
    #[serde(
        rename = "peerDbIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub peer_db_ids: Vec<String>,
    #[doc = "The database OCID of the Disaster Recovery peer database, which is located in a different region from the current peer database."]
    #[serde(rename = "peerDbId", default, skip_serializing_if = "Option::is_none")]
    pub peer_db_id: Option<String>,
    #[doc = "Indicates whether the Autonomous Database has local or called in-region Data Guard enabled."]
    #[serde(rename = "isLocalDataGuardEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_local_data_guard_enabled: Option<bool>,
    #[doc = "Indicates whether the Autonomous Database has Cross Region Data Guard enabled."]
    #[serde(rename = "isRemoteDataGuardEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_remote_data_guard_enabled: Option<bool>,
    #[doc = "Disaster recovery type enum."]
    #[serde(rename = "localDisasterRecoveryType", default, skip_serializing_if = "Option::is_none")]
    pub local_disaster_recovery_type: Option<DisasterRecoveryType>,
    #[doc = "Autonomous Disaster Recovery standby database details."]
    #[serde(rename = "localStandbyDb", default, skip_serializing_if = "Option::is_none")]
    pub local_standby_db: Option<AutonomousDatabaseStandbySummary>,
    #[doc = "Indicates the number of seconds of data loss for a Data Guard failover."]
    #[serde(rename = "failedDataRecoveryInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub failed_data_recovery_in_seconds: Option<i32>,
    #[doc = "Specifies if the Autonomous Database requires mTLS connections."]
    #[serde(rename = "isMtlsConnectionRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_mtls_connection_required: Option<bool>,
    #[doc = "Specifies if the Autonomous Database preview version is being provisioned."]
    #[serde(
        rename = "isPreviewVersionWithServiceTermsAccepted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_preview_version_with_service_terms_accepted: Option<bool>,
    #[doc = "LicenseModel enum"]
    #[serde(rename = "licenseModel", default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<LicenseModel>,
    #[doc = "The character set for the Autonomous Database."]
    #[serde(rename = "ncharacterSet", default, skip_serializing_if = "Option::is_none")]
    pub ncharacter_set: Option<String>,
    #[doc = "Additional information about the current lifecycle state."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "Azure Resource Provisioning State enum"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceProvisioningState>,
    #[doc = "Autonomous database lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<AutonomousDatabaseLifecycleState>,
    #[doc = "The list of scheduled operations."]
    #[serde(rename = "scheduledOperations", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_operations: Option<ScheduledOperationsType>,
    #[doc = "The private endpoint Ip address for the resource."]
    #[serde(rename = "privateEndpointIp", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_ip: Option<String>,
    #[doc = "The resource's private endpoint label."]
    #[serde(rename = "privateEndpointLabel", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_label: Option<String>,
    #[doc = "HTTPS link to OCI resources exposed to Azure Customer via Azure Interface."]
    #[serde(rename = "ociUrl", default, skip_serializing_if = "Option::is_none")]
    pub oci_url: Option<String>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<SubnetId>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "vnetId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_id: Option<VnetId>,
    #[doc = "The date and time that the database was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "The date and time when maintenance will begin."]
    #[serde(rename = "timeMaintenanceBegin", default, with = "azure_core::date::rfc3339::option")]
    pub time_maintenance_begin: Option<::time::OffsetDateTime>,
    #[doc = "The date and time when maintenance will end."]
    #[serde(rename = "timeMaintenanceEnd", default, with = "azure_core::date::rfc3339::option")]
    pub time_maintenance_end: Option<::time::OffsetDateTime>,
    #[doc = "The current amount of storage in use for user and system data, in terabytes (TB)."]
    #[serde(rename = "actualUsedDataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub actual_used_data_storage_size_in_tbs: Option<f64>,
    #[doc = "The amount of storage currently allocated for the database tables and billed for, rounded up."]
    #[serde(rename = "allocatedStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub allocated_storage_size_in_tbs: Option<f64>,
    #[doc = "Information about Oracle APEX Application Development."]
    #[serde(rename = "apexDetails", default, skip_serializing_if = "Option::is_none")]
    pub apex_details: Option<ApexDetailsType>,
    #[doc = "List of Oracle Database versions available for a database upgrade. If there are no version upgrades available, this list is empty."]
    #[serde(
        rename = "availableUpgradeVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_upgrade_versions: Vec<String>,
    #[doc = "Connection strings to connect to an Oracle Autonomous Database."]
    #[serde(rename = "connectionStrings", default, skip_serializing_if = "Option::is_none")]
    pub connection_strings: Option<ConnectionStringType>,
    #[doc = "The URLs for accessing Oracle Application Express (APEX) and SQL Developer Web with a browser from a Compute instance within your VCN or that has a direct connection to your VCN."]
    #[serde(rename = "connectionUrls", default, skip_serializing_if = "Option::is_none")]
    pub connection_urls: Option<ConnectionUrlType>,
    #[doc = "DataSafe status type enum."]
    #[serde(rename = "dataSafeStatus", default, skip_serializing_if = "Option::is_none")]
    pub data_safe_status: Option<DataSafeStatusType>,
    #[doc = "Database edition type enum."]
    #[serde(rename = "databaseEdition", default, skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<DatabaseEditionType>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "autonomousDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub autonomous_database_id: Option<AutonomousDatabaseId>,
    #[doc = "The area assigned to In-Memory tables in Autonomous Database."]
    #[serde(rename = "inMemoryAreaInGbs", default, skip_serializing_if = "Option::is_none")]
    pub in_memory_area_in_gbs: Option<i32>,
    #[doc = "The date and time when the next long-term backup would be created."]
    #[serde(rename = "nextLongTermBackupTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub next_long_term_backup_time_stamp: Option<::time::OffsetDateTime>,
    #[doc = "Details for the long-term backup schedule."]
    #[serde(rename = "longTermBackupSchedule", default, skip_serializing_if = "Option::is_none")]
    pub long_term_backup_schedule: Option<LongTermBackUpScheduleDetails>,
    #[doc = "Indicates if the Autonomous Database version is a preview version."]
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[doc = "Parameter that allows users to select an acceptable maximum data loss limit in seconds, up to which Automatic Failover will be triggered when necessary for a Local Autonomous Data Guard"]
    #[serde(rename = "localAdgAutoFailoverMaxDataLossLimit", default, skip_serializing_if = "Option::is_none")]
    pub local_adg_auto_failover_max_data_loss_limit: Option<i32>,
    #[doc = "The amount of memory (in GBs) enabled per ECPU or OCPU."]
    #[serde(rename = "memoryPerOracleComputeUnitInGbs", default, skip_serializing_if = "Option::is_none")]
    pub memory_per_oracle_compute_unit_in_gbs: Option<i32>,
    #[doc = "Open mode type enum."]
    #[serde(rename = "openMode", default, skip_serializing_if = "Option::is_none")]
    pub open_mode: Option<OpenModeType>,
    #[doc = "Operations Insights status type enum."]
    #[serde(rename = "operationsInsightsStatus", default, skip_serializing_if = "Option::is_none")]
    pub operations_insights_status: Option<OperationsInsightsStatusType>,
    #[doc = "Permission level type enum."]
    #[serde(rename = "permissionLevel", default, skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<PermissionLevelType>,
    #[doc = "The private endpoint for the resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<String>,
    #[doc = "An array of CPU values that an Autonomous Database can be scaled to."]
    #[serde(
        rename = "provisionableCpus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub provisionable_cpus: Vec<i32>,
    #[doc = "Role type enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleType>,
    #[doc = "The URL of the Service Console for the Autonomous Database."]
    #[serde(rename = "serviceConsoleUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_console_url: Option<String>,
    #[doc = "The SQL Web Developer URL for the Oracle Autonomous Database."]
    #[serde(rename = "sqlWebDeveloperUrl", default, skip_serializing_if = "Option::is_none")]
    pub sql_web_developer_url: Option<String>,
    #[doc = "The list of regions that support the creation of an Autonomous Database clone or an Autonomous Data Guard standby database."]
    #[serde(
        rename = "supportedRegionsToCloneTo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_regions_to_clone_to: Vec<String>,
    #[doc = "The date and time the Autonomous Data Guard role was switched for the Autonomous Database."]
    #[serde(rename = "timeDataGuardRoleChanged", default, skip_serializing_if = "Option::is_none")]
    pub time_data_guard_role_changed: Option<String>,
    #[doc = "The date and time the Always Free database will be automatically deleted because of inactivity."]
    #[serde(rename = "timeDeletionOfFreeAutonomousDatabase", default, skip_serializing_if = "Option::is_none")]
    pub time_deletion_of_free_autonomous_database: Option<String>,
    #[doc = "The date and time that Autonomous Data Guard was enabled for an Autonomous Database where the standby was provisioned in the same region as the primary database."]
    #[serde(rename = "timeLocalDataGuardEnabled", default, skip_serializing_if = "Option::is_none")]
    pub time_local_data_guard_enabled: Option<String>,
    #[doc = "The timestamp of the last failover operation."]
    #[serde(rename = "timeOfLastFailover", default, skip_serializing_if = "Option::is_none")]
    pub time_of_last_failover: Option<String>,
    #[doc = "The date and time when last refresh happened."]
    #[serde(rename = "timeOfLastRefresh", default, skip_serializing_if = "Option::is_none")]
    pub time_of_last_refresh: Option<String>,
    #[doc = "The refresh point timestamp (UTC)."]
    #[serde(rename = "timeOfLastRefreshPoint", default, skip_serializing_if = "Option::is_none")]
    pub time_of_last_refresh_point: Option<String>,
    #[doc = "The timestamp of the last switchover operation for the Autonomous Database."]
    #[serde(rename = "timeOfLastSwitchover", default, skip_serializing_if = "Option::is_none")]
    pub time_of_last_switchover: Option<String>,
    #[doc = "The date and time the Always Free database will be stopped because of inactivity."]
    #[serde(
        rename = "timeReclamationOfFreeAutonomousDatabase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_reclamation_of_free_autonomous_database: Option<String>,
    #[doc = "The storage space consumed by Autonomous Database in GBs."]
    #[serde(rename = "usedDataStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub used_data_storage_size_in_gbs: Option<i32>,
    #[doc = "The amount of storage that has been used, in terabytes."]
    #[serde(rename = "usedDataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub used_data_storage_size_in_tbs: Option<i32>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "Retention period, in days, for long-term backups"]
    #[serde(rename = "backupRetentionPeriodInDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_period_in_days: Option<i32>,
    #[doc = "The client IP access control list (ACL). This is an array of CIDR notations and/or IP addresses. Values should be separate strings, separated by commas. Example: ['1.1.1.1','1.1.1.0/24','1.1.2.25']"]
    #[serde(
        rename = "whitelistedIps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub whitelisted_ips: Vec<AclString>,
}
impl AutonomousDatabaseBaseProperties {
    pub fn new() -> Self {
        Self {
            admin_password: None,
            autonomous_maintenance_schedule_type: None,
            character_set: None,
            compute_count: None,
            compute_model: None,
            cpu_core_count: None,
            customer_contacts: Vec::new(),
            data_storage_size_in_tbs: None,
            data_storage_size_in_gbs: None,
            db_version: None,
            db_workload: None,
            display_name: None,
            is_auto_scaling_enabled: None,
            is_auto_scaling_for_storage_enabled: None,
            peer_db_ids: Vec::new(),
            peer_db_id: None,
            is_local_data_guard_enabled: None,
            is_remote_data_guard_enabled: None,
            local_disaster_recovery_type: None,
            local_standby_db: None,
            failed_data_recovery_in_seconds: None,
            is_mtls_connection_required: None,
            is_preview_version_with_service_terms_accepted: None,
            license_model: None,
            ncharacter_set: None,
            lifecycle_details: None,
            provisioning_state: None,
            lifecycle_state: None,
            scheduled_operations: None,
            private_endpoint_ip: None,
            private_endpoint_label: None,
            oci_url: None,
            subnet_id: None,
            vnet_id: None,
            time_created: None,
            time_maintenance_begin: None,
            time_maintenance_end: None,
            actual_used_data_storage_size_in_tbs: None,
            allocated_storage_size_in_tbs: None,
            apex_details: None,
            available_upgrade_versions: Vec::new(),
            connection_strings: None,
            connection_urls: None,
            data_safe_status: None,
            database_edition: None,
            autonomous_database_id: None,
            in_memory_area_in_gbs: None,
            next_long_term_backup_time_stamp: None,
            long_term_backup_schedule: None,
            is_preview: None,
            local_adg_auto_failover_max_data_loss_limit: None,
            memory_per_oracle_compute_unit_in_gbs: None,
            open_mode: None,
            operations_insights_status: None,
            permission_level: None,
            private_endpoint: None,
            provisionable_cpus: Vec::new(),
            role: None,
            service_console_url: None,
            sql_web_developer_url: None,
            supported_regions_to_clone_to: Vec::new(),
            time_data_guard_role_changed: None,
            time_deletion_of_free_autonomous_database: None,
            time_local_data_guard_enabled: None,
            time_of_last_failover: None,
            time_of_last_refresh: None,
            time_of_last_refresh_point: None,
            time_of_last_switchover: None,
            time_reclamation_of_free_autonomous_database: None,
            used_data_storage_size_in_gbs: None,
            used_data_storage_size_in_tbs: None,
            ocid: None,
            backup_retention_period_in_days: None,
            whitelisted_ips: Vec::new(),
        }
    }
}
#[doc = "Database type enum"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "dataBaseType")]
pub enum AutonomousDatabaseBasePropertiesUnion {
    Clone(AutonomousDatabaseCloneProperties),
    Regular(AutonomousDatabaseProperties),
}
#[doc = "AutonomousDatabaseCharacterSets resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseCharacterSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AutonomousDatabaseCharacterSet resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDatabaseCharacterSetProperties>,
}
impl AutonomousDatabaseCharacterSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AutonomousDatabaseCharacterSet list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseCharacterSetListResult {
    #[doc = "The AutonomousDatabaseCharacterSet items on this page"]
    pub value: Vec<AutonomousDatabaseCharacterSet>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutonomousDatabaseCharacterSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutonomousDatabaseCharacterSetListResult {
    pub fn new(value: Vec<AutonomousDatabaseCharacterSet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AutonomousDatabaseCharacterSet resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseCharacterSetProperties {
    #[doc = "The Oracle Autonomous Database supported character sets."]
    #[serde(rename = "characterSet", default, skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
}
impl AutonomousDatabaseCharacterSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous Database clone resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseCloneProperties {
    #[serde(flatten)]
    pub autonomous_database_base_properties: AutonomousDatabaseBaseProperties,
    #[doc = "Source type enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceType>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "sourceId")]
    pub source_id: AutonomousDatabaseId,
    #[doc = "Clone type enum"]
    #[serde(rename = "cloneType")]
    pub clone_type: CloneType,
    #[doc = "Indicates if the refreshable clone can be reconnected to its source database."]
    #[serde(rename = "isReconnectCloneEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_reconnect_clone_enabled: Option<bool>,
    #[doc = "Indicates if the Autonomous Database is a refreshable clone."]
    #[serde(rename = "isRefreshableClone", default, skip_serializing_if = "Option::is_none")]
    pub is_refreshable_clone: Option<bool>,
    #[doc = "Refreshable model type enum"]
    #[serde(rename = "refreshableModel", default, skip_serializing_if = "Option::is_none")]
    pub refreshable_model: Option<RefreshableModelType>,
    #[doc = "Refreshable status type enum."]
    #[serde(rename = "refreshableStatus", default, skip_serializing_if = "Option::is_none")]
    pub refreshable_status: Option<RefreshableStatusType>,
    #[doc = "The time and date as an RFC3339 formatted string, e.g., 2022-01-01T12:00:00.000Z, to set the limit for a refreshable clone to be reconnected to its source database."]
    #[serde(rename = "timeUntilReconnectCloneEnabled", default, skip_serializing_if = "Option::is_none")]
    pub time_until_reconnect_clone_enabled: Option<String>,
}
impl AutonomousDatabaseCloneProperties {
    pub fn new(
        autonomous_database_base_properties: AutonomousDatabaseBaseProperties,
        source_id: AutonomousDatabaseId,
        clone_type: CloneType,
    ) -> Self {
        Self {
            autonomous_database_base_properties,
            source: None,
            source_id,
            clone_type,
            is_reconnect_clone_enabled: None,
            is_refreshable_clone: None,
            refreshable_model: None,
            refreshable_status: None,
            time_until_reconnect_clone_enabled: None,
        }
    }
}
pub type AutonomousDatabaseId = String;
#[doc = "Autonomous database lifecycle state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutonomousDatabaseLifecycleState")]
pub enum AutonomousDatabaseLifecycleState {
    Provisioning,
    Available,
    Stopping,
    Stopped,
    Starting,
    Terminating,
    Terminated,
    Unavailable,
    RestoreInProgress,
    RestoreFailed,
    BackupInProgress,
    ScaleInProgress,
    AvailableNeedsAttention,
    Updating,
    MaintenanceInProgress,
    Restarting,
    Recreating,
    RoleChangeInProgress,
    Upgrading,
    Inaccessible,
    Standby,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutonomousDatabaseLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutonomousDatabaseLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutonomousDatabaseLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 0u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 1u32, "Available"),
            Self::Stopping => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 2u32, "Stopping"),
            Self::Stopped => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 3u32, "Stopped"),
            Self::Starting => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 4u32, "Starting"),
            Self::Terminating => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 5u32, "Terminating"),
            Self::Terminated => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 6u32, "Terminated"),
            Self::Unavailable => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 7u32, "Unavailable"),
            Self::RestoreInProgress => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 8u32, "RestoreInProgress"),
            Self::RestoreFailed => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 9u32, "RestoreFailed"),
            Self::BackupInProgress => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 10u32, "BackupInProgress"),
            Self::ScaleInProgress => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 11u32, "ScaleInProgress"),
            Self::AvailableNeedsAttention => {
                serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 12u32, "AvailableNeedsAttention")
            }
            Self::Updating => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 13u32, "Updating"),
            Self::MaintenanceInProgress => {
                serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 14u32, "MaintenanceInProgress")
            }
            Self::Restarting => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 15u32, "Restarting"),
            Self::Recreating => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 16u32, "Recreating"),
            Self::RoleChangeInProgress => {
                serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 17u32, "RoleChangeInProgress")
            }
            Self::Upgrading => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 18u32, "Upgrading"),
            Self::Inaccessible => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 19u32, "Inaccessible"),
            Self::Standby => serializer.serialize_unit_variant("AutonomousDatabaseLifecycleState", 20u32, "Standby"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a AutonomousDatabase list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseListResult {
    #[doc = "The AutonomousDatabase items on this page"]
    pub value: Vec<AutonomousDatabase>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutonomousDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutonomousDatabaseListResult {
    pub fn new(value: Vec<AutonomousDatabase>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AutonomousDatabaseNationalCharacterSets resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseNationalCharacterSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AutonomousDatabaseNationalCharacterSet resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDatabaseNationalCharacterSetProperties>,
}
impl AutonomousDatabaseNationalCharacterSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AutonomousDatabaseNationalCharacterSet list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseNationalCharacterSetListResult {
    #[doc = "The AutonomousDatabaseNationalCharacterSet items on this page"]
    pub value: Vec<AutonomousDatabaseNationalCharacterSet>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutonomousDatabaseNationalCharacterSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutonomousDatabaseNationalCharacterSetListResult {
    pub fn new(value: Vec<AutonomousDatabaseNationalCharacterSet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AutonomousDatabaseNationalCharacterSet resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseNationalCharacterSetProperties {
    #[doc = "The Oracle Autonomous Database supported national character sets."]
    #[serde(rename = "characterSet", default, skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
}
impl AutonomousDatabaseNationalCharacterSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous Database resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseProperties {
    #[serde(flatten)]
    pub autonomous_database_base_properties: AutonomousDatabaseBaseProperties,
}
impl AutonomousDatabaseProperties {
    pub fn new(autonomous_database_base_properties: AutonomousDatabaseBaseProperties) -> Self {
        Self {
            autonomous_database_base_properties,
        }
    }
}
#[doc = "Autonomous Disaster Recovery standby database details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseStandbySummary {
    #[doc = "The amount of time, in seconds, that the data of the standby database lags the data of the primary database. Can be used to determine the potential data loss in the event of a failover."]
    #[serde(rename = "lagTimeInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub lag_time_in_seconds: Option<i32>,
    #[doc = "Autonomous database lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<AutonomousDatabaseLifecycleState>,
    #[doc = "Additional information about the current lifecycle state."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "The date and time the Autonomous Data Guard role was switched for the standby Autonomous Database."]
    #[serde(rename = "timeDataGuardRoleChanged", default, skip_serializing_if = "Option::is_none")]
    pub time_data_guard_role_changed: Option<String>,
    #[doc = "The date and time the Disaster Recovery role was switched for the standby Autonomous Database."]
    #[serde(rename = "timeDisasterRecoveryRoleChanged", default, skip_serializing_if = "Option::is_none")]
    pub time_disaster_recovery_role_changed: Option<String>,
}
impl AutonomousDatabaseStandbySummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the AutonomousDatabase."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the AutonomousDatabase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDatabaseUpdateProperties>,
}
impl AutonomousDatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the AutonomousDatabase."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDatabaseUpdateProperties {
    #[doc = "Password string."]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<Password>,
    #[doc = "Autonomous database maintenance schedule type enum."]
    #[serde(rename = "autonomousMaintenanceScheduleType", default, skip_serializing_if = "Option::is_none")]
    pub autonomous_maintenance_schedule_type: Option<AutonomousMaintenanceScheduleType>,
    #[doc = "The compute amount (CPUs) available to the database."]
    #[serde(rename = "computeCount", default, skip_serializing_if = "Option::is_none")]
    pub compute_count: Option<f32>,
    #[doc = "The number of CPU cores to be made available to the database."]
    #[serde(rename = "cpuCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_core_count: Option<i32>,
    #[doc = "Customer Contacts."]
    #[serde(
        rename = "customerContacts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customer_contacts: Vec<CustomerContact>,
    #[doc = "The quantity of data in the database, in terabytes."]
    #[serde(rename = "dataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_tbs: Option<i32>,
    #[doc = "The size, in gigabytes, of the data volume that will be created and attached to the database."]
    #[serde(rename = "dataStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_gbs: Option<i32>,
    #[doc = "The user-friendly name for the Autonomous Database."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicates if auto scaling is enabled for the Autonomous Database CPU core count."]
    #[serde(rename = "isAutoScalingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_scaling_enabled: Option<bool>,
    #[doc = "Indicates if auto scaling is enabled for the Autonomous Database storage."]
    #[serde(rename = "isAutoScalingForStorageEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_scaling_for_storage_enabled: Option<bool>,
    #[doc = "The database OCID of the Disaster Recovery peer database, which is located in a different region from the current peer database."]
    #[serde(rename = "peerDbId", default, skip_serializing_if = "Option::is_none")]
    pub peer_db_id: Option<String>,
    #[doc = "Indicates whether the Autonomous Database has local or called in-region Data Guard enabled."]
    #[serde(rename = "isLocalDataGuardEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_local_data_guard_enabled: Option<bool>,
    #[doc = "Specifies if the Autonomous Database requires mTLS connections."]
    #[serde(rename = "isMtlsConnectionRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_mtls_connection_required: Option<bool>,
    #[doc = "LicenseModel enum"]
    #[serde(rename = "licenseModel", default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<LicenseModel>,
    #[doc = "The list of scheduled operations."]
    #[serde(rename = "scheduledOperations", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_operations: Option<ScheduledOperationsTypeUpdate>,
    #[doc = "Database edition type enum."]
    #[serde(rename = "databaseEdition", default, skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<DatabaseEditionType>,
    #[doc = "Details for the long-term backup schedule."]
    #[serde(rename = "longTermBackupSchedule", default, skip_serializing_if = "Option::is_none")]
    pub long_term_backup_schedule: Option<LongTermBackUpScheduleDetails>,
    #[doc = "Parameter that allows users to select an acceptable maximum data loss limit in seconds, up to which Automatic Failover will be triggered when necessary for a Local Autonomous Data Guard"]
    #[serde(rename = "localAdgAutoFailoverMaxDataLossLimit", default, skip_serializing_if = "Option::is_none")]
    pub local_adg_auto_failover_max_data_loss_limit: Option<i32>,
    #[doc = "Open mode type enum."]
    #[serde(rename = "openMode", default, skip_serializing_if = "Option::is_none")]
    pub open_mode: Option<OpenModeType>,
    #[doc = "Permission level type enum."]
    #[serde(rename = "permissionLevel", default, skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<PermissionLevelType>,
    #[doc = "Role type enum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleType>,
    #[doc = "Retention period, in days, for long-term backups"]
    #[serde(rename = "backupRetentionPeriodInDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_period_in_days: Option<i32>,
    #[doc = "The client IP access control list (ACL). This is an array of CIDR notations and/or IP addresses. Values should be separate strings, separated by commas. Example: ['1.1.1.1','1.1.1.0/24','1.1.2.25']"]
    #[serde(
        rename = "whitelistedIps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub whitelisted_ips: Vec<AclString>,
}
impl AutonomousDatabaseUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous Database Wallet File resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDatabaseWalletFile {
    #[doc = "The base64 encoded wallet files"]
    #[serde(rename = "walletFiles")]
    pub wallet_files: String,
}
impl AutonomousDatabaseWalletFile {
    pub fn new(wallet_files: String) -> Self {
        Self { wallet_files }
    }
}
#[doc = "AutonomousDbVersion resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDbVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "AutonomousDbVersion resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutonomousDbVersionProperties>,
}
impl AutonomousDbVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a AutonomousDbVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousDbVersionListResult {
    #[doc = "The AutonomousDbVersion items on this page"]
    pub value: Vec<AutonomousDbVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutonomousDbVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutonomousDbVersionListResult {
    pub fn new(value: Vec<AutonomousDbVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "AutonomousDbVersion resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutonomousDbVersionProperties {
    #[doc = "Supported Autonomous Db versions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "WorkloadType enum"]
    #[serde(rename = "dbWorkload", default, skip_serializing_if = "Option::is_none")]
    pub db_workload: Option<WorkloadType>,
    #[doc = "True if this version of the Oracle Database software's default is free."]
    #[serde(rename = "isDefaultForFree", default, skip_serializing_if = "Option::is_none")]
    pub is_default_for_free: Option<bool>,
    #[doc = "True if this version of the Oracle Database software's default is paid."]
    #[serde(rename = "isDefaultForPaid", default, skip_serializing_if = "Option::is_none")]
    pub is_default_for_paid: Option<bool>,
    #[doc = "True if this version of the Oracle Database software can be used for Always-Free Autonomous Databases."]
    #[serde(rename = "isFreeTierEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_free_tier_enabled: Option<bool>,
    #[doc = "True if this version of the Oracle Database software has payments enabled."]
    #[serde(rename = "isPaidEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_paid_enabled: Option<bool>,
}
impl AutonomousDbVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous database maintenance schedule type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutonomousMaintenanceScheduleType")]
pub enum AutonomousMaintenanceScheduleType {
    Early,
    Regular,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutonomousMaintenanceScheduleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutonomousMaintenanceScheduleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutonomousMaintenanceScheduleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Early => serializer.serialize_unit_variant("AutonomousMaintenanceScheduleType", 0u32, "Early"),
            Self::Regular => serializer.serialize_unit_variant("AutonomousMaintenanceScheduleType", 1u32, "Regular"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The provisioning state of a resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceManagerResourceProvisioningState")]
pub enum AzureResourceManagerResourceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceManagerResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceManagerResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceManagerResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Resource Provisioning State enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceProvisioningState")]
pub enum AzureResourceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AzureResourceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureResourceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureResourceProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("AzureResourceProvisioningState", 3u32, "Provisioning"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Clone type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloneType")]
pub enum CloneType {
    Full,
    Metadata,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloneType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloneType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloneType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Full => serializer.serialize_unit_variant("CloneType", 0u32, "Full"),
            Self::Metadata => serializer.serialize_unit_variant("CloneType", 1u32, "Metadata"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cloud Account Details model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudAccountDetails {
    #[doc = "Cloud Account name"]
    #[serde(rename = "cloudAccountName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_account_name: Option<String>,
    #[doc = "Cloud Account Home region"]
    #[serde(rename = "cloudAccountHomeRegion", default, skip_serializing_if = "Option::is_none")]
    pub cloud_account_home_region: Option<String>,
}
impl CloudAccountDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CloudAccountProvisioningState enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudAccountProvisioningState")]
pub enum CloudAccountProvisioningState {
    Pending,
    Provisioning,
    Available,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudAccountProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudAccountProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudAccountProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("CloudAccountProvisioningState", 0u32, "Pending"),
            Self::Provisioning => serializer.serialize_unit_variant("CloudAccountProvisioningState", 1u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("CloudAccountProvisioningState", 2u32, "Available"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "CloudExadataInfrastructure resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudExadataInfrastructure {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "CloudExadataInfrastructure resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudExadataInfrastructureProperties>,
    #[doc = "CloudExadataInfrastructure zones"]
    pub zones: Vec<String>,
}
impl CloudExadataInfrastructure {
    pub fn new(tracked_resource: TrackedResource, zones: Vec<String>) -> Self {
        Self {
            tracked_resource,
            properties: None,
            zones,
        }
    }
}
pub type CloudExadataInfrastructureId = String;
#[doc = "CloudExadataInfrastructureLifecycleState enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudExadataInfrastructureLifecycleState")]
pub enum CloudExadataInfrastructureLifecycleState {
    Provisioning,
    Available,
    Updating,
    Terminating,
    Terminated,
    MaintenanceInProgress,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudExadataInfrastructureLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudExadataInfrastructureLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudExadataInfrastructureLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 0u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 1u32, "Available"),
            Self::Updating => serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 2u32, "Updating"),
            Self::Terminating => serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 3u32, "Terminating"),
            Self::Terminated => serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 4u32, "Terminated"),
            Self::MaintenanceInProgress => {
                serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 5u32, "MaintenanceInProgress")
            }
            Self::Failed => serializer.serialize_unit_variant("CloudExadataInfrastructureLifecycleState", 6u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a CloudExadataInfrastructure list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudExadataInfrastructureListResult {
    #[doc = "The CloudExadataInfrastructure items on this page"]
    pub value: Vec<CloudExadataInfrastructure>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudExadataInfrastructureListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CloudExadataInfrastructureListResult {
    pub fn new(value: Vec<CloudExadataInfrastructure>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "CloudExadataInfrastructure resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudExadataInfrastructureProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "The number of compute servers for the cloud Exadata infrastructure."]
    #[serde(rename = "computeCount", default, skip_serializing_if = "Option::is_none")]
    pub compute_count: Option<i32>,
    #[doc = "The number of storage servers for the cloud Exadata infrastructure."]
    #[serde(rename = "storageCount", default, skip_serializing_if = "Option::is_none")]
    pub storage_count: Option<i32>,
    #[doc = "The total storage allocated to the cloud Exadata infrastructure resource, in gigabytes (GB)."]
    #[serde(rename = "totalStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub total_storage_size_in_gbs: Option<i32>,
    #[doc = "The available storage can be allocated to the cloud Exadata infrastructure resource, in gigabytes (GB)."]
    #[serde(rename = "availableStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub available_storage_size_in_gbs: Option<i32>,
    #[doc = "The date and time the cloud Exadata infrastructure resource was created."]
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[doc = "Additional information about the current lifecycle state."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "MaintenanceWindow resource properties"]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "The estimated total time required in minutes for all patching operations (database server, storage server, and network switch patching)."]
    #[serde(rename = "estimatedPatchingTime", default, skip_serializing_if = "Option::is_none")]
    pub estimated_patching_time: Option<EstimatedPatchingTime>,
    #[doc = "The list of customer email addresses that receive information from Oracle about the specified OCI Database service resource. Oracle uses these email addresses to send notifications about planned and unplanned software maintenance updates, information about system hardware, and other information needed by administrators. Up to 10 email addresses can be added to the customer contacts for a cloud Exadata infrastructure instance. "]
    #[serde(
        rename = "customerContacts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customer_contacts: Vec<CustomerContact>,
    #[doc = "Azure Resource Provisioning State enum"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceProvisioningState>,
    #[doc = "CloudExadataInfrastructureLifecycleState enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<CloudExadataInfrastructureLifecycleState>,
    #[doc = "The model name of the cloud Exadata infrastructure resource."]
    pub shape: String,
    #[doc = "HTTPS link to OCI resources exposed to Azure Customer via Azure Interface."]
    #[serde(rename = "ociUrl", default, skip_serializing_if = "Option::is_none")]
    pub oci_url: Option<String>,
    #[doc = "The total number of CPU cores allocated."]
    #[serde(rename = "cpuCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i32>,
    #[doc = "The total number of CPU cores available."]
    #[serde(rename = "maxCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub max_cpu_count: Option<i32>,
    #[doc = "The memory allocated in GBs."]
    #[serde(rename = "memorySizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_in_gbs: Option<i32>,
    #[doc = "The total memory available in GBs."]
    #[serde(rename = "maxMemoryInGbs", default, skip_serializing_if = "Option::is_none")]
    pub max_memory_in_gbs: Option<i32>,
    #[doc = "The local node storage to be allocated in GBs."]
    #[serde(rename = "dbNodeStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub db_node_storage_size_in_gbs: Option<i32>,
    #[doc = "The total local node storage available in GBs."]
    #[serde(rename = "maxDbNodeStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub max_db_node_storage_size_in_gbs: Option<i32>,
    #[doc = "The quantity of data in the database, in terabytes."]
    #[serde(rename = "dataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_tbs: Option<f64>,
    #[doc = "The total available DATA disk group size."]
    #[serde(rename = "maxDataStorageInTbs", default, skip_serializing_if = "Option::is_none")]
    pub max_data_storage_in_tbs: Option<f64>,
    #[doc = "The software version of the database servers (dom0) in the Exadata infrastructure."]
    #[serde(rename = "dbServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub db_server_version: Option<String>,
    #[doc = "The software version of the storage servers (cells) in the Exadata infrastructure."]
    #[serde(rename = "storageServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub storage_server_version: Option<String>,
    #[doc = "The requested number of additional storage servers activated for the Exadata infrastructure."]
    #[serde(rename = "activatedStorageCount", default, skip_serializing_if = "Option::is_none")]
    pub activated_storage_count: Option<i32>,
    #[doc = "The requested number of additional storage servers for the Exadata infrastructure."]
    #[serde(rename = "additionalStorageCount", default, skip_serializing_if = "Option::is_none")]
    pub additional_storage_count: Option<i32>,
    #[doc = "The name for the Exadata infrastructure."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "lastMaintenanceRunId", default, skip_serializing_if = "Option::is_none")]
    pub last_maintenance_run_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "nextMaintenanceRunId", default, skip_serializing_if = "Option::is_none")]
    pub next_maintenance_run_id: Option<Ocid>,
    #[doc = "Monthly Db Server version"]
    #[serde(rename = "monthlyDbServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub monthly_db_server_version: Option<String>,
    #[doc = "Monthly Storage Server version"]
    #[serde(rename = "monthlyStorageServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub monthly_storage_server_version: Option<String>,
}
impl CloudExadataInfrastructureProperties {
    pub fn new(shape: String, display_name: String) -> Self {
        Self {
            ocid: None,
            compute_count: None,
            storage_count: None,
            total_storage_size_in_gbs: None,
            available_storage_size_in_gbs: None,
            time_created: None,
            lifecycle_details: None,
            maintenance_window: None,
            estimated_patching_time: None,
            customer_contacts: Vec::new(),
            provisioning_state: None,
            lifecycle_state: None,
            shape,
            oci_url: None,
            cpu_count: None,
            max_cpu_count: None,
            memory_size_in_gbs: None,
            max_memory_in_gbs: None,
            db_node_storage_size_in_gbs: None,
            max_db_node_storage_size_in_gbs: None,
            data_storage_size_in_tbs: None,
            max_data_storage_in_tbs: None,
            db_server_version: None,
            storage_server_version: None,
            activated_storage_count: None,
            additional_storage_count: None,
            display_name,
            last_maintenance_run_id: None,
            next_maintenance_run_id: None,
            monthly_db_server_version: None,
            monthly_storage_server_version: None,
        }
    }
}
#[doc = "The type used for update operations of the CloudExadataInfrastructure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudExadataInfrastructureUpdate {
    #[doc = "CloudExadataInfrastructure zones"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the CloudExadataInfrastructure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudExadataInfrastructureUpdateProperties>,
}
impl CloudExadataInfrastructureUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the CloudExadataInfrastructure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudExadataInfrastructureUpdateProperties {
    #[doc = "The number of compute servers for the cloud Exadata infrastructure."]
    #[serde(rename = "computeCount", default, skip_serializing_if = "Option::is_none")]
    pub compute_count: Option<i32>,
    #[doc = "The number of storage servers for the cloud Exadata infrastructure."]
    #[serde(rename = "storageCount", default, skip_serializing_if = "Option::is_none")]
    pub storage_count: Option<i32>,
    #[doc = "MaintenanceWindow resource properties"]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "The list of customer email addresses that receive information from Oracle about the specified OCI Database service resource. Oracle uses these email addresses to send notifications about planned and unplanned software maintenance updates, information about system hardware, and other information needed by administrators. Up to 10 email addresses can be added to the customer contacts for a cloud Exadata infrastructure instance. "]
    #[serde(
        rename = "customerContacts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customer_contacts: Vec<CustomerContact>,
    #[doc = "The name for the Exadata infrastructure."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl CloudExadataInfrastructureUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CloudVmCluster resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudVmCluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "CloudVmCluster resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudVmClusterProperties>,
}
impl CloudVmCluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Cloud VM Cluster lifecycle state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudVmClusterLifecycleState")]
pub enum CloudVmClusterLifecycleState {
    Provisioning,
    Available,
    Updating,
    Terminating,
    Terminated,
    MaintenanceInProgress,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudVmClusterLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudVmClusterLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudVmClusterLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 0u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 1u32, "Available"),
            Self::Updating => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 2u32, "Updating"),
            Self::Terminating => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 3u32, "Terminating"),
            Self::Terminated => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 4u32, "Terminated"),
            Self::MaintenanceInProgress => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 5u32, "MaintenanceInProgress"),
            Self::Failed => serializer.serialize_unit_variant("CloudVmClusterLifecycleState", 6u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a CloudVmCluster list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudVmClusterListResult {
    #[doc = "The CloudVmCluster items on this page"]
    pub value: Vec<CloudVmCluster>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudVmClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CloudVmClusterListResult {
    pub fn new(value: Vec<CloudVmCluster>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "CloudVmCluster resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudVmClusterProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "The port number configured for the listener on the cloud VM cluster."]
    #[serde(rename = "listenerPort", default, skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i64>,
    #[doc = "The number of nodes in the cloud VM cluster. "]
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[doc = "The data disk group size to be allocated in GBs per VM."]
    #[serde(rename = "storageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_in_gbs: Option<i32>,
    #[doc = "The data disk group size to be allocated in TBs."]
    #[serde(rename = "dataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_tbs: Option<f64>,
    #[doc = "The local node storage to be allocated in GBs."]
    #[serde(rename = "dbNodeStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub db_node_storage_size_in_gbs: Option<i32>,
    #[doc = "The memory to be allocated in GBs."]
    #[serde(rename = "memorySizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_in_gbs: Option<i32>,
    #[doc = "The date and time that the cloud VM cluster was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "Additional information about the current lifecycle state."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "The time zone of the cloud VM cluster. For details, see [Exadata Infrastructure Time Zones](/Content/Database/References/timezones.htm)."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "zoneId", default, skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<Ocid>,
    #[doc = "The hostname for the cloud VM cluster."]
    pub hostname: String,
    #[doc = "The domain name for the cloud VM cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The number of CPU cores enabled on the cloud VM cluster."]
    #[serde(rename = "cpuCoreCount")]
    pub cpu_core_count: i32,
    #[doc = "The number of OCPU cores to enable on the cloud VM cluster. Only 1 decimal place is allowed for the fractional part."]
    #[serde(rename = "ocpuCount", default, skip_serializing_if = "Option::is_none")]
    pub ocpu_count: Option<f32>,
    #[doc = "The cluster name for cloud VM cluster. The cluster name must begin with an alphabetic character, and may contain hyphens (-). Underscores (_) are not permitted. The cluster name can be no longer than 11 characters and is not case sensitive. "]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "The percentage assigned to DATA storage (user data and database files). The remaining percentage is assigned to RECO storage (database redo logs, archive logs, and recovery manager backups). Accepted values are 35, 40, 60 and 80. The default is 80 percent assigned to DATA storage. See [Storage Configuration](/Content/Database/Concepts/exaoverview.htm#Exadata) in the Exadata documentation for details on the impact of the configuration settings on storage. "]
    #[serde(rename = "dataStoragePercentage", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_percentage: Option<i32>,
    #[doc = "If true, database backup on local Exadata storage is configured for the cloud VM cluster. If false, database backup on local Exadata storage is not available in the cloud VM cluster. "]
    #[serde(rename = "isLocalBackupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_local_backup_enabled: Option<bool>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "cloudExadataInfrastructureId")]
    pub cloud_exadata_infrastructure_id: CloudExadataInfrastructureId,
    #[doc = "If true, sparse disk group is configured for the cloud VM cluster. If false, sparse disk group is not created. "]
    #[serde(rename = "isSparseDiskgroupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_sparse_diskgroup_enabled: Option<bool>,
    #[doc = "Operating system version of the image."]
    #[serde(rename = "systemVersion", default, skip_serializing_if = "Option::is_none")]
    pub system_version: Option<String>,
    #[doc = "The public key portion of one or more key pairs used for SSH access to the cloud VM cluster."]
    #[serde(rename = "sshPublicKeys")]
    pub ssh_public_keys: Vec<String>,
    #[doc = "LicenseModel enum"]
    #[serde(rename = "licenseModel", default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<LicenseModel>,
    #[doc = "Disk redundancy enum"]
    #[serde(rename = "diskRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub disk_redundancy: Option<DiskRedundancy>,
    #[doc = "The Single Client Access Name (SCAN) IP addresses associated with the cloud VM cluster. SCAN IP addresses are typically used for load balancing and are not assigned to any interface. Oracle Clusterware directs the requests to the appropriate nodes in the cluster. **Note:** For a single-node DB system, this list is empty."]
    #[serde(
        rename = "scanIpIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scan_ip_ids: Vec<String>,
    #[doc = "The virtual IP (VIP) addresses associated with the cloud VM cluster. The Cluster Ready Services (CRS) creates and maintains one VIP address for each node in the Exadata Cloud Service instance to enable failover. If one node fails, the VIP is reassigned to another active node in the cluster. **Note:** For a single-node DB system, this list is empty."]
    #[serde(
        rename = "vipIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vip_ids: Vec<String>,
    #[doc = "The FQDN of the DNS record for the SCAN IP addresses that are associated with the cloud VM cluster. "]
    #[serde(rename = "scanDnsName", default, skip_serializing_if = "Option::is_none")]
    pub scan_dns_name: Option<String>,
    #[doc = "The TCP Single Client Access Name (SCAN) port. The default port is 1521."]
    #[serde(rename = "scanListenerPortTcp", default, skip_serializing_if = "Option::is_none")]
    pub scan_listener_port_tcp: Option<i32>,
    #[doc = "The TCPS Single Client Access Name (SCAN) port. The default port is 2484."]
    #[serde(rename = "scanListenerPortTcpSsl", default, skip_serializing_if = "Option::is_none")]
    pub scan_listener_port_tcp_ssl: Option<i32>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "scanDnsRecordId", default, skip_serializing_if = "Option::is_none")]
    pub scan_dns_record_id: Option<Ocid>,
    #[doc = "The model name of the Exadata hardware running the cloud VM cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[doc = "Azure Resource Provisioning State enum"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceProvisioningState>,
    #[doc = "Cloud VM Cluster lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<CloudVmClusterLifecycleState>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "vnetId")]
    pub vnet_id: VnetId,
    #[doc = "Oracle Grid Infrastructure (GI) software version"]
    #[serde(rename = "giVersion")]
    pub gi_version: String,
    #[doc = "HTTPS link to OCI resources exposed to Azure Customer via Azure Interface."]
    #[serde(rename = "ociUrl", default, skip_serializing_if = "Option::is_none")]
    pub oci_url: Option<String>,
    #[doc = "HTTPS link to OCI Network Security Group exposed to Azure Customer via the Azure Interface."]
    #[serde(rename = "nsgUrl", default, skip_serializing_if = "Option::is_none")]
    pub nsg_url: Option<String>,
    #[doc = "A type definition that refers the id to an Azure Resource Manager resource."]
    #[serde(rename = "subnetId")]
    pub subnet_id: SubnetId,
    #[doc = "Client OCI backup subnet CIDR, default is 192.168.252.0/22"]
    #[serde(rename = "backupSubnetCidr", default, skip_serializing_if = "Option::is_none")]
    pub backup_subnet_cidr: Option<String>,
    #[doc = "CIDR blocks for additional NSG ingress rules. The VNET CIDRs used to provision the VM Cluster will be added by default."]
    #[serde(
        rename = "nsgCidrs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nsg_cidrs: Vec<NsgCidr>,
    #[doc = "DataCollectionOptions resource properties"]
    #[serde(rename = "dataCollectionOptions", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_options: Option<DataCollectionOptions>,
    #[doc = "Display Name"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The list of compute servers to be added to the cloud VM cluster."]
    #[serde(
        rename = "computeNodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_nodes: Vec<Ocid>,
    #[doc = "ExadataIormConfig for cloud vm cluster"]
    #[serde(rename = "iormConfigCache", default, skip_serializing_if = "Option::is_none")]
    pub iorm_config_cache: Option<ExadataIormConfig>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "lastUpdateHistoryEntryId", default, skip_serializing_if = "Option::is_none")]
    pub last_update_history_entry_id: Option<Ocid>,
    #[doc = "The list of DB servers."]
    #[serde(
        rename = "dbServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub db_servers: Vec<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "compartmentId", default, skip_serializing_if = "Option::is_none")]
    pub compartment_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "subnetOcid", default, skip_serializing_if = "Option::is_none")]
    pub subnet_ocid: Option<Ocid>,
}
impl CloudVmClusterProperties {
    pub fn new(
        hostname: String,
        cpu_core_count: i32,
        cloud_exadata_infrastructure_id: CloudExadataInfrastructureId,
        ssh_public_keys: Vec<String>,
        vnet_id: VnetId,
        gi_version: String,
        subnet_id: SubnetId,
        display_name: String,
    ) -> Self {
        Self {
            ocid: None,
            listener_port: None,
            node_count: None,
            storage_size_in_gbs: None,
            data_storage_size_in_tbs: None,
            db_node_storage_size_in_gbs: None,
            memory_size_in_gbs: None,
            time_created: None,
            lifecycle_details: None,
            time_zone: None,
            zone_id: None,
            hostname,
            domain: None,
            cpu_core_count,
            ocpu_count: None,
            cluster_name: None,
            data_storage_percentage: None,
            is_local_backup_enabled: None,
            cloud_exadata_infrastructure_id,
            is_sparse_diskgroup_enabled: None,
            system_version: None,
            ssh_public_keys,
            license_model: None,
            disk_redundancy: None,
            scan_ip_ids: Vec::new(),
            vip_ids: Vec::new(),
            scan_dns_name: None,
            scan_listener_port_tcp: None,
            scan_listener_port_tcp_ssl: None,
            scan_dns_record_id: None,
            shape: None,
            provisioning_state: None,
            lifecycle_state: None,
            vnet_id,
            gi_version,
            oci_url: None,
            nsg_url: None,
            subnet_id,
            backup_subnet_cidr: None,
            nsg_cidrs: Vec::new(),
            data_collection_options: None,
            display_name,
            compute_nodes: Vec::new(),
            iorm_config_cache: None,
            last_update_history_entry_id: None,
            db_servers: Vec::new(),
            compartment_id: None,
            subnet_ocid: None,
        }
    }
}
#[doc = "The type used for update operations of the CloudVmCluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudVmClusterUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the CloudVmCluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudVmClusterUpdateProperties>,
}
impl CloudVmClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the CloudVmCluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudVmClusterUpdateProperties {
    #[doc = "The data disk group size to be allocated in GBs per VM."]
    #[serde(rename = "storageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_in_gbs: Option<i32>,
    #[doc = "The data disk group size to be allocated in TBs."]
    #[serde(rename = "dataStorageSizeInTbs", default, skip_serializing_if = "Option::is_none")]
    pub data_storage_size_in_tbs: Option<f64>,
    #[doc = "The local node storage to be allocated in GBs."]
    #[serde(rename = "dbNodeStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub db_node_storage_size_in_gbs: Option<i32>,
    #[doc = "The memory to be allocated in GBs."]
    #[serde(rename = "memorySizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_in_gbs: Option<i32>,
    #[doc = "The number of CPU cores enabled on the cloud VM cluster."]
    #[serde(rename = "cpuCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_core_count: Option<i32>,
    #[doc = "The number of OCPU cores to enable on the cloud VM cluster. Only 1 decimal place is allowed for the fractional part."]
    #[serde(rename = "ocpuCount", default, skip_serializing_if = "Option::is_none")]
    pub ocpu_count: Option<f32>,
    #[doc = "The public key portion of one or more key pairs used for SSH access to the cloud VM cluster."]
    #[serde(
        rename = "sshPublicKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ssh_public_keys: Vec<String>,
    #[doc = "LicenseModel enum"]
    #[serde(rename = "licenseModel", default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<LicenseModel>,
    #[doc = "DataCollectionOptions resource properties"]
    #[serde(rename = "dataCollectionOptions", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_options: Option<DataCollectionOptions>,
    #[doc = "Display Name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of compute servers to be added to the cloud VM cluster."]
    #[serde(
        rename = "computeNodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_nodes: Vec<Ocid>,
}
impl CloudVmClusterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compute model enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ComputeModel")]
pub enum ComputeModel {
    #[serde(rename = "ECPU")]
    Ecpu,
    #[serde(rename = "OCPU")]
    Ocpu,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ComputeModel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ComputeModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ComputeModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ecpu => serializer.serialize_unit_variant("ComputeModel", 0u32, "ECPU"),
            Self::Ocpu => serializer.serialize_unit_variant("ComputeModel", 1u32, "OCPU"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Connection strings to connect to an Oracle Autonomous Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionStringType {
    #[doc = "The connection string profile to allow clients to group, filter and select connection string values based on structured metadata."]
    #[serde(rename = "allConnectionStrings", default, skip_serializing_if = "Option::is_none")]
    pub all_connection_strings: Option<AllConnectionStringType>,
    #[doc = "The database service provides the least level of resources to each SQL statement, but supports the most number of concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedicated: Option<String>,
    #[doc = "The High database service provides the highest level of resources to each SQL statement resulting in the highest performance, but supports the fewest number of concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[doc = "The Low database service provides the least level of resources to each SQL statement, but supports the most number of concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[doc = "The Medium database service provides a lower level of resources to each SQL statement potentially resulting a lower level of performance, but supports more concurrent SQL statements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[doc = "A list of connection string profiles to allow clients to group, filter and select connection string values based on structured metadata."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub profiles: Vec<ProfileType>,
}
impl ConnectionStringType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The URLs for accessing Oracle Application Express (APEX) and SQL Developer Web with a browser from a Compute instance within your VCN or that has a direct connection to your VCN."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionUrlType {
    #[doc = "Oracle Application Express (APEX) URL."]
    #[serde(rename = "apexUrl", default, skip_serializing_if = "Option::is_none")]
    pub apex_url: Option<String>,
    #[doc = "The URL of the Database Transforms for the Autonomous Database."]
    #[serde(rename = "databaseTransformsUrl", default, skip_serializing_if = "Option::is_none")]
    pub database_transforms_url: Option<String>,
    #[doc = "The URL of the Graph Studio for the Autonomous Database."]
    #[serde(rename = "graphStudioUrl", default, skip_serializing_if = "Option::is_none")]
    pub graph_studio_url: Option<String>,
    #[doc = "The URL of the Oracle Machine Learning (OML) Notebook for the Autonomous Database."]
    #[serde(rename = "machineLearningNotebookUrl", default, skip_serializing_if = "Option::is_none")]
    pub machine_learning_notebook_url: Option<String>,
    #[doc = "The URL of the MongoDB API for the Autonomous Database."]
    #[serde(rename = "mongoDbUrl", default, skip_serializing_if = "Option::is_none")]
    pub mongo_db_url: Option<String>,
    #[doc = "The Oracle REST Data Services (ORDS) URL of the Web Access for the Autonomous Database."]
    #[serde(rename = "ordsUrl", default, skip_serializing_if = "Option::is_none")]
    pub ords_url: Option<String>,
    #[doc = "Oracle SQL Developer Web URL."]
    #[serde(rename = "sqlDevWebUrl", default, skip_serializing_if = "Option::is_none")]
    pub sql_dev_web_url: Option<String>,
}
impl ConnectionUrlType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Consumer group enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConsumerGroup")]
pub enum ConsumerGroup {
    High,
    Medium,
    Low,
    Tp,
    Tpurgent,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConsumerGroup {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConsumerGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConsumerGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("ConsumerGroup", 0u32, "High"),
            Self::Medium => serializer.serialize_unit_variant("ConsumerGroup", 1u32, "Medium"),
            Self::Low => serializer.serialize_unit_variant("ConsumerGroup", 2u32, "Low"),
            Self::Tp => serializer.serialize_unit_variant("ConsumerGroup", 3u32, "Tp"),
            Self::Tpurgent => serializer.serialize_unit_variant("ConsumerGroup", 4u32, "Tpurgent"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "CustomerContact resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerContact {
    #[doc = "The email address used by Oracle to send notifications regarding databases and infrastructure."]
    pub email: String,
}
impl CustomerContact {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}
#[doc = "Database type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataBaseType")]
pub enum DataBaseType {
    Regular,
    Clone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataBaseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataBaseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataBaseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Regular => serializer.serialize_unit_variant("DataBaseType", 0u32, "Regular"),
            Self::Clone => serializer.serialize_unit_variant("DataBaseType", 1u32, "Clone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DataCollectionOptions resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionOptions {
    #[doc = "Indicates whether diagnostic collection is enabled for the VM cluster/Cloud VM cluster/VMBM DBCS."]
    #[serde(rename = "isDiagnosticsEventsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_diagnostics_events_enabled: Option<bool>,
    #[doc = "Indicates whether health monitoring is enabled for the VM cluster / Cloud VM cluster / VMBM DBCS."]
    #[serde(rename = "isHealthMonitoringEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_health_monitoring_enabled: Option<bool>,
    #[doc = "Indicates whether incident logs and trace collection are enabled for the VM cluster / Cloud VM cluster / VMBM DBCS."]
    #[serde(rename = "isIncidentLogsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_incident_logs_enabled: Option<bool>,
}
impl DataCollectionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DataSafe status type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataSafeStatusType")]
pub enum DataSafeStatusType {
    Registering,
    Registered,
    Deregistering,
    NotRegistered,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataSafeStatusType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataSafeStatusType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataSafeStatusType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Registering => serializer.serialize_unit_variant("DataSafeStatusType", 0u32, "Registering"),
            Self::Registered => serializer.serialize_unit_variant("DataSafeStatusType", 1u32, "Registered"),
            Self::Deregistering => serializer.serialize_unit_variant("DataSafeStatusType", 2u32, "Deregistering"),
            Self::NotRegistered => serializer.serialize_unit_variant("DataSafeStatusType", 3u32, "NotRegistered"),
            Self::Failed => serializer.serialize_unit_variant("DataSafeStatusType", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Database edition type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseEditionType")]
pub enum DatabaseEditionType {
    StandardEdition,
    EnterpriseEdition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseEditionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseEditionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseEditionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardEdition => serializer.serialize_unit_variant("DatabaseEditionType", 0u32, "StandardEdition"),
            Self::EnterpriseEdition => serializer.serialize_unit_variant("DatabaseEditionType", 1u32, "EnterpriseEdition"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DayOfWeek resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayOfWeek {
    #[doc = "DayOfWeekName enum"]
    pub name: DayOfWeekName,
}
impl DayOfWeek {
    pub fn new(name: DayOfWeekName) -> Self {
        Self { name }
    }
}
#[doc = "DayOfWeekName enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DayOfWeekName")]
pub enum DayOfWeekName {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DayOfWeekName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DayOfWeekName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DayOfWeekName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Monday => serializer.serialize_unit_variant("DayOfWeekName", 0u32, "Monday"),
            Self::Tuesday => serializer.serialize_unit_variant("DayOfWeekName", 1u32, "Tuesday"),
            Self::Wednesday => serializer.serialize_unit_variant("DayOfWeekName", 2u32, "Wednesday"),
            Self::Thursday => serializer.serialize_unit_variant("DayOfWeekName", 3u32, "Thursday"),
            Self::Friday => serializer.serialize_unit_variant("DayOfWeekName", 4u32, "Friday"),
            Self::Saturday => serializer.serialize_unit_variant("DayOfWeekName", 5u32, "Saturday"),
            Self::Sunday => serializer.serialize_unit_variant("DayOfWeekName", 6u32, "Sunday"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DayOfWeek resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DayOfWeekUpdate {
    #[doc = "DayOfWeekName enum"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DayOfWeekName>,
}
impl DayOfWeekUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DbIormConfig for cloud vm cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbIormConfig {
    #[doc = "The database name. For the default DbPlan, the dbName is default."]
    #[serde(rename = "dbName", default, skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    #[doc = "The flash cache limit for this database. This value is internally configured based on the share value assigned to the database."]
    #[serde(rename = "flashCacheLimit", default, skip_serializing_if = "Option::is_none")]
    pub flash_cache_limit: Option<String>,
    #[doc = "The relative priority of this database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<i32>,
}
impl DbIormConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The DbNode resource belonging to vmCluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbNode {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of DbNodeResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DbNodeProperties>,
}
impl DbNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DbNode action object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbNodeAction {
    #[doc = "DbNode action enum"]
    pub action: DbNodeActionEnum,
}
impl DbNodeAction {
    pub fn new(action: DbNodeActionEnum) -> Self {
        Self { action }
    }
}
#[doc = "DbNode action enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DbNodeActionEnum")]
pub enum DbNodeActionEnum {
    Start,
    Stop,
    SoftReset,
    Reset,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DbNodeActionEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DbNodeActionEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DbNodeActionEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Start => serializer.serialize_unit_variant("DbNodeActionEnum", 0u32, "Start"),
            Self::Stop => serializer.serialize_unit_variant("DbNodeActionEnum", 1u32, "Stop"),
            Self::SoftReset => serializer.serialize_unit_variant("DbNodeActionEnum", 2u32, "SoftReset"),
            Self::Reset => serializer.serialize_unit_variant("DbNodeActionEnum", 3u32, "Reset"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a DbNode list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbNodeListResult {
    #[doc = "The DbNode items on this page"]
    pub value: Vec<DbNode>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DbNodeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DbNodeListResult {
    pub fn new(value: Vec<DbNode>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type of database node maintenance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DbNodeMaintenanceType")]
pub enum DbNodeMaintenanceType {
    VmdbRebootMigration,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DbNodeMaintenanceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DbNodeMaintenanceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DbNodeMaintenanceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::VmdbRebootMigration => serializer.serialize_unit_variant("DbNodeMaintenanceType", 0u32, "VmdbRebootMigration"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of DbNodeResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbNodeProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "Additional information about the planned maintenance."]
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "backupIpId", default, skip_serializing_if = "Option::is_none")]
    pub backup_ip_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "backupVnic2Id", default, skip_serializing_if = "Option::is_none")]
    pub backup_vnic2_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "backupVnicId", default, skip_serializing_if = "Option::is_none")]
    pub backup_vnic_id: Option<Ocid>,
    #[doc = "The number of CPU cores enabled on the Db node."]
    #[serde(rename = "cpuCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_core_count: Option<i32>,
    #[doc = "The allocated local node storage in GBs on the Db node."]
    #[serde(rename = "dbNodeStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub db_node_storage_size_in_gbs: Option<i32>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "dbServerId", default, skip_serializing_if = "Option::is_none")]
    pub db_server_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "dbSystemId", default, skip_serializing_if = "Option::is_none")]
    pub db_system_id: Option<Ocid>,
    #[doc = "The name of the Fault Domain the instance is contained in."]
    #[serde(rename = "faultDomain", default, skip_serializing_if = "Option::is_none")]
    pub fault_domain: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "hostIpId", default, skip_serializing_if = "Option::is_none")]
    pub host_ip_id: Option<Ocid>,
    #[doc = "The host name for the database node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "DnNode provisioning state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<DbNodeProvisioningState>,
    #[doc = "Lifecycle details of Db Node."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "The type of database node maintenance."]
    #[serde(rename = "maintenanceType", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_type: Option<DbNodeMaintenanceType>,
    #[doc = "The allocated memory in GBs on the Db node."]
    #[serde(rename = "memorySizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_in_gbs: Option<i32>,
    #[doc = "The size (in GB) of the block storage volume allocation for the DB system. This attribute applies only for virtual machine DB systems."]
    #[serde(rename = "softwareStorageSizeInGb", default, skip_serializing_if = "Option::is_none")]
    pub software_storage_size_in_gb: Option<i32>,
    #[doc = "The date and time that the database node was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "End date and time of maintenance window."]
    #[serde(rename = "timeMaintenanceWindowEnd", default, with = "azure_core::date::rfc3339::option")]
    pub time_maintenance_window_end: Option<::time::OffsetDateTime>,
    #[doc = "Start date and time of maintenance window."]
    #[serde(rename = "timeMaintenanceWindowStart", default, with = "azure_core::date::rfc3339::option")]
    pub time_maintenance_window_start: Option<::time::OffsetDateTime>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "vnic2Id", default, skip_serializing_if = "Option::is_none")]
    pub vnic2_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "vnicId", default, skip_serializing_if = "Option::is_none")]
    pub vnic_id: Option<Ocid>,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl DbNodeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DnNode provisioning state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DbNodeProvisioningState")]
pub enum DbNodeProvisioningState {
    Provisioning,
    Available,
    Updating,
    Stopping,
    Stopped,
    Starting,
    Terminating,
    Terminated,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DbNodeProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DbNodeProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DbNodeProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("DbNodeProvisioningState", 0u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("DbNodeProvisioningState", 1u32, "Available"),
            Self::Updating => serializer.serialize_unit_variant("DbNodeProvisioningState", 2u32, "Updating"),
            Self::Stopping => serializer.serialize_unit_variant("DbNodeProvisioningState", 3u32, "Stopping"),
            Self::Stopped => serializer.serialize_unit_variant("DbNodeProvisioningState", 4u32, "Stopped"),
            Self::Starting => serializer.serialize_unit_variant("DbNodeProvisioningState", 5u32, "Starting"),
            Self::Terminating => serializer.serialize_unit_variant("DbNodeProvisioningState", 6u32, "Terminating"),
            Self::Terminated => serializer.serialize_unit_variant("DbNodeProvisioningState", 7u32, "Terminated"),
            Self::Failed => serializer.serialize_unit_variant("DbNodeProvisioningState", 8u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DbServer resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "DbServer resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DbServerProperties>,
}
impl DbServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DbServer list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbServerListResult {
    #[doc = "The DbServer items on this page"]
    pub value: Vec<DbServer>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DbServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DbServerListResult {
    pub fn new(value: Vec<DbServer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "DbServer Patching Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbServerPatchingDetails {
    #[doc = "Estimated Patch Duration"]
    #[serde(rename = "estimatedPatchDuration", default, skip_serializing_if = "Option::is_none")]
    pub estimated_patch_duration: Option<i32>,
    #[doc = "DB Server patching status enum"]
    #[serde(rename = "patchingStatus", default, skip_serializing_if = "Option::is_none")]
    pub patching_status: Option<DbServerPatchingStatus>,
    #[doc = "Time Patching Ended"]
    #[serde(rename = "timePatchingEnded", default, with = "azure_core::date::rfc3339::option")]
    pub time_patching_ended: Option<::time::OffsetDateTime>,
    #[doc = "Time Patching Started"]
    #[serde(rename = "timePatchingStarted", default, with = "azure_core::date::rfc3339::option")]
    pub time_patching_started: Option<::time::OffsetDateTime>,
}
impl DbServerPatchingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DB Server patching status enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DbServerPatchingStatus")]
pub enum DbServerPatchingStatus {
    Scheduled,
    MaintenanceInProgress,
    Failed,
    Complete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DbServerPatchingStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DbServerPatchingStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DbServerPatchingStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Scheduled => serializer.serialize_unit_variant("DbServerPatchingStatus", 0u32, "Scheduled"),
            Self::MaintenanceInProgress => serializer.serialize_unit_variant("DbServerPatchingStatus", 1u32, "MaintenanceInProgress"),
            Self::Failed => serializer.serialize_unit_variant("DbServerPatchingStatus", 2u32, "Failed"),
            Self::Complete => serializer.serialize_unit_variant("DbServerPatchingStatus", 3u32, "Complete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DbServer resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbServerProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "The name for the Db Server."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "compartmentId", default, skip_serializing_if = "Option::is_none")]
    pub compartment_id: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "exadataInfrastructureId", default, skip_serializing_if = "Option::is_none")]
    pub exadata_infrastructure_id: Option<Ocid>,
    #[doc = "The number of CPU cores enabled on the Db server."]
    #[serde(rename = "cpuCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_core_count: Option<i32>,
    #[doc = "DbServer Patching Properties"]
    #[serde(rename = "dbServerPatchingDetails", default, skip_serializing_if = "Option::is_none")]
    pub db_server_patching_details: Option<DbServerPatchingDetails>,
    #[doc = "The total memory available in GBs."]
    #[serde(rename = "maxMemoryInGbs", default, skip_serializing_if = "Option::is_none")]
    pub max_memory_in_gbs: Option<i32>,
    #[doc = "The allocated local node storage in GBs on the Db server."]
    #[serde(rename = "dbNodeStorageSizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub db_node_storage_size_in_gbs: Option<i32>,
    #[doc = "The OCID of the VM Clusters associated with the Db server."]
    #[serde(
        rename = "vmClusterIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_cluster_ids: Vec<Ocid>,
    #[doc = "The OCID of the Db nodes associated with the Db server."]
    #[serde(
        rename = "dbNodeIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub db_node_ids: Vec<Ocid>,
    #[doc = "Lifecycle details of dbServer."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "DbServerProvisioningState enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<DbServerProvisioningState>,
    #[doc = "The total number of CPU cores available."]
    #[serde(rename = "maxCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub max_cpu_count: Option<i32>,
    #[doc = "The list of OCIDs of the Autonomous VM Clusters associated with the Db server."]
    #[serde(
        rename = "autonomousVmClusterIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub autonomous_vm_cluster_ids: Vec<Ocid>,
    #[doc = "The list of OCIDs of the Autonomous Virtual Machines associated with the Db server."]
    #[serde(
        rename = "autonomousVirtualMachineIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub autonomous_virtual_machine_ids: Vec<Ocid>,
    #[doc = "The total max dbNode storage in GBs."]
    #[serde(rename = "maxDbNodeStorageInGbs", default, skip_serializing_if = "Option::is_none")]
    pub max_db_node_storage_in_gbs: Option<i32>,
    #[doc = "The total memory size in GBs."]
    #[serde(rename = "memorySizeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_in_gbs: Option<i32>,
    #[doc = "The shape of the Db server. The shape determines the amount of CPU, storage, and memory resources available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[doc = "The date and time that the Db Server was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl DbServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DbServerProvisioningState enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DbServerProvisioningState")]
pub enum DbServerProvisioningState {
    Creating,
    Available,
    Unavailable,
    Deleting,
    Deleted,
    MaintenanceInProgress,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DbServerProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DbServerProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DbServerProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("DbServerProvisioningState", 0u32, "Creating"),
            Self::Available => serializer.serialize_unit_variant("DbServerProvisioningState", 1u32, "Available"),
            Self::Unavailable => serializer.serialize_unit_variant("DbServerProvisioningState", 2u32, "Unavailable"),
            Self::Deleting => serializer.serialize_unit_variant("DbServerProvisioningState", 3u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("DbServerProvisioningState", 4u32, "Deleted"),
            Self::MaintenanceInProgress => serializer.serialize_unit_variant("DbServerProvisioningState", 5u32, "MaintenanceInProgress"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DbSystemShape resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbSystemShape {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "DbSystemShape resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DbSystemShapeProperties>,
}
impl DbSystemShape {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DbSystemShape list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbSystemShapeListResult {
    #[doc = "The DbSystemShape items on this page"]
    pub value: Vec<DbSystemShape>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DbSystemShapeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DbSystemShapeListResult {
    pub fn new(value: Vec<DbSystemShape>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "DbSystemShape resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbSystemShapeProperties {
    #[doc = "The family of the shape used for the DB system."]
    #[serde(rename = "shapeFamily", default, skip_serializing_if = "Option::is_none")]
    pub shape_family: Option<String>,
    #[doc = "The maximum number of CPU cores that can be enabled on the DB system for this shape."]
    #[serde(rename = "availableCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub available_core_count: Option<i32>,
    #[doc = "The minimum number of CPU cores that can be enabled on the DB system for this shape."]
    #[serde(rename = "minimumCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub minimum_core_count: Option<i32>,
    #[doc = "The runtime minimum number of CPU cores that can be enabled on the DB system for this shape."]
    #[serde(rename = "runtimeMinimumCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub runtime_minimum_core_count: Option<i32>,
    #[doc = "The discrete number by which the CPU core count for this shape can be increased or decreased."]
    #[serde(rename = "coreCountIncrement", default, skip_serializing_if = "Option::is_none")]
    pub core_count_increment: Option<i32>,
    #[doc = "The minimum number of Exadata storage servers available for the Exadata infrastructure."]
    #[serde(rename = "minStorageCount", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_count: Option<i32>,
    #[doc = "The maximum number of Exadata storage servers available for the Exadata infrastructure."]
    #[serde(rename = "maxStorageCount", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_count: Option<i32>,
    #[doc = "The maximum data storage available per storage server for this shape. Only applicable to ExaCC Elastic shapes."]
    #[serde(rename = "availableDataStoragePerServerInTbs", default, skip_serializing_if = "Option::is_none")]
    pub available_data_storage_per_server_in_tbs: Option<f64>,
    #[doc = "The maximum memory available per database node for this shape. Only applicable to ExaCC Elastic shapes."]
    #[serde(rename = "availableMemoryPerNodeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_per_node_in_gbs: Option<i32>,
    #[doc = "The maximum Db Node storage available per database node for this shape. Only applicable to ExaCC Elastic shapes."]
    #[serde(rename = "availableDbNodePerNodeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub available_db_node_per_node_in_gbs: Option<i32>,
    #[doc = "The minimum number of CPU cores that can be enabled per node for this shape."]
    #[serde(rename = "minCoreCountPerNode", default, skip_serializing_if = "Option::is_none")]
    pub min_core_count_per_node: Option<i32>,
    #[doc = "The maximum memory that can be enabled for this shape."]
    #[serde(rename = "availableMemoryInGbs", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_gbs: Option<i32>,
    #[doc = "The minimum memory that need be allocated per node for this shape."]
    #[serde(rename = "minMemoryPerNodeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub min_memory_per_node_in_gbs: Option<i32>,
    #[doc = "The maximum Db Node storage that can be enabled for this shape."]
    #[serde(rename = "availableDbNodeStorageInGbs", default, skip_serializing_if = "Option::is_none")]
    pub available_db_node_storage_in_gbs: Option<i32>,
    #[doc = "The minimum Db Node storage that need be allocated per node for this shape."]
    #[serde(rename = "minDbNodeStoragePerNodeInGbs", default, skip_serializing_if = "Option::is_none")]
    pub min_db_node_storage_per_node_in_gbs: Option<i32>,
    #[doc = "The maximum DATA storage that can be enabled for this shape."]
    #[serde(rename = "availableDataStorageInTbs", default, skip_serializing_if = "Option::is_none")]
    pub available_data_storage_in_tbs: Option<i32>,
    #[doc = "The minimum data storage that need be allocated for this shape."]
    #[serde(rename = "minDataStorageInTbs", default, skip_serializing_if = "Option::is_none")]
    pub min_data_storage_in_tbs: Option<i32>,
    #[doc = "The minimum number of database nodes available for this shape."]
    #[serde(rename = "minimumNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub minimum_node_count: Option<i32>,
    #[doc = "The maximum number of database nodes available for this shape."]
    #[serde(rename = "maximumNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub maximum_node_count: Option<i32>,
    #[doc = "The maximum number of CPU cores per database node that can be enabled for this shape. Only applicable to the flex Exadata shape and ExaCC Elastic shapes."]
    #[serde(rename = "availableCoreCountPerNode", default, skip_serializing_if = "Option::is_none")]
    pub available_core_count_per_node: Option<i32>,
}
impl DbSystemShapeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disaster recovery type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DisasterRecoveryType")]
pub enum DisasterRecoveryType {
    Adg,
    BackupBased,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DisasterRecoveryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DisasterRecoveryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DisasterRecoveryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Adg => serializer.serialize_unit_variant("DisasterRecoveryType", 0u32, "Adg"),
            Self::BackupBased => serializer.serialize_unit_variant("DisasterRecoveryType", 1u32, "BackupBased"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Disk redundancy enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskRedundancy")]
pub enum DiskRedundancy {
    High,
    Normal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskRedundancy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskRedundancy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskRedundancy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("DiskRedundancy", 0u32, "High"),
            Self::Normal => serializer.serialize_unit_variant("DiskRedundancy", 1u32, "Normal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DnsPrivateView resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsPrivateView {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Views resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsPrivateViewProperties>,
}
impl DnsPrivateView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DnsPrivateView list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsPrivateViewListResult {
    #[doc = "The DnsPrivateView items on this page"]
    pub value: Vec<DnsPrivateView>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsPrivateViewListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsPrivateViewListResult {
    pub fn new(value: Vec<DnsPrivateView>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Views resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsPrivateViewProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "The display name of the view resource"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A Boolean flag indicating whether or not parts of the resource are unable to be explicitly managed."]
    #[serde(rename = "isProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[doc = "DNS Private Views lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<DnsPrivateViewsLifecycleState>,
    #[doc = "The canonical absolute URL of the resource."]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[doc = "views timeCreated"]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "views timeCreated"]
    #[serde(rename = "timeUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub time_updated: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl DnsPrivateViewProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DNS Private Views lifecycle state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DnsPrivateViewsLifecycleState")]
pub enum DnsPrivateViewsLifecycleState {
    Active,
    Deleted,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DnsPrivateViewsLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DnsPrivateViewsLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DnsPrivateViewsLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("DnsPrivateViewsLifecycleState", 0u32, "Active"),
            Self::Deleted => serializer.serialize_unit_variant("DnsPrivateViewsLifecycleState", 1u32, "Deleted"),
            Self::Deleting => serializer.serialize_unit_variant("DnsPrivateViewsLifecycleState", 2u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("DnsPrivateViewsLifecycleState", 3u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DnsPrivateZone resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsPrivateZone {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Zones resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsPrivateZoneProperties>,
}
impl DnsPrivateZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DnsPrivateZone list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsPrivateZoneListResult {
    #[doc = "The DnsPrivateZone items on this page"]
    pub value: Vec<DnsPrivateZone>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DnsPrivateZoneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DnsPrivateZoneListResult {
    pub fn new(value: Vec<DnsPrivateZone>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Zones resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsPrivateZoneProperties {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "A Boolean flag indicating whether or not parts of the resource are unable to be explicitly managed."]
    #[serde(rename = "isProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[doc = "DNS Private Zones lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<DnsPrivateZonesLifecycleState>,
    #[doc = "The canonical absolute URL of the resource."]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[doc = "The current serial of the zone. As seen in the zone's SOA record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<i32>,
    #[doc = "Version is the never-repeating, totally-orderable, version of the zone, from which the serial field of the zone's SOA record is derived."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "viewId", default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<Ocid>,
    #[doc = "Zone type enum"]
    #[serde(rename = "zoneType", default, skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<ZoneType>,
    #[doc = "Zones timeCreated"]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
}
impl DnsPrivateZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DNS Private Zones lifecycle state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DnsPrivateZonesLifecycleState")]
pub enum DnsPrivateZonesLifecycleState {
    Active,
    Creating,
    Deleted,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DnsPrivateZonesLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DnsPrivateZonesLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DnsPrivateZonesLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("DnsPrivateZonesLifecycleState", 0u32, "Active"),
            Self::Creating => serializer.serialize_unit_variant("DnsPrivateZonesLifecycleState", 1u32, "Creating"),
            Self::Deleted => serializer.serialize_unit_variant("DnsPrivateZonesLifecycleState", 2u32, "Deleted"),
            Self::Deleting => serializer.serialize_unit_variant("DnsPrivateZonesLifecycleState", 3u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("DnsPrivateZonesLifecycleState", 4u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The estimated total time required in minutes for all patching operations (database server, storage server, and network switch patching)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EstimatedPatchingTime {
    #[doc = "The estimated time required in minutes for database server patching."]
    #[serde(rename = "estimatedDbServerPatchingTime", default, skip_serializing_if = "Option::is_none")]
    pub estimated_db_server_patching_time: Option<i32>,
    #[doc = "The estimated time required in minutes for network switch patching."]
    #[serde(rename = "estimatedNetworkSwitchesPatchingTime", default, skip_serializing_if = "Option::is_none")]
    pub estimated_network_switches_patching_time: Option<i32>,
    #[doc = "The estimated time required in minutes for storage server patching."]
    #[serde(rename = "estimatedStorageServerPatchingTime", default, skip_serializing_if = "Option::is_none")]
    pub estimated_storage_server_patching_time: Option<i32>,
    #[doc = "The estimated total time required in minutes for all patching operations."]
    #[serde(rename = "totalEstimatedPatchingTime", default, skip_serializing_if = "Option::is_none")]
    pub total_estimated_patching_time: Option<i32>,
}
impl EstimatedPatchingTime {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExadataIormConfig for cloud vm cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExadataIormConfig {
    #[doc = "An array of IORM settings for all the database in the Exadata DB system."]
    #[serde(
        rename = "dbPlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub db_plans: Vec<DbIormConfig>,
    #[doc = "Additional information about the current lifecycleState."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "ORM lifecycle state enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<IormLifecycleState>,
    #[doc = "Objective enum"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective: Option<Objective>,
}
impl ExadataIormConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Autonomous Database Generate Wallet resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateAutonomousDatabaseWalletDetails {
    #[doc = "Generate type enum"]
    #[serde(rename = "generateType", default, skip_serializing_if = "Option::is_none")]
    pub generate_type: Option<GenerateType>,
    #[doc = "True when requesting regional connection strings in PDB connect info, applicable to cross-region DG only."]
    #[serde(rename = "isRegional", default, skip_serializing_if = "Option::is_none")]
    pub is_regional: Option<bool>,
    #[doc = "Password string."]
    pub password: Password,
}
impl GenerateAutonomousDatabaseWalletDetails {
    pub fn new(password: Password) -> Self {
        Self {
            generate_type: None,
            is_regional: None,
            password,
        }
    }
}
#[doc = "Generate type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GenerateType")]
pub enum GenerateType {
    Single,
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GenerateType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GenerateType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GenerateType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single => serializer.serialize_unit_variant("GenerateType", 0u32, "Single"),
            Self::All => serializer.serialize_unit_variant("GenerateType", 1u32, "All"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "GiVersion resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GiVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "GiVersion resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GiVersionProperties>,
}
impl GiVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a GiVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiVersionListResult {
    #[doc = "The GiVersion items on this page"]
    pub value: Vec<GiVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GiVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GiVersionListResult {
    pub fn new(value: Vec<GiVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "GiVersion resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GiVersionProperties {
    #[doc = "A valid Oracle Grid Infrastructure (GI) software version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl GiVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Host format type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HostFormatType")]
pub enum HostFormatType {
    Fqdn,
    Ip,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HostFormatType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HostFormatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HostFormatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Fqdn => serializer.serialize_unit_variant("HostFormatType", 0u32, "Fqdn"),
            Self::Ip => serializer.serialize_unit_variant("HostFormatType", 1u32, "Ip"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Intent enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Intent")]
pub enum Intent {
    Retain,
    Reset,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Intent {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Intent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Intent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Retain => serializer.serialize_unit_variant("Intent", 0u32, "Retain"),
            Self::Reset => serializer.serialize_unit_variant("Intent", 1u32, "Reset"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ORM lifecycle state enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IormLifecycleState")]
pub enum IormLifecycleState {
    BootStrapping,
    Enabled,
    Disabled,
    Updating,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IormLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IormLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IormLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BootStrapping => serializer.serialize_unit_variant("IormLifecycleState", 0u32, "BootStrapping"),
            Self::Enabled => serializer.serialize_unit_variant("IormLifecycleState", 1u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("IormLifecycleState", 2u32, "Disabled"),
            Self::Updating => serializer.serialize_unit_variant("IormLifecycleState", 3u32, "Updating"),
            Self::Failed => serializer.serialize_unit_variant("IormLifecycleState", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "LicenseModel enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseModel")]
pub enum LicenseModel {
    LicenseIncluded,
    BringYourOwnLicense,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseModel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseModel", 0u32, "LicenseIncluded"),
            Self::BringYourOwnLicense => serializer.serialize_unit_variant("LicenseModel", 1u32, "BringYourOwnLicense"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details for the long-term backup schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermBackUpScheduleDetails {
    #[doc = "Repeat cadence type enum"]
    #[serde(rename = "repeatCadence", default, skip_serializing_if = "Option::is_none")]
    pub repeat_cadence: Option<RepeatCadenceType>,
    #[doc = "The timestamp for the long-term backup schedule. For a MONTHLY cadence, months having fewer days than the provided date will have the backup taken on the last day of that month."]
    #[serde(rename = "timeOfBackup", default, with = "azure_core::date::rfc3339::option")]
    pub time_of_backup: Option<::time::OffsetDateTime>,
    #[doc = "Retention period, in days, for backups."]
    #[serde(rename = "retentionPeriodInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<RetentionPeriod>,
    #[doc = "Indicates if the long-term backup schedule should be deleted. The default value is `FALSE`."]
    #[serde(rename = "isDisabled", default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}
impl LongTermBackUpScheduleDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MaintenanceWindow resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindow {
    #[doc = "Preference enum"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<Preference>,
    #[doc = "Months during the year when maintenance should be performed."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub months: Vec<Month>,
    #[doc = "Weeks during the month when maintenance should be performed. Weeks start on the 1st, 8th, 15th, and 22nd days of the month, and have a duration of 7 days. Weeks start and end based on calendar dates, not days of the week. For example, to allow maintenance during the 2nd week of the month (from the 8th day to the 14th day of the month), use the value 2. Maintenance cannot be scheduled for the fifth week of months that contain more than 28 days. Note that this parameter works in conjunction with the  daysOfWeek and hoursOfDay parameters to allow you to specify specific days of the week and hours that maintenance will be performed. "]
    #[serde(
        rename = "weeksOfMonth",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub weeks_of_month: Vec<i32>,
    #[doc = "Days during the week when maintenance should be performed."]
    #[serde(
        rename = "daysOfWeek",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_of_week: Vec<DayOfWeek>,
    #[doc = "The window of hours during the day when maintenance should be performed. The window is a 4 hour slot. Valid values are - 0 - represents time slot 0:00 - 3:59 UTC - 4 - represents time slot 4:00 - 7:59 UTC - 8 - represents time slot 8:00 - 11:59 UTC - 12 - represents time slot 12:00 - 15:59 UTC - 16 - represents time slot 16:00 - 19:59 UTC - 20 - represents time slot 20:00 - 23:59 UTC"]
    #[serde(
        rename = "hoursOfDay",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hours_of_day: Vec<i32>,
    #[doc = "Lead time window allows user to set a lead time to prepare for a down time. The lead time is in weeks and valid value is between 1 to 4. "]
    #[serde(rename = "leadTimeInWeeks", default, skip_serializing_if = "Option::is_none")]
    pub lead_time_in_weeks: Option<i32>,
    #[doc = "Patching mode enum"]
    #[serde(rename = "patchingMode", default, skip_serializing_if = "Option::is_none")]
    pub patching_mode: Option<PatchingMode>,
    #[doc = "Determines the amount of time the system will wait before the start of each database server patching operation. Custom action timeout is in minutes and valid value is between 15 to 120 (inclusive)."]
    #[serde(rename = "customActionTimeoutInMins", default, skip_serializing_if = "Option::is_none")]
    pub custom_action_timeout_in_mins: Option<i32>,
    #[doc = "If true, enables the configuration of a custom action timeout (waiting period) between database server patching operations."]
    #[serde(rename = "isCustomActionTimeoutEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_custom_action_timeout_enabled: Option<bool>,
    #[doc = "is Monthly Patching Enabled"]
    #[serde(rename = "isMonthlyPatchingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_monthly_patching_enabled: Option<bool>,
}
impl MaintenanceWindow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Month resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Month {
    #[doc = "MonthName enum"]
    pub name: MonthName,
}
impl Month {
    pub fn new(name: MonthName) -> Self {
        Self { name }
    }
}
#[doc = "MonthName enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonthName")]
pub enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonthName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonthName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonthName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::January => serializer.serialize_unit_variant("MonthName", 0u32, "January"),
            Self::February => serializer.serialize_unit_variant("MonthName", 1u32, "February"),
            Self::March => serializer.serialize_unit_variant("MonthName", 2u32, "March"),
            Self::April => serializer.serialize_unit_variant("MonthName", 3u32, "April"),
            Self::May => serializer.serialize_unit_variant("MonthName", 4u32, "May"),
            Self::June => serializer.serialize_unit_variant("MonthName", 5u32, "June"),
            Self::July => serializer.serialize_unit_variant("MonthName", 6u32, "July"),
            Self::August => serializer.serialize_unit_variant("MonthName", 7u32, "August"),
            Self::September => serializer.serialize_unit_variant("MonthName", 8u32, "September"),
            Self::October => serializer.serialize_unit_variant("MonthName", 9u32, "October"),
            Self::November => serializer.serialize_unit_variant("MonthName", 10u32, "November"),
            Self::December => serializer.serialize_unit_variant("MonthName", 11u32, "December"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Header to distinguish between resource creation or update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsRpaasNewResourceHeader {}
impl MsRpaasNewResourceHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule for allowing inbound (INGRESS) IP packets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NsgCidr {
    #[doc = "Conceptually, this is the range of IP addresses that a packet coming into the instance can come from."]
    pub source: String,
    #[doc = "Port Range to specify particular destination ports for TCP rules."]
    #[serde(rename = "destinationPortRange", default, skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<PortRange>,
}
impl NsgCidr {
    pub fn new(source: String) -> Self {
        Self {
            source,
            destination_port_range: None,
        }
    }
}
#[doc = "Objective enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Objective")]
pub enum Objective {
    LowLatency,
    HighThroughput,
    Balanced,
    Auto,
    Basic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Objective {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Objective {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Objective {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LowLatency => serializer.serialize_unit_variant("Objective", 0u32, "LowLatency"),
            Self::HighThroughput => serializer.serialize_unit_variant("Objective", 1u32, "HighThroughput"),
            Self::Balanced => serializer.serialize_unit_variant("Objective", 2u32, "Balanced"),
            Self::Auto => serializer.serialize_unit_variant("Objective", 3u32, "Auto"),
            Self::Basic => serializer.serialize_unit_variant("Objective", 4u32, "Basic"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type Ocid = String;
#[doc = "Open mode type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OpenModeType")]
pub enum OpenModeType {
    ReadOnly,
    ReadWrite,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OpenModeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OpenModeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OpenModeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadOnly => serializer.serialize_unit_variant("OpenModeType", 0u32, "ReadOnly"),
            Self::ReadWrite => serializer.serialize_unit_variant("OpenModeType", 1u32, "ReadWrite"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations Insights status type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationsInsightsStatusType")]
pub enum OperationsInsightsStatusType {
    Enabling,
    Enabled,
    Disabling,
    NotEnabled,
    FailedEnabling,
    FailedDisabling,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationsInsightsStatusType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationsInsightsStatusType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationsInsightsStatusType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabling => serializer.serialize_unit_variant("OperationsInsightsStatusType", 0u32, "Enabling"),
            Self::Enabled => serializer.serialize_unit_variant("OperationsInsightsStatusType", 1u32, "Enabled"),
            Self::Disabling => serializer.serialize_unit_variant("OperationsInsightsStatusType", 2u32, "Disabling"),
            Self::NotEnabled => serializer.serialize_unit_variant("OperationsInsightsStatusType", 3u32, "NotEnabled"),
            Self::FailedEnabling => serializer.serialize_unit_variant("OperationsInsightsStatusType", 4u32, "FailedEnabling"),
            Self::FailedDisabling => serializer.serialize_unit_variant("OperationsInsightsStatusType", 5u32, "FailedDisabling"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "OracleSubscription resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleSubscription {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Oracle Subscription resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleSubscriptionProperties>,
    #[doc = "Plan for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
}
impl OracleSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a OracleSubscription list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OracleSubscriptionListResult {
    #[doc = "The OracleSubscription items on this page"]
    pub value: Vec<OracleSubscription>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OracleSubscriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OracleSubscriptionListResult {
    pub fn new(value: Vec<OracleSubscription>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Oracle Subscription resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleSubscriptionProperties {
    #[doc = "OracleSubscriptionProvisioningState enum"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<OracleSubscriptionProvisioningState>,
    #[doc = "SAAS subscription ID generated by Marketplace"]
    #[serde(rename = "saasSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub saas_subscription_id: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "cloudAccountId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_account_id: Option<Ocid>,
    #[doc = "CloudAccountProvisioningState enum"]
    #[serde(rename = "cloudAccountState", default, skip_serializing_if = "Option::is_none")]
    pub cloud_account_state: Option<CloudAccountProvisioningState>,
    #[doc = "Term Unit. P1Y, P3Y, etc, see Durations https://en.wikipedia.org/wiki/ISO_8601"]
    #[serde(rename = "termUnit", default, skip_serializing_if = "Option::is_none")]
    pub term_unit: Option<String>,
    #[doc = "Product code for the term unit"]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Intent enum"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<Intent>,
}
impl OracleSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OracleSubscriptionProvisioningState enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OracleSubscriptionProvisioningState")]
pub enum OracleSubscriptionProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OracleSubscriptionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OracleSubscriptionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OracleSubscriptionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("OracleSubscriptionProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OracleSubscriptionProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("OracleSubscriptionProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the OracleSubscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleSubscriptionUpdate {
    #[doc = "ResourcePlanTypeUpdate model definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanUpdate>,
    #[doc = "The updatable properties of the OracleSubscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OracleSubscriptionUpdateProperties>,
}
impl OracleSubscriptionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the OracleSubscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleSubscriptionUpdateProperties {
    #[doc = "Product code for the term unit"]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "Intent enum"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<Intent>,
}
impl OracleSubscriptionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Password = String;
#[doc = "Patching mode enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PatchingMode")]
pub enum PatchingMode {
    Rolling,
    NonRolling,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PatchingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PatchingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PatchingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Rolling => serializer.serialize_unit_variant("PatchingMode", 0u32, "Rolling"),
            Self::NonRolling => serializer.serialize_unit_variant("PatchingMode", 1u32, "NonRolling"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "PeerDb Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerDbDetails {
    #[doc = "The database OCID of the Disaster Recovery peer database, which is located in a different region from the current peer database."]
    #[serde(rename = "peerDbId", default, skip_serializing_if = "Option::is_none")]
    pub peer_db_id: Option<String>,
}
impl PeerDbDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permission level type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PermissionLevelType")]
pub enum PermissionLevelType {
    Restricted,
    Unrestricted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PermissionLevelType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PermissionLevelType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PermissionLevelType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Restricted => serializer.serialize_unit_variant("PermissionLevelType", 0u32, "Restricted"),
            Self::Unrestricted => serializer.serialize_unit_variant("PermissionLevelType", 1u32, "Unrestricted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Plan for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "A user defined name of the 3rd Party Artifact that is being procured."]
    pub name: String,
    #[doc = "The publisher of the 3rd Party Artifact that is being bought. E.g. NewRelic"]
    pub publisher: String,
    #[doc = "The 3rd Party artifact that is being procured. E.g. NewRelic. Product maps to the OfferID specified for the artifact at the time of Data Market onboarding. "]
    pub product: String,
    #[doc = "A publisher provided promotion code as provisioned in Data Market for the said product/artifact."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The version of the desired product/artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String) -> Self {
        Self {
            name,
            publisher,
            product,
            promotion_code: None,
            version: None,
        }
    }
}
#[doc = "ResourcePlanTypeUpdate model definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanUpdate {
    #[doc = "A user defined name of the 3rd Party Artifact that is being procured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher of the 3rd Party Artifact that is being bought. E.g. NewRelic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The 3rd Party artifact that is being procured. E.g. NewRelic. Product maps to the OfferID specified for the artifact at the time of Data Market onboarding. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "A publisher provided promotion code as provisioned in Data Market for the said product/artifact."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The version of the desired product/artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PlanUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Port Range to specify particular destination ports for TCP rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortRange {
    #[doc = "The minimum port number, which must not be greater than the maximum port number."]
    pub min: i32,
    #[doc = "The maximum port number, which must not be less than the minimum port number. To specify a single port number, set both the min and max to the same value."]
    pub max: i32,
}
impl PortRange {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}
#[doc = "Preference enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Preference")]
pub enum Preference {
    NoPreference,
    CustomPreference,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Preference {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Preference {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Preference {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NoPreference => serializer.serialize_unit_variant("Preference", 0u32, "NoPreference"),
            Self::CustomPreference => serializer.serialize_unit_variant("Preference", 1u32, "CustomPreference"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "PrivateIpAddress resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateIpAddressProperties {
    #[doc = "PrivateIpAddresses displayName"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "PrivateIpAddresses hostnameLabel"]
    #[serde(rename = "hostnameLabel")]
    pub hostname_label: String,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    pub ocid: Ocid,
    #[doc = "PrivateIpAddresses ipAddress"]
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "subnetId")]
    pub subnet_id: Ocid,
}
impl PrivateIpAddressProperties {
    pub fn new(display_name: String, hostname_label: String, ocid: Ocid, ip_address: String, subnet_id: Ocid) -> Self {
        Self {
            display_name,
            hostname_label,
            ocid,
            ip_address,
            subnet_id,
        }
    }
}
#[doc = "Private Ip Addresses filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateIpAddressesFilter {
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "subnetId")]
    pub subnet_id: Ocid,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "vnicId")]
    pub vnic_id: Ocid,
}
impl PrivateIpAddressesFilter {
    pub fn new(subnet_id: Ocid, vnic_id: Ocid) -> Self {
        Self { subnet_id, vnic_id }
    }
}
#[doc = "The connection string profile to allow clients to group, filter and select connection string values based on structured metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileType {
    #[doc = "Consumer group enum."]
    #[serde(rename = "consumerGroup", default, skip_serializing_if = "Option::is_none")]
    pub consumer_group: Option<ConsumerGroup>,
    #[doc = "A user-friendly name for the connection."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Host format type enum."]
    #[serde(rename = "hostFormat")]
    pub host_format: HostFormatType,
    #[doc = "True for a regional connection string, applicable to cross-region DG only."]
    #[serde(rename = "isRegional", default, skip_serializing_if = "Option::is_none")]
    pub is_regional: Option<bool>,
    #[doc = "Protocol type enum."]
    pub protocol: ProtocolType,
    #[doc = "Session mode type enum."]
    #[serde(rename = "sessionMode")]
    pub session_mode: SessionModeType,
    #[doc = "Syntax format type enum."]
    #[serde(rename = "syntaxFormat")]
    pub syntax_format: SyntaxFormatType,
    #[doc = "TLS authentication type enum."]
    #[serde(rename = "tlsAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub tls_authentication: Option<TlsAuthenticationType>,
    #[doc = "Connection string value."]
    pub value: String,
}
impl ProfileType {
    pub fn new(
        display_name: String,
        host_format: HostFormatType,
        protocol: ProtocolType,
        session_mode: SessionModeType,
        syntax_format: SyntaxFormatType,
        value: String,
    ) -> Self {
        Self {
            consumer_group: None,
            display_name,
            host_format,
            is_regional: None,
            protocol,
            session_mode,
            syntax_format,
            tls_authentication: None,
            value,
        }
    }
}
#[doc = "Protocol type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProtocolType")]
pub enum ProtocolType {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "TCPS")]
    Tcps,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProtocolType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProtocolType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProtocolType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Tcp => serializer.serialize_unit_variant("ProtocolType", 0u32, "TCP"),
            Self::Tcps => serializer.serialize_unit_variant("ProtocolType", 1u32, "TCPS"),
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
#[doc = "Refreshable model type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RefreshableModelType")]
pub enum RefreshableModelType {
    Automatic,
    Manual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RefreshableModelType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RefreshableModelType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RefreshableModelType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Automatic => serializer.serialize_unit_variant("RefreshableModelType", 0u32, "Automatic"),
            Self::Manual => serializer.serialize_unit_variant("RefreshableModelType", 1u32, "Manual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Refreshable status type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RefreshableStatusType")]
pub enum RefreshableStatusType {
    Refreshing,
    NotRefreshing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RefreshableStatusType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RefreshableStatusType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RefreshableStatusType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Refreshing => serializer.serialize_unit_variant("RefreshableStatusType", 0u32, "Refreshing"),
            Self::NotRefreshing => serializer.serialize_unit_variant("RefreshableStatusType", 1u32, "NotRefreshing"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Repeat cadence type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RepeatCadenceType")]
pub enum RepeatCadenceType {
    OneTime,
    Weekly,
    Monthly,
    Yearly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RepeatCadenceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RepeatCadenceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RepeatCadenceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OneTime => serializer.serialize_unit_variant("RepeatCadenceType", 0u32, "OneTime"),
            Self::Weekly => serializer.serialize_unit_variant("RepeatCadenceType", 1u32, "Weekly"),
            Self::Monthly => serializer.serialize_unit_variant("RepeatCadenceType", 2u32, "Monthly"),
            Self::Yearly => serializer.serialize_unit_variant("RepeatCadenceType", 3u32, "Yearly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details to restore an Oracle Autonomous Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreAutonomousDatabaseDetails {
    #[doc = "The time to restore the database to."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
}
impl RestoreAutonomousDatabaseDetails {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self { timestamp }
    }
}
pub type RetentionPeriod = i32;
#[doc = "Role type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RoleType")]
pub enum RoleType {
    Primary,
    Standby,
    DisabledStandby,
    BackupCopy,
    SnapshotStandby,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RoleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RoleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RoleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primary => serializer.serialize_unit_variant("RoleType", 0u32, "Primary"),
            Self::Standby => serializer.serialize_unit_variant("RoleType", 1u32, "Standby"),
            Self::DisabledStandby => serializer.serialize_unit_variant("RoleType", 2u32, "DisabledStandby"),
            Self::BackupCopy => serializer.serialize_unit_variant("RoleType", 3u32, "BackupCopy"),
            Self::SnapshotStandby => serializer.serialize_unit_variant("RoleType", 4u32, "SnapshotStandby"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SaaS Subscription Details model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasSubscriptionDetails {
    #[doc = "Purchased SaaS subscription ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "SaaS subscription name"]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "Creation Date and Time"]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<::time::OffsetDateTime>,
    #[doc = "Purchased offer ID"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Purchased offer's plan ID"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Indicates the status of the Subscription."]
    #[serde(rename = "saasSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub saas_subscription_status: Option<String>,
    #[doc = "Publisher ID"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Purchaser Email ID"]
    #[serde(rename = "purchaserEmailId", default, skip_serializing_if = "Option::is_none")]
    pub purchaser_email_id: Option<String>,
    #[doc = "Purchaser Tenant ID"]
    #[serde(rename = "purchaserTenantId", default, skip_serializing_if = "Option::is_none")]
    pub purchaser_tenant_id: Option<String>,
    #[doc = "Purchase Term Unit"]
    #[serde(rename = "termUnit", default, skip_serializing_if = "Option::is_none")]
    pub term_unit: Option<String>,
    #[doc = "AutoRenew flag"]
    #[serde(rename = "isAutoRenew", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_renew: Option<bool>,
    #[doc = "FreeTrial flag"]
    #[serde(rename = "isFreeTrial", default, skip_serializing_if = "Option::is_none")]
    pub is_free_trial: Option<bool>,
}
impl SaasSubscriptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of scheduled operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledOperationsType {
    #[doc = "DayOfWeek resource properties"]
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: DayOfWeek,
    #[doc = "auto start time. value must be of ISO-8601 format HH:mm"]
    #[serde(rename = "scheduledStartTime", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<String>,
    #[doc = "auto stop time. value must be of ISO-8601 format HH:mm"]
    #[serde(rename = "scheduledStopTime", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_stop_time: Option<String>,
}
impl ScheduledOperationsType {
    pub fn new(day_of_week: DayOfWeek) -> Self {
        Self {
            day_of_week,
            scheduled_start_time: None,
            scheduled_stop_time: None,
        }
    }
}
#[doc = "The list of scheduled operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledOperationsTypeUpdate {
    #[doc = "DayOfWeek resource properties"]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeekUpdate>,
    #[doc = "auto start time. value must be of ISO-8601 format HH:mm"]
    #[serde(rename = "scheduledStartTime", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<String>,
    #[doc = "auto stop time. value must be of ISO-8601 format HH:mm"]
    #[serde(rename = "scheduledStopTime", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_stop_time: Option<String>,
}
impl ScheduledOperationsTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Session mode type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SessionModeType")]
pub enum SessionModeType {
    Direct,
    Redirect,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SessionModeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SessionModeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SessionModeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Direct => serializer.serialize_unit_variant("SessionModeType", 0u32, "Direct"),
            Self::Redirect => serializer.serialize_unit_variant("SessionModeType", 1u32, "Redirect"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Source type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SourceType")]
pub enum SourceType {
    None,
    Database,
    BackupFromId,
    BackupFromTimestamp,
    CloneToRefreshable,
    CrossRegionDataguard,
    CrossRegionDisasterRecovery,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("SourceType", 0u32, "None"),
            Self::Database => serializer.serialize_unit_variant("SourceType", 1u32, "Database"),
            Self::BackupFromId => serializer.serialize_unit_variant("SourceType", 2u32, "BackupFromId"),
            Self::BackupFromTimestamp => serializer.serialize_unit_variant("SourceType", 3u32, "BackupFromTimestamp"),
            Self::CloneToRefreshable => serializer.serialize_unit_variant("SourceType", 4u32, "CloneToRefreshable"),
            Self::CrossRegionDataguard => serializer.serialize_unit_variant("SourceType", 5u32, "CrossRegionDataguard"),
            Self::CrossRegionDisasterRecovery => serializer.serialize_unit_variant("SourceType", 6u32, "CrossRegionDisasterRecovery"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type SubnetId = String;
#[doc = "Syntax format type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SyntaxFormatType")]
pub enum SyntaxFormatType {
    Long,
    Ezconnect,
    Ezconnectplus,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SyntaxFormatType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SyntaxFormatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SyntaxFormatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Long => serializer.serialize_unit_variant("SyntaxFormatType", 0u32, "Long"),
            Self::Ezconnect => serializer.serialize_unit_variant("SyntaxFormatType", 1u32, "Ezconnect"),
            Self::Ezconnectplus => serializer.serialize_unit_variant("SyntaxFormatType", 2u32, "Ezconnectplus"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SystemVersion resource Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "System Version Resource model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SystemVersionProperties>,
}
impl SystemVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SystemVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemVersionListResult {
    #[doc = "The SystemVersion items on this page"]
    pub value: Vec<SystemVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SystemVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SystemVersionListResult {
    pub fn new(value: Vec<SystemVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "System Version Resource model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemVersionProperties {
    #[doc = "A valid Oracle System Version"]
    #[serde(rename = "systemVersion", default, skip_serializing_if = "Option::is_none")]
    pub system_version: Option<String>,
}
impl SystemVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SystemVersions filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemVersionsFilter {
    #[doc = "Grid Infrastructure version"]
    #[serde(rename = "giVersion")]
    pub gi_version: String,
    #[doc = "Exadata shape"]
    pub shape: String,
    #[doc = "Check If we have to list only latest versions"]
    #[serde(rename = "isLatestVersion", default, skip_serializing_if = "Option::is_none")]
    pub is_latest_version: Option<bool>,
}
impl SystemVersionsFilter {
    pub fn new(gi_version: String, shape: String) -> Self {
        Self {
            gi_version,
            shape,
            is_latest_version: None,
        }
    }
}
#[doc = "TLS authentication type enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TlsAuthenticationType")]
pub enum TlsAuthenticationType {
    Server,
    Mutual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TlsAuthenticationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TlsAuthenticationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TlsAuthenticationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Server => serializer.serialize_unit_variant("TlsAuthenticationType", 0u32, "Server"),
            Self::Mutual => serializer.serialize_unit_variant("TlsAuthenticationType", 1u32, "Mutual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Update action enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpdateAction")]
pub enum UpdateAction {
    RollingApply,
    NonRollingApply,
    PreCheck,
    RollBack,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpdateAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpdateAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpdateAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RollingApply => serializer.serialize_unit_variant("UpdateAction", 0u32, "RollingApply"),
            Self::NonRollingApply => serializer.serialize_unit_variant("UpdateAction", 1u32, "NonRollingApply"),
            Self::PreCheck => serializer.serialize_unit_variant("UpdateAction", 2u32, "PreCheck"),
            Self::RollBack => serializer.serialize_unit_variant("UpdateAction", 3u32, "RollBack"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "validation error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    #[doc = "error code"]
    pub code: String,
    #[doc = "error message"]
    pub message: String,
}
impl ValidationError {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "validation result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    #[doc = "validation status"]
    pub status: ValidationStatus,
    #[doc = "validation error"]
    pub error: ValidationError,
}
impl ValidationResult {
    pub fn new(status: ValidationStatus, error: ValidationError) -> Self {
        Self { status, error }
    }
}
#[doc = "validation status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ValidationStatus")]
pub enum ValidationStatus {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ValidationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ValidationStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ValidationStatus", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Virtual IP resource belonging to a vm cluster resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkAddress {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "virtualNetworkAddress resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkAddressProperties>,
}
impl VirtualNetworkAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VirtualNetworkAddressLifecycleState enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VirtualNetworkAddressLifecycleState")]
pub enum VirtualNetworkAddressLifecycleState {
    Provisioning,
    Available,
    Terminating,
    Terminated,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VirtualNetworkAddressLifecycleState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VirtualNetworkAddressLifecycleState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VirtualNetworkAddressLifecycleState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("VirtualNetworkAddressLifecycleState", 0u32, "Provisioning"),
            Self::Available => serializer.serialize_unit_variant("VirtualNetworkAddressLifecycleState", 1u32, "Available"),
            Self::Terminating => serializer.serialize_unit_variant("VirtualNetworkAddressLifecycleState", 2u32, "Terminating"),
            Self::Terminated => serializer.serialize_unit_variant("VirtualNetworkAddressLifecycleState", 3u32, "Terminated"),
            Self::Failed => serializer.serialize_unit_variant("VirtualNetworkAddressLifecycleState", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a VirtualNetworkAddress list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkAddressListResult {
    #[doc = "The VirtualNetworkAddress items on this page"]
    pub value: Vec<VirtualNetworkAddress>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkAddressListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkAddressListResult {
    pub fn new(value: Vec<VirtualNetworkAddress>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "virtualNetworkAddress resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkAddressProperties {
    #[doc = "Virtual network Address address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(rename = "vmOcid", default, skip_serializing_if = "Option::is_none")]
    pub vm_ocid: Option<Ocid>,
    #[doc = "The [OCID](/Content/General/Concepts/identifiers.htm) of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocid: Option<Ocid>,
    #[doc = "Virtual network address fully qualified domain name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "Additional information about the current lifecycle state of the application virtual IP (VIP) address."]
    #[serde(rename = "lifecycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_details: Option<String>,
    #[doc = "Azure Resource Provisioning State enum"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceProvisioningState>,
    #[doc = "VirtualNetworkAddressLifecycleState enum"]
    #[serde(rename = "lifecycleState", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<VirtualNetworkAddressLifecycleState>,
    #[doc = "The date and time when the create operation for the application virtual IP (VIP) address completed."]
    #[serde(rename = "timeAssigned", default, with = "azure_core::date::rfc3339::option")]
    pub time_assigned: Option<::time::OffsetDateTime>,
}
impl VirtualNetworkAddressProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type VnetId = String;
#[doc = "WorkloadType enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkloadType")]
pub enum WorkloadType {
    #[serde(rename = "OLTP")]
    Oltp,
    #[serde(rename = "DW")]
    Dw,
    #[serde(rename = "AJD")]
    Ajd,
    #[serde(rename = "APEX")]
    Apex,
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
            Self::Oltp => serializer.serialize_unit_variant("WorkloadType", 0u32, "OLTP"),
            Self::Dw => serializer.serialize_unit_variant("WorkloadType", 1u32, "DW"),
            Self::Ajd => serializer.serialize_unit_variant("WorkloadType", 2u32, "AJD"),
            Self::Apex => serializer.serialize_unit_variant("WorkloadType", 3u32, "APEX"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Zone type enum"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ZoneType")]
pub enum ZoneType {
    Primary,
    Secondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ZoneType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ZoneType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ZoneType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primary => serializer.serialize_unit_variant("ZoneType", 0u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("ZoneType", 1u32, "Secondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type AclString = String;
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
