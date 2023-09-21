#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[doc = "Error information in OData format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ODataError>,
}
impl azure_core::Continuable for ApiError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ApiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An enumeration of possible authentication types when connecting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AuthenticationType")]
pub enum AuthenticationType {
    None,
    WindowsAuthentication,
    SqlAuthentication,
    ActiveDirectoryIntegrated,
    ActiveDirectoryPassword,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AuthenticationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AuthenticationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AuthenticationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("AuthenticationType", 0u32, "None"),
            Self::WindowsAuthentication => serializer.serialize_unit_variant("AuthenticationType", 1u32, "WindowsAuthentication"),
            Self::SqlAuthentication => serializer.serialize_unit_variant("AuthenticationType", 2u32, "SqlAuthentication"),
            Self::ActiveDirectoryIntegrated => serializer.serialize_unit_variant("AuthenticationType", 3u32, "ActiveDirectoryIntegrated"),
            Self::ActiveDirectoryPassword => serializer.serialize_unit_variant("AuthenticationType", 4u32, "ActiveDirectoryPassword"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the available service SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableServiceSku {
    #[doc = "The resource type, including the provider namespace"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "SKU name, tier, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<available_service_sku::Sku>,
    #[doc = "A description of the scaling capacities of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<available_service_sku::Capacity>,
}
impl AvailableServiceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_service_sku {
    use super::*;
    #[doc = "SKU name, tier, etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Sku {
        #[doc = "The name of the SKU"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "SKU family"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub family: Option<String>,
        #[doc = "SKU size"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
        #[doc = "The tier of the SKU, such as \"Free\", \"Basic\", \"Standard\", or \"Premium\""]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tier: Option<String>,
    }
    impl Sku {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "A description of the scaling capacities of the SKU"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Capacity {
        #[doc = "The minimum capacity, usually 0 or 1."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub minimum: Option<i64>,
        #[doc = "The maximum capacity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maximum: Option<i64>,
        #[doc = "The default capacity"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default: Option<i64>,
        #[doc = "The scalability approach"]
        #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
        pub scale_type: Option<capacity::ScaleType>,
    }
    impl Capacity {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod capacity {
        use super::*;
        #[doc = "The scalability approach"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ScaleType")]
        pub enum ScaleType {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "manual")]
            Manual,
            #[serde(rename = "automatic")]
            Automatic,
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
                    Self::None => serializer.serialize_unit_variant("ScaleType", 0u32, "none"),
                    Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "manual"),
                    Self::Automatic => serializer.serialize_unit_variant("ScaleType", 2u32, "automatic"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Azure Active Directory Application"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureActiveDirectoryApp {
    #[doc = "Application ID of the Azure Active Directory Application"]
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "Key used to authenticate to the Azure Active Directory Application"]
    #[serde(rename = "appKey")]
    pub app_key: String,
    #[doc = "Tenant id of the customer"]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl AzureActiveDirectoryApp {
    pub fn new(application_id: String, app_key: String, tenant_id: String) -> Self {
        Self {
            application_id,
            app_key,
            tenant_id,
        }
    }
}
#[doc = "Information of the backup file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupFileInfo {
    #[doc = "Location of the backup file in shared folder"]
    #[serde(rename = "fileLocation", default, skip_serializing_if = "Option::is_none")]
    pub file_location: Option<String>,
    #[doc = "Sequence number of the backup file in the backup set"]
    #[serde(rename = "familySequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub family_sequence_number: Option<i64>,
    #[doc = "An enumeration of Status of the log backup file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BackupFileStatus>,
}
impl BackupFileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An enumeration of Status of the log backup file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupFileStatus")]
pub enum BackupFileStatus {
    Arrived,
    Queued,
    Uploading,
    Uploaded,
    Restoring,
    Restored,
    Cancelled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupFileStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupFileStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupFileStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Arrived => serializer.serialize_unit_variant("BackupFileStatus", 0u32, "Arrived"),
            Self::Queued => serializer.serialize_unit_variant("BackupFileStatus", 1u32, "Queued"),
            Self::Uploading => serializer.serialize_unit_variant("BackupFileStatus", 2u32, "Uploading"),
            Self::Uploaded => serializer.serialize_unit_variant("BackupFileStatus", 3u32, "Uploaded"),
            Self::Restoring => serializer.serialize_unit_variant("BackupFileStatus", 4u32, "Restoring"),
            Self::Restored => serializer.serialize_unit_variant("BackupFileStatus", 5u32, "Restored"),
            Self::Cancelled => serializer.serialize_unit_variant("BackupFileStatus", 6u32, "Cancelled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An enumeration of backup modes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupMode")]
pub enum BackupMode {
    CreateBackup,
    ExistingBackup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreateBackup => serializer.serialize_unit_variant("BackupMode", 0u32, "CreateBackup"),
            Self::ExistingBackup => serializer.serialize_unit_variant("BackupMode", 1u32, "ExistingBackup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information of backup set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupSetInfo {
    #[doc = "Id for the set of backup files"]
    #[serde(rename = "backupSetId", default, skip_serializing_if = "Option::is_none")]
    pub backup_set_id: Option<String>,
    #[doc = "First log sequence number of the backup file"]
    #[serde(rename = "firstLsn", default, skip_serializing_if = "Option::is_none")]
    pub first_lsn: Option<String>,
    #[doc = "Last log sequence number of the backup file"]
    #[serde(rename = "lastLsn", default, skip_serializing_if = "Option::is_none")]
    pub last_lsn: Option<String>,
    #[doc = "Last modified time of the backup file in share location"]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Enum of the different backup types."]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<BackupType>,
    #[doc = "List of files in the backup set"]
    #[serde(
        rename = "listOfBackupFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub list_of_backup_files: Vec<BackupFileInfo>,
    #[doc = "Name of the database to which the backup set belongs"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Date and time that the backup operation began"]
    #[serde(rename = "backupStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub backup_start_date: Option<time::OffsetDateTime>,
    #[doc = "Date and time that the backup operation finished"]
    #[serde(rename = "backupFinishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub backup_finished_date: Option<time::OffsetDateTime>,
    #[doc = "Whether the backup set is restored or not"]
    #[serde(rename = "isBackupRestored", default, skip_serializing_if = "Option::is_none")]
    pub is_backup_restored: Option<bool>,
}
impl BackupSetInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum of the different backup types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupType")]
pub enum BackupType {
    Database,
    TransactionLog,
    File,
    DifferentialDatabase,
    DifferentialFile,
    Partial,
    DifferentialPartial,
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
            Self::Database => serializer.serialize_unit_variant("BackupType", 0u32, "Database"),
            Self::TransactionLog => serializer.serialize_unit_variant("BackupType", 1u32, "TransactionLog"),
            Self::File => serializer.serialize_unit_variant("BackupType", 2u32, "File"),
            Self::DifferentialDatabase => serializer.serialize_unit_variant("BackupType", 3u32, "DifferentialDatabase"),
            Self::DifferentialFile => serializer.serialize_unit_variant("BackupType", 4u32, "DifferentialFile"),
            Self::Partial => serializer.serialize_unit_variant("BackupType", 5u32, "Partial"),
            Self::DifferentialPartial => serializer.serialize_unit_variant("BackupType", 6u32, "DifferentialPartial"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Blob container storage information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobShare {
    #[doc = "SAS URI of Azure Storage Account Container."]
    #[serde(rename = "sasUri")]
    pub sas_uri: String,
}
impl BlobShare {
    pub fn new(sas_uri: String) -> Self {
        Self { sas_uri }
    }
}
#[doc = "Base class for all types of DMS command properties. If command is not supported by current client, this object is returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandProperties {
    #[doc = "Command type."]
    #[serde(rename = "commandType")]
    pub command_type: String,
    #[doc = "Array of errors. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ODataError>,
    #[doc = "The state of the command. This is ignored if submitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<command_properties::State>,
}
impl CommandProperties {
    pub fn new(command_type: String) -> Self {
        Self {
            command_type,
            errors: Vec::new(),
            state: None,
        }
    }
}
pub mod command_properties {
    use super::*;
    #[doc = "The state of the command. This is ignored if submitted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unknown,
        Accepted,
        Running,
        Succeeded,
        Failed,
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
                Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
                Self::Accepted => serializer.serialize_unit_variant("State", 1u32, "Accepted"),
                Self::Running => serializer.serialize_unit_variant("State", 2u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 3u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 4u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "commandType")]
pub enum CommandPropertiesUnion {
    #[serde(rename = "Migrate.SqlServer.AzureDbSqlMi.Complete")]
    MigrateSqlServerAzureDbSqlMiComplete(MigrateMiSyncCompleteCommandProperties),
    #[serde(rename = "Migrate.Sync.Complete.Database")]
    MigrateSyncCompleteDatabase(MigrateSyncCompleteCommandProperties),
}
#[doc = "Input for the task that validates MySQL database connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceMySqlTaskInput {
    #[doc = "Information for connecting to MySQL server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: MySqlConnectionInfo,
    #[doc = "An enumeration of possible target types when migrating from MySQL"]
    #[serde(rename = "targetPlatform", default, skip_serializing_if = "Option::is_none")]
    pub target_platform: Option<MySqlTargetPlatform>,
    #[doc = "Permission group for validations. These groups will run a set of permissions for validating user activity. Select the permission group for the activity that you are performing."]
    #[serde(rename = "checkPermissionsGroup", default, skip_serializing_if = "Option::is_none")]
    pub check_permissions_group: Option<ServerLevelPermissionsGroup>,
}
impl ConnectToSourceMySqlTaskInput {
    pub fn new(source_connection_info: MySqlConnectionInfo) -> Self {
        Self {
            source_connection_info,
            target_platform: None,
            check_permissions_group: None,
        }
    }
}
#[doc = "Properties for the task that validates MySQL database connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceMySqlTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates MySQL database connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToSourceMySqlTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToSourceNonSqlTaskOutput>,
}
impl ConnectToSourceMySqlTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Output for connect to Oracle, MySQL type source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToSourceNonSqlTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Server brand version"]
    #[serde(rename = "sourceServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_brand_version: Option<String>,
    #[doc = "Server properties for Oracle, MySQL type source"]
    #[serde(rename = "serverProperties", default, skip_serializing_if = "Option::is_none")]
    pub server_properties: Option<ServerProperties>,
    #[doc = "List of databases on the server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "Validation errors associated with the task"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToSourceNonSqlTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for the task that validates connection to PostgreSQL and source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourcePostgreSqlSyncTaskInput {
    #[doc = "Information for connecting to PostgreSQL server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: PostgreSqlConnectionInfo,
}
impl ConnectToSourcePostgreSqlSyncTaskInput {
    pub fn new(source_connection_info: PostgreSqlConnectionInfo) -> Self {
        Self { source_connection_info }
    }
}
#[doc = "Output for the task that validates connection to PostgreSQL and source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToSourcePostgreSqlSyncTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Version of the source server"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "List of databases on source server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "Source server brand version"]
    #[serde(rename = "sourceServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_brand_version: Option<String>,
    #[doc = "Validation errors associated with the task"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToSourcePostgreSqlSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that validates connection to PostgreSQL server and source server requirements for online migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourcePostgreSqlSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to PostgreSQL and source server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToSourcePostgreSqlSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToSourcePostgreSqlSyncTaskOutput>,
}
impl ConnectToSourcePostgreSqlSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Properties for the task that validates connection to SQL Server and source server requirements for online migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to SQL Server and also validates source server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToSourceSqlServerTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToSourceSqlServerTaskOutputUnion>,
}
impl ConnectToSourceSqlServerSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Permission group for validations. These groups will run a set of permissions for validating user activity. Select the permission group for the activity that you are performing."]
    #[serde(rename = "checkPermissionsGroup", default, skip_serializing_if = "Option::is_none")]
    pub check_permissions_group: Option<ServerLevelPermissionsGroup>,
    #[doc = "Flag for whether to collect logins from source server."]
    #[serde(rename = "collectLogins", default, skip_serializing_if = "Option::is_none")]
    pub collect_logins: Option<bool>,
    #[doc = "Flag for whether to collect agent jobs from source server."]
    #[serde(rename = "collectAgentJobs", default, skip_serializing_if = "Option::is_none")]
    pub collect_agent_jobs: Option<bool>,
}
impl ConnectToSourceSqlServerTaskInput {
    pub fn new(source_connection_info: SqlConnectionInfo) -> Self {
        Self {
            source_connection_info,
            check_permissions_group: None,
            collect_logins: None,
            collect_agent_jobs: None,
        }
    }
}
#[doc = "Output for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of result - database level or task level"]
    #[serde(rename = "resultType")]
    pub result_type: String,
}
impl ConnectToSourceSqlServerTaskOutput {
    pub fn new(result_type: String) -> Self {
        Self { id: None, result_type }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum ConnectToSourceSqlServerTaskOutputUnion {
    AgentJobLevelOutput(ConnectToSourceSqlServerTaskOutputAgentJobLevel),
    DatabaseLevelOutput(ConnectToSourceSqlServerTaskOutputDatabaseLevel),
    LoginLevelOutput(ConnectToSourceSqlServerTaskOutputLoginLevel),
    TaskLevelOutput(ConnectToSourceSqlServerTaskOutputTaskLevel),
}
#[doc = "AgentJob level output for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskOutputAgentJobLevel {
    #[serde(flatten)]
    pub connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput,
    #[doc = "AgentJob name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of AgentJob."]
    #[serde(rename = "jobCategory", default, skip_serializing_if = "Option::is_none")]
    pub job_category: Option<String>,
    #[doc = "The state of the original AgentJob."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "The owner of the AgentJob"]
    #[serde(rename = "jobOwner", default, skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<String>,
    #[doc = "UTC Date and time when the AgentJob was last executed."]
    #[serde(rename = "lastExecutedOn", default, with = "azure_core::date::rfc3339::option")]
    pub last_executed_on: Option<time::OffsetDateTime>,
    #[doc = "Information about migration eligibility of a server object"]
    #[serde(rename = "migrationEligibility", default, skip_serializing_if = "Option::is_none")]
    pub migration_eligibility: Option<MigrationEligibilityInfo>,
}
impl ConnectToSourceSqlServerTaskOutputAgentJobLevel {
    pub fn new(connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput) -> Self {
        Self {
            connect_to_source_sql_server_task_output,
            name: None,
            job_category: None,
            is_enabled: None,
            job_owner: None,
            last_executed_on: None,
            migration_eligibility: None,
        }
    }
}
#[doc = "Database level output for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput,
    #[doc = "Database name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Size of the file in megabytes"]
    #[serde(rename = "sizeMB", default, skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<f64>,
    #[doc = "The list of database files"]
    #[serde(
        rename = "databaseFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub database_files: Vec<DatabaseFileInfo>,
    #[doc = "An enumeration of SQL Server database compatibility levels"]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<DatabaseCompatLevel>,
    #[doc = "An enumeration of SQL Server Database states"]
    #[serde(rename = "databaseState", default, skip_serializing_if = "Option::is_none")]
    pub database_state: Option<DatabaseState>,
}
impl ConnectToSourceSqlServerTaskOutputDatabaseLevel {
    pub fn new(connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput) -> Self {
        Self {
            connect_to_source_sql_server_task_output,
            name: None,
            size_mb: None,
            database_files: Vec::new(),
            compatibility_level: None,
            database_state: None,
        }
    }
}
#[doc = "Login level output for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskOutputLoginLevel {
    #[serde(flatten)]
    pub connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput,
    #[doc = "Login name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Enum mapping of SMO LoginType."]
    #[serde(rename = "loginType", default, skip_serializing_if = "Option::is_none")]
    pub login_type: Option<LoginType>,
    #[doc = "The default database for the login."]
    #[serde(rename = "defaultDatabase", default, skip_serializing_if = "Option::is_none")]
    pub default_database: Option<String>,
    #[doc = "The state of the login."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Information about migration eligibility of a server object"]
    #[serde(rename = "migrationEligibility", default, skip_serializing_if = "Option::is_none")]
    pub migration_eligibility: Option<MigrationEligibilityInfo>,
}
impl ConnectToSourceSqlServerTaskOutputLoginLevel {
    pub fn new(connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput) -> Self {
        Self {
            connect_to_source_sql_server_task_output,
            name: None,
            login_type: None,
            default_database: None,
            is_enabled: None,
            migration_eligibility: None,
        }
    }
}
#[doc = "Task level output for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskOutputTaskLevel {
    #[serde(flatten)]
    pub connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput,
    #[doc = "Source databases as a map from database name to database id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub databases: Option<String>,
    #[doc = "Source logins as a map from login name to login id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logins: Option<String>,
    #[doc = "Source agent jobs as a map from agent job name to id."]
    #[serde(rename = "agentJobs", default, skip_serializing_if = "Option::is_none")]
    pub agent_jobs: Option<String>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server brand version"]
    #[serde(rename = "sourceServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_brand_version: Option<String>,
    #[doc = "Validation errors"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToSourceSqlServerTaskOutputTaskLevel {
    pub fn new(connect_to_source_sql_server_task_output: ConnectToSourceSqlServerTaskOutput) -> Self {
        Self {
            connect_to_source_sql_server_task_output,
            databases: None,
            logins: None,
            agent_jobs: None,
            source_server_version: None,
            source_server_brand_version: None,
            validation_errors: Vec::new(),
        }
    }
}
#[doc = "Properties for the task that validates connection to SQL Server and also validates source server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToSourceSqlServerTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to SQL Server and also validates source server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToSourceSqlServerTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToSourceSqlServerTaskOutputUnion>,
}
impl ConnectToSourceSqlServerTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to Azure Database for MySQL and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetAzureDbForMySqlTaskInput {
    #[doc = "Information for connecting to MySQL server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: MySqlConnectionInfo,
    #[doc = "Information for connecting to MySQL server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: MySqlConnectionInfo,
}
impl ConnectToTargetAzureDbForMySqlTaskInput {
    pub fn new(source_connection_info: MySqlConnectionInfo, target_connection_info: MySqlConnectionInfo) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
        }
    }
}
#[doc = "Output for the task that validates connection to Azure Database for MySQL and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToTargetAzureDbForMySqlTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Version of the target server"]
    #[serde(rename = "serverVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    #[doc = "List of databases on target server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "Validation errors associated with the task"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToTargetAzureDbForMySqlTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that validates connection to Azure Database for MySQL and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetAzureDbForMySqlTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to Azure Database for MySQL and target server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToTargetAzureDbForMySqlTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToTargetAzureDbForMySqlTaskOutput>,
}
impl ConnectToTargetAzureDbForMySqlTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to Azure Database for PostgreSQL and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetAzureDbForPostgreSqlSyncTaskInput {
    #[doc = "Information for connecting to PostgreSQL server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: PostgreSqlConnectionInfo,
    #[doc = "Information for connecting to PostgreSQL server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: PostgreSqlConnectionInfo,
}
impl ConnectToTargetAzureDbForPostgreSqlSyncTaskInput {
    pub fn new(source_connection_info: PostgreSqlConnectionInfo, target_connection_info: PostgreSqlConnectionInfo) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
        }
    }
}
#[doc = "Output for the task that validates connection to Azure Database for PostgreSQL and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToTargetAzureDbForPostgreSqlSyncTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Version of the target server"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "List of databases on target server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "Validation errors associated with the task"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToTargetAzureDbForPostgreSqlSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that validates connection to Azure Database For PostgreSQL server and target server requirements for online migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetAzureDbForPostgreSqlSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to Azure Database for PostgreSQL and target server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToTargetAzureDbForPostgreSqlSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToTargetAzureDbForPostgreSqlSyncTaskOutput>,
}
impl ConnectToTargetAzureDbForPostgreSqlSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to SQL DB and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlDbTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
}
impl ConnectToTargetSqlDbTaskInput {
    pub fn new(target_connection_info: SqlConnectionInfo) -> Self {
        Self { target_connection_info }
    }
}
#[doc = "Output for the task that validates connection to SQL DB and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToTargetSqlDbTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Source databases as a map from database name to database id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub databases: Option<String>,
    #[doc = "Version of the target server"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
}
impl ConnectToTargetSqlDbTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that validates connection to SQL DB and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlDbTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to SQL DB and target server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToTargetSqlDbTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToTargetSqlDbTaskOutput>,
}
impl ConnectToTargetSqlDbTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to Azure SQL Database Managed Instance online scenario."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlMiSyncTaskInput {
    #[doc = "Properties required to create a connection to Azure SQL database Managed instance"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: MiSqlConnectionInfo,
    #[doc = "Azure Active Directory Application"]
    #[serde(rename = "azureApp")]
    pub azure_app: AzureActiveDirectoryApp,
}
impl ConnectToTargetSqlMiSyncTaskInput {
    pub fn new(target_connection_info: MiSqlConnectionInfo, azure_app: AzureActiveDirectoryApp) -> Self {
        Self {
            target_connection_info,
            azure_app,
        }
    }
}
#[doc = "Output for the task that validates connection to Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToTargetSqlMiSyncTaskOutput {
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "Validation errors"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToTargetSqlMiSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that validates connection to Azure SQL Database Managed Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlMiSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to Azure SQL Database Managed Instance online scenario."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToTargetSqlMiSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToTargetSqlMiSyncTaskOutput>,
}
impl ConnectToTargetSqlMiSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlMiTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
}
impl ConnectToTargetSqlMiTaskInput {
    pub fn new(target_connection_info: SqlConnectionInfo) -> Self {
        Self { target_connection_info }
    }
}
#[doc = "Output for the task that validates connection to Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectToTargetSqlMiTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "List of logins on the target server."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logins: Vec<String>,
    #[doc = "List of agent jobs on the target server."]
    #[serde(
        rename = "agentJobs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub agent_jobs: Vec<String>,
    #[doc = "Validation errors"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ConnectToTargetSqlMiTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that validates connection to Azure SQL Database Managed Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlMiTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to Azure SQL Database Managed Instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToTargetSqlMiTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToTargetSqlMiTaskOutput>,
}
impl ConnectToTargetSqlMiTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that validates connection to Azure SQL DB and target server requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlSqlDbSyncTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
}
impl ConnectToTargetSqlSqlDbSyncTaskInput {
    pub fn new(source_connection_info: SqlConnectionInfo, target_connection_info: SqlConnectionInfo) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
        }
    }
}
#[doc = "Properties for the task that validates connection to SQL DB and target server requirements for online migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToTargetSqlSqlDbSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that validates connection to Azure SQL DB and target server requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ConnectToTargetSqlSqlDbSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ConnectToTargetSqlDbTaskOutput>,
}
impl ConnectToTargetSqlSqlDbSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Defines the connection properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionInfo {
    #[doc = "Type of connection info"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "User name"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Password credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ConnectionInfo {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            user_name: None,
            password: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionInfoUnion {
    MiSqlConnectionInfo(MiSqlConnectionInfo),
    MySqlConnectionInfo(MySqlConnectionInfo),
    PostgreSqlConnectionInfo(PostgreSqlConnectionInfo),
    SqlConnectionInfo(SqlConnectionInfo),
}
#[doc = "Results for checksum based Data Integrity validation results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataIntegrityValidationResult {
    #[doc = "List of failed table names of source and target pair"]
    #[serde(rename = "failedObjects", default, skip_serializing_if = "Option::is_none")]
    pub failed_objects: Option<serde_json::Value>,
    #[doc = "Description about the errors happen while performing migration validation"]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<ValidationError>,
}
impl DataIntegrityValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Basic summary of a data item migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataItemMigrationSummaryResult {
    #[doc = "Name of the item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Status message"]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Number of items"]
    #[serde(rename = "itemsCount", default, skip_serializing_if = "Option::is_none")]
    pub items_count: Option<i64>,
    #[doc = "Number of successfully completed items"]
    #[serde(rename = "itemsCompletedCount", default, skip_serializing_if = "Option::is_none")]
    pub items_completed_count: Option<i64>,
    #[doc = "Wildcard string prefix to use for querying all errors of the item"]
    #[serde(rename = "errorPrefix", default, skip_serializing_if = "Option::is_none")]
    pub error_prefix: Option<String>,
    #[doc = "Wildcard string prefix to use for querying all sub-tem results of the item"]
    #[serde(rename = "resultPrefix", default, skip_serializing_if = "Option::is_none")]
    pub result_prefix: Option<String>,
}
impl DataItemMigrationSummaryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration Task errors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMigrationError {
    #[doc = "Error description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ErrorType>,
}
impl DataMigrationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common metadata for migration projects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMigrationProjectMetadata {
    #[doc = "Source server name"]
    #[serde(rename = "sourceServerName", default, skip_serializing_if = "Option::is_none")]
    pub source_server_name: Option<String>,
    #[doc = "Source server port number"]
    #[serde(rename = "sourceServerPort", default, skip_serializing_if = "Option::is_none")]
    pub source_server_port: Option<String>,
    #[doc = "Source username"]
    #[serde(rename = "sourceUsername", default, skip_serializing_if = "Option::is_none")]
    pub source_username: Option<String>,
    #[doc = "Target server name"]
    #[serde(rename = "targetServerName", default, skip_serializing_if = "Option::is_none")]
    pub target_server_name: Option<String>,
    #[doc = "Target username"]
    #[serde(rename = "targetUsername", default, skip_serializing_if = "Option::is_none")]
    pub target_username: Option<String>,
    #[doc = "Target database name"]
    #[serde(rename = "targetDbName", default, skip_serializing_if = "Option::is_none")]
    pub target_db_name: Option<String>,
    #[doc = "Whether target connection is Windows authentication"]
    #[serde(rename = "targetUsingWinAuth", default, skip_serializing_if = "Option::is_none")]
    pub target_using_win_auth: Option<bool>,
    #[doc = "List of tables selected for migration"]
    #[serde(
        rename = "selectedMigrationTables",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub selected_migration_tables: Vec<MigrationTableMetadata>,
}
impl DataMigrationProjectMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Database Migration Service resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMigrationService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "HTTP strong entity tag value. Ignored if submitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The resource kind. Only 'vm' (the default) is supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties of the Data Migration service instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMigrationServiceProperties>,
    #[doc = "An Azure SKU instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ServiceSku>,
}
impl DataMigrationService {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            kind: None,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "OData page of service objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMigrationServiceList {
    #[doc = "List of services"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataMigrationService>,
    #[doc = "URL to load the next page of services"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataMigrationServiceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataMigrationServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Data Migration service instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMigrationServiceProperties {
    #[doc = "The resource's provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_migration_service_properties::ProvisioningState>,
    #[doc = "The public key of the service, used to encrypt secrets sent to the service"]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "The ID of the Microsoft.Network/virtualNetworks/subnets resource to which the service should be joined"]
    #[serde(rename = "virtualSubnetId")]
    pub virtual_subnet_id: String,
}
impl DataMigrationServiceProperties {
    pub fn new(virtual_subnet_id: String) -> Self {
        Self {
            provisioning_state: None,
            public_key: None,
            virtual_subnet_id,
        }
    }
}
pub mod data_migration_service_properties {
    use super::*;
    #[doc = "The resource's provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Deleting,
        Deploying,
        Stopped,
        Stopping,
        Starting,
        FailedToStart,
        FailedToStop,
        Succeeded,
        Failed,
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
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Deploying => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deploying"),
                Self::Stopped => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Stopped"),
                Self::Stopping => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Stopping"),
                Self::Starting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Starting"),
                Self::FailedToStart => serializer.serialize_unit_variant("ProvisioningState", 6u32, "FailedToStart"),
                Self::FailedToStop => serializer.serialize_unit_variant("ProvisioningState", 7u32, "FailedToStop"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Service health status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMigrationServiceStatusResponse {
    #[doc = "The DMS instance agent version"]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The machine-readable status, such as 'Initializing', 'Offline', 'Online', 'Deploying', 'Deleting', 'Stopped', 'Stopping', 'Starting', 'FailedToStart', 'FailedToStop' or 'Failed'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The services virtual machine size, such as 'Standard_D2_v2'"]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "The list of supported task types"]
    #[serde(
        rename = "supportedTaskTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_task_types: Vec<String>,
}
impl DataMigrationServiceStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a single database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[doc = "Unique identifier for the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An enumeration of SQL Server database compatibility levels"]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<DatabaseCompatLevel>,
    #[doc = "Collation name of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "Name of the server"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Fully qualified name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Install id of the database"]
    #[serde(rename = "installId", default, skip_serializing_if = "Option::is_none")]
    pub install_id: Option<String>,
    #[doc = "Version of the server"]
    #[serde(rename = "serverVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    #[doc = "Edition of the server"]
    #[serde(rename = "serverEdition", default, skip_serializing_if = "Option::is_none")]
    pub server_edition: Option<String>,
    #[doc = "Product level of the server (RTM, SP, CTP)."]
    #[serde(rename = "serverLevel", default, skip_serializing_if = "Option::is_none")]
    pub server_level: Option<String>,
    #[doc = "Default path of the data files"]
    #[serde(rename = "serverDefaultDataPath", default, skip_serializing_if = "Option::is_none")]
    pub server_default_data_path: Option<String>,
    #[doc = "Default path of the log files"]
    #[serde(rename = "serverDefaultLogPath", default, skip_serializing_if = "Option::is_none")]
    pub server_default_log_path: Option<String>,
    #[doc = "Default path of the backup folder"]
    #[serde(rename = "serverDefaultBackupPath", default, skip_serializing_if = "Option::is_none")]
    pub server_default_backup_path: Option<String>,
    #[doc = "Number of cores on the server"]
    #[serde(rename = "serverCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub server_core_count: Option<i64>,
    #[doc = "Number of cores on the server that have VISIBLE ONLINE status"]
    #[serde(rename = "serverVisibleOnlineCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub server_visible_online_core_count: Option<i64>,
    #[doc = "An enumeration of SQL Server Database states"]
    #[serde(rename = "databaseState", default, skip_serializing_if = "Option::is_none")]
    pub database_state: Option<DatabaseState>,
    #[doc = "The unique Server Id"]
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about backup files when existing backup mode is used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseBackupInfo {
    #[doc = "Database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Enum of the different backup types."]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<BackupType>,
    #[doc = "The list of backup files for the current database."]
    #[serde(
        rename = "backupFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_files: Vec<String>,
    #[doc = "Position of current database backup in the file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[doc = "Database was damaged when backed up, but the backup operation was requested to continue despite errors."]
    #[serde(rename = "isDamaged", default, skip_serializing_if = "Option::is_none")]
    pub is_damaged: Option<bool>,
    #[doc = "Whether the backup set is compressed"]
    #[serde(rename = "isCompressed", default, skip_serializing_if = "Option::is_none")]
    pub is_compressed: Option<bool>,
    #[doc = "Number of files in the backup set."]
    #[serde(rename = "familyCount", default, skip_serializing_if = "Option::is_none")]
    pub family_count: Option<i64>,
    #[doc = "Date and time when the backup operation finished."]
    #[serde(rename = "backupFinishDate", default, with = "azure_core::date::rfc3339::option")]
    pub backup_finish_date: Option<time::OffsetDateTime>,
}
impl DatabaseBackupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An enumeration of SQL Server database compatibility levels"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseCompatLevel")]
pub enum DatabaseCompatLevel {
    CompatLevel80,
    CompatLevel90,
    CompatLevel100,
    CompatLevel110,
    CompatLevel120,
    CompatLevel130,
    CompatLevel140,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseCompatLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseCompatLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseCompatLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CompatLevel80 => serializer.serialize_unit_variant("DatabaseCompatLevel", 0u32, "CompatLevel80"),
            Self::CompatLevel90 => serializer.serialize_unit_variant("DatabaseCompatLevel", 1u32, "CompatLevel90"),
            Self::CompatLevel100 => serializer.serialize_unit_variant("DatabaseCompatLevel", 2u32, "CompatLevel100"),
            Self::CompatLevel110 => serializer.serialize_unit_variant("DatabaseCompatLevel", 3u32, "CompatLevel110"),
            Self::CompatLevel120 => serializer.serialize_unit_variant("DatabaseCompatLevel", 4u32, "CompatLevel120"),
            Self::CompatLevel130 => serializer.serialize_unit_variant("DatabaseCompatLevel", 5u32, "CompatLevel130"),
            Self::CompatLevel140 => serializer.serialize_unit_variant("DatabaseCompatLevel", 6u32, "CompatLevel140"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Database file specific information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseFileInfo {
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Unique identifier for database file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Logical name of the file"]
    #[serde(rename = "logicalName", default, skip_serializing_if = "Option::is_none")]
    pub logical_name: Option<String>,
    #[doc = "Operating-system full path of the file"]
    #[serde(rename = "physicalFullName", default, skip_serializing_if = "Option::is_none")]
    pub physical_full_name: Option<String>,
    #[doc = "Suggested full path of the file for restoring"]
    #[serde(rename = "restoreFullName", default, skip_serializing_if = "Option::is_none")]
    pub restore_full_name: Option<String>,
    #[doc = "An enumeration of SQL Server database file types"]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<DatabaseFileType>,
    #[doc = "Size of the file in megabytes"]
    #[serde(rename = "sizeMB", default, skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<f64>,
}
impl DatabaseFileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database file specific information for input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseFileInput {
    #[doc = "Unique identifier for database file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Logical name of the file"]
    #[serde(rename = "logicalName", default, skip_serializing_if = "Option::is_none")]
    pub logical_name: Option<String>,
    #[doc = "Operating-system full path of the file"]
    #[serde(rename = "physicalFullName", default, skip_serializing_if = "Option::is_none")]
    pub physical_full_name: Option<String>,
    #[doc = "Suggested full path of the file for restoring"]
    #[serde(rename = "restoreFullName", default, skip_serializing_if = "Option::is_none")]
    pub restore_full_name: Option<String>,
    #[doc = "An enumeration of SQL Server database file types"]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<DatabaseFileType>,
}
impl DatabaseFileInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An enumeration of SQL Server database file types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseFileType")]
pub enum DatabaseFileType {
    Rows,
    Log,
    Filestream,
    NotSupported,
    Fulltext,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseFileType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseFileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseFileType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Rows => serializer.serialize_unit_variant("DatabaseFileType", 0u32, "Rows"),
            Self::Log => serializer.serialize_unit_variant("DatabaseFileType", 1u32, "Log"),
            Self::Filestream => serializer.serialize_unit_variant("DatabaseFileType", 2u32, "Filestream"),
            Self::NotSupported => serializer.serialize_unit_variant("DatabaseFileType", 3u32, "NotSupported"),
            Self::Fulltext => serializer.serialize_unit_variant("DatabaseFileType", 4u32, "Fulltext"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Project Database Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseInfo {
    #[doc = "Name of the database"]
    #[serde(rename = "sourceDatabaseName")]
    pub source_database_name: String,
}
impl DatabaseInfo {
    pub fn new(source_database_name: String) -> Self {
        Self { source_database_name }
    }
}
#[doc = "Current stage of migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseMigrationStage")]
pub enum DatabaseMigrationStage {
    None,
    Initialize,
    Backup,
    FileCopy,
    Restore,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseMigrationStage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseMigrationStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseMigrationStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("DatabaseMigrationStage", 0u32, "None"),
            Self::Initialize => serializer.serialize_unit_variant("DatabaseMigrationStage", 1u32, "Initialize"),
            Self::Backup => serializer.serialize_unit_variant("DatabaseMigrationStage", 2u32, "Backup"),
            Self::FileCopy => serializer.serialize_unit_variant("DatabaseMigrationStage", 3u32, "FileCopy"),
            Self::Restore => serializer.serialize_unit_variant("DatabaseMigrationStage", 4u32, "Restore"),
            Self::Completed => serializer.serialize_unit_variant("DatabaseMigrationStage", 5u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Database level migration state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseMigrationState")]
pub enum DatabaseMigrationState {
    #[serde(rename = "UNDEFINED")]
    Undefined,
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "FULL_BACKUP_UPLOAD_START")]
    FullBackupUploadStart,
    #[serde(rename = "LOG_SHIPPING_START")]
    LogShippingStart,
    #[serde(rename = "UPLOAD_LOG_FILES_START")]
    UploadLogFilesStart,
    #[serde(rename = "CUTOVER_START")]
    CutoverStart,
    #[serde(rename = "POST_CUTOVER_COMPLETE")]
    PostCutoverComplete,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseMigrationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseMigrationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseMigrationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Undefined => serializer.serialize_unit_variant("DatabaseMigrationState", 0u32, "UNDEFINED"),
            Self::Initial => serializer.serialize_unit_variant("DatabaseMigrationState", 1u32, "INITIAL"),
            Self::FullBackupUploadStart => serializer.serialize_unit_variant("DatabaseMigrationState", 2u32, "FULL_BACKUP_UPLOAD_START"),
            Self::LogShippingStart => serializer.serialize_unit_variant("DatabaseMigrationState", 3u32, "LOG_SHIPPING_START"),
            Self::UploadLogFilesStart => serializer.serialize_unit_variant("DatabaseMigrationState", 4u32, "UPLOAD_LOG_FILES_START"),
            Self::CutoverStart => serializer.serialize_unit_variant("DatabaseMigrationState", 5u32, "CUTOVER_START"),
            Self::PostCutoverComplete => serializer.serialize_unit_variant("DatabaseMigrationState", 6u32, "POST_CUTOVER_COMPLETE"),
            Self::Completed => serializer.serialize_unit_variant("DatabaseMigrationState", 7u32, "COMPLETED"),
            Self::Cancelled => serializer.serialize_unit_variant("DatabaseMigrationState", 8u32, "CANCELLED"),
            Self::Failed => serializer.serialize_unit_variant("DatabaseMigrationState", 9u32, "FAILED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A representation of the name of an object in a database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseObjectName {
    #[doc = "The unescaped name of the database containing the object"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The unescaped name of the object"]
    #[serde(rename = "objectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The unescaped name of the schema containing the object"]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "An enumeration of type of objects"]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<DatabaseObjectType>,
}
impl DatabaseObjectName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An enumeration of type of objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseObjectType")]
pub enum DatabaseObjectType {
    StoredProcedures,
    Table,
    User,
    View,
    Function,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseObjectType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseObjectType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseObjectType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StoredProcedures => serializer.serialize_unit_variant("DatabaseObjectType", 0u32, "StoredProcedures"),
            Self::Table => serializer.serialize_unit_variant("DatabaseObjectType", 1u32, "Table"),
            Self::User => serializer.serialize_unit_variant("DatabaseObjectType", 2u32, "User"),
            Self::View => serializer.serialize_unit_variant("DatabaseObjectType", 3u32, "View"),
            Self::Function => serializer.serialize_unit_variant("DatabaseObjectType", 4u32, "Function"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An enumeration of SQL Server Database states"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DatabaseState")]
pub enum DatabaseState {
    Online,
    Restoring,
    Recovering,
    RecoveryPending,
    Suspect,
    Emergency,
    Offline,
    Copying,
    OfflineSecondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DatabaseState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DatabaseState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DatabaseState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Online => serializer.serialize_unit_variant("DatabaseState", 0u32, "Online"),
            Self::Restoring => serializer.serialize_unit_variant("DatabaseState", 1u32, "Restoring"),
            Self::Recovering => serializer.serialize_unit_variant("DatabaseState", 2u32, "Recovering"),
            Self::RecoveryPending => serializer.serialize_unit_variant("DatabaseState", 3u32, "RecoveryPending"),
            Self::Suspect => serializer.serialize_unit_variant("DatabaseState", 4u32, "Suspect"),
            Self::Emergency => serializer.serialize_unit_variant("DatabaseState", 5u32, "Emergency"),
            Self::Offline => serializer.serialize_unit_variant("DatabaseState", 6u32, "Offline"),
            Self::Copying => serializer.serialize_unit_variant("DatabaseState", 7u32, "Copying"),
            Self::OfflineSecondary => serializer.serialize_unit_variant("DatabaseState", 8u32, "OfflineSecondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Summary of database results in the migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSummaryResult {
    #[serde(flatten)]
    pub data_item_migration_summary_result: DataItemMigrationSummaryResult,
    #[doc = "Size of the database in megabytes"]
    #[serde(rename = "sizeMB", default, skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<f64>,
}
impl DatabaseSummaryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Table properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseTable {
    #[doc = "Indicates whether table is empty or not"]
    #[serde(rename = "hasRows", default, skip_serializing_if = "Option::is_none")]
    pub has_rows: Option<bool>,
    #[doc = "Schema-qualified name of the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DatabaseTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ErrorType")]
pub enum ErrorType {
    Default,
    Warning,
    Error,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ErrorType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ErrorType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ErrorType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("ErrorType", 0u32, "Default"),
            Self::Warning => serializer.serialize_unit_variant("ErrorType", 1u32, "Warning"),
            Self::Error => serializer.serialize_unit_variant("ErrorType", 2u32, "Error"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Description about the errors happen while performing migration validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExecutionStatistics {
    #[doc = "No. of query executions"]
    #[serde(rename = "executionCount", default, skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<i64>,
    #[doc = "CPU Time in millisecond(s) for the query execution"]
    #[serde(rename = "cpuTimeMs", default, skip_serializing_if = "Option::is_none")]
    pub cpu_time_ms: Option<f32>,
    #[doc = "Time taken in millisecond(s) for executing the query"]
    #[serde(rename = "elapsedTimeMs", default, skip_serializing_if = "Option::is_none")]
    pub elapsed_time_ms: Option<f32>,
    #[doc = "Dictionary of sql query execution wait types and the respective statistics"]
    #[serde(rename = "waitStats", default, skip_serializing_if = "Option::is_none")]
    pub wait_stats: Option<serde_json::Value>,
    #[doc = "Indicates whether the query resulted in an error"]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "List of sql Errors"]
    #[serde(
        rename = "sqlErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_errors: Vec<String>,
}
impl ExecutionStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "File share information with Path, Username, and Password."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileShare {
    #[doc = "User name credential to connect to the share location"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Password credential used to connect to the share location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The folder path for this share."]
    pub path: String,
}
impl FileShare {
    pub fn new(path: String) -> Self {
        Self {
            user_name: None,
            password: None,
            path,
        }
    }
}
#[doc = "Input for the task that reads configuration from project artifacts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProjectDetailsNonSqlTaskInput {
    #[doc = "Name of the migration project"]
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[doc = "A URL that points to the location to access project artifacts"]
    #[serde(rename = "projectLocation")]
    pub project_location: String,
}
impl GetProjectDetailsNonSqlTaskInput {
    pub fn new(project_name: String, project_location: String) -> Self {
        Self {
            project_name,
            project_location,
        }
    }
}
#[doc = "Input for the task that gets TDE certificates in Base64 encoded format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTdeCertificatesSqlTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "connectionInfo")]
    pub connection_info: SqlConnectionInfo,
    #[doc = "File share information with Path, Username, and Password."]
    #[serde(rename = "backupFileShare")]
    pub backup_file_share: FileShare,
    #[doc = "List containing certificate names and corresponding password to use for encrypting the exported certificate."]
    #[serde(rename = "selectedCertificates")]
    pub selected_certificates: Vec<SelectedCertificateInput>,
}
impl GetTdeCertificatesSqlTaskInput {
    pub fn new(
        connection_info: SqlConnectionInfo,
        backup_file_share: FileShare,
        selected_certificates: Vec<SelectedCertificateInput>,
    ) -> Self {
        Self {
            connection_info,
            backup_file_share,
            selected_certificates,
        }
    }
}
#[doc = "Output of the task that gets TDE certificates in Base64 encoded format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetTdeCertificatesSqlTaskOutput {
    #[doc = "Mapping from certificate name to base 64 encoded format."]
    #[serde(rename = "base64EncodedCertificates", default, skip_serializing_if = "Option::is_none")]
    pub base64_encoded_certificates: Option<String>,
    #[doc = "Validation errors"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl GetTdeCertificatesSqlTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that gets TDE certificates in Base64 encoded format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTdeCertificatesSqlTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that gets TDE certificates in Base64 encoded format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<GetTdeCertificatesSqlTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<GetTdeCertificatesSqlTaskOutput>,
}
impl GetTdeCertificatesSqlTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that collects user tables for the given list of databases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserTablesSqlSyncTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
    #[doc = "List of source database names to collect tables for"]
    #[serde(rename = "selectedSourceDatabases")]
    pub selected_source_databases: Vec<String>,
    #[doc = "List of target database names to collect tables for"]
    #[serde(rename = "selectedTargetDatabases")]
    pub selected_target_databases: Vec<String>,
}
impl GetUserTablesSqlSyncTaskInput {
    pub fn new(
        source_connection_info: SqlConnectionInfo,
        target_connection_info: SqlConnectionInfo,
        selected_source_databases: Vec<String>,
        selected_target_databases: Vec<String>,
    ) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
            selected_source_databases,
            selected_target_databases,
        }
    }
}
#[doc = "Output of the task that collects user tables for the given list of databases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetUserTablesSqlSyncTaskOutput {
    #[doc = "Mapping from database name to list of source tables"]
    #[serde(rename = "databasesToSourceTables", default, skip_serializing_if = "Option::is_none")]
    pub databases_to_source_tables: Option<String>,
    #[doc = "Mapping from database name to list of target tables"]
    #[serde(rename = "databasesToTargetTables", default, skip_serializing_if = "Option::is_none")]
    pub databases_to_target_tables: Option<String>,
    #[doc = "Mapping from database name to list of validation errors"]
    #[serde(rename = "tableValidationErrors", default, skip_serializing_if = "Option::is_none")]
    pub table_validation_errors: Option<String>,
    #[doc = "Validation errors"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl GetUserTablesSqlSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that collects user tables for the given list of databases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserTablesSqlSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that collects user tables for the given list of databases"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<GetUserTablesSqlSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<GetUserTablesSqlSyncTaskOutput>,
}
impl GetUserTablesSqlSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that collects user tables for the given list of databases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserTablesSqlTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "connectionInfo")]
    pub connection_info: SqlConnectionInfo,
    #[doc = "List of database names to collect tables for"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<String>,
}
impl GetUserTablesSqlTaskInput {
    pub fn new(connection_info: SqlConnectionInfo, selected_databases: Vec<String>) -> Self {
        Self {
            connection_info,
            selected_databases,
        }
    }
}
#[doc = "Output of the task that collects user tables for the given list of databases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetUserTablesSqlTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Mapping from database name to list of tables"]
    #[serde(rename = "databasesToTables", default, skip_serializing_if = "Option::is_none")]
    pub databases_to_tables: Option<String>,
    #[doc = "Validation errors"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl GetUserTablesSqlTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that collects user tables for the given list of databases"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserTablesSqlTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that collects user tables for the given list of databases"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<GetUserTablesSqlTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<GetUserTablesSqlTaskOutput>,
}
impl GetUserTablesSqlTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Enum of the different stage of login migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LoginMigrationStage")]
pub enum LoginMigrationStage {
    None,
    Initialize,
    LoginMigration,
    EstablishUserMapping,
    AssignRoleMembership,
    AssignRoleOwnership,
    EstablishServerPermissions,
    EstablishObjectPermissions,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LoginMigrationStage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LoginMigrationStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LoginMigrationStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("LoginMigrationStage", 0u32, "None"),
            Self::Initialize => serializer.serialize_unit_variant("LoginMigrationStage", 1u32, "Initialize"),
            Self::LoginMigration => serializer.serialize_unit_variant("LoginMigrationStage", 2u32, "LoginMigration"),
            Self::EstablishUserMapping => serializer.serialize_unit_variant("LoginMigrationStage", 3u32, "EstablishUserMapping"),
            Self::AssignRoleMembership => serializer.serialize_unit_variant("LoginMigrationStage", 4u32, "AssignRoleMembership"),
            Self::AssignRoleOwnership => serializer.serialize_unit_variant("LoginMigrationStage", 5u32, "AssignRoleOwnership"),
            Self::EstablishServerPermissions => {
                serializer.serialize_unit_variant("LoginMigrationStage", 6u32, "EstablishServerPermissions")
            }
            Self::EstablishObjectPermissions => {
                serializer.serialize_unit_variant("LoginMigrationStage", 7u32, "EstablishObjectPermissions")
            }
            Self::Completed => serializer.serialize_unit_variant("LoginMigrationStage", 8u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Enum mapping of SMO LoginType."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LoginType")]
