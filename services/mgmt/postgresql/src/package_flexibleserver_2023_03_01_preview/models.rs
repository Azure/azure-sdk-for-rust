#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents an Active Directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryAdministrator {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an Active Directory administrator."]
    pub properties: AdministratorProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ActiveDirectoryAdministrator {
    pub fn new(properties: AdministratorProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "Represents an Active Directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryAdministratorAdd {
    #[doc = "The properties of an Active Directory administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdministratorPropertiesForAdd>,
}
impl ActiveDirectoryAdministratorAdd {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server admin credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminCredentials {
    #[doc = "Password for source server."]
    #[serde(rename = "sourceServerPassword")]
    pub source_server_password: String,
    #[doc = "Password for target server."]
    #[serde(rename = "targetServerPassword")]
    pub target_server_password: String,
}
impl AdminCredentials {
    pub fn new(source_server_password: String, target_server_password: String) -> Self {
        Self {
            source_server_password,
            target_server_password,
        }
    }
}
#[doc = "A list of active directory administrators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorListResult {
    #[doc = "The list of active directory administrators"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ActiveDirectoryAdministrator>,
    #[doc = "The link used to get the next page of active directory."]
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
#[doc = "The properties of an Active Directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorProperties {
    #[doc = "The principal type used to represent the type of Active Directory Administrator."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<administrator_properties::PrincipalType>,
    #[doc = "Active Directory administrator principal name."]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "The objectId of the Active Directory administrator."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The tenantId of the Active Directory administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AdministratorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod administrator_properties {
    use super::*;
    #[doc = "The principal type used to represent the type of Active Directory Administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        Unknown,
        User,
        Group,
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PrincipalType", 0u32, "Unknown"),
                Self::User => serializer.serialize_unit_variant("PrincipalType", 1u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 2u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 3u32, "ServicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of an Active Directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorPropertiesForAdd {
    #[doc = "The principal type used to represent the type of Active Directory Administrator."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<administrator_properties_for_add::PrincipalType>,
    #[doc = "Active Directory administrator principal name."]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "The tenantId of the Active Directory administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AdministratorPropertiesForAdd {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod administrator_properties_for_add {
    use super::*;
    #[doc = "The principal type used to represent the type of Active Directory Administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        Unknown,
        User,
        Group,
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PrincipalType", 0u32, "Unknown"),
                Self::User => serializer.serialize_unit_variant("PrincipalType", 1u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 2u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 3u32, "ServicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Authentication configuration properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[doc = "If Enabled, Azure Active Directory authentication is enabled."]
    #[serde(rename = "activeDirectoryAuth", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_auth: Option<auth_config::ActiveDirectoryAuth>,
    #[doc = "If Enabled, Password authentication is enabled."]
    #[serde(rename = "passwordAuth", default, skip_serializing_if = "Option::is_none")]
    pub password_auth: Option<auth_config::PasswordAuth>,
    #[doc = "Tenant id of the server."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl AuthConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auth_config {
    use super::*;
    #[doc = "If Enabled, Azure Active Directory authentication is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActiveDirectoryAuth")]
    pub enum ActiveDirectoryAuth {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActiveDirectoryAuth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActiveDirectoryAuth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActiveDirectoryAuth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ActiveDirectoryAuth", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ActiveDirectoryAuth", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "If Enabled, Password authentication is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PasswordAuth")]
    pub enum PasswordAuth {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PasswordAuth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PasswordAuth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PasswordAuth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PasswordAuth", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PasswordAuth", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PasswordAuth {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "Backup properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Backup {
    #[doc = "Backup retention days for the server."]
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[doc = "A value indicating whether Geo-Redundant backup is enabled on the server."]
    #[serde(rename = "geoRedundantBackup", default, skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<backup::GeoRedundantBackup>,
    #[doc = "The earliest restore point time (ISO8601 format) for server."]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
}
impl Backup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup {
    use super::*;
    #[doc = "A value indicating whether Geo-Redundant backup is enabled on the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "GeoRedundantBackup")]
    pub enum GeoRedundantBackup {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for GeoRedundantBackup {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for GeoRedundantBackup {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for GeoRedundantBackup {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("GeoRedundantBackup", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("GeoRedundantBackup", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for GeoRedundantBackup {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "BackupRequestBase is the base for all backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupRequestBase {
    #[doc = "The settings for the long term backup."]
    #[serde(rename = "backupSettings")]
    pub backup_settings: BackupSettings,
}
impl BackupRequestBase {
    pub fn new(backup_settings: BackupSettings) -> Self {
        Self { backup_settings }
    }
}
#[doc = "The settings for the long term backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupSettings {
    #[doc = "Backup Name for the current backup"]
    #[serde(rename = "backupName")]
    pub backup_name: String,
}
impl BackupSettings {
    pub fn new(backup_name: String) -> Self {
        Self { backup_name }
    }
}
#[doc = "Details about the target where the backup content will be stored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupStoreDetails {
    #[doc = "List of SAS uri of storage containers where backup data is to be streamed/copied."]
    #[serde(rename = "sasUriList")]
    pub sas_uri_list: Vec<String>,
}
impl BackupStoreDetails {
    pub fn new(sas_uri_list: Vec<String>) -> Self {
        Self { sas_uri_list }
    }
}
#[doc = "Capability for the PostgreSQL server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilitiesListResult {
    #[doc = "A list of supported capabilities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FlexibleServerCapability>,
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
#[doc = "Base object for representing capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityBase {
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<capability_base::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl CapabilityBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod capability_base {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
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
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
pub struct ConfigurationForUpdate {
    #[doc = "The properties of a configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
impl ConfigurationForUpdate {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Description of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Default value of the configuration."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "Data type of the configuration."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<configuration_properties::DataType>,
    #[doc = "Allowed values of the configuration."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[doc = "Source of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Configuration dynamic or static."]
    #[serde(rename = "isDynamicConfig", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_config: Option<bool>,
    #[doc = "Configuration read-only or not."]
    #[serde(rename = "isReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[doc = "Configuration is pending restart or not."]
    #[serde(rename = "isConfigPendingRestart", default, skip_serializing_if = "Option::is_none")]
    pub is_config_pending_restart: Option<bool>,
    #[doc = "Configuration unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Configuration documentation link."]
    #[serde(rename = "documentationLink", default, skip_serializing_if = "Option::is_none")]
    pub documentation_link: Option<String>,
}
impl ConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_properties {
    use super::*;
    #[doc = "Data type of the configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataType")]
    pub enum DataType {
        Boolean,
        Numeric,
        Integer,
        Enumeration,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Boolean => serializer.serialize_unit_variant("DataType", 0u32, "Boolean"),
                Self::Numeric => serializer.serialize_unit_variant("DataType", 1u32, "Numeric"),
                Self::Integer => serializer.serialize_unit_variant("DataType", 2u32, "Integer"),
                Self::Enumeration => serializer.serialize_unit_variant("DataType", 3u32, "Enumeration"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Data encryption properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataEncryption {
    #[doc = "URI for the key in keyvault for data encryption of the primary server."]
    #[serde(rename = "primaryKeyURI", default, skip_serializing_if = "Option::is_none")]
    pub primary_key_uri: Option<String>,
    #[doc = "Resource Id for the User assigned identity to be used for data encryption of the primary server."]
    #[serde(rename = "primaryUserAssignedIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_assigned_identity_id: Option<String>,
    #[doc = "URI for the key in keyvault for data encryption for geo-backup of server."]
    #[serde(rename = "geoBackupKeyURI", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_key_uri: Option<String>,
    #[doc = "Resource Id for the User assigned identity to be used for data encryption for geo-backup of server."]
    #[serde(rename = "geoBackupUserAssignedIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_user_assigned_identity_id: Option<String>,
    #[doc = "Data encryption type to depict if it is System Managed vs Azure Key vault."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<data_encryption::Type>,
    #[doc = "Primary encryption key status for Data encryption enabled server."]
    #[serde(rename = "primaryEncryptionKeyStatus", default, skip_serializing_if = "Option::is_none")]
    pub primary_encryption_key_status: Option<data_encryption::PrimaryEncryptionKeyStatus>,
    #[doc = "Geo-backup encryption key status for Data encryption enabled server."]
    #[serde(rename = "geoBackupEncryptionKeyStatus", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_encryption_key_status: Option<data_encryption::GeoBackupEncryptionKeyStatus>,
}
impl DataEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_encryption {
    use super::*;
    #[doc = "Data encryption type to depict if it is System Managed vs Azure Key vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemManaged,
        AzureKeyVault,
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
                Self::SystemManaged => serializer.serialize_unit_variant("Type", 0u32, "SystemManaged"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("Type", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Primary encryption key status for Data encryption enabled server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrimaryEncryptionKeyStatus")]
    pub enum PrimaryEncryptionKeyStatus {
        Valid,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrimaryEncryptionKeyStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrimaryEncryptionKeyStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrimaryEncryptionKeyStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Valid => serializer.serialize_unit_variant("PrimaryEncryptionKeyStatus", 0u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("PrimaryEncryptionKeyStatus", 1u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Geo-backup encryption key status for Data encryption enabled server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "GeoBackupEncryptionKeyStatus")]
    pub enum GeoBackupEncryptionKeyStatus {
        Valid,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for GeoBackupEncryptionKeyStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for GeoBackupEncryptionKeyStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for GeoBackupEncryptionKeyStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Valid => serializer.serialize_unit_variant("GeoBackupEncryptionKeyStatus", 0u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("GeoBackupEncryptionKeyStatus", 1u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "The link used to get the next page of databases."]
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
#[doc = "Database server metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbServerMetadata {
    #[doc = "Location of database server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Version for database engine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Storage size in MB for database server"]
    #[serde(rename = "storageMb", default, skip_serializing_if = "Option::is_none")]
    pub storage_mb: Option<i32>,
    #[doc = "Sku information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ServerSku>,
}
impl DbServerMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delegated subnet usage data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedSubnetUsage {
    #[doc = "Name of the delegated subnet for which IP addresses are in use"]
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[doc = "Number of IP addresses used by the delegated subnet"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}
impl DelegatedSubnetUsage {
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
#[doc = "Represents capability of a fast provisioning edition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FastProvisioningEditionCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Fast provisioning supported tier name"]
    #[serde(rename = "supportedTier", default, skip_serializing_if = "Option::is_none")]
    pub supported_tier: Option<String>,
    #[doc = "Fast provisioning supported sku name"]
    #[serde(rename = "supportedSku", default, skip_serializing_if = "Option::is_none")]
    pub supported_sku: Option<String>,
    #[doc = "Fast provisioning supported storage in Gb"]
    #[serde(rename = "supportedStorageGb", default, skip_serializing_if = "Option::is_none")]
    pub supported_storage_gb: Option<i32>,
    #[doc = "Fast provisioning supported version"]
    #[serde(rename = "supportedServerVersions", default, skip_serializing_if = "Option::is_none")]
    pub supported_server_versions: Option<String>,
    #[doc = "Count of servers in cache matching the spec"]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
}
impl FastProvisioningEditionCapability {
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
#[doc = "Capability for the PostgreSQL server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlexibleServerCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Name of flexible servers capability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of supported flexible server editions"]
    #[serde(
        rename = "supportedServerEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_editions: Vec<FlexibleServerEditionCapability>,
    #[doc = "The list of server versions supported for this capability."]
    #[serde(
        rename = "supportedServerVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[doc = "Gets a value indicating whether fast provisioning is supported. \"Enabled\" means fast provisioning is supported. \"Disabled\" stands for fast provisioning is not supported."]
    #[serde(rename = "fastProvisioningSupported", default, skip_serializing_if = "Option::is_none")]
    pub fast_provisioning_supported: Option<flexible_server_capability::FastProvisioningSupported>,
    #[doc = "List of supported server editions for fast provisioning"]
    #[serde(
        rename = "supportedFastProvisioningEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_fast_provisioning_editions: Vec<FastProvisioningEditionCapability>,
    #[doc = "Determines if geo-backup is supported in this region. \"Enabled\" means geo-backup is supported. \"Disabled\" stands for geo-back is not supported."]
    #[serde(rename = "geoBackupSupported", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_supported: Option<flexible_server_capability::GeoBackupSupported>,
    #[doc = "A value indicating whether Zone Redundant HA is supported in this region. \"Enabled\" means zone redundant HA is supported. \"Disabled\" stands for zone redundant HA is not supported."]
    #[serde(rename = "zoneRedundantHaSupported", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant_ha_supported: Option<flexible_server_capability::ZoneRedundantHaSupported>,
    #[doc = "A value indicating whether Zone Redundant HA and Geo-backup is supported in this region. \"Enabled\" means zone redundant HA and geo-backup is supported. \"Disabled\" stands for zone redundant HA and geo-backup is not supported."]
    #[serde(rename = "zoneRedundantHaAndGeoBackupSupported", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant_ha_and_geo_backup_supported: Option<flexible_server_capability::ZoneRedundantHaAndGeoBackupSupported>,
    #[doc = "A value indicating whether storage auto-grow is supported in this region. \"Enabled\" means storage auto-grow is supported. \"Disabled\" stands for storage auto-grow is not supported."]
    #[serde(rename = "storageAutoGrowthSupported", default, skip_serializing_if = "Option::is_none")]
    pub storage_auto_growth_supported: Option<flexible_server_capability::StorageAutoGrowthSupported>,
    #[doc = "A value indicating whether online resize is supported in this region for the given subscription. \"Enabled\" means storage online resize is supported. \"Disabled\" means storage online resize is not supported."]
    #[serde(rename = "onlineResizeSupported", default, skip_serializing_if = "Option::is_none")]
    pub online_resize_supported: Option<flexible_server_capability::OnlineResizeSupported>,
    #[doc = "A value indicating whether this region is restricted. \"Enabled\" means region is restricted. \"Disabled\" stands for region is not restricted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<flexible_server_capability::Restricted>,
}
impl FlexibleServerCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod flexible_server_capability {
    use super::*;
    #[doc = "Gets a value indicating whether fast provisioning is supported. \"Enabled\" means fast provisioning is supported. \"Disabled\" stands for fast provisioning is not supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FastProvisioningSupported")]
    pub enum FastProvisioningSupported {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FastProvisioningSupported {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FastProvisioningSupported {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FastProvisioningSupported {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("FastProvisioningSupported", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("FastProvisioningSupported", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Determines if geo-backup is supported in this region. \"Enabled\" means geo-backup is supported. \"Disabled\" stands for geo-back is not supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "GeoBackupSupported")]
    pub enum GeoBackupSupported {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for GeoBackupSupported {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for GeoBackupSupported {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for GeoBackupSupported {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("GeoBackupSupported", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("GeoBackupSupported", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether Zone Redundant HA is supported in this region. \"Enabled\" means zone redundant HA is supported. \"Disabled\" stands for zone redundant HA is not supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ZoneRedundantHaSupported")]
    pub enum ZoneRedundantHaSupported {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ZoneRedundantHaSupported {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ZoneRedundantHaSupported {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ZoneRedundantHaSupported {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ZoneRedundantHaSupported", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ZoneRedundantHaSupported", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether Zone Redundant HA and Geo-backup is supported in this region. \"Enabled\" means zone redundant HA and geo-backup is supported. \"Disabled\" stands for zone redundant HA and geo-backup is not supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ZoneRedundantHaAndGeoBackupSupported")]
    pub enum ZoneRedundantHaAndGeoBackupSupported {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ZoneRedundantHaAndGeoBackupSupported {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ZoneRedundantHaAndGeoBackupSupported {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ZoneRedundantHaAndGeoBackupSupported {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ZoneRedundantHaAndGeoBackupSupported", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ZoneRedundantHaAndGeoBackupSupported", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether storage auto-grow is supported in this region. \"Enabled\" means storage auto-grow is supported. \"Disabled\" stands for storage auto-grow is not supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAutoGrowthSupported")]
    pub enum StorageAutoGrowthSupported {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAutoGrowthSupported {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAutoGrowthSupported {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAutoGrowthSupported {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("StorageAutoGrowthSupported", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("StorageAutoGrowthSupported", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether online resize is supported in this region for the given subscription. \"Enabled\" means storage online resize is supported. \"Disabled\" means storage online resize is not supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OnlineResizeSupported")]
    pub enum OnlineResizeSupported {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OnlineResizeSupported {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OnlineResizeSupported {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OnlineResizeSupported {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("OnlineResizeSupported", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("OnlineResizeSupported", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether this region is restricted. \"Enabled\" means region is restricted. \"Disabled\" stands for region is not restricted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Restricted")]
    pub enum Restricted {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Restricted {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Restricted {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Restricted {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Restricted", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Restricted", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Flexible server edition capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlexibleServerEditionCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Server edition name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Default sku name for the server edition"]
    #[serde(rename = "defaultSkuName", default, skip_serializing_if = "Option::is_none")]
    pub default_sku_name: Option<String>,
    #[doc = "The list of editions supported by this server edition."]
    #[serde(
        rename = "supportedStorageEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_editions: Vec<StorageEditionCapability>,
    #[doc = "List of supported server SKUs."]
    #[serde(
        rename = "supportedServerSkus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_skus: Vec<ServerSkuCapability>,
}
impl FlexibleServerEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "High availability properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HighAvailability {
    #[doc = "The HA mode for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<high_availability::Mode>,
    #[doc = "A state of a HA server that is visible to user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<high_availability::State>,
    #[doc = "availability zone information of the standby."]
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
    #[doc = "The HA mode for the server."]
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
    impl Default for Mode {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "A state of a HA server that is visible to user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        NotEnabled,
        CreatingStandby,
        ReplicatingData,
        FailingOver,
        Healthy,
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
                Self::ReplicatingData => serializer.serialize_unit_variant("State", 2u32, "ReplicatingData"),
                Self::FailingOver => serializer.serialize_unit_variant("State", 3u32, "FailingOver"),
                Self::Healthy => serializer.serialize_unit_variant("State", 4u32, "Healthy"),
                Self::RemovingStandby => serializer.serialize_unit_variant("State", 5u32, "RemovingStandby"),
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
    #[doc = "Creation timestamp of the log file."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Last modified timestamp of the log file."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The size in kb of the logFile."]
    #[serde(rename = "sizeInKb", default, skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<i64>,
    #[doc = "Type of the log file."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The url to download the log file from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl LogFileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtrBackupOperationResponseProperties {
    #[doc = "Size of datasource in bytes"]
    #[serde(rename = "datasourceSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub datasource_size_in_bytes: Option<i64>,
    #[doc = "Data transferred in bytes"]
    #[serde(rename = "dataTransferredInBytes", default, skip_serializing_if = "Option::is_none")]
    pub data_transferred_in_bytes: Option<i64>,
    #[doc = "Name of Backup operation"]
    #[serde(rename = "backupName", default, skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    #[doc = "Metadata to be stored in RP. Store everything that will be required to perform a successful restore using this Recovery point. e.g. Versions, DataFormat etc"]
    #[serde(rename = "backupMetadata", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<String>,
    #[doc = "Service-set extensible enum indicating the status of operation"]
    pub status: ltr_backup_operation_response_properties::Status,
    #[doc = "Start time of the operation."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "End time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "PercentageCompleted"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl LtrBackupOperationResponseProperties {
    pub fn new(status: ltr_backup_operation_response_properties::Status, start_time: time::OffsetDateTime) -> Self {
        Self {
            datasource_size_in_bytes: None,
            data_transferred_in_bytes: None,
            backup_name: None,
            backup_metadata: None,
            status,
            start_time,
            end_time: None,
            percent_complete: None,
            error_code: None,
            error_message: None,
        }
    }
}
pub mod ltr_backup_operation_response_properties {
    use super::*;
    #[doc = "Service-set extensible enum indicating the status of operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Running,
        Cancelled,
        Failed,
        Succeeded,
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
                Self::Running => serializer.serialize_unit_variant("Status", 0u32, "Running"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 1u32, "Cancelled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The request that is made for a long term retention backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtrBackupRequest {
    #[serde(flatten)]
    pub backup_request_base: BackupRequestBase,
    #[doc = "Details about the target where the backup content will be stored."]
    #[serde(rename = "targetDetails")]
    pub target_details: BackupStoreDetails,
}
impl LtrBackupRequest {
    pub fn new(backup_request_base: BackupRequestBase, target_details: BackupStoreDetails) -> Self {
        Self {
            backup_request_base,
            target_details,
        }
    }
}
#[doc = "Response for the LTR backup API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LtrBackupResponse {
    #[doc = "Response for the backup request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LtrBackupOperationResponseProperties>,
}
impl LtrBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request that is made for pre-backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtrPreBackupRequest {
    #[serde(flatten)]
    pub backup_request_base: BackupRequestBase,
}
impl LtrPreBackupRequest {
    pub fn new(backup_request_base: BackupRequestBase) -> Self {
        Self { backup_request_base }
    }
}
#[doc = "Response for the LTR pre-backup API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtrPreBackupResponse {
    #[doc = "Response for the pre-backup request."]
    pub properties: LtrPreBackupResponseProperties,
}
impl LtrPreBackupResponse {
    pub fn new(properties: LtrPreBackupResponseProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Response for the pre-backup request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtrPreBackupResponseProperties {
    #[doc = "Number of storage containers the plugin will use during backup. More than one containers may be used for size limitations, parallelism, or redundancy etc."]
    #[serde(rename = "numberOfContainers")]
    pub number_of_containers: i32,
}
impl LtrPreBackupResponseProperties {
    pub fn new(number_of_containers: i32) -> Self {
        Self { number_of_containers }
    }
}
#[doc = "Response for the LTR backup Operation API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LtrServerBackupOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Response for the backup request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LtrBackupOperationResponseProperties>,
}
impl LtrServerBackupOperation {
    pub fn new() -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties: None,
        }
    }
}
#[doc = "A list of long term retention backup operations for server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LtrServerBackupOperationList {
    #[doc = "The list of long term retention server backup operations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LtrServerBackupOperation>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LtrServerBackupOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LtrServerBackupOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window properties of a server."]
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
#[doc = "Migration details level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationDetailsLevel")]
pub enum MigrationDetailsLevel {
    Default,
    Summary,
    Full,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MigrationDetailsLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MigrationDetailsLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MigrationDetailsLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("MigrationDetailsLevel", 0u32, "Default"),
            Self::Summary => serializer.serialize_unit_variant("MigrationDetailsLevel", 1u32, "Summary"),
            Self::Full => serializer.serialize_unit_variant("MigrationDetailsLevel", 2u32, "Full"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "There are two types of migration modes Online and Offline"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationMode")]
pub enum MigrationMode {
    Offline,
    Online,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MigrationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MigrationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MigrationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Offline => serializer.serialize_unit_variant("MigrationMode", 0u32, "Offline"),
            Self::Online => serializer.serialize_unit_variant("MigrationMode", 1u32, "Online"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Migration name availability reason."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationNameAvailabilityReason")]
pub enum MigrationNameAvailabilityReason {
    Invalid,
    AlreadyExists,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MigrationNameAvailabilityReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MigrationNameAvailabilityReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MigrationNameAvailabilityReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("MigrationNameAvailabilityReason", 0u32, "Invalid"),
            Self::AlreadyExists => serializer.serialize_unit_variant("MigrationNameAvailabilityReason", 1u32, "AlreadyExists"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a migration name's availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationNameAvailabilityResource {
    #[doc = "The resource name to verify."]
    pub name: String,
    #[doc = "The type of the resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Indicates whether the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Migration name availability reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<MigrationNameAvailabilityReason>,
    #[doc = "Migration name availability message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl MigrationNameAvailabilityResource {
    pub fn new(name: String, type_: String) -> Self {
        Self {
            name,
            type_,
            name_available: None,
            reason: None,
            message: None,
        }
    }
}
#[doc = "Represents a migration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Migration resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrationResourceProperties>,
}
impl MigrationResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Represents a migration resource for patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationResourceForPatch {
    #[doc = "Migration resource properties for patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrationResourcePropertiesForPatch>,
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MigrationResourceForPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of migration resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationResourceListResult {
    #[doc = "A list of migration resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MigrationResource>,
    #[doc = "The link used to get the next page of migrations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MigrationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MigrationResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationResourceProperties {
    #[doc = "ID for migration, a GUID."]
    #[serde(rename = "migrationId", default, skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<String>,
    #[doc = "Migration status."]
    #[serde(rename = "currentStatus", default, skip_serializing_if = "Option::is_none")]
    pub current_status: Option<MigrationStatus>,
    #[doc = "There are two types of migration modes Online and Offline"]
    #[serde(rename = "migrationMode", default, skip_serializing_if = "Option::is_none")]
    pub migration_mode: Option<MigrationMode>,
    #[doc = "Database server metadata."]
    #[serde(rename = "sourceDbServerMetadata", default, skip_serializing_if = "Option::is_none")]
    pub source_db_server_metadata: Option<DbServerMetadata>,
    #[doc = "Database server metadata."]
    #[serde(rename = "targetDbServerMetadata", default, skip_serializing_if = "Option::is_none")]
    pub target_db_server_metadata: Option<DbServerMetadata>,
    #[doc = "ResourceId of the source database server"]
    #[serde(rename = "sourceDbServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_db_server_resource_id: Option<String>,
    #[doc = "Source server fully qualified domain name or ip. It is a optional value, if customer provide it, dms will always use it for connection"]
    #[serde(
        rename = "sourceDbServerFullyQualifiedDomainName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_db_server_fully_qualified_domain_name: Option<String>,
    #[doc = "ResourceId of the source database server"]
    #[serde(rename = "targetDbServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_db_server_resource_id: Option<String>,
    #[doc = "Target server fully qualified domain name or ip. It is a optional value, if customer provide it, dms will always use it for connection"]
    #[serde(
        rename = "targetDbServerFullyQualifiedDomainName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_db_server_fully_qualified_domain_name: Option<String>,
    #[doc = "Migration secret parameters."]
    #[serde(rename = "secretParameters", default, skip_serializing_if = "Option::is_none")]
    pub secret_parameters: Option<MigrationSecretParameters>,
    #[doc = "Number of databases to migrate"]
    #[serde(
        rename = "dbsToMigrate",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dbs_to_migrate: Vec<String>,
    #[doc = "Indicates whether to setup LogicalReplicationOnSourceDb, if needed"]
    #[serde(
        rename = "setupLogicalReplicationOnSourceDbIfNeeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub setup_logical_replication_on_source_db_if_needed: Option<migration_resource_properties::SetupLogicalReplicationOnSourceDbIfNeeded>,
    #[doc = "Indicates whether the databases on the target server can be overwritten, if already present. If set to False, the migration workflow will wait for a confirmation, if it detects that the database already exists."]
    #[serde(rename = "overwriteDbsInTarget", default, skip_serializing_if = "Option::is_none")]
    pub overwrite_dbs_in_target: Option<migration_resource_properties::OverwriteDbsInTarget>,
    #[doc = "Start time in UTC for migration window"]
    #[serde(rename = "migrationWindowStartTimeInUtc", default, with = "azure_core::date::rfc3339::option")]
    pub migration_window_start_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "End time in UTC for migration window"]
    #[serde(rename = "migrationWindowEndTimeInUtc", default, with = "azure_core::date::rfc3339::option")]
    pub migration_window_end_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether the data migration should start right away"]
    #[serde(rename = "startDataMigration", default, skip_serializing_if = "Option::is_none")]
    pub start_data_migration: Option<migration_resource_properties::StartDataMigration>,
    #[doc = "To trigger cutover for entire migration we need to send this flag as True"]
    #[serde(rename = "triggerCutover", default, skip_serializing_if = "Option::is_none")]
    pub trigger_cutover: Option<migration_resource_properties::TriggerCutover>,
    #[doc = "When you want to trigger cutover for specific databases send triggerCutover flag as True and database names in this array"]
    #[serde(
        rename = "dbsToTriggerCutoverOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dbs_to_trigger_cutover_on: Vec<String>,
    #[doc = "To trigger cancel for entire migration we need to send this flag as True"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel: Option<migration_resource_properties::Cancel>,
    #[doc = "When you want to trigger cancel for specific databases send cancel flag as True and database names in this array"]
    #[serde(
        rename = "dbsToCancelMigrationOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dbs_to_cancel_migration_on: Vec<String>,
}
impl MigrationResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migration_resource_properties {
    use super::*;
    #[doc = "Indicates whether to setup LogicalReplicationOnSourceDb, if needed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SetupLogicalReplicationOnSourceDbIfNeeded")]
    pub enum SetupLogicalReplicationOnSourceDbIfNeeded {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SetupLogicalReplicationOnSourceDbIfNeeded {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SetupLogicalReplicationOnSourceDbIfNeeded {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SetupLogicalReplicationOnSourceDbIfNeeded {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("SetupLogicalReplicationOnSourceDbIfNeeded", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("SetupLogicalReplicationOnSourceDbIfNeeded", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether the databases on the target server can be overwritten, if already present. If set to False, the migration workflow will wait for a confirmation, if it detects that the database already exists."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OverwriteDbsInTarget")]
    pub enum OverwriteDbsInTarget {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OverwriteDbsInTarget {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OverwriteDbsInTarget {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OverwriteDbsInTarget {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("OverwriteDbsInTarget", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("OverwriteDbsInTarget", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether the data migration should start right away"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartDataMigration")]
    pub enum StartDataMigration {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StartDataMigration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartDataMigration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartDataMigration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("StartDataMigration", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("StartDataMigration", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "To trigger cutover for entire migration we need to send this flag as True"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerCutover")]
    pub enum TriggerCutover {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerCutover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerCutover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerCutover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("TriggerCutover", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("TriggerCutover", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "To trigger cancel for entire migration we need to send this flag as True"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Cancel")]
    pub enum Cancel {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Cancel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Cancel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Cancel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Cancel", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Cancel", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Migration resource properties for patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationResourcePropertiesForPatch {
    #[doc = "ResourceId of the source database server"]
    #[serde(rename = "sourceDbServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_db_server_resource_id: Option<String>,
    #[doc = "Source server fully qualified domain name or ip. It is a optional value, if customer provide it, dms will always use it for connection"]
    #[serde(
        rename = "sourceDbServerFullyQualifiedDomainName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_db_server_fully_qualified_domain_name: Option<String>,
    #[doc = "Target server fully qualified domain name or ip. It is a optional value, if customer provide it, dms will always use it for connection"]
    #[serde(
        rename = "targetDbServerFullyQualifiedDomainName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_db_server_fully_qualified_domain_name: Option<String>,
    #[doc = "Migration secret parameters."]
    #[serde(rename = "secretParameters", default, skip_serializing_if = "Option::is_none")]
    pub secret_parameters: Option<MigrationSecretParameters>,
    #[doc = "Number of databases to migrate"]
    #[serde(
        rename = "dbsToMigrate",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dbs_to_migrate: Vec<String>,
    #[doc = "Indicates whether to setup LogicalReplicationOnSourceDb, if needed"]
    #[serde(
        rename = "setupLogicalReplicationOnSourceDbIfNeeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub setup_logical_replication_on_source_db_if_needed:
        Option<migration_resource_properties_for_patch::SetupLogicalReplicationOnSourceDbIfNeeded>,
    #[doc = "Indicates whether the databases on the target server can be overwritten, if already present. If set to False, the migration workflow will wait for a confirmation, if it detects that the database already exists."]
    #[serde(rename = "overwriteDbsInTarget", default, skip_serializing_if = "Option::is_none")]
    pub overwrite_dbs_in_target: Option<migration_resource_properties_for_patch::OverwriteDbsInTarget>,
    #[doc = "Start time in UTC for migration window"]
    #[serde(rename = "migrationWindowStartTimeInUtc", default, with = "azure_core::date::rfc3339::option")]
    pub migration_window_start_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether the data migration should start right away"]
    #[serde(rename = "startDataMigration", default, skip_serializing_if = "Option::is_none")]
    pub start_data_migration: Option<migration_resource_properties_for_patch::StartDataMigration>,
    #[doc = "To trigger cutover for entire migration we need to send this flag as True"]
    #[serde(rename = "triggerCutover", default, skip_serializing_if = "Option::is_none")]
    pub trigger_cutover: Option<migration_resource_properties_for_patch::TriggerCutover>,
    #[doc = "When you want to trigger cutover for specific databases send triggerCutover flag as True and database names in this array"]
    #[serde(
        rename = "dbsToTriggerCutoverOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dbs_to_trigger_cutover_on: Vec<String>,
    #[doc = "To trigger cancel for entire migration we need to send this flag as True"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel: Option<migration_resource_properties_for_patch::Cancel>,
    #[doc = "When you want to trigger cancel for specific databases send cancel flag as True and database names in this array"]
    #[serde(
        rename = "dbsToCancelMigrationOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dbs_to_cancel_migration_on: Vec<String>,
    #[doc = "There are two types of migration modes Online and Offline"]
    #[serde(rename = "migrationMode", default, skip_serializing_if = "Option::is_none")]
    pub migration_mode: Option<MigrationMode>,
}
impl MigrationResourcePropertiesForPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migration_resource_properties_for_patch {
    use super::*;
    #[doc = "Indicates whether to setup LogicalReplicationOnSourceDb, if needed"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SetupLogicalReplicationOnSourceDbIfNeeded")]
    pub enum SetupLogicalReplicationOnSourceDbIfNeeded {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SetupLogicalReplicationOnSourceDbIfNeeded {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SetupLogicalReplicationOnSourceDbIfNeeded {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SetupLogicalReplicationOnSourceDbIfNeeded {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("SetupLogicalReplicationOnSourceDbIfNeeded", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("SetupLogicalReplicationOnSourceDbIfNeeded", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether the databases on the target server can be overwritten, if already present. If set to False, the migration workflow will wait for a confirmation, if it detects that the database already exists."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OverwriteDbsInTarget")]
    pub enum OverwriteDbsInTarget {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OverwriteDbsInTarget {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OverwriteDbsInTarget {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OverwriteDbsInTarget {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("OverwriteDbsInTarget", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("OverwriteDbsInTarget", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether the data migration should start right away"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartDataMigration")]
    pub enum StartDataMigration {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StartDataMigration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartDataMigration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartDataMigration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("StartDataMigration", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("StartDataMigration", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "To trigger cutover for entire migration we need to send this flag as True"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerCutover")]
    pub enum TriggerCutover {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerCutover {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerCutover {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerCutover {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("TriggerCutover", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("TriggerCutover", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "To trigger cancel for entire migration we need to send this flag as True"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Cancel")]
    pub enum Cancel {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Cancel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Cancel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Cancel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Cancel", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Cancel", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Migration secret parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationSecretParameters {
    #[doc = "Server admin credentials."]
    #[serde(rename = "adminCredentials")]
    pub admin_credentials: AdminCredentials,
    #[doc = "Gets or sets the username for the source server. This user need not be an admin."]
    #[serde(rename = "sourceServerUsername", default, skip_serializing_if = "Option::is_none")]
    pub source_server_username: Option<String>,
    #[doc = "Gets or sets the username for the target server. This user need not be an admin."]
    #[serde(rename = "targetServerUsername", default, skip_serializing_if = "Option::is_none")]
    pub target_server_username: Option<String>,
}
impl MigrationSecretParameters {
    pub fn new(admin_credentials: AdminCredentials) -> Self {
        Self {
            admin_credentials,
            source_server_username: None,
            target_server_username: None,
        }
    }
}
#[doc = "Migration state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationState")]
pub enum MigrationState {
    InProgress,
    WaitingForUserAction,
    Canceled,
    Failed,
    Succeeded,
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
            Self::InProgress => serializer.serialize_unit_variant("MigrationState", 0u32, "InProgress"),
            Self::WaitingForUserAction => serializer.serialize_unit_variant("MigrationState", 1u32, "WaitingForUserAction"),
            Self::Canceled => serializer.serialize_unit_variant("MigrationState", 2u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("MigrationState", 3u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("MigrationState", 4u32, "Succeeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Migration status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationStatus {
    #[doc = "Migration state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MigrationState>,
    #[doc = "Error message, if any, for the migration state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Migration sub state details."]
    #[serde(rename = "currentSubStateDetails", default, skip_serializing_if = "Option::is_none")]
    pub current_sub_state_details: Option<MigrationSubStateDetails>,
}
impl MigrationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration sub state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MigrationSubState")]
pub enum MigrationSubState {
    PerformingPreRequisiteSteps,
    #[serde(rename = "WaitingForLogicalReplicationSetupRequestOnSourceDB")]
    WaitingForLogicalReplicationSetupRequestOnSourceDb,
    WaitingForDBsToMigrateSpecification,
    #[serde(rename = "WaitingForTargetDBOverwriteConfirmation")]
    WaitingForTargetDbOverwriteConfirmation,
    WaitingForDataMigrationScheduling,
    WaitingForDataMigrationWindow,
    MigratingData,
    WaitingForCutoverTrigger,
    CompletingMigration,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MigrationSubState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MigrationSubState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MigrationSubState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PerformingPreRequisiteSteps => {
                serializer.serialize_unit_variant("MigrationSubState", 0u32, "PerformingPreRequisiteSteps")
            }
            Self::WaitingForLogicalReplicationSetupRequestOnSourceDb => {
                serializer.serialize_unit_variant("MigrationSubState", 1u32, "WaitingForLogicalReplicationSetupRequestOnSourceDB")
            }
            Self::WaitingForDBsToMigrateSpecification => {
                serializer.serialize_unit_variant("MigrationSubState", 2u32, "WaitingForDBsToMigrateSpecification")
            }
            Self::WaitingForTargetDbOverwriteConfirmation => {
                serializer.serialize_unit_variant("MigrationSubState", 3u32, "WaitingForTargetDBOverwriteConfirmation")
            }
            Self::WaitingForDataMigrationScheduling => {
                serializer.serialize_unit_variant("MigrationSubState", 4u32, "WaitingForDataMigrationScheduling")
            }
            Self::WaitingForDataMigrationWindow => {
                serializer.serialize_unit_variant("MigrationSubState", 5u32, "WaitingForDataMigrationWindow")
            }
            Self::MigratingData => serializer.serialize_unit_variant("MigrationSubState", 6u32, "MigratingData"),
            Self::WaitingForCutoverTrigger => serializer.serialize_unit_variant("MigrationSubState", 7u32, "WaitingForCutoverTrigger"),
            Self::CompletingMigration => serializer.serialize_unit_variant("MigrationSubState", 8u32, "CompletingMigration"),
            Self::Completed => serializer.serialize_unit_variant("MigrationSubState", 9u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Migration sub state details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationSubStateDetails {
    #[doc = "Migration sub state."]
    #[serde(rename = "currentSubState", default, skip_serializing_if = "Option::is_none")]
    pub current_sub_state: Option<MigrationSubState>,
}
impl MigrationSubStateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a resource name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[serde(flatten)]
    pub check_name_availability_response: CheckNameAvailabilityResponse,
    #[doc = "name of the PostgreSQL server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "type of the server"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[doc = "public network access is enabled or not"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<network::PublicNetworkAccess>,
    #[doc = "Delegated subnet arm resource id. This is required to be passed during create, in case we want the server to be VNET injected, i.e. Private access server. During update, pass this only if we want to update the value for Private DNS zone."]
    #[serde(rename = "delegatedSubnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_subnet_resource_id: Option<String>,
    #[doc = "Private dns zone arm resource id. This is required to be passed during create, in case we want the server to be VNET injected, i.e. Private access server. During update, pass this only if we want to update the value for Private DNS zone."]
    #[serde(rename = "privateDnsZoneArmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_dns_zone_arm_resource_id: Option<String>,
}
impl Network {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network {
    use super::*;
    #[doc = "public network access is enabled or not"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
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
        NotSpecified,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Origin", 0u32, "NotSpecified"),
                Self::User => serializer.serialize_unit_variant("Origin", 1u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 2u32, "system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PrivateDnsZoneSuffix = String;
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
#[doc = "Used to indicate role of the server in replication set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicationRole")]
pub enum ReplicationRole {
    None,
    Primary,
    AsyncReplica,
    GeoAsyncReplica,
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
            Self::Primary => serializer.serialize_unit_variant("ReplicationRole", 1u32, "Primary"),
            Self::AsyncReplica => serializer.serialize_unit_variant("ReplicationRole", 2u32, "AsyncReplica"),
            Self::GeoAsyncReplica => serializer.serialize_unit_variant("ReplicationRole", 3u32, "GeoAsyncReplica"),
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents server restart parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestartParameter {
    #[doc = "Indicates whether to restart the server with failover."]
    #[serde(rename = "restartWithFailover", default, skip_serializing_if = "Option::is_none")]
    pub restart_with_failover: Option<bool>,
    #[doc = "Failover mode."]
    #[serde(rename = "failoverMode", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<restart_parameter::FailoverMode>,
}
impl RestartParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restart_parameter {
    use super::*;
    #[doc = "Failover mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverMode")]
    pub enum FailoverMode {
        PlannedFailover,
        ForcedFailover,
        PlannedSwitchover,
        ForcedSwitchover,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PlannedFailover => serializer.serialize_unit_variant("FailoverMode", 0u32, "PlannedFailover"),
                Self::ForcedFailover => serializer.serialize_unit_variant("FailoverMode", 1u32, "ForcedFailover"),
                Self::PlannedSwitchover => serializer.serialize_unit_variant("FailoverMode", 2u32, "PlannedSwitchover"),
                Self::ForcedSwitchover => serializer.serialize_unit_variant("FailoverMode", 3u32, "ForcedSwitchover"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Sku information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Information describing the identities associated with this application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedIdentity>,
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
            sku: None,
            identity: None,
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
    pub backup_type: Option<server_backup_properties::BackupType>,
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
pub mod server_backup_properties {
    use super::*;
    #[doc = "Backup type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupType")]
    pub enum BackupType {
        Full,
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
                Self::Full => serializer.serialize_unit_variant("BackupType", 0u32, "Full"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a server to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerForUpdate {
    #[doc = "Sku information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Information describing the identities associated with this application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedIdentity>,
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
#[doc = "A list of servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerListResult {
    #[doc = "The list of flexible servers"]
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
    #[doc = "The administrator login password (required for server creation)."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The version of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[doc = "The minor version of the server."]
    #[serde(rename = "minorVersion", default, skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    #[doc = "A state of a server that is visible to user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<server_properties::State>,
    #[doc = "The fully qualified domain name of a server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[doc = "Storage properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[doc = "Authentication configuration properties of a server"]
    #[serde(rename = "authConfig", default, skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<AuthConfig>,
    #[doc = "Data encryption properties of a server"]
    #[serde(rename = "dataEncryption", default, skip_serializing_if = "Option::is_none")]
    pub data_encryption: Option<DataEncryption>,
    #[doc = "Backup properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[doc = "Network properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[doc = "High availability properties of a server"]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[doc = "Maintenance window properties of a server."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "The source server resource ID to restore from. It's required when 'createMode' is 'PointInTimeRestore' or 'GeoRestore' or 'Replica' or 'ReviveDropped'. This property is returned only for Replica server"]
    #[serde(rename = "sourceServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_server_resource_id: Option<String>,
    #[doc = "Restore point creation time (ISO8601 format), specifying the time to restore from. It's required when 'createMode' is 'PointInTimeRestore' or 'GeoRestore' or 'ReviveDropped'."]
    #[serde(rename = "pointInTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub point_in_time_utc: Option<time::OffsetDateTime>,
    #[doc = "availability zone information of the server."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "Used to indicate role of the server in replication set."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<ReplicationRole>,
    #[doc = "Replicas allowed for a server."]
    #[serde(rename = "replicaCapacity", default, skip_serializing_if = "Option::is_none")]
    pub replica_capacity: Option<i32>,
    #[doc = "The mode to create a new PostgreSQL server."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_properties::CreateMode>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_properties {
    use super::*;
    #[doc = "A state of a server that is visible to user."]
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
    #[doc = "The mode to create a new PostgreSQL server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        Create,
        Update,
        PointInTimeRestore,
        GeoRestore,
        Replica,
        ReviveDropped,
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
                Self::Create => serializer.serialize_unit_variant("CreateMode", 1u32, "Create"),
                Self::Update => serializer.serialize_unit_variant("CreateMode", 2u32, "Update"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 3u32, "PointInTimeRestore"),
                Self::GeoRestore => serializer.serialize_unit_variant("CreateMode", 4u32, "GeoRestore"),
                Self::Replica => serializer.serialize_unit_variant("CreateMode", 5u32, "Replica"),
                Self::ReviveDropped => serializer.serialize_unit_variant("CreateMode", 6u32, "ReviveDropped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerPropertiesForUpdate {
    #[doc = "The password of the administrator login."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The version of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[doc = "Storage properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[doc = "Backup properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[doc = "High availability properties of a server"]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[doc = "Maintenance window properties of a server."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "Authentication configuration properties of a server"]
    #[serde(rename = "authConfig", default, skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<AuthConfig>,
    #[doc = "Data encryption properties of a server"]
    #[serde(rename = "dataEncryption", default, skip_serializing_if = "Option::is_none")]
    pub data_encryption: Option<DataEncryption>,
    #[doc = "The mode to update a new PostgreSQL server."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_properties_for_update::CreateMode>,
    #[doc = "Used to indicate role of the server in replication set."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<ReplicationRole>,
    #[doc = "Network properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}
impl ServerPropertiesForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_properties_for_update {
    use super::*;
    #[doc = "The mode to update a new PostgreSQL server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        Update,
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
                Self::Update => serializer.serialize_unit_variant("CreateMode", 1u32, "Update"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Sku information related properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSku {
    #[doc = "The name of the sku, typically, tier + family + cores, e.g. Standard_D4s_v3."]
    pub name: String,
    #[doc = "The tier of the particular SKU, e.g. Burstable."]
    pub tier: server_sku::Tier,
}
impl ServerSku {
    pub fn new(name: String, tier: server_sku::Tier) -> Self {
        Self { name, tier }
    }
}
pub mod server_sku {
    use super::*;
    #[doc = "The tier of the particular SKU, e.g. Burstable."]
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
pub struct ServerSkuCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Sku name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Supported vCores"]
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i32>,
    #[doc = "Supported IOPS"]
    #[serde(rename = "supportedIops", default, skip_serializing_if = "Option::is_none")]
    pub supported_iops: Option<i32>,
    #[doc = "Supported memory per vCore in MB"]
    #[serde(rename = "supportedMemoryPerVcoreMb", default, skip_serializing_if = "Option::is_none")]
    pub supported_memory_per_vcore_mb: Option<i64>,
    #[doc = "List of supported Availability Zones. E.g. \"1\", \"2\", \"3\""]
    #[serde(
        rename = "supportedZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_zones: Vec<String>,
    #[doc = "Supported high availability mode"]
    #[serde(
        rename = "supportedHaMode",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_ha_mode: Vec<String>,
}
impl ServerSkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The version of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerVersion")]
pub enum ServerVersion {
    #[serde(rename = "15")]
    N15,
    #[serde(rename = "14")]
    N14,
    #[serde(rename = "13")]
    N13,
    #[serde(rename = "12")]
    N12,
    #[serde(rename = "11")]
    N11,
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
            Self::N15 => serializer.serialize_unit_variant("ServerVersion", 0u32, "15"),
            Self::N14 => serializer.serialize_unit_variant("ServerVersion", 1u32, "14"),
            Self::N13 => serializer.serialize_unit_variant("ServerVersion", 2u32, "13"),
            Self::N12 => serializer.serialize_unit_variant("ServerVersion", 3u32, "12"),
            Self::N11 => serializer.serialize_unit_variant("ServerVersion", 4u32, "11"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Server version capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVersionCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Server version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Supported servers versions to upgrade"]
    #[serde(
        rename = "supportedVersionsToUpgrade",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_versions_to_upgrade: Vec<String>,
}
impl ServerVersionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku information related properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the sku, typically, tier + family + cores, e.g. Standard_D4s_v3."]
    pub name: String,
    #[doc = "The tier of the particular SKU, e.g. Burstable."]
    pub tier: sku::Tier,
}
impl Sku {
    pub fn new(name: String, tier: sku::Tier) -> Self {
        Self { name, tier }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The tier of the particular SKU, e.g. Burstable."]
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
#[doc = "Storage properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Storage {
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "storageSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_gb: Option<i32>,
    #[doc = "Flag to enable / disable Storage Auto grow for flexible server."]
    #[serde(rename = "autoGrow", default, skip_serializing_if = "Option::is_none")]
    pub auto_grow: Option<storage::AutoGrow>,
    #[doc = "Name of storage tier for IOPS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<storage::Tier>,
    #[doc = "Storage tier IOPS quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
}
impl Storage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage {
    use super::*;
    #[doc = "Flag to enable / disable Storage Auto grow for flexible server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoGrow")]
    pub enum AutoGrow {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoGrow {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoGrow {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoGrow {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("AutoGrow", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("AutoGrow", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Name of storage tier for IOPS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        P1,
        P2,
        P3,
        P4,
        P6,
        P10,
        P15,
        P20,
        P30,
        P40,
        P50,
        P60,
        P70,
        P80,
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
                Self::P1 => serializer.serialize_unit_variant("Tier", 0u32, "P1"),
                Self::P2 => serializer.serialize_unit_variant("Tier", 1u32, "P2"),
                Self::P3 => serializer.serialize_unit_variant("Tier", 2u32, "P3"),
                Self::P4 => serializer.serialize_unit_variant("Tier", 3u32, "P4"),
                Self::P6 => serializer.serialize_unit_variant("Tier", 4u32, "P6"),
                Self::P10 => serializer.serialize_unit_variant("Tier", 5u32, "P10"),
                Self::P15 => serializer.serialize_unit_variant("Tier", 6u32, "P15"),
                Self::P20 => serializer.serialize_unit_variant("Tier", 7u32, "P20"),
                Self::P30 => serializer.serialize_unit_variant("Tier", 8u32, "P30"),
                Self::P40 => serializer.serialize_unit_variant("Tier", 9u32, "P40"),
                Self::P50 => serializer.serialize_unit_variant("Tier", 10u32, "P50"),
                Self::P60 => serializer.serialize_unit_variant("Tier", 11u32, "P60"),
                Self::P70 => serializer.serialize_unit_variant("Tier", 12u32, "P70"),
                Self::P80 => serializer.serialize_unit_variant("Tier", 13u32, "P80"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage edition capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageEditionCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Storage edition name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Default storage size in MB for storage edition"]
    #[serde(rename = "defaultStorageSizeMb", default, skip_serializing_if = "Option::is_none")]
    pub default_storage_size_mb: Option<i64>,
    #[doc = "Flexible server supported storage range in MB"]
    #[serde(
        rename = "supportedStorageMb",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_mb: Vec<StorageMbCapability>,
}
impl StorageEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "storage size in MB capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMbCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Supported IOPS"]
    #[serde(rename = "supportedIops", default, skip_serializing_if = "Option::is_none")]
    pub supported_iops: Option<i32>,
    #[doc = "Storage size in MB"]
    #[serde(rename = "storageSizeMb", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_mb: Option<i64>,
    #[doc = "Default tier for IOPS"]
    #[serde(rename = "defaultIopsTier", default, skip_serializing_if = "Option::is_none")]
    pub default_iops_tier: Option<String>,
    #[doc = "List of available options to upgrade the storage performance"]
    #[serde(
        rename = "supportedIopsTiers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_iops_tiers: Vec<StorageTierCapability>,
}
impl StorageMbCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents capability of a storage tier"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTierCapability {
    #[serde(flatten)]
    pub capability_base: CapabilityBase,
    #[doc = "Name to represent Storage tier capability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Supported IOPS for this storage tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
}
impl StorageTierCapability {
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
#[doc = "Information describing the identities associated with this application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentity {
    #[doc = "Defines a map that contains user assigned identities."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentityMap>,
    #[doc = "the types of identities associated with this resource; currently restricted to 'None and UserAssigned'"]
    #[serde(rename = "type")]
    pub type_: user_assigned_identity::Type,
    #[doc = "Tenant id of the server."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new(type_: user_assigned_identity::Type) -> Self {
        Self {
            user_assigned_identities: None,
            type_,
            tenant_id: None,
        }
    }
}
pub mod user_assigned_identity {
    use super::*;
    #[doc = "the types of identities associated with this resource; currently restricted to 'None and UserAssigned'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines a map that contains user assigned identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityMap {}
impl UserAssignedIdentityMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a single user-assigned identity associated with the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "the object identifier of the Service Principal which this identity represents."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "the client identifier of the Service Principal which this identity represents."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual network subnet usage parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkSubnetUsageParameter {
    #[doc = "Virtual network resource id."]
    #[serde(rename = "virtualNetworkArmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_arm_resource_id: Option<String>,
}
impl VirtualNetworkSubnetUsageParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual network subnet usage data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkSubnetUsageResult {
    #[serde(
        rename = "delegatedSubnetsUsage",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delegated_subnets_usage: Vec<DelegatedSubnetUsage>,
    #[doc = "location of the delegated subnet usage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "subscriptionId of the delegated subnet usage"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
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
