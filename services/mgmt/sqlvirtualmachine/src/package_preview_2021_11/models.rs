#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Additional SQL Server feature settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalFeaturesServerConfigurations {
    #[doc = "Enable or disable R services (SQL 2016 onwards)."]
    #[serde(rename = "isRServicesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_r_services_enabled: Option<bool>,
}
impl AdditionalFeaturesServerConfigurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Availability group configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgConfiguration {
    #[doc = "Replica configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replicas: Vec<AgReplica>,
}
impl AgConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Availability group replica configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgReplica {
    #[doc = "Sql VirtualMachine Instance Id."]
    #[serde(rename = "sqlVirtualMachineInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub sql_virtual_machine_instance_id: Option<String>,
    #[doc = "Replica Role in availability group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ag_replica::Role>,
    #[doc = "Replica commit mode in availability group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<ag_replica::Commit>,
    #[doc = "Replica failover mode in availability group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failover: Option<ag_replica::Failover>,
    #[doc = "Replica readable secondary mode in availability group."]
    #[serde(rename = "readableSecondary", default, skip_serializing_if = "Option::is_none")]
    pub readable_secondary: Option<ag_replica::ReadableSecondary>,
}
impl AgReplica {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ag_replica {
    use super::*;
    #[doc = "Replica Role in availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        #[serde(rename = "PRIMARY")]
        Primary,
        #[serde(rename = "SECONDARY")]
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("Role", 0u32, "PRIMARY"),
                Self::Secondary => serializer.serialize_unit_variant("Role", 1u32, "SECONDARY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Replica commit mode in availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Commit")]
    pub enum Commit {
        #[serde(rename = "SYNCHRONOUS_COMMIT")]
        SynchronousCommit,
        #[serde(rename = "ASYNCHRONOUS_COMMIT")]
        AsynchronousCommit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Commit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Commit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Commit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SynchronousCommit => serializer.serialize_unit_variant("Commit", 0u32, "SYNCHRONOUS_COMMIT"),
                Self::AsynchronousCommit => serializer.serialize_unit_variant("Commit", 1u32, "ASYNCHRONOUS_COMMIT"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Replica failover mode in availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Failover")]
    pub enum Failover {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Failover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Failover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Failover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("Failover", 0u32, "AUTOMATIC"),
                Self::Manual => serializer.serialize_unit_variant("Failover", 1u32, "MANUAL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Replica readable secondary mode in availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReadableSecondary")]
    pub enum ReadableSecondary {
        #[serde(rename = "NO")]
        No,
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "READ_ONLY")]
        ReadOnly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReadableSecondary {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReadableSecondary {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReadableSecondary {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::No => serializer.serialize_unit_variant("ReadableSecondary", 0u32, "NO"),
                Self::All => serializer.serialize_unit_variant("ReadableSecondary", 1u32, "ALL"),
                Self::ReadOnly => serializer.serialize_unit_variant("ReadableSecondary", 2u32, "READ_ONLY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Configure assessment for databases in your SQL virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentSettings {
    #[doc = "Enable or disable assessment feature on SQL virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[doc = "Run assessment immediately on SQL virtual machine."]
    #[serde(rename = "runImmediately", default, skip_serializing_if = "Option::is_none")]
    pub run_immediately: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}
impl AssessmentSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configure backups for databases in your SQL virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoBackupSettings {
    #[doc = "Enable or disable autobackup on SQL virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[doc = "Enable or disable encryption for backup on SQL virtual machine."]
    #[serde(rename = "enableEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enable_encryption: Option<bool>,
    #[doc = "Retention period of backup: 1-90 days."]
    #[serde(rename = "retentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[doc = "Storage account url where backup will be taken to."]
    #[serde(rename = "storageAccountUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_url: Option<String>,
    #[doc = "Storage container name where backup will be taken to."]
    #[serde(rename = "storageContainerName", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_name: Option<String>,
    #[doc = "Storage account key where backup will be taken to."]
    #[serde(rename = "storageAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_access_key: Option<String>,
    #[doc = "Password for encryption on backup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Include or exclude system databases from auto backup."]
    #[serde(rename = "backupSystemDbs", default, skip_serializing_if = "Option::is_none")]
    pub backup_system_dbs: Option<bool>,
    #[doc = "Backup schedule type."]
    #[serde(rename = "backupScheduleType", default, skip_serializing_if = "Option::is_none")]
    pub backup_schedule_type: Option<auto_backup_settings::BackupScheduleType>,
    #[doc = "Frequency of full backups. In both cases, full backups begin during the next scheduled time window."]
    #[serde(rename = "fullBackupFrequency", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_frequency: Option<auto_backup_settings::FullBackupFrequency>,
    #[doc = "Days of the week for the backups when FullBackupFrequency is set to Weekly."]
    #[serde(rename = "daysOfWeek", default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_week: Vec<String>,
    #[doc = "Start time of a given day during which full backups can take place. 0-23 hours."]
    #[serde(rename = "fullBackupStartTime", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_start_time: Option<i32>,
    #[doc = "Duration of the time window of a given day during which full backups can take place. 1-23 hours."]
    #[serde(rename = "fullBackupWindowHours", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_window_hours: Option<i32>,
    #[doc = "Frequency of log backups. 5-60 minutes."]
    #[serde(rename = "logBackupFrequency", default, skip_serializing_if = "Option::is_none")]
    pub log_backup_frequency: Option<i32>,
}
impl AutoBackupSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_backup_settings {
    use super::*;
    #[doc = "Backup schedule type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupScheduleType")]
    pub enum BackupScheduleType {
        Manual,
        Automated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupScheduleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupScheduleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupScheduleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("BackupScheduleType", 0u32, "Manual"),
                Self::Automated => serializer.serialize_unit_variant("BackupScheduleType", 1u32, "Automated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Frequency of full backups. In both cases, full backups begin during the next scheduled time window."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FullBackupFrequency")]
    pub enum FullBackupFrequency {
        Daily,
        Weekly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FullBackupFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FullBackupFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FullBackupFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("FullBackupFrequency", 0u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("FullBackupFrequency", 1u32, "Weekly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Set a patching window during which Windows and SQL patches will be applied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoPatchingSettings {
    #[doc = "Enable or disable autopatching on SQL virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[doc = "Day of week to apply the patch on."]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<auto_patching_settings::DayOfWeek>,
    #[doc = "Hour of the day when patching is initiated. Local VM time."]
    #[serde(rename = "maintenanceWindowStartingHour", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_starting_hour: Option<i32>,
    #[doc = "Duration of patching."]
    #[serde(rename = "maintenanceWindowDuration", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_duration: Option<i32>,
}
impl AutoPatchingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_patching_settings {
    use super::*;
    #[doc = "Day of week to apply the patch on."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Everyday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[doc = "A SQL Server availability group listener."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupListener {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an availability group listener."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailabilityGroupListenerProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AvailabilityGroupListener {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of availability group listeners."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupListenerListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailabilityGroupListener>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailabilityGroupListenerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailabilityGroupListenerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an availability group listener."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupListenerProperties {
    #[doc = "Provisioning state to track the async operation status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Name of the availability group."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[doc = "List of load balancer configurations for an availability group listener."]
    #[serde(rename = "loadBalancerConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_configurations: Vec<LoadBalancerConfiguration>,
    #[doc = "Create a default availability group if it does not exist."]
    #[serde(
        rename = "createDefaultAvailabilityGroupIfNotExist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_default_availability_group_if_not_exist: Option<bool>,
    #[doc = "Listener port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Availability group configuration."]
    #[serde(rename = "availabilityGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_configuration: Option<AgConfiguration>,
}
impl AvailabilityGroupListenerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configure your SQL virtual machine to be able to connect to the Azure Key Vault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCredentialSettings {
    #[doc = "Enable or disable key vault credential setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[doc = "Credential name."]
    #[serde(rename = "credentialName", default, skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
    #[doc = "Azure Key Vault url."]
    #[serde(rename = "azureKeyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub azure_key_vault_url: Option<String>,
    #[doc = "Service principal name to access key vault."]
    #[serde(rename = "servicePrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[doc = "Service principal name secret to access key vault."]
    #[serde(rename = "servicePrincipalSecret", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_secret: Option<String>,
}
impl KeyVaultCredentialSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A load balancer configuration for an availability group listener."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerConfiguration {
    #[doc = "A private IP address bound to the availability group listener."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<PrivateIpAddress>,
    #[doc = "Resource id of the public IP."]
    #[serde(rename = "publicIpAddressResourceId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_resource_id: Option<String>,
    #[doc = "Resource id of the load balancer."]
    #[serde(rename = "loadBalancerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_resource_id: Option<String>,
    #[doc = "Probe port."]
    #[serde(rename = "probePort", default, skip_serializing_if = "Option::is_none")]
    pub probe_port: Option<i32>,
    #[doc = "List of the SQL virtual machine instance resource id's that are enrolled into the availability group listener."]
    #[serde(rename = "sqlVirtualMachineInstances", default, skip_serializing_if = "Vec::is_empty")]
    pub sql_virtual_machine_instances: Vec<String>,
}
impl LoadBalancerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Additional descriptions for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The intended executor of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Display metadata associated with the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The localized friendly form of the resource provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly form of the resource type related to this action/operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list SQL operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Link to retrieve next page of results."]
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
#[doc = "A private IP address bound to the availability group listener."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateIpAddress {
    #[doc = "Private IP address bound to the availability group listener."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Subnet used to include private IP."]
    #[serde(rename = "subnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_resource_id: Option<String>,
}
impl PrivateIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM proxy resource."]
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
#[doc = "ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[doc = "The Azure Active Directory principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The identity type. Set this to 'SystemAssigned' in order to automatically create and assign an Azure Active Directory principal for the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[doc = "The Azure Active Directory tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_identity {
    use super::*;
    #[doc = "The identity type. Set this to 'SystemAssigned' in order to automatically create and assign an Azure Active Directory principal for the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Set the server/instance-level settings for SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlInstanceSettings {
    #[doc = "SQL Server Collation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "SQL Server MAXDOP."]
    #[serde(rename = "maxDop", default, skip_serializing_if = "Option::is_none")]
    pub max_dop: Option<i32>,
    #[doc = "SQL Server Optimize for Adhoc workloads."]
    #[serde(rename = "isOptimizeForAdHocWorkloadsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_optimize_for_ad_hoc_workloads_enabled: Option<bool>,
    #[doc = "SQL Server minimum memory."]
    #[serde(rename = "minServerMemoryMB", default, skip_serializing_if = "Option::is_none")]
    pub min_server_memory_mb: Option<i32>,
    #[doc = "SQL Server maximum memory."]
    #[serde(rename = "maxServerMemoryMB", default, skip_serializing_if = "Option::is_none")]
    pub max_server_memory_mb: Option<i32>,
}
impl SqlInstanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set disk storage settings for SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStorageSettings {
    #[doc = "Logical Unit Numbers for the disks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<i32>,
    #[doc = "SQL Server default file path"]
    #[serde(rename = "defaultFilePath", default, skip_serializing_if = "Option::is_none")]
    pub default_file_path: Option<String>,
}
impl SqlStorageSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlTempDbSettings {
    #[doc = "SQL Server default file size"]
    #[serde(rename = "dataFileSize", default, skip_serializing_if = "Option::is_none")]
    pub data_file_size: Option<i32>,
    #[doc = "SQL Server default file autoGrowth size"]
    #[serde(rename = "dataGrowth", default, skip_serializing_if = "Option::is_none")]
    pub data_growth: Option<i32>,
    #[doc = "SQL Server default file size"]
    #[serde(rename = "logFileSize", default, skip_serializing_if = "Option::is_none")]
    pub log_file_size: Option<i32>,
    #[doc = "SQL Server default file autoGrowth size"]
    #[serde(rename = "logGrowth", default, skip_serializing_if = "Option::is_none")]
    pub log_growth: Option<i32>,
    #[doc = "SQL Server default file count"]
    #[serde(rename = "dataFileCount", default, skip_serializing_if = "Option::is_none")]
    pub data_file_count: Option<i32>,
    #[doc = "Logical Unit Numbers for the disks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<i32>,
    #[doc = "SQL Server default file path"]
    #[serde(rename = "defaultFilePath", default, skip_serializing_if = "Option::is_none")]
    pub default_file_path: Option<String>,
}
impl SqlTempDbSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[doc = "Enable or disable assessment schedule on SQL virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[doc = "Number of weeks to schedule between 2 assessment runs. Takes value from 1-6"]
    #[serde(rename = "weeklyInterval", default, skip_serializing_if = "Option::is_none")]
    pub weekly_interval: Option<i32>,
    #[doc = "Occurrence of the DayOfWeek day within a month to schedule assessment. Takes values: 1,2,3,4 and -1. Use -1 for last DayOfWeek day of the month"]
    #[serde(rename = "monthlyOccurrence", default, skip_serializing_if = "Option::is_none")]
    pub monthly_occurrence: Option<i32>,
    #[doc = "Day of the week to run assessment."]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<schedule::DayOfWeek>,
    #[doc = "Time of the day in HH:mm format. Eg. 17:30"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod schedule {
    use super::*;
    #[doc = "Day of the week to run assessment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[doc = "Set the connectivity, storage and workload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfigurationsManagementSettings {
    #[doc = "Set the access level and network port settings for SQL Server."]
    #[serde(rename = "sqlConnectivityUpdateSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_connectivity_update_settings: Option<SqlConnectivityUpdateSettings>,
    #[doc = "Set workload type to optimize storage for SQL Server."]
    #[serde(rename = "sqlWorkloadTypeUpdateSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_workload_type_update_settings: Option<SqlWorkloadTypeUpdateSettings>,
    #[doc = "Set disk storage settings for SQL Server."]
    #[serde(rename = "sqlStorageUpdateSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_storage_update_settings: Option<SqlStorageUpdateSettings>,
    #[doc = "Additional SQL Server feature settings."]
    #[serde(
        rename = "additionalFeaturesServerConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_features_server_configurations: Option<AdditionalFeaturesServerConfigurations>,
    #[doc = "Set the server/instance-level settings for SQL Server."]
    #[serde(rename = "sqlInstanceSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_instance_settings: Option<SqlInstanceSettings>,
}
impl ServerConfigurationsManagementSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set the access level and network port settings for SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlConnectivityUpdateSettings {
    #[doc = "SQL Server connectivity option."]
    #[serde(rename = "connectivityType", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_type: Option<sql_connectivity_update_settings::ConnectivityType>,
    #[doc = "SQL Server port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "SQL Server sysadmin login to create."]
    #[serde(rename = "sqlAuthUpdateUserName", default, skip_serializing_if = "Option::is_none")]
    pub sql_auth_update_user_name: Option<String>,
    #[doc = "SQL Server sysadmin login password."]
    #[serde(rename = "sqlAuthUpdatePassword", default, skip_serializing_if = "Option::is_none")]
    pub sql_auth_update_password: Option<String>,
}
impl SqlConnectivityUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_connectivity_update_settings {
    use super::*;
    #[doc = "SQL Server connectivity option."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectivityType")]
    pub enum ConnectivityType {
        #[serde(rename = "LOCAL")]
        Local,
        #[serde(rename = "PRIVATE")]
        Private,
        #[serde(rename = "PUBLIC")]
        Public,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectivityType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectivityType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectivityType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Local => serializer.serialize_unit_variant("ConnectivityType", 0u32, "LOCAL"),
                Self::Private => serializer.serialize_unit_variant("ConnectivityType", 1u32, "PRIVATE"),
                Self::Public => serializer.serialize_unit_variant("ConnectivityType", 2u32, "PUBLIC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Set disk storage settings for SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStorageUpdateSettings {
    #[doc = "Virtual machine disk count."]
    #[serde(rename = "diskCount", default, skip_serializing_if = "Option::is_none")]
    pub disk_count: Option<i32>,
    #[doc = "Device id of the first disk to be updated."]
    #[serde(rename = "startingDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub starting_device_id: Option<i32>,
    #[doc = "Disk configuration to apply to SQL Server."]
    #[serde(rename = "diskConfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub disk_configuration_type: Option<sql_storage_update_settings::DiskConfigurationType>,
}
impl SqlStorageUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_storage_update_settings {
    use super::*;
    #[doc = "Disk configuration to apply to SQL Server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskConfigurationType")]
    pub enum DiskConfigurationType {
        #[serde(rename = "NEW")]
        New,
        #[serde(rename = "EXTEND")]
        Extend,
        #[serde(rename = "ADD")]
        Add,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskConfigurationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskConfigurationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskConfigurationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::New => serializer.serialize_unit_variant("DiskConfigurationType", 0u32, "NEW"),
                Self::Extend => serializer.serialize_unit_variant("DiskConfigurationType", 1u32, "EXTEND"),
                Self::Add => serializer.serialize_unit_variant("DiskConfigurationType", 2u32, "ADD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A SQL virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlVirtualMachine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The SQL virtual machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVirtualMachineProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SqlVirtualMachine {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "A SQL virtual machine group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlVirtualMachineGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of a SQL virtual machine group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVirtualMachineGroupProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SqlVirtualMachineGroup {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "A list of SQL virtual machine groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineGroupListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlVirtualMachineGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlVirtualMachineGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlVirtualMachineGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a SQL virtual machine group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineGroupProperties {
    #[doc = "Provisioning state to track the async operation status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "SQL image offer. Examples may include SQL2016-WS2016, SQL2017-WS2016."]
    #[serde(rename = "sqlImageOffer", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_offer: Option<String>,
    #[doc = "SQL image sku."]
    #[serde(rename = "sqlImageSku", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_sku: Option<sql_virtual_machine_group_properties::SqlImageSku>,
    #[doc = "Scale type."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sql_virtual_machine_group_properties::ScaleType>,
    #[doc = "Type of cluster manager: Windows Server Failover Cluster (WSFC), implied by the scale type of the group and the OS type."]
    #[serde(rename = "clusterManagerType", default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager_type: Option<sql_virtual_machine_group_properties::ClusterManagerType>,
    #[doc = "Cluster type."]
    #[serde(rename = "clusterConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cluster_configuration: Option<sql_virtual_machine_group_properties::ClusterConfiguration>,
    #[doc = "Active Directory account details to operate Windows Server Failover Cluster."]
    #[serde(rename = "wsfcDomainProfile", default, skip_serializing_if = "Option::is_none")]
    pub wsfc_domain_profile: Option<WsfcDomainProfile>,
}
impl SqlVirtualMachineGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_virtual_machine_group_properties {
    use super::*;
    #[doc = "SQL image sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlImageSku")]
    pub enum SqlImageSku {
        Developer,
        Enterprise,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlImageSku {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlImageSku {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlImageSku {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Developer => serializer.serialize_unit_variant("SqlImageSku", 0u32, "Developer"),
                Self::Enterprise => serializer.serialize_unit_variant("SqlImageSku", 1u32, "Enterprise"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Scale type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        #[serde(rename = "HA")]
        Ha,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ha => serializer.serialize_unit_variant("ScaleType", 0u32, "HA"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of cluster manager: Windows Server Failover Cluster (WSFC), implied by the scale type of the group and the OS type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusterManagerType")]
    pub enum ClusterManagerType {
        #[serde(rename = "WSFC")]
        Wsfc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusterManagerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusterManagerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusterManagerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Wsfc => serializer.serialize_unit_variant("ClusterManagerType", 0u32, "WSFC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Cluster type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusterConfiguration")]
    pub enum ClusterConfiguration {
        Domainful,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusterConfiguration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusterConfiguration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusterConfiguration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Domainful => serializer.serialize_unit_variant("ClusterConfiguration", 0u32, "Domainful"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to a SQL virtual machine group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineGroupUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlVirtualMachineGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SQL virtual machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlVirtualMachine>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlVirtualMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SqlVirtualMachineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SQL virtual machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineProperties {
    #[doc = "ARM Resource id of underlying virtual machine created from SQL marketplace image."]
    #[serde(rename = "virtualMachineResourceId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_resource_id: Option<String>,
    #[doc = "Provisioning state to track the async operation status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "SQL image offer. Examples include SQL2016-WS2016, SQL2017-WS2016."]
    #[serde(rename = "sqlImageOffer", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_offer: Option<String>,
    #[doc = "SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<sql_virtual_machine_properties::SqlServerLicenseType>,
    #[doc = "SQL Server Management type."]
    #[serde(rename = "sqlManagement", default, skip_serializing_if = "Option::is_none")]
    pub sql_management: Option<sql_virtual_machine_properties::SqlManagement>,
    #[doc = "SQL Server edition type."]
    #[serde(rename = "sqlImageSku", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_sku: Option<sql_virtual_machine_properties::SqlImageSku>,
    #[doc = "ARM resource id of the SQL virtual machine group this SQL virtual machine is or will be part of."]
    #[serde(rename = "sqlVirtualMachineGroupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub sql_virtual_machine_group_resource_id: Option<String>,
    #[doc = "Domain credentials for setting up Windows Server Failover Cluster for SQL availability group."]
    #[serde(rename = "wsfcDomainCredentials", default, skip_serializing_if = "Option::is_none")]
    pub wsfc_domain_credentials: Option<WsfcDomainCredentials>,
    #[doc = "Set a patching window during which Windows and SQL patches will be applied."]
    #[serde(rename = "autoPatchingSettings", default, skip_serializing_if = "Option::is_none")]
    pub auto_patching_settings: Option<AutoPatchingSettings>,
    #[doc = "Configure backups for databases in your SQL virtual machine."]
    #[serde(rename = "autoBackupSettings", default, skip_serializing_if = "Option::is_none")]
    pub auto_backup_settings: Option<AutoBackupSettings>,
    #[doc = "Configure your SQL virtual machine to be able to connect to the Azure Key Vault service."]
    #[serde(rename = "keyVaultCredentialSettings", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_credential_settings: Option<KeyVaultCredentialSettings>,
    #[doc = "Set the connectivity, storage and workload settings."]
    #[serde(
        rename = "serverConfigurationsManagementSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_configurations_management_settings: Option<ServerConfigurationsManagementSettings>,
    #[doc = "Storage Configurations for SQL Data, Log and TempDb."]
    #[serde(rename = "storageConfigurationSettings", default, skip_serializing_if = "Option::is_none")]
    pub storage_configuration_settings: Option<StorageConfigurationSettings>,
    #[doc = "Configure assessment for databases in your SQL virtual machine."]
    #[serde(rename = "assessmentSettings", default, skip_serializing_if = "Option::is_none")]
    pub assessment_settings: Option<AssessmentSettings>,
}
impl SqlVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_virtual_machine_properties {
    use super::*;
    #[doc = "SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(rename = "DR")]
        Dr,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "AHUB"),
                Self::Dr => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "DR"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server Management type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlManagement")]
    pub enum SqlManagement {
        Full,
        LightWeight,
        NoAgent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlManagement {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlManagement {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlManagement {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Full => serializer.serialize_unit_variant("SqlManagement", 0u32, "Full"),
                Self::LightWeight => serializer.serialize_unit_variant("SqlManagement", 1u32, "LightWeight"),
                Self::NoAgent => serializer.serialize_unit_variant("SqlManagement", 2u32, "NoAgent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server edition type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlImageSku")]
    pub enum SqlImageSku {
        Developer,
        Express,
        Standard,
        Enterprise,
        Web,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlImageSku {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlImageSku {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlImageSku {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Developer => serializer.serialize_unit_variant("SqlImageSku", 0u32, "Developer"),
                Self::Express => serializer.serialize_unit_variant("SqlImageSku", 1u32, "Express"),
                Self::Standard => serializer.serialize_unit_variant("SqlImageSku", 2u32, "Standard"),
                Self::Enterprise => serializer.serialize_unit_variant("SqlImageSku", 3u32, "Enterprise"),
                Self::Web => serializer.serialize_unit_variant("SqlImageSku", 4u32, "Web"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to a SQL virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlVirtualMachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set workload type to optimize storage for SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlWorkloadTypeUpdateSettings {
    #[doc = "SQL Server workload type."]
    #[serde(rename = "sqlWorkloadType", default, skip_serializing_if = "Option::is_none")]
    pub sql_workload_type: Option<sql_workload_type_update_settings::SqlWorkloadType>,
}
impl SqlWorkloadTypeUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_workload_type_update_settings {
    use super::*;
    #[doc = "SQL Server workload type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlWorkloadType")]
    pub enum SqlWorkloadType {
        #[serde(rename = "GENERAL")]
        General,
        #[serde(rename = "OLTP")]
        Oltp,
        #[serde(rename = "DW")]
        Dw,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlWorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlWorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlWorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::General => serializer.serialize_unit_variant("SqlWorkloadType", 0u32, "GENERAL"),
                Self::Oltp => serializer.serialize_unit_variant("SqlWorkloadType", 1u32, "OLTP"),
                Self::Dw => serializer.serialize_unit_variant("SqlWorkloadType", 2u32, "DW"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage Configurations for SQL Data, Log and TempDb."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageConfigurationSettings {
    #[doc = "Set disk storage settings for SQL Server."]
    #[serde(rename = "sqlDataSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_data_settings: Option<SqlStorageSettings>,
    #[doc = "Set disk storage settings for SQL Server."]
    #[serde(rename = "sqlLogSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_log_settings: Option<SqlStorageSettings>,
    #[serde(rename = "sqlTempDbSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_temp_db_settings: Option<SqlTempDbSettings>,
    #[doc = "SQL Server SystemDb Storage on DataPool if true."]
    #[serde(rename = "sqlSystemDbOnDataDisk", default, skip_serializing_if = "Option::is_none")]
    pub sql_system_db_on_data_disk: Option<bool>,
    #[doc = "Disk configuration to apply to SQL Server."]
    #[serde(rename = "diskConfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub disk_configuration_type: Option<storage_configuration_settings::DiskConfigurationType>,
    #[doc = "Storage workload type."]
    #[serde(rename = "storageWorkloadType", default, skip_serializing_if = "Option::is_none")]
    pub storage_workload_type: Option<storage_configuration_settings::StorageWorkloadType>,
}
impl StorageConfigurationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_configuration_settings {
    use super::*;
    #[doc = "Disk configuration to apply to SQL Server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskConfigurationType")]
    pub enum DiskConfigurationType {
        #[serde(rename = "NEW")]
        New,
        #[serde(rename = "EXTEND")]
        Extend,
        #[serde(rename = "ADD")]
        Add,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskConfigurationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskConfigurationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskConfigurationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::New => serializer.serialize_unit_variant("DiskConfigurationType", 0u32, "NEW"),
                Self::Extend => serializer.serialize_unit_variant("DiskConfigurationType", 1u32, "EXTEND"),
                Self::Add => serializer.serialize_unit_variant("DiskConfigurationType", 2u32, "ADD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Storage workload type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageWorkloadType")]
    pub enum StorageWorkloadType {
        #[serde(rename = "GENERAL")]
        General,
        #[serde(rename = "OLTP")]
        Oltp,
        #[serde(rename = "DW")]
        Dw,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageWorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageWorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageWorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::General => serializer.serialize_unit_variant("StorageWorkloadType", 0u32, "GENERAL"),
                Self::Oltp => serializer.serialize_unit_variant("StorageWorkloadType", 1u32, "OLTP"),
                Self::Dw => serializer.serialize_unit_variant("StorageWorkloadType", 2u32, "DW"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Domain credentials for setting up Windows Server Failover Cluster for SQL availability group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsfcDomainCredentials {
    #[doc = "Cluster bootstrap account password."]
    #[serde(rename = "clusterBootstrapAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub cluster_bootstrap_account_password: Option<String>,
    #[doc = "Cluster operator account password."]
    #[serde(rename = "clusterOperatorAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub cluster_operator_account_password: Option<String>,
    #[doc = "SQL service account password."]
    #[serde(rename = "sqlServiceAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub sql_service_account_password: Option<String>,
}
impl WsfcDomainCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory account details to operate Windows Server Failover Cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsfcDomainProfile {
    #[doc = "Fully qualified name of the domain."]
    #[serde(rename = "domainFqdn", default, skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[doc = "Organizational Unit path in which the nodes and cluster will be present."]
    #[serde(rename = "ouPath", default, skip_serializing_if = "Option::is_none")]
    pub ou_path: Option<String>,
    #[doc = "Account name used for creating cluster (at minimum needs permissions to 'Create Computer Objects' in domain)."]
    #[serde(rename = "clusterBootstrapAccount", default, skip_serializing_if = "Option::is_none")]
    pub cluster_bootstrap_account: Option<String>,
    #[doc = "Account name used for operating cluster i.e. will be part of administrators group on all the participating virtual machines in the cluster."]
    #[serde(rename = "clusterOperatorAccount", default, skip_serializing_if = "Option::is_none")]
    pub cluster_operator_account: Option<String>,
    #[doc = "Account name under which SQL service will run on all participating SQL virtual machines in the cluster."]
    #[serde(rename = "sqlServiceAccount", default, skip_serializing_if = "Option::is_none")]
    pub sql_service_account: Option<String>,
    #[doc = "Optional path for fileshare witness."]
    #[serde(rename = "fileShareWitnessPath", default, skip_serializing_if = "Option::is_none")]
    pub file_share_witness_path: Option<String>,
    #[doc = "Fully qualified ARM resource id of the witness storage account."]
    #[serde(rename = "storageAccountUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_url: Option<String>,
    #[doc = "Primary key of the witness storage account."]
    #[serde(rename = "storageAccountPrimaryKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_primary_key: Option<String>,
}
impl WsfcDomainProfile {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
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