pub enum LoginType {
    WindowsUser,
    WindowsGroup,
    SqlLogin,
    Certificate,
    AsymmetricKey,
    ExternalUser,
    ExternalGroup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LoginType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LoginType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LoginType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WindowsUser => serializer.serialize_unit_variant("LoginType", 0u32, "WindowsUser"),
            Self::WindowsGroup => serializer.serialize_unit_variant("LoginType", 1u32, "WindowsGroup"),
            Self::SqlLogin => serializer.serialize_unit_variant("LoginType", 2u32, "SqlLogin"),
            Self::Certificate => serializer.serialize_unit_variant("LoginType", 3u32, "Certificate"),
            Self::AsymmetricKey => serializer.serialize_unit_variant("LoginType", 4u32, "AsymmetricKey"),
            Self::ExternalUser => serializer.serialize_unit_variant("LoginType", 5u32, "ExternalUser"),
            Self::ExternalGroup => serializer.serialize_unit_variant("LoginType", 6u32, "ExternalGroup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties required to create a connection to Azure SQL database Managed instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MiSqlConnectionInfo {
    #[serde(flatten)]
    pub connection_info: ConnectionInfo,
    #[doc = "Resource id for Azure SQL database Managed instance"]
    #[serde(rename = "managedInstanceResourceId")]
    pub managed_instance_resource_id: String,
}
impl MiSqlConnectionInfo {
    pub fn new(connection_info: ConnectionInfo, managed_instance_resource_id: String) -> Self {
        Self {
            connection_info,
            managed_instance_resource_id,
        }
    }
}
#[doc = "Input for command that completes online migration for an Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateMiSyncCompleteCommandInput {
    #[doc = "Name of managed instance database"]
    #[serde(rename = "sourceDatabaseName")]
    pub source_database_name: String,
}
impl MigrateMiSyncCompleteCommandInput {
    pub fn new(source_database_name: String) -> Self {
        Self { source_database_name }
    }
}
#[doc = "Output for command that completes online migration for an Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMiSyncCompleteCommandOutput {
    #[doc = "List of errors that happened during the command execution"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ReportableException>,
}
impl MigrateMiSyncCompleteCommandOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the command that completes online migration for an Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateMiSyncCompleteCommandProperties {
    #[serde(flatten)]
    pub command_properties: CommandProperties,
    #[doc = "Input for command that completes online migration for an Azure SQL Database Managed Instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateMiSyncCompleteCommandInput>,
    #[doc = "Output for command that completes online migration for an Azure SQL Database Managed Instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<MigrateMiSyncCompleteCommandOutput>,
}
impl MigrateMiSyncCompleteCommandProperties {
    pub fn new(command_properties: CommandProperties) -> Self {
        Self {
            command_properties,
            input: None,
            output: None,
        }
    }
}
#[doc = "Database specific information for MySQL to Azure Database for MySQL migration task inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncDatabaseInput {
    #[doc = "Name of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Name of target database. Note: Target database will be truncated before starting migration."]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Migration settings which tune the migration behavior"]
    #[serde(rename = "migrationSetting", default, skip_serializing_if = "Option::is_none")]
    pub migration_setting: Option<serde_json::Value>,
    #[doc = "Source settings to tune source endpoint migration behavior"]
    #[serde(rename = "sourceSetting", default, skip_serializing_if = "Option::is_none")]
    pub source_setting: Option<serde_json::Value>,
    #[doc = "Target settings to tune target endpoint migration behavior"]
    #[serde(rename = "targetSetting", default, skip_serializing_if = "Option::is_none")]
    pub target_setting: Option<serde_json::Value>,
}
impl MigrateMySqlAzureDbForMySqlSyncDatabaseInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for the task that migrates MySQL databases to Azure Database for MySQL for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskInput {
    #[doc = "Information for connecting to MySQL server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: MySqlConnectionInfo,
    #[doc = "Information for connecting to MySQL server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: MySqlConnectionInfo,
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateMySqlAzureDbForMySqlSyncDatabaseInput>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskInput {
    pub fn new(
        source_connection_info: MySqlConnectionInfo,
        target_connection_info: MySqlConnectionInfo,
        selected_databases: Vec<MigrateMySqlAzureDbForMySqlSyncDatabaseInput>,
    ) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
            selected_databases,
        }
    }
}
#[doc = "Output for the task that migrates MySQL databases to Azure Database for MySQL for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Result type"]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum MigrateMySqlAzureDbForMySqlSyncTaskOutputUnion {
    DatabaseLevelErrorOutput(MigrateMySqlAzureDbForMySqlSyncTaskOutputDatabaseError),
    DatabaseLevelOutput(MigrateMySqlAzureDbForMySqlSyncTaskOutputDatabaseLevel),
    ErrorOutput(MigrateMySqlAzureDbForMySqlSyncTaskOutputError),
    MigrationLevelOutput(MigrateMySqlAzureDbForMySqlSyncTaskOutputMigrationLevel),
    TableLevelOutput(MigrateMySqlAzureDbForMySqlSyncTaskOutputTableLevel),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskOutputDatabaseError {
    #[serde(flatten)]
    pub migrate_my_sql_azure_db_for_my_sql_sync_task_output: MigrateMySqlAzureDbForMySqlSyncTaskOutput,
    #[doc = "Error message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "List of error events."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<SyncMigrationDatabaseErrorEvent>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskOutputDatabaseError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub migrate_my_sql_azure_db_for_my_sql_sync_task_output: MigrateMySqlAzureDbForMySqlSyncTaskOutput,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Enum of the different state of database level online migration."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<SyncDatabaseMigrationReportingState>,
    #[doc = "Number of incoming changes"]
    #[serde(rename = "incomingChanges", default, skip_serializing_if = "Option::is_none")]
    pub incoming_changes: Option<i64>,
    #[doc = "Number of applied changes"]
    #[serde(rename = "appliedChanges", default, skip_serializing_if = "Option::is_none")]
    pub applied_changes: Option<i64>,
    #[doc = "Number of cdc inserts"]
    #[serde(rename = "cdcInsertCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_insert_counter: Option<i64>,
    #[doc = "Number of cdc deletes"]
    #[serde(rename = "cdcDeleteCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_delete_counter: Option<i64>,
    #[doc = "Number of cdc updates"]
    #[serde(rename = "cdcUpdateCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_update_counter: Option<i64>,
    #[doc = "Number of tables completed in full load"]
    #[serde(rename = "fullLoadCompletedTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_completed_tables: Option<i64>,
    #[doc = "Number of tables loading in full load"]
    #[serde(rename = "fullLoadLoadingTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_loading_tables: Option<i64>,
    #[doc = "Number of tables queued in full load"]
    #[serde(rename = "fullLoadQueuedTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_queued_tables: Option<i64>,
    #[doc = "Number of tables errored in full load"]
    #[serde(rename = "fullLoadErroredTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_errored_tables: Option<i64>,
    #[doc = "Indicates if initial load (full load) has been completed"]
    #[serde(rename = "initializationCompleted", default, skip_serializing_if = "Option::is_none")]
    pub initialization_completed: Option<bool>,
    #[doc = "CDC apply latency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskOutputDatabaseLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskOutputError {
    #[serde(flatten)]
    pub migrate_my_sql_azure_db_for_my_sql_sync_task_output: MigrateMySqlAzureDbForMySqlSyncTaskOutput,
    #[doc = "Exception object for all custom exceptions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportableException>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskOutputError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskOutputMigrationLevel {
    #[serde(flatten)]
    pub migrate_my_sql_azure_db_for_my_sql_sync_task_output: MigrateMySqlAzureDbForMySqlSyncTaskOutput,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server name"]
    #[serde(rename = "sourceServer", default, skip_serializing_if = "Option::is_none")]
    pub source_server: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server name"]
    #[serde(rename = "targetServer", default, skip_serializing_if = "Option::is_none")]
    pub target_server: Option<String>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskOutputMigrationLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskOutputTableLevel {
    #[serde(flatten)]
    pub migrate_my_sql_azure_db_for_my_sql_sync_task_output: MigrateMySqlAzureDbForMySqlSyncTaskOutput,
    #[doc = "Name of the table"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Number of applied inserts"]
    #[serde(rename = "cdcInsertCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_insert_counter: Option<String>,
    #[doc = "Number of applied updates"]
    #[serde(rename = "cdcUpdateCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_update_counter: Option<String>,
    #[doc = "Number of applied deletes"]
    #[serde(rename = "cdcDeleteCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_delete_counter: Option<String>,
    #[doc = "Estimate to finish full load"]
    #[serde(rename = "fullLoadEstFinishTime", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_est_finish_time: Option<time::OffsetDateTime>,
    #[doc = "Full load start time"]
    #[serde(rename = "fullLoadStartedOn", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_started_on: Option<time::OffsetDateTime>,
    #[doc = "Full load end time"]
    #[serde(rename = "fullLoadEndedOn", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_ended_on: Option<time::OffsetDateTime>,
    #[doc = "Number of rows applied in full load"]
    #[serde(rename = "fullLoadTotalRows", default, skip_serializing_if = "Option::is_none")]
    pub full_load_total_rows: Option<i64>,
    #[doc = "Enum of the different state of table level online migration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SyncTableMigrationState>,
    #[doc = "Total number of applied changes"]
    #[serde(rename = "totalChangesApplied", default, skip_serializing_if = "Option::is_none")]
    pub total_changes_applied: Option<i64>,
    #[doc = "Number of data errors occurred"]
    #[serde(rename = "dataErrorsCounter", default, skip_serializing_if = "Option::is_none")]
    pub data_errors_counter: Option<i64>,
    #[doc = "Last modified time on target"]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskOutputTableLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that migrates MySQL databases to Azure Database for MySQL for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateMySqlAzureDbForMySqlSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that migrates MySQL databases to Azure Database for MySQL for online migrations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateMySqlAzureDbForMySqlSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<MigrateMySqlAzureDbForMySqlSyncTaskOutputUnion>,
}
impl MigrateMySqlAzureDbForMySqlSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Database specific information for PostgreSQL to Azure Database for PostgreSQL migration task inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncDatabaseInput {
    #[doc = "Name of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Name of target database. Note: Target database will be truncated before starting migration."]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Migration settings which tune the migration behavior"]
    #[serde(rename = "migrationSetting", default, skip_serializing_if = "Option::is_none")]
    pub migration_setting: Option<serde_json::Value>,
    #[doc = "Source settings to tune source endpoint migration behavior"]
    #[serde(rename = "sourceSetting", default, skip_serializing_if = "Option::is_none")]
    pub source_setting: Option<serde_json::Value>,
    #[doc = "Target settings to tune target endpoint migration behavior"]
    #[serde(rename = "targetSetting", default, skip_serializing_if = "Option::is_none")]
    pub target_setting: Option<serde_json::Value>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncDatabaseInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for the task that migrates PostgreSQL databases to Azure Database for PostgreSQL for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskInput {
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigratePostgreSqlAzureDbForPostgreSqlSyncDatabaseInput>,
    #[doc = "Information for connecting to PostgreSQL server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: PostgreSqlConnectionInfo,
    #[doc = "Information for connecting to PostgreSQL server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: PostgreSqlConnectionInfo,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskInput {
    pub fn new(
        selected_databases: Vec<MigratePostgreSqlAzureDbForPostgreSqlSyncDatabaseInput>,
        target_connection_info: PostgreSqlConnectionInfo,
        source_connection_info: PostgreSqlConnectionInfo,
    ) -> Self {
        Self {
            selected_databases,
            target_connection_info,
            source_connection_info,
        }
    }
}
#[doc = "Output for the task that migrates PostgreSQL databases to Azure Database for PostgreSQL for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Result type"]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputUnion {
    DatabaseLevelErrorOutput(MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputDatabaseError),
    DatabaseLevelOutput(MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputDatabaseLevel),
    ErrorOutput(MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputError),
    MigrationLevelOutput(MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputMigrationLevel),
    TableLevelOutput(MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputTableLevel),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputDatabaseError {
    #[serde(flatten)]
    pub migrate_postgre_sql_azure_db_for_postgre_sql_sync_task_output: MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput,
    #[doc = "Error message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "List of error events."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<SyncMigrationDatabaseErrorEvent>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputDatabaseError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub migrate_postgre_sql_azure_db_for_postgre_sql_sync_task_output: MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Enum of the different state of database level online migration."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<SyncDatabaseMigrationReportingState>,
    #[doc = "Number of incoming changes"]
    #[serde(rename = "incomingChanges", default, skip_serializing_if = "Option::is_none")]
    pub incoming_changes: Option<i64>,
    #[doc = "Number of applied changes"]
    #[serde(rename = "appliedChanges", default, skip_serializing_if = "Option::is_none")]
    pub applied_changes: Option<i64>,
    #[doc = "Number of cdc inserts"]
    #[serde(rename = "cdcInsertCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_insert_counter: Option<i64>,
    #[doc = "Number of cdc deletes"]
    #[serde(rename = "cdcDeleteCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_delete_counter: Option<i64>,
    #[doc = "Number of cdc updates"]
    #[serde(rename = "cdcUpdateCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_update_counter: Option<i64>,
    #[doc = "Number of tables completed in full load"]
    #[serde(rename = "fullLoadCompletedTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_completed_tables: Option<i64>,
    #[doc = "Number of tables loading in full load"]
    #[serde(rename = "fullLoadLoadingTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_loading_tables: Option<i64>,
    #[doc = "Number of tables queued in full load"]
    #[serde(rename = "fullLoadQueuedTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_queued_tables: Option<i64>,
    #[doc = "Number of tables errored in full load"]
    #[serde(rename = "fullLoadErroredTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_errored_tables: Option<i64>,
    #[doc = "Indicates if initial load (full load) has been completed"]
    #[serde(rename = "initializationCompleted", default, skip_serializing_if = "Option::is_none")]
    pub initialization_completed: Option<bool>,
    #[doc = "CDC apply latency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputDatabaseLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputError {
    #[serde(flatten)]
    pub migrate_postgre_sql_azure_db_for_postgre_sql_sync_task_output: MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput,
    #[doc = "Exception object for all custom exceptions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportableException>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputMigrationLevel {
    #[serde(flatten)]
    pub migrate_postgre_sql_azure_db_for_postgre_sql_sync_task_output: MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server name"]
    #[serde(rename = "sourceServer", default, skip_serializing_if = "Option::is_none")]
    pub source_server: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server name"]
    #[serde(rename = "targetServer", default, skip_serializing_if = "Option::is_none")]
    pub target_server: Option<String>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputMigrationLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputTableLevel {
    #[serde(flatten)]
    pub migrate_postgre_sql_azure_db_for_postgre_sql_sync_task_output: MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutput,
    #[doc = "Name of the table"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Number of applied inserts"]
    #[serde(rename = "cdcInsertCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_insert_counter: Option<i64>,
    #[doc = "Number of applied updates"]
    #[serde(rename = "cdcUpdateCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_update_counter: Option<i64>,
    #[doc = "Number of applied deletes"]
    #[serde(rename = "cdcDeleteCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_delete_counter: Option<i64>,
    #[doc = "Estimate to finish full load"]
    #[serde(rename = "fullLoadEstFinishTime", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_est_finish_time: Option<time::OffsetDateTime>,
    #[doc = "Full load start time"]
    #[serde(rename = "fullLoadStartedOn", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_started_on: Option<time::OffsetDateTime>,
    #[doc = "Full load end time"]
    #[serde(rename = "fullLoadEndedOn", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_ended_on: Option<time::OffsetDateTime>,
    #[doc = "Number of rows applied in full load"]
    #[serde(rename = "fullLoadTotalRows", default, skip_serializing_if = "Option::is_none")]
    pub full_load_total_rows: Option<i64>,
    #[doc = "Enum of the different state of table level online migration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SyncTableMigrationState>,
    #[doc = "Total number of applied changes"]
    #[serde(rename = "totalChangesApplied", default, skip_serializing_if = "Option::is_none")]
    pub total_changes_applied: Option<i64>,
    #[doc = "Number of data errors occurred"]
    #[serde(rename = "dataErrorsCounter", default, skip_serializing_if = "Option::is_none")]
    pub data_errors_counter: Option<i64>,
    #[doc = "Last modified time on target"]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputTableLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that migrates PostgreSQL databases to Azure Database for PostgreSQL for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigratePostgreSqlAzureDbForPostgreSqlSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that migrates PostgreSQL databases to Azure Database for PostgreSQL for online migrations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigratePostgreSqlAzureDbForPostgreSqlSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<MigratePostgreSqlAzureDbForPostgreSqlSyncTaskOutputUnion>,
}
impl MigratePostgreSqlAzureDbForPostgreSqlSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Database specific information for SQL to Azure SQL DB migration task inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbDatabaseInput {
    #[doc = "Name of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Name of target database. Note: Target database will be truncated before starting migration."]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Whether to set database read only before migration"]
    #[serde(rename = "makeSourceDbReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub make_source_db_read_only: Option<bool>,
    #[doc = "Mapping of source to target tables"]
    #[serde(rename = "tableMap", default, skip_serializing_if = "Option::is_none")]
    pub table_map: Option<serde_json::Value>,
}
impl MigrateSqlServerSqlDbDatabaseInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database specific information for SQL to Azure SQL DB sync migration task inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncDatabaseInput {
    #[doc = "Unique identifier for database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Target database name"]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Schema name to be migrated"]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "Mapping of source to target tables"]
    #[serde(rename = "tableMap", default, skip_serializing_if = "Option::is_none")]
    pub table_map: Option<serde_json::Value>,
    #[doc = "Migration settings which tune the migration behavior"]
    #[serde(rename = "migrationSetting", default, skip_serializing_if = "Option::is_none")]
    pub migration_setting: Option<serde_json::Value>,
    #[doc = "Source settings to tune source endpoint migration behavior"]
    #[serde(rename = "sourceSetting", default, skip_serializing_if = "Option::is_none")]
    pub source_setting: Option<serde_json::Value>,
    #[doc = "Target settings to tune target endpoint migration behavior"]
    #[serde(rename = "targetSetting", default, skip_serializing_if = "Option::is_none")]
    pub target_setting: Option<serde_json::Value>,
}
impl MigrateSqlServerSqlDbSyncDatabaseInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for the task that migrates on-prem SQL Server databases to Azure SQL Database for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbSyncTaskInput {
    #[serde(flatten)]
    pub sql_migration_task_input: SqlMigrationTaskInput,
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateSqlServerSqlDbSyncDatabaseInput>,
    #[doc = "Types of validations to run after the migration"]
    #[serde(rename = "validationOptions", default, skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<MigrationValidationOptions>,
}
impl MigrateSqlServerSqlDbSyncTaskInput {
    pub fn new(sql_migration_task_input: SqlMigrationTaskInput, selected_databases: Vec<MigrateSqlServerSqlDbSyncDatabaseInput>) -> Self {
        Self {
            sql_migration_task_input,
            selected_databases,
            validation_options: None,
        }
    }
}
#[doc = "Output for the task that migrates on-prem SQL Server databases to Azure SQL Database for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Result type"]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}
impl MigrateSqlServerSqlDbSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum MigrateSqlServerSqlDbSyncTaskOutputUnion {
    DatabaseLevelErrorOutput(MigrateSqlServerSqlDbSyncTaskOutputDatabaseError),
    DatabaseLevelOutput(MigrateSqlServerSqlDbSyncTaskOutputDatabaseLevel),
    ErrorOutput(MigrateSqlServerSqlDbSyncTaskOutputError),
    MigrationLevelOutput(MigrateSqlServerSqlDbSyncTaskOutputMigrationLevel),
    TableLevelOutput(MigrateSqlServerSqlDbSyncTaskOutputTableLevel),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncTaskOutputDatabaseError {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_sync_task_output: MigrateSqlServerSqlDbSyncTaskOutput,
    #[doc = "Error message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "List of error events."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<SyncMigrationDatabaseErrorEvent>,
}
impl MigrateSqlServerSqlDbSyncTaskOutputDatabaseError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_sync_task_output: MigrateSqlServerSqlDbSyncTaskOutput,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Enum of the different state of database level online migration."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<SyncDatabaseMigrationReportingState>,
    #[doc = "Number of incoming changes"]
    #[serde(rename = "incomingChanges", default, skip_serializing_if = "Option::is_none")]
    pub incoming_changes: Option<i64>,
    #[doc = "Number of applied changes"]
    #[serde(rename = "appliedChanges", default, skip_serializing_if = "Option::is_none")]
    pub applied_changes: Option<i64>,
    #[doc = "Number of cdc inserts"]
    #[serde(rename = "cdcInsertCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_insert_counter: Option<i64>,
    #[doc = "Number of cdc deletes"]
    #[serde(rename = "cdcDeleteCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_delete_counter: Option<i64>,
    #[doc = "Number of cdc updates"]
    #[serde(rename = "cdcUpdateCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_update_counter: Option<i64>,
    #[doc = "Number of tables completed in full load"]
    #[serde(rename = "fullLoadCompletedTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_completed_tables: Option<i64>,
    #[doc = "Number of tables loading in full load"]
    #[serde(rename = "fullLoadLoadingTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_loading_tables: Option<i64>,
    #[doc = "Number of tables queued in full load"]
    #[serde(rename = "fullLoadQueuedTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_queued_tables: Option<i64>,
    #[doc = "Number of tables errored in full load"]
    #[serde(rename = "fullLoadErroredTables", default, skip_serializing_if = "Option::is_none")]
    pub full_load_errored_tables: Option<i64>,
    #[doc = "Indicates if initial load (full load) has been completed"]
    #[serde(rename = "initializationCompleted", default, skip_serializing_if = "Option::is_none")]
    pub initialization_completed: Option<bool>,
    #[doc = "CDC apply latency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
}
impl MigrateSqlServerSqlDbSyncTaskOutputDatabaseLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncTaskOutputError {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_sync_task_output: MigrateSqlServerSqlDbSyncTaskOutput,
    #[doc = "Exception object for all custom exceptions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportableException>,
}
impl MigrateSqlServerSqlDbSyncTaskOutputError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncTaskOutputMigrationLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_sync_task_output: MigrateSqlServerSqlDbSyncTaskOutput,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server name"]
    #[serde(rename = "sourceServer", default, skip_serializing_if = "Option::is_none")]
    pub source_server: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server name"]
    #[serde(rename = "targetServer", default, skip_serializing_if = "Option::is_none")]
    pub target_server: Option<String>,
    #[doc = "Count of databases"]
    #[serde(rename = "databaseCount", default, skip_serializing_if = "Option::is_none")]
    pub database_count: Option<i64>,
}
impl MigrateSqlServerSqlDbSyncTaskOutputMigrationLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlDbSyncTaskOutputTableLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_sync_task_output: MigrateSqlServerSqlDbSyncTaskOutput,
    #[doc = "Name of the table"]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Number of applied inserts"]
    #[serde(rename = "cdcInsertCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_insert_counter: Option<i64>,
    #[doc = "Number of applied updates"]
    #[serde(rename = "cdcUpdateCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_update_counter: Option<i64>,
    #[doc = "Number of applied deletes"]
    #[serde(rename = "cdcDeleteCounter", default, skip_serializing_if = "Option::is_none")]
    pub cdc_delete_counter: Option<i64>,
    #[doc = "Estimate to finish full load"]
    #[serde(rename = "fullLoadEstFinishTime", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_est_finish_time: Option<time::OffsetDateTime>,
    #[doc = "Full load start time"]
    #[serde(rename = "fullLoadStartedOn", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_started_on: Option<time::OffsetDateTime>,
    #[doc = "Full load end time"]
    #[serde(rename = "fullLoadEndedOn", default, with = "azure_core::date::rfc3339::option")]
    pub full_load_ended_on: Option<time::OffsetDateTime>,
    #[doc = "Number of rows applied in full load"]
    #[serde(rename = "fullLoadTotalRows", default, skip_serializing_if = "Option::is_none")]
    pub full_load_total_rows: Option<i64>,
    #[doc = "Enum of the different state of table level online migration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SyncTableMigrationState>,
    #[doc = "Total number of applied changes"]
    #[serde(rename = "totalChangesApplied", default, skip_serializing_if = "Option::is_none")]
    pub total_changes_applied: Option<i64>,
    #[doc = "Number of data errors occurred"]
    #[serde(rename = "dataErrorsCounter", default, skip_serializing_if = "Option::is_none")]
    pub data_errors_counter: Option<i64>,
    #[doc = "Last modified time on target"]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl MigrateSqlServerSqlDbSyncTaskOutputTableLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the task that migrates on-prem SQL Server databases to Azure SQL Database for online migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that migrates on-prem SQL Server databases to Azure SQL Database for online migrations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateSqlServerSqlDbSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<MigrateSqlServerSqlDbSyncTaskOutputUnion>,
}
impl MigrateSqlServerSqlDbSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for the task that migrates on-prem SQL Server databases to Azure SQL Database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskInput {
    #[serde(flatten)]
    pub sql_migration_task_input: SqlMigrationTaskInput,
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateSqlServerSqlDbDatabaseInput>,
    #[doc = "Types of validations to run after the migration"]
    #[serde(rename = "validationOptions", default, skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<MigrationValidationOptions>,
}
impl MigrateSqlServerSqlDbTaskInput {
    pub fn new(sql_migration_task_input: SqlMigrationTaskInput, selected_databases: Vec<MigrateSqlServerSqlDbDatabaseInput>) -> Self {
        Self {
            sql_migration_task_input,
            selected_databases,
            validation_options: None,
        }
    }
}
#[doc = "Output for the task that migrates on-prem SQL Server databases to Azure SQL Database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Result type"]
    #[serde(rename = "resultType")]
    pub result_type: String,
}
impl MigrateSqlServerSqlDbTaskOutput {
    pub fn new(result_type: String) -> Self {
        Self { id: None, result_type }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum MigrateSqlServerSqlDbTaskOutputUnion {
    DatabaseLevelOutput(MigrateSqlServerSqlDbTaskOutputDatabaseLevel),
    MigrationDatabaseLevelValidationOutput(MigrateSqlServerSqlDbTaskOutputDatabaseLevelValidationResult),
    ErrorOutput(MigrateSqlServerSqlDbTaskOutputError),
    MigrationLevelOutput(MigrateSqlServerSqlDbTaskOutputMigrationLevel),
    TableLevelOutput(MigrateSqlServerSqlDbTaskOutputTableLevel),
    MigrationValidationOutput(MigrateSqlServerSqlDbTaskOutputValidationResult),
}
#[doc = "Database level result for Sql Server to Azure Sql DB migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput,
    #[doc = "Name of the item"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Current stage of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<DatabaseMigrationStage>,
    #[doc = "Status message"]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Migration progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Number of objects"]
    #[serde(rename = "numberOfObjects", default, skip_serializing_if = "Option::is_none")]
    pub number_of_objects: Option<i64>,
    #[doc = "Number of successfully completed objects"]
    #[serde(rename = "numberOfObjectsCompleted", default, skip_serializing_if = "Option::is_none")]
    pub number_of_objects_completed: Option<i64>,
    #[doc = "Number of database/object errors."]
    #[serde(rename = "errorCount", default, skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    #[doc = "Wildcard string prefix to use for querying all errors of the item"]
    #[serde(rename = "errorPrefix", default, skip_serializing_if = "Option::is_none")]
    pub error_prefix: Option<String>,
    #[doc = "Wildcard string prefix to use for querying all sub-tem results of the item"]
    #[serde(rename = "resultPrefix", default, skip_serializing_if = "Option::is_none")]
    pub result_prefix: Option<String>,
    #[doc = "Migration exceptions and warnings."]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
    #[doc = "Summary of object results in the migration"]
    #[serde(rename = "objectSummary", default, skip_serializing_if = "Option::is_none")]
    pub object_summary: Option<String>,
}
impl MigrateSqlServerSqlDbTaskOutputDatabaseLevel {
    pub fn new(migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput) -> Self {
        Self {
            migrate_sql_server_sql_db_task_output,
            database_name: None,
            started_on: None,
            ended_on: None,
            state: None,
            stage: None,
            status_message: None,
            message: None,
            number_of_objects: None,
            number_of_objects_completed: None,
            error_count: None,
            error_prefix: None,
            result_prefix: None,
            exceptions_and_warnings: Vec::new(),
            object_summary: None,
        }
    }
}
#[doc = "Database validation result for Sql Server to Azure Sql DB migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutputDatabaseLevelValidationResult {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput,
    #[doc = "Migration Identifier"]
    #[serde(rename = "migrationId", default, skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<String>,
    #[doc = "Name of the source database"]
    #[serde(rename = "sourceDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub source_database_name: Option<String>,
    #[doc = "Name of the target database"]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Validation start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Validation end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Results for checksum based Data Integrity validation results"]
    #[serde(rename = "dataIntegrityValidationResult", default, skip_serializing_if = "Option::is_none")]
    pub data_integrity_validation_result: Option<DataIntegrityValidationResult>,
    #[doc = "Results for schema comparison between the source and target"]
    #[serde(rename = "schemaValidationResult", default, skip_serializing_if = "Option::is_none")]
    pub schema_validation_result: Option<SchemaComparisonValidationResult>,
    #[doc = "Results for query analysis comparison between the source and target"]
    #[serde(rename = "queryAnalysisValidationResult", default, skip_serializing_if = "Option::is_none")]
    pub query_analysis_validation_result: Option<QueryAnalysisValidationResult>,
    #[doc = "Current status of the validation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ValidationStatus>,
}
impl MigrateSqlServerSqlDbTaskOutputDatabaseLevelValidationResult {
    pub fn new(migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput) -> Self {
        Self {
            migrate_sql_server_sql_db_task_output,
            migration_id: None,
            source_database_name: None,
            target_database_name: None,
            started_on: None,
            ended_on: None,
            data_integrity_validation_result: None,
            schema_validation_result: None,
            query_analysis_validation_result: None,
            status: None,
        }
    }
}
#[doc = "Task errors for Sql Server to Azure Sql DB migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutputError {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput,
    #[doc = "Unique identifier for the exception"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Exception object for all custom exceptions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportableException>,
}
impl MigrateSqlServerSqlDbTaskOutputError {
    pub fn new(migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput) -> Self {
        Self {
            migrate_sql_server_sql_db_task_output,
            id: None,
            error: None,
        }
    }
}
#[doc = "Migration level result for Sql server to Azure Sql DB migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutputMigrationLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Duration of task execution in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[doc = "Current status of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MigrationStatus>,
    #[doc = "Migration status message"]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Migration progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Selected databases as a map from database name to database id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub databases: Option<String>,
    #[doc = "Summary of database results in the migration"]
    #[serde(rename = "databaseSummary", default, skip_serializing_if = "Option::is_none")]
    pub database_summary: Option<String>,
    #[doc = "Migration validation report result, contains the url for downloading the generated report."]
    #[serde(rename = "migrationReport", default, skip_serializing_if = "Option::is_none")]
    pub migration_report: Option<MigrationReportResult>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server brand version"]
    #[serde(rename = "sourceServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_brand_version: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "Migration exceptions and warnings."]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl MigrateSqlServerSqlDbTaskOutputMigrationLevel {
    pub fn new(migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput) -> Self {
        Self {
            migrate_sql_server_sql_db_task_output,
            started_on: None,
            ended_on: None,
            duration_in_seconds: None,
            status: None,
            status_message: None,
            message: None,
            databases: None,
            database_summary: None,
            migration_report: None,
            source_server_version: None,
            source_server_brand_version: None,
            target_server_version: None,
            target_server_brand_version: None,
            exceptions_and_warnings: Vec::new(),
        }
    }
}
#[doc = "Table level result for Sql Server to Azure Sql DB migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutputTableLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput,
    #[doc = "Name of the item"]
    #[serde(rename = "objectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Status message"]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "Number of items"]
    #[serde(rename = "itemsCount", default, skip_serializing_if = "Option::is_none")]
    pub items_count: Option<i64>,
    #[doc = "Number of successfully completed items"]
    #[serde(rename = "itemsCompletedCount", default, skip_serializing_if = "Option::is_none")]
    pub items_completed_count: Option<i64>,
    #[doc = "Wildcard string prefix to use for querying all errors of the item"]
    #[serde(rename = "errorPrefix", default, skip_serializing_if = "Option::is_none")]
    pub error_prefix: Option<String>,
    #[doc = "Wildcard string prefix to use for querying all sub-tem results of the item"]
    #[serde(rename = "resultPrefix", default, skip_serializing_if = "Option::is_none")]
    pub result_prefix: Option<String>,
}
impl MigrateSqlServerSqlDbTaskOutputTableLevel {
    pub fn new(migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput) -> Self {
        Self {
            migrate_sql_server_sql_db_task_output,
            object_name: None,
            started_on: None,
            ended_on: None,
            state: None,
            status_message: None,
            items_count: None,
            items_completed_count: None,
            error_prefix: None,
            result_prefix: None,
        }
    }
}
#[doc = "Validation result for Sql Server to Azure Sql DB migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskOutputValidationResult {
    #[serde(flatten)]
    pub migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput,
    #[doc = "Migration Identifier"]
    #[serde(rename = "migrationId", default, skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<String>,
    #[doc = "Validation summary results for each database"]
    #[serde(rename = "summaryResults", default, skip_serializing_if = "Option::is_none")]
    pub summary_results: Option<serde_json::Value>,
    #[doc = "Current status of the validation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ValidationStatus>,
}
impl MigrateSqlServerSqlDbTaskOutputValidationResult {
    pub fn new(migrate_sql_server_sql_db_task_output: MigrateSqlServerSqlDbTaskOutput) -> Self {
        Self {
            migrate_sql_server_sql_db_task_output,
            migration_id: None,
            summary_results: None,
            status: None,
        }
    }
}
#[doc = "Properties for the task that migrates on-prem SQL Server databases to Azure SQL Database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlDbTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for the task that migrates on-prem SQL Server databases to Azure SQL Database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateSqlServerSqlDbTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<MigrateSqlServerSqlDbTaskOutputUnion>,
}
impl MigrateSqlServerSqlDbTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Database specific information for SQL to Azure SQL DB Managed Instance migration task inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlMiDatabaseInput {
    #[doc = "Name of the database"]
    pub name: String,
    #[doc = "Name of the database at destination"]
    #[serde(rename = "restoreDatabaseName")]
    pub restore_database_name: String,
    #[doc = "File share information with Path, Username, and Password."]
    #[serde(rename = "backupFileShare", default, skip_serializing_if = "Option::is_none")]
    pub backup_file_share: Option<FileShare>,
    #[doc = "The list of backup files to be used in case of existing backups."]
    #[serde(
        rename = "backupFilePaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_file_paths: Vec<String>,
}
impl MigrateSqlServerSqlMiDatabaseInput {
    pub fn new(name: String, restore_database_name: String) -> Self {
        Self {
            name,
            restore_database_name,
            backup_file_share: None,
            backup_file_paths: Vec::new(),
        }
    }
}
#[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance online scenario."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlMiSyncTaskInput {
    #[serde(flatten)]
    pub sql_server_sql_mi_sync_task_input: SqlServerSqlMiSyncTaskInput,
}
impl MigrateSqlServerSqlMiSyncTaskInput {
    pub fn new(sql_server_sql_mi_sync_task_input: SqlServerSqlMiSyncTaskInput) -> Self {
        Self {
            sql_server_sql_mi_sync_task_input,
        }
    }
}
#[doc = "Output for task that migrates SQL Server databases to Azure SQL Database Managed Instance using Log Replay Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiSyncTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Result type"]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}
impl MigrateSqlServerSqlMiSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum MigrateSqlServerSqlMiSyncTaskOutputUnion {
    DatabaseLevelOutput(MigrateSqlServerSqlMiSyncTaskOutputDatabaseLevel),
    ErrorOutput(MigrateSqlServerSqlMiSyncTaskOutputError),
    MigrationLevelOutput(MigrateSqlServerSqlMiSyncTaskOutputMigrationLevel),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiSyncTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_sync_task_output: MigrateSqlServerSqlMiSyncTaskOutput,
    #[doc = "Name of the database"]
    #[serde(rename = "sourceDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub source_database_name: Option<String>,
    #[doc = "Database level migration state."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<DatabaseMigrationState>,
    #[doc = "Database migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Database migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Information of backup set"]
    #[serde(rename = "fullBackupSetInfo", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_set_info: Option<BackupSetInfo>,
    #[doc = "Information of backup set"]
    #[serde(rename = "lastRestoredBackupSetInfo", default, skip_serializing_if = "Option::is_none")]
    pub last_restored_backup_set_info: Option<BackupSetInfo>,
    #[doc = "Backup sets that are currently active (Either being uploaded or getting restored)"]
    #[serde(
        rename = "activeBackupSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub active_backup_sets: Vec<BackupSetInfo>,
    #[doc = "Name of container created in the Azure Storage account where backups are copied to"]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "prefix string to use for querying errors for this database"]
    #[serde(rename = "errorPrefix", default, skip_serializing_if = "Option::is_none")]
    pub error_prefix: Option<String>,
    #[doc = "Whether full backup has been applied to the target database or not"]
    #[serde(rename = "isFullBackupRestored", default, skip_serializing_if = "Option::is_none")]
    pub is_full_backup_restored: Option<bool>,
    #[doc = "Migration exceptions and warnings"]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl MigrateSqlServerSqlMiSyncTaskOutputDatabaseLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiSyncTaskOutputError {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_sync_task_output: MigrateSqlServerSqlMiSyncTaskOutput,
    #[doc = "Unique identifier for the exception"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Exception object for all custom exceptions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportableException>,
}
impl MigrateSqlServerSqlMiSyncTaskOutputError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiSyncTaskOutputMigrationLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_sync_task_output: MigrateSqlServerSqlMiSyncTaskOutput,
    #[doc = "Count of databases"]
    #[serde(rename = "databaseCount", default, skip_serializing_if = "Option::is_none")]
    pub database_count: Option<i64>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Source server name"]
    #[serde(rename = "sourceServerName", default, skip_serializing_if = "Option::is_none")]
    pub source_server_name: Option<String>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server brand version"]
    #[serde(rename = "sourceServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_brand_version: Option<String>,
    #[doc = "Target server name"]
    #[serde(rename = "targetServerName", default, skip_serializing_if = "Option::is_none")]
    pub target_server_name: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "Number of database level errors"]
    #[serde(rename = "databaseErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub database_error_count: Option<i64>,
}
impl MigrateSqlServerSqlMiSyncTaskOutputMigrationLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for task that migrates SQL Server databases to Azure SQL Database Managed Instance sync scenario"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlMiSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance online scenario."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateSqlServerSqlMiSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<MigrateSqlServerSqlMiSyncTaskOutputUnion>,
}
impl MigrateSqlServerSqlMiSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlMiTaskInput {
    #[serde(flatten)]
    pub sql_migration_task_input: SqlMigrationTaskInput,
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateSqlServerSqlMiDatabaseInput>,
    #[doc = "Logins to migrate."]
    #[serde(
        rename = "selectedLogins",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub selected_logins: Vec<String>,
    #[doc = "Agent Jobs to migrate."]
    #[serde(
        rename = "selectedAgentJobs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub selected_agent_jobs: Vec<String>,
    #[doc = "File share information with Path, Username, and Password."]
    #[serde(rename = "backupFileShare", default, skip_serializing_if = "Option::is_none")]
    pub backup_file_share: Option<FileShare>,
    #[doc = "Blob container storage information."]
    #[serde(rename = "backupBlobShare")]
    pub backup_blob_share: BlobShare,
    #[doc = "An enumeration of backup modes"]
    #[serde(rename = "backupMode", default, skip_serializing_if = "Option::is_none")]
    pub backup_mode: Option<BackupMode>,
}
impl MigrateSqlServerSqlMiTaskInput {
    pub fn new(
        sql_migration_task_input: SqlMigrationTaskInput,
        selected_databases: Vec<MigrateSqlServerSqlMiDatabaseInput>,
        backup_blob_share: BlobShare,
    ) -> Self {
        Self {
            sql_migration_task_input,
            selected_databases,
            selected_logins: Vec::new(),
            selected_agent_jobs: Vec::new(),
            backup_file_share: None,
            backup_blob_share,
            backup_mode: None,
        }
    }
}
#[doc = "Output for task that migrates SQL Server databases to Azure SQL Database Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Result type"]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}
impl MigrateSqlServerSqlMiTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType")]
pub enum MigrateSqlServerSqlMiTaskOutputUnion {
    AgentJobLevelOutput(MigrateSqlServerSqlMiTaskOutputAgentJobLevel),
    DatabaseLevelOutput(MigrateSqlServerSqlMiTaskOutputDatabaseLevel),
    ErrorOutput(MigrateSqlServerSqlMiTaskOutputError),
    LoginLevelOutput(MigrateSqlServerSqlMiTaskOutputLoginLevel),
    MigrationLevelOutput(MigrateSqlServerSqlMiTaskOutputMigrationLevel),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiTaskOutputAgentJobLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_task_output: MigrateSqlServerSqlMiTaskOutput,
    #[doc = "Agent Job name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The state of the original Agent Job."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Migration progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Migration errors and warnings per job"]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl MigrateSqlServerSqlMiTaskOutputAgentJobLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiTaskOutputDatabaseLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_task_output: MigrateSqlServerSqlMiTaskOutput,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Size of the database in megabytes"]
    #[serde(rename = "sizeMB", default, skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<f64>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Current stage of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<DatabaseMigrationStage>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Migration progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Migration exceptions and warnings"]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl MigrateSqlServerSqlMiTaskOutputDatabaseLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiTaskOutputError {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_task_output: MigrateSqlServerSqlMiTaskOutput,
    #[doc = "Unique identifier for the exception"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Exception object for all custom exceptions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportableException>,
}
impl MigrateSqlServerSqlMiTaskOutputError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiTaskOutputLoginLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_task_output: MigrateSqlServerSqlMiTaskOutput,
    #[doc = "Login name."]
    #[serde(rename = "loginName", default, skip_serializing_if = "Option::is_none")]
    pub login_name: Option<String>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Enum of the different stage of login migration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<LoginMigrationStage>,
    #[doc = "Login migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Login migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Login migration progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Login migration errors and warnings per login"]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl MigrateSqlServerSqlMiTaskOutputLoginLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlMiTaskOutputMigrationLevel {
    #[serde(flatten)]
    pub migrate_sql_server_sql_mi_task_output: MigrateSqlServerSqlMiTaskOutput,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Current status of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MigrationStatus>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Selected agent jobs as a map from name to id"]
    #[serde(rename = "agentJobs", default, skip_serializing_if = "Option::is_none")]
    pub agent_jobs: Option<String>,
    #[doc = "Selected logins as a map from name to id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logins: Option<String>,
    #[doc = "Migration progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Map of server role migration results."]
    #[serde(rename = "serverRoleResults", default, skip_serializing_if = "Option::is_none")]
    pub server_role_results: Option<String>,
    #[doc = "List of orphaned users."]
    #[serde(
        rename = "orphanedUsersInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orphaned_users_info: Vec<OrphanedUserInfo>,
    #[doc = "Selected databases as a map from database name to database id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub databases: Option<String>,
    #[doc = "Source server version"]
    #[serde(rename = "sourceServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_version: Option<String>,
    #[doc = "Source server brand version"]
    #[serde(rename = "sourceServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub source_server_brand_version: Option<String>,
    #[doc = "Target server version"]
    #[serde(rename = "targetServerVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_version: Option<String>,
    #[doc = "Target server brand version"]
    #[serde(rename = "targetServerBrandVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_server_brand_version: Option<String>,
    #[doc = "Migration exceptions and warnings."]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl MigrateSqlServerSqlMiTaskOutputMigrationLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for task that migrates SQL Server databases to Azure SQL Database Managed Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSqlServerSqlMiTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateSqlServerSqlMiTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<MigrateSqlServerSqlMiTaskOutputUnion>,
}
impl MigrateSqlServerSqlMiTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Database specific information for SQL to SQL migration task inputs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSqlServerSqlServerDatabaseInput {
    #[doc = "Name of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Name of the database at destination"]
    #[serde(rename = "restoreDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub restore_database_name: Option<String>,
    #[doc = "The backup and restore folder"]
    #[serde(rename = "backupAndRestoreFolder", default, skip_serializing_if = "Option::is_none")]
    pub backup_and_restore_folder: Option<String>,
    #[doc = "The list of database files"]
    #[serde(
        rename = "databaseFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub database_files: Vec<DatabaseFileInput>,
}
impl MigrateSqlServerSqlServerDatabaseInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for command that completes sync migration for a database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSyncCompleteCommandInput {
    #[doc = "Name of database"]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "Time stamp to complete"]
    #[serde(rename = "commitTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub commit_time_stamp: Option<time::OffsetDateTime>,
}
impl MigrateSyncCompleteCommandInput {
    pub fn new(database_name: String) -> Self {
        Self {
            database_name,
            commit_time_stamp: None,
        }
    }
}
#[doc = "Output for command that completes sync migration for a database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrateSyncCompleteCommandOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of errors that happened during the command execution"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ReportableException>,
}
impl MigrateSyncCompleteCommandOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the command that completes sync migration for a database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateSyncCompleteCommandProperties {
    #[serde(flatten)]
    pub command_properties: CommandProperties,
    #[doc = "Input for command that completes sync migration for a database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<MigrateSyncCompleteCommandInput>,
    #[doc = "Output for command that completes sync migration for a database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<MigrateSyncCompleteCommandOutput>,
}
impl MigrateSyncCompleteCommandProperties {
    pub fn new(command_properties: CommandProperties) -> Self {
        Self {
            command_properties,
            input: None,
            output: None,
        }
    }
}
#[doc = "Information about migration eligibility of a server object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationEligibilityInfo {
    #[doc = "Whether object is eligible for migration or not."]
    #[serde(rename = "isEligibleForMigration", default, skip_serializing_if = "Option::is_none")]
    pub is_eligible_for_migration: Option<bool>,
    #[doc = "Information about eligibility failure for the server object."]
    #[serde(
        rename = "validationMessages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_messages: Vec<String>,
}
impl MigrationEligibilityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration validation report result, contains the url for downloading the generated report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationReportResult {
    #[doc = "Migration validation result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The url of the report."]
    #[serde(rename = "reportUrl", default, skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
}
impl MigrationReportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current state of migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationState")]
pub enum MigrationState {
    None,
    InProgress,
    Failed,
    Warning,
    Completed,
    Skipped,
    Stopped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MigrationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MigrationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MigrationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("MigrationState", 0u32, "None"),
            Self::InProgress => serializer.serialize_unit_variant("MigrationState", 1u32, "InProgress"),
            Self::Failed => serializer.serialize_unit_variant("MigrationState", 2u32, "Failed"),
            Self::Warning => serializer.serialize_unit_variant("MigrationState", 3u32, "Warning"),
            Self::Completed => serializer.serialize_unit_variant("MigrationState", 4u32, "Completed"),
            Self::Skipped => serializer.serialize_unit_variant("MigrationState", 5u32, "Skipped"),
            Self::Stopped => serializer.serialize_unit_variant("MigrationState", 6u32, "Stopped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Current status of migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationStatus")]
