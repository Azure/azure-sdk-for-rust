#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "Represents a resource name of a cached server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CachedServerName {
    #[doc = "The name of available cached server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CachedServerName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request from client to get available cached server name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedServerNameRequest {
    #[doc = "The version of a server."]
    pub version: ServerVersion,
    #[doc = "Storage properties of a server"]
    pub storage: Storage,
    #[doc = "Sku information related properties of a server."]
    pub sku: Sku,
}
impl CachedServerNameRequest {
    pub fn new(version: ServerVersion, storage: Storage, sku: Sku) -> Self {
        Self { version, storage, sku }
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
    #[doc = "A value indicating whether a new server in this region can have geo-backups to paired region."]
    #[serde(rename = "geoBackupSupported", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_supported: Option<bool>,
    #[doc = "A value indicating whether a new server in this region can support multi zone HA."]
    #[serde(rename = "zoneRedundantHaSupported", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant_ha_supported: Option<bool>,
    #[doc = "A value indicating whether a new server in this region can have geo-backups to paired region."]
    #[serde(rename = "zoneRedundantHaAndGeoBackupSupported", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant_ha_and_geo_backup_supported: Option<bool>,
    #[serde(
        rename = "supportedFlexibleServerEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_flexible_server_editions: Vec<FlexibleServerEditionCapability>,
    #[serde(
        rename = "supportedHyperscaleNodeEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_hyperscale_node_editions: Vec<HyperscaleNodeEditionCapability>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
#[doc = "Flexible server edition capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlexibleServerEditionCapability {
    #[doc = "Server edition name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of editions supported by this server edition."]
    #[serde(
        rename = "supportedStorageEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_editions: Vec<StorageEditionCapability>,
    #[doc = "The list of server versions supported by this server edition."]
    #[serde(
        rename = "supportedServerVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
#[doc = "Hyperscale node edition capabilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperscaleNodeEditionCapability {
    #[doc = "Server edition name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of editions supported by this server edition."]
    #[serde(
        rename = "supportedStorageEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_editions: Vec<StorageEditionCapability>,
    #[doc = "The list of server versions supported by this server edition."]
    #[serde(
        rename = "supportedServerVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[doc = "The list of Node Types supported by this server edition."]
    #[serde(
        rename = "supportedNodeTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_node_types: Vec<NodeTypeCapability>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl HyperscaleNodeEditionCapability {
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
#[doc = "Represents a resource name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Error Message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Indicates whether the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "name of the PostgreSQL server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "type of the server"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name availability reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<name_availability::Reason>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod name_availability {
    use super::*;
    #[doc = "The name availability reason."]
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
#[doc = "Network properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[doc = "public network access is enabled or not"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<network::PublicNetworkAccess>,
    #[doc = "delegated subnet arm resource id."]
    #[serde(rename = "delegatedSubnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_subnet_resource_id: Option<String>,
    #[doc = "private dns zone arm resource id."]
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
#[doc = "node type capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeCapability {
    #[doc = "note type name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "note type"]
    #[serde(rename = "nodeType", default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl NodeTypeCapability {
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
#[doc = "Represents a server to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerForUpdate {
    #[doc = "The location the resource resides in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Sku information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
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
    #[doc = "Backup properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[doc = "Network properties of a server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[doc = "High availability properties of a server"]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[doc = "Maintenance window properties of a server."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "The source server resource ID to restore from. It's required when 'createMode' is 'PointInTimeRestore'."]
    #[serde(rename = "sourceServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_server_resource_id: Option<String>,
    #[doc = "Restore point creation time (ISO8601 format), specifying the time to restore from. It's required when 'createMode' is 'PointInTimeRestore'."]
    #[serde(rename = "pointInTimeUTC", default, with = "azure_core::date::rfc3339::option")]
    pub point_in_time_utc: Option<time::OffsetDateTime>,
    #[doc = "availability zone information of the server."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
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
    #[doc = "The mode to update a new PostgreSQL server."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_properties_for_update::CreateMode>,
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
#[doc = "The version of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerVersion")]
pub enum ServerVersion {
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
            Self::N14 => serializer.serialize_unit_variant("ServerVersion", 0u32, "14"),
            Self::N13 => serializer.serialize_unit_variant("ServerVersion", 1u32, "13"),
            Self::N12 => serializer.serialize_unit_variant("ServerVersion", 2u32, "12"),
            Self::N11 => serializer.serialize_unit_variant("ServerVersion", 3u32, "11"),
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
    #[serde(
        rename = "supportedVcores",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_vcores: Vec<VcoreCapability>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    #[serde(
        rename = "supportedStorageMB",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_mb: Vec<StorageMbCapability>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl StorageEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "storage size in MB capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMbCapability {
    #[doc = "storage MB name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "supported IOPS"]
    #[serde(rename = "supportedIops", default, skip_serializing_if = "Option::is_none")]
    pub supported_iops: Option<i64>,
    #[doc = "storage size in MB"]
    #[serde(rename = "storageSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_mb: Option<i64>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl StorageMbCapability {
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
#[doc = "Vcores capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VcoreCapability {
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
    #[serde(rename = "supportedMemoryPerVcoreMB", default, skip_serializing_if = "Option::is_none")]
    pub supported_memory_per_vcore_mb: Option<i64>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl VcoreCapability {
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
