#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A List of azure ad administrators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorListResult {
    #[doc = "The list of azure ad administrator of a server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureAdAdministrator>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AdministratorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorProperties {
    #[doc = "Type of the sever administrator."]
    #[serde(rename = "administratorType", default, skip_serializing_if = "Option::is_none")]
    pub administrator_type: Option<administrator_properties::AdministratorType>,
    #[doc = "Login name of the server administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[doc = "SID (object ID) of the server administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "Tenant ID of the administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The resource id of the identity used for AAD Authentication."]
    #[serde(rename = "identityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub identity_resource_id: Option<String>,
}
impl AdministratorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod administrator_properties {
    use super::*;
    #[doc = "Type of the sever administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AdministratorType")]
    pub enum AdministratorType {
        ActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AdministratorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AdministratorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AdministratorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("AdministratorType", 0u32, "ActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a Administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAdAdministrator {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdministratorProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AzureAdAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Profile properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Backup {
    #[doc = "Backup retention days for the server."]
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
    #[serde(rename = "geoRedundantBackup", default, skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<EnableStatusEnum>,
    #[doc = "Earliest restore point creation time (ISO8601 format)"]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
}
impl Backup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BackupAndExport API Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupAndExportRequest {
    #[serde(flatten)]
    pub backup_request_base: BackupRequestBase,
    #[doc = "Details about the target where the backup content will be stored."]
    #[serde(rename = "targetDetails")]
    pub target_details: BackupStoreDetails,
}
impl BackupAndExportRequest {
    pub fn new(backup_request_base: BackupRequestBase, target_details: BackupStoreDetails) -> Self {
        Self {
            backup_request_base,
            target_details,
        }
    }
}
#[doc = "Represents BackupAndExport API Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupAndExportResponse {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The operation status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<backup_and_export_response::Status>,
    #[doc = "Start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Operation progress (0-100)."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "BackupAndExport Response Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupAndExportResponseProperties>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl BackupAndExportResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_and_export_response {
    use super::*;
    #[doc = "The operation status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Succeeded,
        Failed,
        CancelInProgress,
        Canceled,
    }
}
#[doc = "BackupAndExport Response Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupAndExportResponseProperties {
    #[doc = "Size of datasource in bytes"]
    #[serde(rename = "datasourceSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub datasource_size_in_bytes: Option<i64>,
    #[doc = "Data transferred in bytes"]
    #[serde(rename = "dataTransferredInBytes", default, skip_serializing_if = "Option::is_none")]
    pub data_transferred_in_bytes: Option<i64>,
    #[doc = "Metadata related to backup to be stored for restoring resource in key-value pairs."]
    #[serde(rename = "backupMetadata", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<String>,
}
impl BackupAndExportResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BackupName = String;
#[doc = "BackupRequestBase is the base for all backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupRequestBase {
    #[doc = "Backup Settings"]
    #[serde(rename = "backupSettings")]
    pub backup_settings: BackupSettings,
}
impl BackupRequestBase {
    pub fn new(backup_settings: BackupSettings) -> Self {
        Self { backup_settings }
    }
}
#[doc = "Backup Settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupSettings {
    #[doc = "The name of the backup."]
    #[serde(rename = "backupName")]
    pub backup_name: BackupName,
    #[doc = "Backup Format for the current backup. (CollatedFormat is INTERNAL – DO NOT USE)"]
    #[serde(rename = "backupFormat", default, skip_serializing_if = "Option::is_none")]
    pub backup_format: Option<backup_settings::BackupFormat>,
}
impl BackupSettings {
    pub fn new(backup_name: BackupName) -> Self {
        Self {
            backup_name,
            backup_format: None,
        }
    }
}
pub mod backup_settings {
    use super::*;
    #[doc = "Backup Format for the current backup. (CollatedFormat is INTERNAL – DO NOT USE)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupFormat")]
    pub enum BackupFormat {
        None,
        CollatedFormat,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("BackupFormat", 0u32, "None"),
                Self::CollatedFormat => serializer.serialize_unit_variant("BackupFormat", 1u32, "CollatedFormat"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details about the target where the backup content will be stored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupStoreDetails {
    #[doc = "Type of the specific object - used for deserializing"]
    #[serde(rename = "objectType")]
    pub object_type: String,
}
impl BackupStoreDetails {
    pub fn new(object_type: String) -> Self {
        Self { object_type }
    }
}
#[doc = "location capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilitiesListResult {
    #[doc = "A list of supported capabilities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CapabilityProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CapabilitiesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CapabilitiesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityProperties {
    #[doc = "zone name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[doc = "Supported high availability mode"]
    #[serde(
        rename = "supportedHAMode",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_ha_mode: Vec<String>,
    #[doc = "supported geo backup regions"]
    #[serde(
        rename = "supportedGeoBackupRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_geo_backup_regions: Vec<String>,
    #[doc = "A list of supported flexible server editions."]
    #[serde(
        rename = "supportedFlexibleServerEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_flexible_server_editions: Vec<ServerEditionCapability>,
}
impl CapabilityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
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
#[doc = "Represents a Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationForBatchUpdate {
    #[doc = "Name of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties can be updated for a configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<configuration_for_batch_update::Properties>,
}
impl ConfigurationForBatchUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_for_batch_update {
    use super::*;
    #[doc = "The properties can be updated for a configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Value of the configuration."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[doc = "Source of the configuration."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of server configurations to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationListForBatchUpdate {
    #[doc = "The list of server configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConfigurationForBatchUpdate>,
    #[doc = "Whether to reset all server parameters to default."]
    #[serde(rename = "resetAllToDefault", default, skip_serializing_if = "Option::is_none")]
    pub reset_all_to_default: Option<configuration_list_for_batch_update::ResetAllToDefault>,
}
impl ConfigurationListForBatchUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_list_for_batch_update {
    use super::*;
    #[doc = "Whether to reset all server parameters to default."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResetAllToDefault")]
    pub enum ResetAllToDefault {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResetAllToDefault {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResetAllToDefault {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResetAllToDefault {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("ResetAllToDefault", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("ResetAllToDefault", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of server configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationListResult {
    #[doc = "The list of server configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Configuration>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[doc = "Value of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Current value of the configuration."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<String>,
    #[doc = "Description of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The link used to get the document from community or Azure site."]
    #[serde(rename = "documentationLink", default, skip_serializing_if = "Option::is_none")]
    pub documentation_link: Option<String>,
    #[doc = "Default value of the configuration."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "Data type of the configuration."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "Allowed values of the configuration."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[doc = "Source of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<configuration_properties::Source>,
    #[doc = "If is the configuration read only."]
    #[serde(rename = "isReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<configuration_properties::IsReadOnly>,
    #[doc = "If is the configuration pending restart or not."]
    #[serde(rename = "isConfigPendingRestart", default, skip_serializing_if = "Option::is_none")]
    pub is_config_pending_restart: Option<configuration_properties::IsConfigPendingRestart>,
    #[doc = "If is the configuration dynamic."]
    #[serde(rename = "isDynamicConfig", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_config: Option<configuration_properties::IsDynamicConfig>,
}
impl ConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_properties {
    use super::*;
    #[doc = "Source of the configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        #[serde(rename = "system-default")]
        SystemDefault,
        #[serde(rename = "user-override")]
        UserOverride,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SystemDefault => serializer.serialize_unit_variant("Source", 0u32, "system-default"),
                Self::UserOverride => serializer.serialize_unit_variant("Source", 1u32, "user-override"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If is the configuration read only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsReadOnly")]
    pub enum IsReadOnly {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsReadOnly {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsReadOnly {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsReadOnly {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsReadOnly", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsReadOnly", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If is the configuration pending restart or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsConfigPendingRestart")]
    pub enum IsConfigPendingRestart {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsConfigPendingRestart {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsConfigPendingRestart {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsConfigPendingRestart {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsConfigPendingRestart", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsConfigPendingRestart", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If is the configuration dynamic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsDynamicConfig")]
    pub enum IsDynamicConfig {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsDynamicConfig {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsDynamicConfig {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsDynamicConfig {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsDynamicConfig", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsDynamicConfig", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The date encryption for cmk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataEncryption {
    #[doc = "Primary user identity resource id"]
    #[serde(rename = "primaryUserAssignedIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_assigned_identity_id: Option<String>,
    #[doc = "Primary key uri"]
    #[serde(rename = "primaryKeyURI", default, skip_serializing_if = "Option::is_none")]
    pub primary_key_uri: Option<String>,
    #[doc = "Geo backup user identity resource id as identity can't cross region, need identity in same region as geo backup"]
    #[serde(rename = "geoBackupUserAssignedIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_user_assigned_identity_id: Option<String>,
    #[doc = "Geo backup key uri as key vault can't cross region, need cmk in same region as geo backup"]
    #[serde(rename = "geoBackupKeyURI", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_key_uri: Option<String>,
    #[doc = "The key type, AzureKeyVault for enable cmk, SystemManaged for disable cmk."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<data_encryption::Type>,
}
impl DataEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_encryption {
    use super::*;
    #[doc = "The key type, AzureKeyVault for enable cmk, SystemManaged for disable cmk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        AzureKeyVault,
        SystemManaged,
    }
}
#[doc = "Represents a Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A List of databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[doc = "The list of databases housed in a server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Database>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[doc = "The charset of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[doc = "The collation of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
}
impl DatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delegated subnet usage data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedSubnetUsage {
    #[doc = "name of the subnet"]
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[doc = "Number of used delegated subnets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}
impl DelegatedSubnetUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnableStatusEnum")]
pub enum EnableStatusEnum {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnableStatusEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnableStatusEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnableStatusEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EnableStatusEnum", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EnableStatusEnum", 1u32, "Disabled"),
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a server firewall rule."]
    pub properties: FirewallRuleProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FirewallRule {
    pub fn new(properties: FirewallRuleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "A list of firewall rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[doc = "The list of firewall rules in a server."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FirewallRule>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[doc = "The start IP address of the server firewall rule. Must be IPv4 format."]
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[doc = "The end IP address of the server firewall rule. Must be IPv4 format."]
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
impl FirewallRuleProperties {
    pub fn new(start_ip_address: String, end_ip_address: String) -> Self {
        Self {
            start_ip_address,
            end_ip_address,
        }
    }
}
#[doc = "FullBackupStoreDetails is used for scenarios where backup data is streamed/copied over to a storage destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullBackupStoreDetails {
    #[serde(flatten)]
    pub backup_store_details: BackupStoreDetails,
    #[doc = "SASUriList of storage containers where backup data is to be streamed/copied."]
    #[serde(rename = "sasUriList")]
    pub sas_uri_list: Vec<String>,
}
impl FullBackupStoreDetails {
    pub fn new(backup_store_details: BackupStoreDetails, sas_uri_list: Vec<String>) -> Self {
        Self {
            backup_store_details,
            sas_uri_list,
        }
    }
}
#[doc = "The response of get private dns zone suffix."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetPrivateDnsZoneSuffixResponse {
    #[doc = "Represents the private DNS zone suffix."]
    #[serde(rename = "privateDnsZoneSuffix", default, skip_serializing_if = "Option::is_none")]
    pub private_dns_zone_suffix: Option<String>,
}
impl GetPrivateDnsZoneSuffixResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network related properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HighAvailability {
    #[doc = "High availability mode for a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<high_availability::Mode>,
    #[doc = "The state of server high availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<high_availability::State>,
    #[doc = "Availability zone of the standby server."]
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
}
impl HighAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod high_availability {
    use super::*;
    #[doc = "High availability mode for a server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Disabled,
        ZoneRedundant,
        SameZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("Mode", 0u32, "Disabled"),
                Self::ZoneRedundant => serializer.serialize_unit_variant("Mode", 1u32, "ZoneRedundant"),
                Self::SameZone => serializer.serialize_unit_variant("Mode", 2u32, "SameZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of server high availability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        NotEnabled,
        CreatingStandby,
        Healthy,
        FailingOver,
        RemovingStandby,
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
                Self::NotEnabled => serializer.serialize_unit_variant("State", 0u32, "NotEnabled"),
                Self::CreatingStandby => serializer.serialize_unit_variant("State", 1u32, "CreatingStandby"),
                Self::Healthy => serializer.serialize_unit_variant("State", 2u32, "Healthy"),
                Self::FailingOver => serializer.serialize_unit_variant("State", 3u32, "FailingOver"),
                Self::RemovingStandby => serializer.serialize_unit_variant("State", 4u32, "RemovingStandby"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties to configure Identity for Bring your Own Keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "ObjectId from the KeyVault"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "TenantId from the KeyVault"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "Metadata of user assigned identity."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "Type of managed service identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        UserAssigned,
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
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 0u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a logFile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a logFile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogFileProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl LogFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A List of logFiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFileListResult {
    #[doc = "The list of logFiles in a server"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LogFile>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogFileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LogFileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a logFile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFileProperties {
    #[doc = "The size in kb of the logFile."]
    #[serde(rename = "sizeInKB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<i64>,
    #[doc = "Creation timestamp of the log file."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Type of the log file."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Last modified timestamp of the log file."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The url to download the log file from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl LogFileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindow {
    #[doc = "indicates whether custom window is enabled or disabled"]
    #[serde(rename = "customWindow", default, skip_serializing_if = "Option::is_none")]
    pub custom_window: Option<String>,
    #[doc = "start hour for maintenance window"]
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i32>,
    #[doc = "start minute for maintenance window"]
    #[serde(rename = "startMinute", default, skip_serializing_if = "Option::is_none")]
    pub start_minute: Option<i32>,
    #[doc = "day of week for maintenance window"]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}
impl MaintenanceWindow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a resource name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Error Message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Indicates whether the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Reason for name being unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request from client to check resource name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityRequest {
    #[doc = "Resource name to verify."]
    pub name: String,
    #[doc = "Resource type used for verification."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NameAvailabilityRequest {
    pub fn new(name: String) -> Self {
        Self { name, type_: None }
    }
}
#[doc = "Network related properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<EnableStatusEnum>,
    #[doc = "Delegated subnet resource id used to setup vnet for a server."]
    #[serde(rename = "delegatedSubnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_subnet_resource_id: Option<String>,
    #[doc = "Private DNS zone resource id."]
    #[serde(rename = "privateDnsZoneResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_dns_zone_resource_id: Option<String>,
}
impl Network {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation definition."]
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
    pub origin: Option<String>,
    #[doc = "Additional descriptions for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Display metadata associated with the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Operation resource provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Localized friendly name for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of resource provider operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Collection of available operation details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL client should use to fetch the next page (per server side paging)."]
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
#[doc = "The replication role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicationRole")]
pub enum ReplicationRole {
    None,
    Source,
    Replica,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReplicationRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReplicationRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReplicationRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ReplicationRole", 0u32, "None"),
            Self::Source => serializer.serialize_unit_variant("ReplicationRole", 1u32, "Source"),
            Self::Replica => serializer.serialize_unit_variant("ReplicationRole", 2u32, "Replica"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Represents a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties to configure Identity for Bring your Own Keys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Billing information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Server {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            sku: None,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Server backup properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBackup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a server backup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerBackupProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ServerBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBackupListResult {
    #[doc = "The list of backups of a server."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerBackup>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerBackupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerBackupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBackupProperties {
    #[doc = "Backup type."]
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    #[doc = "Backup completed time (ISO8601 format)."]
    #[serde(rename = "completedTime", default, with = "azure_core::date::rfc3339::option")]
    pub completed_time: Option<time::OffsetDateTime>,
    #[doc = "Backup source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl ServerBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server edition capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEditionCapability {
    #[doc = "Server edition name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A list of supported storage editions"]
    #[serde(
        rename = "supportedStorageEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_editions: Vec<StorageEditionCapability>,
    #[doc = "A list of supported server versions."]
    #[serde(
        rename = "supportedServerVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_versions: Vec<ServerVersionCapability>,
}
impl ServerEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters allowed to update for a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerForUpdate {
    #[doc = "Properties to configure Identity for Bring your Own Keys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Billing information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties that can be updated for a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerPropertiesForUpdate>,
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServerForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server gtid set parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGtidSetParameter {
    #[doc = "The gtid set of server."]
    #[serde(rename = "gtidSet", default, skip_serializing_if = "Option::is_none")]
    pub gtid_set: Option<String>,
}
impl ServerGtidSetParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerListResult {
    #[doc = "The list of servers"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Server>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[doc = "The administrator's login name of a server. Can only be specified when the server is being created (and is required for creation)."]
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[doc = "The password of the administrator login (required for server creation)."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The version of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[doc = "availability Zone information of the server."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "The mode to create a new MySQL server."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_properties::CreateMode>,
    #[doc = "The source MySQL server id."]
    #[serde(rename = "sourceServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_server_resource_id: Option<String>,
    #[doc = "Restore point creation time (ISO8601 format), specifying the time to restore from."]
    #[serde(rename = "restorePointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "The replication role."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<ReplicationRole>,
    #[doc = "The maximum number of replicas that a primary server can have."]
    #[serde(rename = "replicaCapacity", default, skip_serializing_if = "Option::is_none")]
    pub replica_capacity: Option<i32>,
    #[doc = "The date encryption for cmk."]
    #[serde(rename = "dataEncryption", default, skip_serializing_if = "Option::is_none")]
    pub data_encryption: Option<DataEncryption>,
    #[doc = "The state of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<server_properties::State>,
    #[doc = "The fully qualified domain name of a server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[doc = "Storage Profile properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[doc = "Storage Profile properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[doc = "Network related properties of a server"]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[doc = "Network related properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[doc = "Maintenance window of a server."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_properties {
    use super::*;
    #[doc = "The mode to create a new MySQL server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        Replica,
        GeoRestore,
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
                Self::Default => serializer.serialize_unit_variant("CreateMode", 0u32, "Default"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 1u32, "PointInTimeRestore"),
                Self::Replica => serializer.serialize_unit_variant("CreateMode", 2u32, "Replica"),
                Self::GeoRestore => serializer.serialize_unit_variant("CreateMode", 3u32, "GeoRestore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of a server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Ready,
        Dropping,
        Disabled,
        Starting,
        Stopping,
        Stopped,
        Updating,
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
                Self::Ready => serializer.serialize_unit_variant("State", 0u32, "Ready"),
                Self::Dropping => serializer.serialize_unit_variant("State", 1u32, "Dropping"),
                Self::Disabled => serializer.serialize_unit_variant("State", 2u32, "Disabled"),
                Self::Starting => serializer.serialize_unit_variant("State", 3u32, "Starting"),
                Self::Stopping => serializer.serialize_unit_variant("State", 4u32, "Stopping"),
                Self::Stopped => serializer.serialize_unit_variant("State", 5u32, "Stopped"),
                Self::Updating => serializer.serialize_unit_variant("State", 6u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that can be updated for a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerPropertiesForUpdate {
    #[doc = "The password of the administrator login."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The version of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[doc = "Storage Profile properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[doc = "Storage Profile properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[doc = "Network related properties of a server"]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[doc = "Maintenance window of a server."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "The replication role."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<ReplicationRole>,
    #[doc = "The date encryption for cmk."]
    #[serde(rename = "dataEncryption", default, skip_serializing_if = "Option::is_none")]
    pub data_encryption: Option<DataEncryption>,
    #[doc = "Network related properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}
impl ServerPropertiesForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server restart parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerRestartParameter {
    #[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
    #[serde(rename = "restartWithFailover", default, skip_serializing_if = "Option::is_none")]
    pub restart_with_failover: Option<EnableStatusEnum>,
    #[doc = "The maximum allowed failover time in seconds."]
    #[serde(rename = "maxFailoverSeconds", default, skip_serializing_if = "Option::is_none")]
    pub max_failover_seconds: Option<i32>,
}
impl ServerRestartParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The version of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerVersion")]
pub enum ServerVersion {
    #[serde(rename = "5.7")]
    N5_7,
    #[serde(rename = "8.0.21")]
    N8_0_21,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N5_7 => serializer.serialize_unit_variant("ServerVersion", 0u32, "5.7"),
            Self::N8_0_21 => serializer.serialize_unit_variant("ServerVersion", 1u32, "8.0.21"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Server version capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVersionCapability {
    #[doc = "server version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A list of supported Skus"]
    #[serde(
        rename = "supportedSkus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_skus: Vec<SkuCapability>,
}
impl ServerVersionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing information related properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the sku, e.g. Standard_D32s_v3."]
    pub name: String,
    #[doc = "The tier of the particular SKU, e.g. GeneralPurpose."]
    pub tier: sku::Tier,
}
impl Sku {
    pub fn new(name: String, tier: sku::Tier) -> Self {
        Self { name, tier }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The tier of the particular SKU, e.g. GeneralPurpose."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Burstable,
        GeneralPurpose,
        MemoryOptimized,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Burstable => serializer.serialize_unit_variant("Tier", 0u32, "Burstable"),
                Self::GeneralPurpose => serializer.serialize_unit_variant("Tier", 1u32, "GeneralPurpose"),
                Self::MemoryOptimized => serializer.serialize_unit_variant("Tier", 2u32, "MemoryOptimized"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Sku capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "vCore name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "supported vCores"]
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i64>,
    #[doc = "supported IOPS"]
    #[serde(rename = "supportedIops", default, skip_serializing_if = "Option::is_none")]
    pub supported_iops: Option<i64>,
    #[doc = "supported memory per vCore in MB"]
    #[serde(rename = "supportedMemoryPerVCoreMB", default, skip_serializing_if = "Option::is_none")]
    pub supported_memory_per_v_core_mb: Option<i64>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Profile properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Storage {
    #[doc = "Max storage size allowed for a server."]
    #[serde(rename = "storageSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_gb: Option<i32>,
    #[doc = "Storage IOPS for a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
    #[serde(rename = "autoGrow", default, skip_serializing_if = "Option::is_none")]
    pub auto_grow: Option<EnableStatusEnum>,
    #[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
    #[serde(rename = "logOnDisk", default, skip_serializing_if = "Option::is_none")]
    pub log_on_disk: Option<EnableStatusEnum>,
    #[doc = "The sku name of the server storage."]
    #[serde(rename = "storageSku", default, skip_serializing_if = "Option::is_none")]
    pub storage_sku: Option<String>,
    #[doc = "Enum to indicate whether value is 'Enabled' or 'Disabled'"]
    #[serde(rename = "autoIoScaling", default, skip_serializing_if = "Option::is_none")]
    pub auto_io_scaling: Option<EnableStatusEnum>,
}
impl Storage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "storage edition capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageEditionCapability {
    #[doc = "storage edition name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The minimal supported storage size."]
    #[serde(rename = "minStorageSize", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_size: Option<i64>,
    #[doc = "The maximum supported storage size."]
    #[serde(rename = "maxStorageSize", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_size: Option<i64>,
    #[doc = "Minimal backup retention days"]
    #[serde(rename = "minBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i64>,
    #[doc = "Maximum backup retention days"]
    #[serde(rename = "maxBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i64>,
}
impl StorageEditionCapability {
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
#[doc = "Metadata of user assigned identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "Principal Id of user assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client Id of user assigned identity"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents ValidateBackup API Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateBackupResponse {
    #[doc = "ValidateBackup Response Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ValidateBackupResponseProperties>,
}
impl ValidateBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ValidateBackup Response Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateBackupResponseProperties {
    #[doc = "Estimated no of storage containers required for resource data to be backed up."]
    #[serde(rename = "numberOfContainers", default, skip_serializing_if = "Option::is_none")]
    pub number_of_containers: Option<i32>,
}
impl ValidateBackupResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual network subnet usage parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkSubnetUsageParameter {
    #[doc = "Virtual network resource id."]
    #[serde(rename = "virtualNetworkResourceId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_resource_id: Option<String>,
}
impl VirtualNetworkSubnetUsageParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual network subnet usage data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkSubnetUsageResult {
    #[doc = "The location name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The subscription id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "A list of delegated subnet usage"]
    #[serde(
        rename = "delegatedSubnetsUsage",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delegated_subnets_usage: Vec<DelegatedSubnetUsage>,
}
impl VirtualNetworkSubnetUsageResult {
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