pub enum MigrationStatus {
    Default,
    Connecting,
    SourceAndTargetSelected,
    SelectLogins,
    Configured,
    Running,
    Error,
    Stopped,
    Completed,
    CompletedWithWarnings,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MigrationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MigrationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MigrationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("MigrationStatus", 0u32, "Default"),
            Self::Connecting => serializer.serialize_unit_variant("MigrationStatus", 1u32, "Connecting"),
            Self::SourceAndTargetSelected => serializer.serialize_unit_variant("MigrationStatus", 2u32, "SourceAndTargetSelected"),
            Self::SelectLogins => serializer.serialize_unit_variant("MigrationStatus", 3u32, "SelectLogins"),
            Self::Configured => serializer.serialize_unit_variant("MigrationStatus", 4u32, "Configured"),
            Self::Running => serializer.serialize_unit_variant("MigrationStatus", 5u32, "Running"),
            Self::Error => serializer.serialize_unit_variant("MigrationStatus", 6u32, "Error"),
            Self::Stopped => serializer.serialize_unit_variant("MigrationStatus", 7u32, "Stopped"),
            Self::Completed => serializer.serialize_unit_variant("MigrationStatus", 8u32, "Completed"),
            Self::CompletedWithWarnings => serializer.serialize_unit_variant("MigrationStatus", 9u32, "CompletedWithWarnings"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Metadata for tables selected in migration project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationTableMetadata {
    #[doc = "Source table name"]
    #[serde(rename = "sourceTableName", default, skip_serializing_if = "Option::is_none")]
    pub source_table_name: Option<String>,
    #[doc = "Target table name"]
    #[serde(rename = "targetTableName", default, skip_serializing_if = "Option::is_none")]
    pub target_table_name: Option<String>,
}
impl MigrationTableMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration Validation Database level summary result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationValidationDatabaseSummaryResult {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Migration Identifier"]
    #[serde(rename = "migrationId", default, skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<String>,
    #[doc = "Name of the source database"]
    #[serde(rename = "sourceDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub source_database_name: Option<String>,
    #[doc = "Name of the target database"]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Validation start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Validation end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Current status of the validation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ValidationStatus>,
}
impl MigrationValidationDatabaseSummaryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Types of validations to run after the migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationValidationOptions {
    #[doc = "Allows to compare the schema information between source and target."]
    #[serde(rename = "enableSchemaValidation", default, skip_serializing_if = "Option::is_none")]
    pub enable_schema_validation: Option<bool>,
    #[doc = "Allows to perform a checksum based data integrity validation between source and target for the selected database / tables ."]
    #[serde(rename = "enableDataIntegrityValidation", default, skip_serializing_if = "Option::is_none")]
    pub enable_data_integrity_validation: Option<bool>,
    #[doc = "Allows to perform a quick and intelligent query analysis by retrieving queries from the source database and executes them in the target. The result will have execution statistics for executions in source and target databases for the extracted queries."]
    #[serde(rename = "enableQueryAnalysisValidation", default, skip_serializing_if = "Option::is_none")]
    pub enable_query_analysis_validation: Option<bool>,
}
impl MigrationValidationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information for connecting to MySQL server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MySqlConnectionInfo {
    #[serde(flatten)]
    pub connection_info: ConnectionInfo,
    #[doc = "Name of the server"]
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[doc = "Port for Server"]
    pub port: i64,
}
impl MySqlConnectionInfo {
    pub fn new(connection_info: ConnectionInfo, server_name: String, port: i64) -> Self {
        Self {
            connection_info,
            server_name,
            port,
        }
    }
}
#[doc = "An enumeration of possible target types when migrating from MySQL"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MySqlTargetPlatform")]
pub enum MySqlTargetPlatform {
    SqlServer,
    #[serde(rename = "AzureDbForMySQL")]
    AzureDbForMySql,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MySqlTargetPlatform {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MySqlTargetPlatform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MySqlTargetPlatform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SqlServer => serializer.serialize_unit_variant("MySqlTargetPlatform", 0u32, "SqlServer"),
            Self::AzureDbForMySql => serializer.serialize_unit_variant("MySqlTargetPlatform", 1u32, "AzureDbForMySQL"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A resource type and proposed name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityRequest {
    #[doc = "The proposed resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type chain (e.g. virtualMachines/extensions)"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether a proposed resource name is available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityResponse {
    #[doc = "If true, the name is valid and available. If false, 'reason' describes why not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the name is not available, if nameAvailable is false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<name_availability_response::Reason>,
    #[doc = "The localized reason why the name is not available, if nameAvailable is false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod name_availability_response {
    use super::*;
    #[doc = "The reason why the name is not available, if nameAvailable is false"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        AlreadyExists,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 0u32, "AlreadyExists"),
                Self::Invalid => serializer.serialize_unit_variant("Reason", 1u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines metadata for table to be migrated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NonSqlDataMigrationTable {
    #[doc = "Source table name"]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}
impl NonSqlDataMigrationTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object used to report the data migration results of a table"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NonSqlDataMigrationTableResult {
    #[doc = "Result code of the data migration"]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<non_sql_data_migration_table_result::ResultCode>,
    #[doc = "Name of the source table"]
    #[serde(rename = "sourceName", default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[doc = "Name of the target table"]
    #[serde(rename = "targetName", default, skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    #[doc = "Number of rows in the source table"]
    #[serde(rename = "sourceRowCount", default, skip_serializing_if = "Option::is_none")]
    pub source_row_count: Option<i64>,
    #[doc = "Number of rows in the target table"]
    #[serde(rename = "targetRowCount", default, skip_serializing_if = "Option::is_none")]
    pub target_row_count: Option<i64>,
    #[doc = "Time taken to migrate the data"]
    #[serde(rename = "elapsedTimeInMiliseconds", default, skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_miliseconds: Option<f64>,
    #[doc = "List of errors, if any, during migration"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<DataMigrationError>,
}
impl NonSqlDataMigrationTableResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod non_sql_data_migration_table_result {
    use super::*;
    #[doc = "Result code of the data migration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResultCode")]
    pub enum ResultCode {
        Initial,
        Completed,
        ObjectNotExistsInSource,
        ObjectNotExistsInTarget,
        TargetObjectIsInaccessible,
        FatalError,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResultCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResultCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResultCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Initial => serializer.serialize_unit_variant("ResultCode", 0u32, "Initial"),
                Self::Completed => serializer.serialize_unit_variant("ResultCode", 1u32, "Completed"),
                Self::ObjectNotExistsInSource => serializer.serialize_unit_variant("ResultCode", 2u32, "ObjectNotExistsInSource"),
                Self::ObjectNotExistsInTarget => serializer.serialize_unit_variant("ResultCode", 3u32, "ObjectNotExistsInTarget"),
                Self::TargetObjectIsInaccessible => serializer.serialize_unit_variant("ResultCode", 4u32, "TargetObjectIsInaccessible"),
                Self::FatalError => serializer.serialize_unit_variant("ResultCode", 5u32, "FatalError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for non sql migration task input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NonSqlMigrationTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
    #[doc = "Target database name"]
    #[serde(rename = "targetDatabaseName")]
    pub target_database_name: String,
    #[doc = "Name of the migration project"]
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[doc = "A URL that points to the drop location to access project artifacts"]
    #[serde(rename = "projectLocation")]
    pub project_location: String,
    #[doc = "Metadata of the tables selected for migration"]
    #[serde(rename = "selectedTables")]
    pub selected_tables: Vec<NonSqlDataMigrationTable>,
}
impl NonSqlMigrationTaskInput {
    pub fn new(
        target_connection_info: SqlConnectionInfo,
        target_database_name: String,
        project_name: String,
        project_location: String,
        selected_tables: Vec<NonSqlDataMigrationTable>,
    ) -> Self {
        Self {
            target_connection_info,
            target_database_name,
            project_name,
            project_location,
            selected_tables,
        }
    }
}
#[doc = "Base class for non sql migration task output"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NonSqlMigrationTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Migration start time"]
    #[serde(rename = "startedOn", default, with = "azure_core::date::rfc3339::option")]
    pub started_on: Option<time::OffsetDateTime>,
    #[doc = "Migration end time"]
    #[serde(rename = "endedOn", default, with = "azure_core::date::rfc3339::option")]
    pub ended_on: Option<time::OffsetDateTime>,
    #[doc = "Current status of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MigrationStatus>,
    #[doc = "Results of the migration. The key contains the table name and the value the table result object"]
    #[serde(rename = "dataMigrationTableResults", default, skip_serializing_if = "Option::is_none")]
    pub data_migration_table_results: Option<String>,
    #[doc = "Message about the progress of the migration"]
    #[serde(rename = "progressMessage", default, skip_serializing_if = "Option::is_none")]
    pub progress_message: Option<String>,
    #[doc = "Name of source server"]
    #[serde(rename = "sourceServerName", default, skip_serializing_if = "Option::is_none")]
    pub source_server_name: Option<String>,
    #[doc = "Name of target server"]
    #[serde(rename = "targetServerName", default, skip_serializing_if = "Option::is_none")]
    pub target_server_name: Option<String>,
}
impl NonSqlMigrationTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error information in OData format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ODataError {
    #[doc = "The machine-readable description of the error, such as 'InvalidRequest' or 'InternalServerError'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The human-readable description of the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Inner errors that caused this error"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ODataError>,
}
impl ODataError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of orphaned users on the SQL server database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrphanedUserInfo {
    #[doc = "Name of the orphaned user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Parent database of the user"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl OrphanedUserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information for connecting to PostgreSQL server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgreSqlConnectionInfo {
    #[serde(flatten)]
    pub connection_info: ConnectionInfo,
    #[doc = "Name of the server"]
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[doc = "Name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Port for Server"]
    pub port: i64,
}
impl PostgreSqlConnectionInfo {
    pub fn new(connection_info: ConnectionInfo, server_name: String, port: i64) -> Self {
        Self {
            connection_info,
            server_name,
            database_name: None,
            port,
        }
    }
}
#[doc = "A project resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Project-specific properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
}
impl Project {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "OData page of project resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectList {
    #[doc = "List of projects"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Project>,
    #[doc = "URL to load the next page of projects"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProjectList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project-specific properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProperties {
    #[doc = "Source platform of the project"]
    #[serde(rename = "sourcePlatform")]
    pub source_platform: ProjectSourcePlatform,
    #[doc = "Target platform of the project"]
    #[serde(rename = "targetPlatform")]
    pub target_platform: ProjectTargetPlatform,
    #[doc = "UTC Date and time when project was created"]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Defines the connection properties of a server"]
    #[serde(rename = "sourceConnectionInfo", default, skip_serializing_if = "Option::is_none")]
    pub source_connection_info: Option<ConnectionInfoUnion>,
    #[doc = "Defines the connection properties of a server"]
    #[serde(rename = "targetConnectionInfo", default, skip_serializing_if = "Option::is_none")]
    pub target_connection_info: Option<ConnectionInfoUnion>,
    #[doc = "List of DatabaseInfo"]
    #[serde(
        rename = "databasesInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases_info: Vec<DatabaseInfo>,
    #[doc = "The project's provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<project_properties::ProvisioningState>,
}
impl ProjectProperties {
    pub fn new(source_platform: ProjectSourcePlatform, target_platform: ProjectTargetPlatform) -> Self {
        Self {
            source_platform,
            target_platform,
            creation_time: None,
            source_connection_info: None,
            target_connection_info: None,
            databases_info: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod project_properties {
    use super::*;
    #[doc = "The project's provisioning state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Source platform of the project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProjectSourcePlatform")]
pub enum ProjectSourcePlatform {
    #[serde(rename = "SQL")]
    Sql,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProjectSourcePlatform {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProjectSourcePlatform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProjectSourcePlatform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sql => serializer.serialize_unit_variant("ProjectSourcePlatform", 0u32, "SQL"),
            Self::Unknown => serializer.serialize_unit_variant("ProjectSourcePlatform", 1u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Target platform of the project"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProjectTargetPlatform")]
pub enum ProjectTargetPlatform {
    #[serde(rename = "SQLDB")]
    Sqldb,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProjectTargetPlatform {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProjectTargetPlatform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProjectTargetPlatform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sqldb => serializer.serialize_unit_variant("ProjectTargetPlatform", 0u32, "SQLDB"),
            Self::Unknown => serializer.serialize_unit_variant("ProjectTargetPlatform", 1u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A task resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectTask {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "HTTP strong entity tag value. This is ignored if submitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Base class for all types of DMS task properties. If task is not supported by current client, this object is returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectTaskPropertiesUnion>,
}
impl ProjectTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for all types of DMS task properties. If task is not supported by current client, this object is returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskProperties {
    #[doc = "Task type."]
    #[serde(rename = "taskType")]
    pub task_type: String,
    #[doc = "Array of errors. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ODataError>,
    #[doc = "The state of the task. This is ignored if submitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<project_task_properties::State>,
    #[doc = "Array of command properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub commands: Vec<CommandPropertiesUnion>,
}
impl ProjectTaskProperties {
    pub fn new(task_type: String) -> Self {
        Self {
            task_type,
            errors: Vec::new(),
            state: None,
            commands: Vec::new(),
        }
    }
}
pub mod project_task_properties {
    use super::*;
    #[doc = "The state of the task. This is ignored if submitted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unknown,
        Queued,
        Running,
        Canceled,
        Succeeded,
        Failed,
        FailedInputValidation,
        Faulted,
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
                Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
                Self::Queued => serializer.serialize_unit_variant("State", 1u32, "Queued"),
                Self::Running => serializer.serialize_unit_variant("State", 2u32, "Running"),
                Self::Canceled => serializer.serialize_unit_variant("State", 3u32, "Canceled"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 4u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 5u32, "Failed"),
                Self::FailedInputValidation => serializer.serialize_unit_variant("State", 6u32, "FailedInputValidation"),
                Self::Faulted => serializer.serialize_unit_variant("State", 7u32, "Faulted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "taskType")]
pub enum ProjectTaskPropertiesUnion {
    #[serde(rename = "ConnectToSource.MySql")]
    ConnectToSourceMySql(ConnectToSourceMySqlTaskProperties),
    #[serde(rename = "ConnectToSource.PostgreSql.Sync")]
    ConnectToSourcePostgreSqlSync(ConnectToSourcePostgreSqlSyncTaskProperties),
    #[serde(rename = "ConnectToSource.SqlServer.Sync")]
    ConnectToSourceSqlServerSync(ConnectToSourceSqlServerSyncTaskProperties),
    #[serde(rename = "ConnectToSource.SqlServer")]
    ConnectToSourceSqlServer(ConnectToSourceSqlServerTaskProperties),
    #[serde(rename = "ConnectToTarget.AzureDbForMySql")]
    ConnectToTargetAzureDbForMySql(ConnectToTargetAzureDbForMySqlTaskProperties),
    #[serde(rename = "ConnectToTarget.AzureDbForPostgreSql.Sync")]
    ConnectToTargetAzureDbForPostgreSqlSync(ConnectToTargetAzureDbForPostgreSqlSyncTaskProperties),
    #[serde(rename = "ConnectToTarget.SqlDb")]
    ConnectToTargetSqlDb(ConnectToTargetSqlDbTaskProperties),
    #[serde(rename = "ConnectToTarget.AzureSqlDbMI.Sync.LRS")]
    ConnectToTargetAzureSqlDbMiSyncLrs(ConnectToTargetSqlMiSyncTaskProperties),
    #[serde(rename = "ConnectToTarget.AzureSqlDbMI")]
    ConnectToTargetAzureSqlDbMi(ConnectToTargetSqlMiTaskProperties),
    #[serde(rename = "ConnectToTarget.SqlDb.Sync")]
    ConnectToTargetSqlDbSync(ConnectToTargetSqlSqlDbSyncTaskProperties),
    #[serde(rename = "GetTDECertificates.Sql")]
    GetTdeCertificatesSql(GetTdeCertificatesSqlTaskProperties),
    #[serde(rename = "GetUserTables.AzureSqlDb.Sync")]
    GetUserTablesAzureSqlDbSync(GetUserTablesSqlSyncTaskProperties),
    #[serde(rename = "GetUserTables.Sql")]
    GetUserTablesSql(GetUserTablesSqlTaskProperties),
    #[serde(rename = "Migrate.MySql.AzureDbForMySql.Sync")]
    MigrateMySqlAzureDbForMySqlSync(MigrateMySqlAzureDbForMySqlSyncTaskProperties),
    #[serde(rename = "Migrate.PostgreSql.AzureDbForPostgreSql.Sync")]
    MigratePostgreSqlAzureDbForPostgreSqlSync(MigratePostgreSqlAzureDbForPostgreSqlSyncTaskProperties),
    #[serde(rename = "Migrate.SqlServer.AzureSqlDb.Sync")]
    MigrateSqlServerAzureSqlDbSync(MigrateSqlServerSqlDbSyncTaskProperties),
    #[serde(rename = "Migrate.SqlServer.SqlDb")]
    MigrateSqlServerSqlDb(MigrateSqlServerSqlDbTaskProperties),
    #[serde(rename = "Migrate.SqlServer.AzureSqlDbMI.Sync.LRS")]
    MigrateSqlServerAzureSqlDbMiSyncLrs(MigrateSqlServerSqlMiSyncTaskProperties),
    #[serde(rename = "Migrate.SqlServer.AzureSqlDbMI")]
    MigrateSqlServerAzureSqlDbMi(MigrateSqlServerSqlMiTaskProperties),
    #[serde(rename = "ValidateMigrationInput.SqlServer.SqlDb.Sync")]
    ValidateMigrationInputSqlServerSqlDbSync(ValidateMigrationInputSqlServerSqlDbSyncTaskProperties),
    #[serde(rename = "ValidateMigrationInput.SqlServer.AzureSqlDbMI.Sync.LRS")]
    ValidateMigrationInputSqlServerAzureSqlDbMiSyncLrs(ValidateMigrationInputSqlServerSqlMiSyncTaskProperties),
    #[serde(rename = "ValidateMigrationInput.SqlServer.AzureSqlDbMI")]
    ValidateMigrationInputSqlServerAzureSqlDbMi(ValidateMigrationInputSqlServerSqlMiTaskProperties),
}
#[doc = "Results for query analysis comparison between the source and target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryAnalysisValidationResult {
    #[doc = "Describes query analysis results for execution in source and target"]
    #[serde(rename = "queryResults", default, skip_serializing_if = "Option::is_none")]
    pub query_results: Option<QueryExecutionResult>,
    #[doc = "Description about the errors happen while performing migration validation"]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<ValidationError>,
}
impl QueryAnalysisValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes query analysis results for execution in source and target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryExecutionResult {
    #[doc = "Query text retrieved from the source server"]
    #[serde(rename = "queryText", default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    #[doc = "Total no. of statements in the batch"]
    #[serde(rename = "statementsInBatch", default, skip_serializing_if = "Option::is_none")]
    pub statements_in_batch: Option<i64>,
    #[doc = "Description about the errors happen while performing migration validation"]
    #[serde(rename = "sourceResult", default, skip_serializing_if = "Option::is_none")]
    pub source_result: Option<ExecutionStatistics>,
    #[doc = "Description about the errors happen while performing migration validation"]
    #[serde(rename = "targetResult", default, skip_serializing_if = "Option::is_none")]
    pub target_result: Option<ExecutionStatistics>,
}
impl QueryExecutionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a quota for or usage details about a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Quota {
    #[doc = "The current value of the quota. If null or missing, the current value cannot be determined in the context of the request."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The resource ID of the quota object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The maximum value of the quota. If null or missing, the quota has no maximum, in which case it merely tracks usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The name of the quota"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<quota::Name>,
    #[doc = "The unit for the quota, such as Count, Bytes, BytesPerSecond, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl Quota {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod quota {
    use super::*;
    #[doc = "The name of the quota"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Name {
        #[doc = "The localized name of the quota"]
        #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
        #[doc = "The unlocalized name (or ID) of the quota"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl Name {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "OData page of quota objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaList {
    #[doc = "List of quotas"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Quota>,
    #[doc = "URL to load the next page of quotas, or null or missing if this is the last page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for QuotaList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl QuotaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exception object for all custom exceptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportableException {
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Actionable steps for this exception"]
    #[serde(rename = "actionableMessage", default, skip_serializing_if = "Option::is_none")]
    pub actionable_message: Option<String>,
    #[doc = "The path to the file where exception occurred"]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "The line number where exception occurred"]
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<String>,
    #[doc = "Coded numerical value that is assigned to a specific exception"]
    #[serde(rename = "hResult", default, skip_serializing_if = "Option::is_none")]
    pub h_result: Option<i64>,
    #[doc = "Stack trace"]
    #[serde(rename = "stackTrace", default, skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
}
impl ReportableException {
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
#[doc = "Describes an available DMS SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of DMS in a scale set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The Size of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The Family of this particular SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The Kind of resources that are supported in this SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Describes scaling information of a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<ResourceSkuCapacity>,
    #[doc = "The set of locations that the SKU is available."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "The api versions that support this SKU."]
    #[serde(
        rename = "apiVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub api_versions: Vec<String>,
    #[doc = "Metadata for retrieving price info."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub costs: Vec<ResourceSkuCosts>,
    #[doc = "A name value pair to describe the capability."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[doc = "The restrictions because of which SKU cannot be used. This is empty if there are no restrictions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes The SKU capabilities object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapabilities {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ResourceSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[doc = "The maximum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[doc = "The scale type applicable to the SKU."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<resource_sku_capacity::ScaleType>,
}
impl ResourceSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_capacity {
    use super::*;
    #[doc = "The scale type applicable to the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
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
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 0u32, "Automatic"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::None => serializer.serialize_unit_variant("ScaleType", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes metadata for retrieving price info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCosts {
    #[doc = "Used for querying price from commerce."]
    #[serde(rename = "meterID", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The multiplier is needed to extend the base metered cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "An invariant to show the extended unit."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl ResourceSkuCosts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictions {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
    #[doc = "The reason code for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
impl ResourceSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_restrictions {
    use super::*;
    #[doc = "The type of restrictions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "location")]
        Location,
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
                Self::Location => serializer.serialize_unit_variant("Type", 0u32, "location"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reason code for restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 0u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The DMS List SKUs operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkusResult {
    #[doc = "The list of SKUs available for the subscription."]
    pub value: Vec<ResourceSku>,
    #[doc = "The uri to fetch the next page of DMS SKUs. Call ListNext() with this to fetch the next page of DMS SKUs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceSkusResult {
    pub fn new(value: Vec<ResourceSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Results for schema comparison between the source and target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaComparisonValidationResult {
    #[doc = "Description about the errors happen while performing migration validation"]
    #[serde(rename = "schemaDifferences", default, skip_serializing_if = "Option::is_none")]
    pub schema_differences: Option<SchemaComparisonValidationResultType>,
    #[doc = "Description about the errors happen while performing migration validation"]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<ValidationError>,
    #[doc = "Count of source database objects"]
    #[serde(rename = "sourceDatabaseObjectCount", default, skip_serializing_if = "Option::is_none")]
    pub source_database_object_count: Option<serde_json::Value>,
    #[doc = "Count of target database objects"]
    #[serde(rename = "targetDatabaseObjectCount", default, skip_serializing_if = "Option::is_none")]
    pub target_database_object_count: Option<serde_json::Value>,
}
impl SchemaComparisonValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description about the errors happen while performing migration validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaComparisonValidationResultType {
    #[doc = "Name of the object that has the difference"]
    #[serde(rename = "objectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "An enumeration of type of objects"]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<DatabaseObjectType>,
    #[doc = "Type of the actual difference for the compared object, while performing schema comparison"]
    #[serde(rename = "updateAction", default, skip_serializing_if = "Option::is_none")]
    pub update_action: Option<UpdateActionType>,
}
impl SchemaComparisonValidationResultType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current stage of schema migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SchemaMigrationStage")]
pub enum SchemaMigrationStage {
    NotStarted,
    ValidatingInputs,
    CollectingObjects,
    DownloadingScript,
    GeneratingScript,
    UploadingScript,
    DeployingSchema,
    Completed,
    CompletedWithWarnings,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SchemaMigrationStage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SchemaMigrationStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SchemaMigrationStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("SchemaMigrationStage", 0u32, "NotStarted"),
            Self::ValidatingInputs => serializer.serialize_unit_variant("SchemaMigrationStage", 1u32, "ValidatingInputs"),
            Self::CollectingObjects => serializer.serialize_unit_variant("SchemaMigrationStage", 2u32, "CollectingObjects"),
            Self::DownloadingScript => serializer.serialize_unit_variant("SchemaMigrationStage", 3u32, "DownloadingScript"),
            Self::GeneratingScript => serializer.serialize_unit_variant("SchemaMigrationStage", 4u32, "GeneratingScript"),
            Self::UploadingScript => serializer.serialize_unit_variant("SchemaMigrationStage", 5u32, "UploadingScript"),
            Self::DeployingSchema => serializer.serialize_unit_variant("SchemaMigrationStage", 6u32, "DeployingSchema"),
            Self::Completed => serializer.serialize_unit_variant("SchemaMigrationStage", 7u32, "Completed"),
            Self::CompletedWithWarnings => serializer.serialize_unit_variant("SchemaMigrationStage", 8u32, "CompletedWithWarnings"),
            Self::Failed => serializer.serialize_unit_variant("SchemaMigrationStage", 9u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Info for certificate to be exported for TDE enabled databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectedCertificateInput {
    #[doc = "Name of certificate to be exported."]
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    #[doc = "Password to use for encrypting the exported certificate."]
    pub password: String,
}
impl SelectedCertificateInput {
    pub fn new(certificate_name: String, password: String) -> Self {
        Self {
            certificate_name,
            password,
        }
    }
}
#[doc = "Permission group for validations. These groups will run a set of permissions for validating user activity. Select the permission group for the activity that you are performing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerLevelPermissionsGroup {
    Default,
    #[serde(rename = "MigrationFromSqlServerToAzureDB")]
    MigrationFromSqlServerToAzureDb,
    #[serde(rename = "MigrationFromSqlServerToAzureMI")]
    MigrationFromSqlServerToAzureMi,
    #[serde(rename = "MigrationFromMySQLToAzureDBForMySQL")]
    MigrationFromMySqlToAzureDbForMySql,
}
#[doc = "Server properties for Oracle, MySQL type source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[doc = "Name of the server platform"]
    #[serde(rename = "serverPlatform", default, skip_serializing_if = "Option::is_none")]
    pub server_platform: Option<String>,
    #[doc = "Name of the server"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Version of the database server"]
    #[serde(rename = "serverVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    #[doc = "Edition of the database server"]
    #[serde(rename = "serverEdition", default, skip_serializing_if = "Option::is_none")]
    pub server_edition: Option<String>,
    #[doc = "Version of the operating system"]
    #[serde(rename = "serverOperatingSystemVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_operating_system_version: Option<String>,
    #[doc = "Number of databases in the server"]
    #[serde(rename = "serverDatabaseCount", default, skip_serializing_if = "Option::is_none")]
    pub server_database_count: Option<i64>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of an action supported by the Database Migration Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceOperation {
    #[doc = "The fully qualified action name, e.g. Microsoft.DataMigration/services/read"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<service_operation::Display>,
}
impl ServiceOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_operation {
    use super::*;
    #[doc = "Localized display text"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized resource provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized resource type name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The localized operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The localized operation description"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "OData page of action (operation) objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceOperationList {
    #[doc = "List of actions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceOperation>,
    #[doc = "URL to load the next page of actions"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServiceOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SKU instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSku {
    #[doc = "The unique name of the SKU, such as 'P3'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the SKU, such as 'Free', 'Basic', 'Standard', or 'Premium'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU family, used when the service has multiple performance classes within a tier, such as 'A', 'D', etc. for virtual machines"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The size of the SKU, used when the name alone does not denote a service size or when a SKU has multiple performance classes within a family, e.g. 'A1' for virtual machines"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The capacity of the SKU, if it supports scaling"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
impl ServiceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OData page of available SKUs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSkuList {
    #[doc = "List of service SKUs"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AvailableServiceSku>,
    #[doc = "URL to load the next page of service SKUs"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceSkuList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServiceSkuList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Severity of the validation error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Severity")]
pub enum Severity {
    Message,
    Warning,
    Error,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Severity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Message => serializer.serialize_unit_variant("Severity", 0u32, "Message"),
            Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "Warning"),
            Self::Error => serializer.serialize_unit_variant("Severity", 2u32, "Error"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information for connecting to SQL database server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlConnectionInfo {
    #[serde(flatten)]
    pub connection_info: ConnectionInfo,
    #[doc = "Data source in the format Protocol:MachineName\\SQLServerInstanceName,PortNumber"]
    #[serde(rename = "dataSource")]
    pub data_source: String,
    #[doc = "An enumeration of possible authentication types when connecting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<AuthenticationType>,
    #[doc = "Whether to encrypt the connection"]
    #[serde(rename = "encryptConnection", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_connection: Option<bool>,
    #[doc = "Additional connection settings"]
    #[serde(rename = "additionalSettings", default, skip_serializing_if = "Option::is_none")]
    pub additional_settings: Option<String>,
    #[doc = "Whether to trust the server certificate"]
    #[serde(rename = "trustServerCertificate", default, skip_serializing_if = "Option::is_none")]
    pub trust_server_certificate: Option<bool>,
    #[doc = "An enumeration of source platform types"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<SqlServerSourcePlatform>,
}
impl SqlConnectionInfo {
    pub fn new(connection_info: ConnectionInfo, data_source: String) -> Self {
        Self {
            connection_info,
            data_source,
            authentication: None,
            encrypt_connection: None,
            additional_settings: None,
            trust_server_certificate: None,
            platform: None,
        }
    }
}
#[doc = "Base class for migration task input"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlMigrationTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
}
impl SqlMigrationTaskInput {
    pub fn new(source_connection_info: SqlConnectionInfo, target_connection_info: SqlConnectionInfo) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
        }
    }
}
#[doc = "An enumeration of source platform types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlServerSourcePlatform")]
pub enum SqlServerSourcePlatform {
    SqlOnPrem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlServerSourcePlatform {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlServerSourcePlatform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlServerSourcePlatform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SqlOnPrem => serializer.serialize_unit_variant("SqlServerSourcePlatform", 0u32, "SqlOnPrem"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance online scenario."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerSqlMiSyncTaskInput {
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateSqlServerSqlMiDatabaseInput>,
    #[doc = "File share information with Path, Username, and Password."]
    #[serde(rename = "backupFileShare", default, skip_serializing_if = "Option::is_none")]
    pub backup_file_share: Option<FileShare>,
    #[doc = "Fully qualified resourceId of storage"]
    #[serde(rename = "storageResourceId")]
    pub storage_resource_id: String,
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Properties required to create a connection to Azure SQL database Managed instance"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: MiSqlConnectionInfo,
    #[doc = "Azure Active Directory Application"]
    #[serde(rename = "azureApp")]
    pub azure_app: AzureActiveDirectoryApp,
}
impl SqlServerSqlMiSyncTaskInput {
    pub fn new(
        selected_databases: Vec<MigrateSqlServerSqlMiDatabaseInput>,
        storage_resource_id: String,
        source_connection_info: SqlConnectionInfo,
        target_connection_info: MiSqlConnectionInfo,
        azure_app: AzureActiveDirectoryApp,
    ) -> Self {
        Self {
            selected_databases,
            backup_file_share: None,
            storage_resource_id,
            source_connection_info,
            target_connection_info,
            azure_app,
        }
    }
}
#[doc = "Server role migration result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartMigrationScenarioServerRoleResult {
    #[doc = "Name of server role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Current state of migration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Migration exceptions and warnings."]
    #[serde(
        rename = "exceptionsAndWarnings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exceptions_and_warnings: Vec<ReportableException>,
}
impl StartMigrationScenarioServerRoleResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum of the different state of database level online migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SyncDatabaseMigrationReportingState")]
pub enum SyncDatabaseMigrationReportingState {
    #[serde(rename = "UNDEFINED")]
    Undefined,
    #[serde(rename = "CONFIGURING")]
    Configuring,
    #[serde(rename = "INITIALIAZING")]
    Initialiazing,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "READY_TO_COMPLETE")]
    ReadyToComplete,
    #[serde(rename = "COMPLETING")]
    Completing,
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "CANCELLING")]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SyncDatabaseMigrationReportingState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SyncDatabaseMigrationReportingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SyncDatabaseMigrationReportingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Undefined => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 0u32, "UNDEFINED"),
            Self::Configuring => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 1u32, "CONFIGURING"),
            Self::Initialiazing => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 2u32, "INITIALIAZING"),
            Self::Starting => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 3u32, "STARTING"),
            Self::Running => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 4u32, "RUNNING"),
            Self::ReadyToComplete => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 5u32, "READY_TO_COMPLETE"),
            Self::Completing => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 6u32, "COMPLETING"),
            Self::Complete => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 7u32, "COMPLETE"),
            Self::Cancelling => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 8u32, "CANCELLING"),
            Self::Cancelled => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 9u32, "CANCELLED"),
            Self::Failed => serializer.serialize_unit_variant("SyncDatabaseMigrationReportingState", 10u32, "FAILED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Database migration errors for online migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncMigrationDatabaseErrorEvent {
    #[doc = "String value of timestamp."]
    #[serde(rename = "timestampString", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_string: Option<String>,
    #[doc = "Event type."]
    #[serde(rename = "eventTypeString", default, skip_serializing_if = "Option::is_none")]
    pub event_type_string: Option<String>,
    #[doc = "Event text."]
    #[serde(rename = "eventText", default, skip_serializing_if = "Option::is_none")]
    pub event_text: Option<String>,
}
impl SyncMigrationDatabaseErrorEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum of the different state of table level online migration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SyncTableMigrationState")]
pub enum SyncTableMigrationState {
    #[serde(rename = "BEFORE_LOAD")]
    BeforeLoad,
    #[serde(rename = "FULL_LOAD")]
    FullLoad,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SyncTableMigrationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SyncTableMigrationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SyncTableMigrationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BeforeLoad => serializer.serialize_unit_variant("SyncTableMigrationState", 0u32, "BEFORE_LOAD"),
            Self::FullLoad => serializer.serialize_unit_variant("SyncTableMigrationState", 1u32, "FULL_LOAD"),
            Self::Completed => serializer.serialize_unit_variant("SyncTableMigrationState", 2u32, "COMPLETED"),
            Self::Canceled => serializer.serialize_unit_variant("SyncTableMigrationState", 3u32, "CANCELED"),
            Self::Error => serializer.serialize_unit_variant("SyncTableMigrationState", 4u32, "ERROR"),
            Self::Failed => serializer.serialize_unit_variant("SyncTableMigrationState", 5u32, "FAILED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "OData page of tasks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskList {
    #[doc = "List of tasks"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProjectTask>,
    #[doc = "URL to load the next page of tasks"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TaskList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource location."]
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
#[doc = "Type of the actual difference for the compared object, while performing schema comparison"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpdateActionType")]
pub enum UpdateActionType {
    DeletedOnTarget,
    ChangedOnTarget,
    AddedOnTarget,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpdateActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpdateActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpdateActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DeletedOnTarget => serializer.serialize_unit_variant("UpdateActionType", 0u32, "DeletedOnTarget"),
            Self::ChangedOnTarget => serializer.serialize_unit_variant("UpdateActionType", 1u32, "ChangedOnTarget"),
            Self::AddedOnTarget => serializer.serialize_unit_variant("UpdateActionType", 2u32, "AddedOnTarget"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties for task that validates migration input for SQL to Azure SQL DB sync migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateMigrationInputSqlServerSqlDbSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for task that validates migration input for SQL sync migrations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ValidateSyncMigrationInputSqlServerTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ValidateSyncMigrationInputSqlServerTaskOutput>,
}
impl ValidateMigrationInputSqlServerSqlDbSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance online scenario."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateMigrationInputSqlServerSqlMiSyncTaskInput {
    #[serde(flatten)]
    pub sql_server_sql_mi_sync_task_input: SqlServerSqlMiSyncTaskInput,
}
impl ValidateMigrationInputSqlServerSqlMiSyncTaskInput {
    pub fn new(sql_server_sql_mi_sync_task_input: SqlServerSqlMiSyncTaskInput) -> Self {
        Self {
            sql_server_sql_mi_sync_task_input,
        }
    }
}
#[doc = "Output for task that validates migration input for Azure SQL Database Managed Instance online migration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateMigrationInputSqlServerSqlMiSyncTaskOutput {
    #[doc = "Database identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Errors associated with a selected database object"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ValidateMigrationInputSqlServerSqlMiSyncTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for task that validates migration input for SQL to Azure SQL Database Managed Instance sync scenario"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateMigrationInputSqlServerSqlMiSyncTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for task that migrates SQL Server databases to Azure SQL Database Managed Instance online scenario."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ValidateMigrationInputSqlServerSqlMiSyncTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ValidateMigrationInputSqlServerSqlMiSyncTaskOutput>,
}
impl ValidateMigrationInputSqlServerSqlMiSyncTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for task that validates migration input for SQL to Azure SQL Managed Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateMigrationInputSqlServerSqlMiTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateSqlServerSqlMiDatabaseInput>,
    #[doc = "Logins to migrate"]
    #[serde(
        rename = "selectedLogins",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub selected_logins: Vec<String>,
    #[doc = "File share information with Path, Username, and Password."]
    #[serde(rename = "backupFileShare", default, skip_serializing_if = "Option::is_none")]
    pub backup_file_share: Option<FileShare>,
    #[doc = "Blob container storage information."]
    #[serde(rename = "backupBlobShare")]
    pub backup_blob_share: BlobShare,
    #[doc = "An enumeration of backup modes"]
    #[serde(rename = "backupMode", default, skip_serializing_if = "Option::is_none")]
    pub backup_mode: Option<BackupMode>,
}
impl ValidateMigrationInputSqlServerSqlMiTaskInput {
    pub fn new(
        source_connection_info: SqlConnectionInfo,
        target_connection_info: SqlConnectionInfo,
        selected_databases: Vec<MigrateSqlServerSqlMiDatabaseInput>,
        backup_blob_share: BlobShare,
    ) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
            selected_databases,
            selected_logins: Vec::new(),
            backup_file_share: None,
            backup_blob_share,
            backup_mode: None,
        }
    }
}
#[doc = "Output for task that validates migration input for SQL to Azure SQL Managed Instance migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateMigrationInputSqlServerSqlMiTaskOutput {
    #[doc = "Result identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Errors associated with the RestoreDatabaseName"]
    #[serde(
        rename = "restoreDatabaseNameErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restore_database_name_errors: Vec<ReportableException>,
    #[doc = "Errors associated with the BackupFolder path"]
    #[serde(
        rename = "backupFolderErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_folder_errors: Vec<ReportableException>,
    #[doc = "Errors associated with backup share user name and password credentials"]
    #[serde(
        rename = "backupShareCredentialsErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_share_credentials_errors: Vec<ReportableException>,
    #[doc = "Errors associated with the storage account provided."]
    #[serde(
        rename = "backupStorageAccountErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_storage_account_errors: Vec<ReportableException>,
    #[doc = "Errors associated with existing backup files."]
    #[serde(
        rename = "existingBackupErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub existing_backup_errors: Vec<ReportableException>,
    #[doc = "Information about backup files when existing backup mode is used."]
    #[serde(rename = "databaseBackupInfo", default, skip_serializing_if = "Option::is_none")]
    pub database_backup_info: Option<DatabaseBackupInfo>,
}
impl ValidateMigrationInputSqlServerSqlMiTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for task that validates migration input for SQL to Azure SQL Database Managed Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateMigrationInputSqlServerSqlMiTaskProperties {
    #[serde(flatten)]
    pub project_task_properties: ProjectTaskProperties,
    #[doc = "Input for task that validates migration input for SQL to Azure SQL Managed Instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ValidateMigrationInputSqlServerSqlMiTaskInput>,
    #[doc = "Task output. This is ignored if submitted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<ValidateMigrationInputSqlServerSqlMiTaskOutput>,
}
impl ValidateMigrationInputSqlServerSqlMiTaskProperties {
    pub fn new(project_task_properties: ProjectTaskProperties) -> Self {
        Self {
            project_task_properties,
            input: None,
            output: Vec::new(),
        }
    }
}
#[doc = "Input for task that validates migration input for SQL sync migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateSyncMigrationInputSqlServerTaskInput {
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "sourceConnectionInfo")]
    pub source_connection_info: SqlConnectionInfo,
    #[doc = "Information for connecting to SQL database server"]
    #[serde(rename = "targetConnectionInfo")]
    pub target_connection_info: SqlConnectionInfo,
    #[doc = "Databases to migrate"]
    #[serde(rename = "selectedDatabases")]
    pub selected_databases: Vec<MigrateSqlServerSqlDbSyncDatabaseInput>,
}
impl ValidateSyncMigrationInputSqlServerTaskInput {
    pub fn new(
        source_connection_info: SqlConnectionInfo,
        target_connection_info: SqlConnectionInfo,
        selected_databases: Vec<MigrateSqlServerSqlDbSyncDatabaseInput>,
    ) -> Self {
        Self {
            source_connection_info,
            target_connection_info,
            selected_databases,
        }
    }
}
#[doc = "Output for task that validates migration input for SQL sync migrations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateSyncMigrationInputSqlServerTaskOutput {
    #[doc = "Database identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Errors associated with a selected database object"]
    #[serde(
        rename = "validationErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_errors: Vec<ReportableException>,
}
impl ValidateSyncMigrationInputSqlServerTaskOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description about the errors happen while performing migration validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationError {
    #[doc = "Error Text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Severity of the validation error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
}
impl ValidationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current status of the validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ValidationStatus")]
pub enum ValidationStatus {
    Default,
    NotStarted,
    Initialized,
    InProgress,
    Completed,
    CompletedWithIssues,
    Failed,
    Stopped,
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
            Self::Default => serializer.serialize_unit_variant("ValidationStatus", 0u32, "Default"),
            Self::NotStarted => serializer.serialize_unit_variant("ValidationStatus", 1u32, "NotStarted"),
            Self::Initialized => serializer.serialize_unit_variant("ValidationStatus", 2u32, "Initialized"),
            Self::InProgress => serializer.serialize_unit_variant("ValidationStatus", 3u32, "InProgress"),
            Self::Completed => serializer.serialize_unit_variant("ValidationStatus", 4u32, "Completed"),
            Self::CompletedWithIssues => serializer.serialize_unit_variant("ValidationStatus", 5u32, "CompletedWithIssues"),
            Self::Failed => serializer.serialize_unit_variant("ValidationStatus", 6u32, "Failed"),
            Self::Stopped => serializer.serialize_unit_variant("ValidationStatus", 7u32, "Stopped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Wait statistics gathered during query batch execution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WaitStatistics {
    #[doc = "Type of the Wait"]
    #[serde(rename = "waitType", default, skip_serializing_if = "Option::is_none")]
    pub wait_type: Option<String>,
    #[doc = "Total wait time in millisecond(s)"]
    #[serde(rename = "waitTimeMs", default, skip_serializing_if = "Option::is_none")]
    pub wait_time_ms: Option<f32>,
    #[doc = "Total no. of waits"]
    #[serde(rename = "waitCount", default, skip_serializing_if = "Option::is_none")]
    pub wait_count: Option<i64>,
}
impl WaitStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
