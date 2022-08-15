#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Batch service."]
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
#[doc = "An error response from the Batch service."]
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
}
impl CloudErrorBody {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
}
impl azure_core::Continuable for ConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
    pub data_type: Option<String>,
    #[doc = "Allowed values of the configuration."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[doc = "Source of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl ConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
impl azure_core::Continuable for DatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
#[doc = "Represents a server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a server firewall rule."]
    pub properties: FirewallRuleProperties,
}
impl FirewallRule {
    pub fn new(properties: FirewallRuleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "A list of firewall rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[doc = "The list of firewall rules in a server."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
}
impl azure_core::Continuable for FirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
#[doc = "Represents a log file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The name of the log file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of a log file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogFileProperties>,
}
impl LogFile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of log files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFileListResult {
    #[doc = "The list of log files."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogFile>,
}
impl azure_core::Continuable for LogFileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LogFileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a log file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFileProperties {
    #[doc = "Size of the log file."]
    #[serde(rename = "sizeInKB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<i64>,
    #[doc = "Creation timestamp of the log file."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Last modified timestamp of the log file."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
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
#[doc = "Enforce a minimal Tls version for the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MinimalTlsVersion")]
pub enum MinimalTlsVersion {
    #[serde(rename = "TLS1_0")]
    Tls10,
    #[serde(rename = "TLS1_1")]
    Tls11,
    #[serde(rename = "TLS1_2")]
    Tls12,
    #[serde(rename = "TLSEnforcementDisabled")]
    TlsEnforcementDisabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MinimalTlsVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MinimalTlsVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MinimalTlsVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Tls10 => serializer.serialize_unit_variant("MinimalTlsVersion", 0u32, "TLS1_0"),
            Self::Tls11 => serializer.serialize_unit_variant("MinimalTlsVersion", 1u32, "TLS1_1"),
            Self::Tls12 => serializer.serialize_unit_variant("MinimalTlsVersion", 2u32, "TLS1_2"),
            Self::TlsEnforcementDisabled => serializer.serialize_unit_variant("MinimalTlsVersion", 3u32, "TLSEnforcementDisabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "The list of resource provider operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of performance tiers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceTierListResult {
    #[doc = "The list of performance tiers"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PerformanceTierProperties>,
}
impl azure_core::Continuable for PerformanceTierListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PerformanceTierListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Performance tier properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceTierProperties {
    #[doc = "ID of the performance tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Maximum Backup retention in days for the performance tier edition"]
    #[serde(rename = "maxBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i32>,
    #[doc = "Minimum Backup retention in days for the performance tier edition"]
    #[serde(rename = "minBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i32>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "maxStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "minLargeStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_large_storage_mb: Option<i32>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "maxLargeStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_large_storage_mb: Option<i32>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "minStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
    #[doc = "Service level objectives associated with the performance tier"]
    #[serde(rename = "serviceLevelObjectives", default, skip_serializing_if = "Vec::is_empty")]
    pub service_level_objectives: Vec<PerformanceTierServiceLevelObjectives>,
}
impl PerformanceTierProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service level objectives for performance tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceTierServiceLevelObjectives {
    #[doc = "ID for the service level objective."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Edition of the performance tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "vCore associated with the service level objective"]
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i32>,
    #[doc = "Hardware generation associated with the service level objective"]
    #[serde(rename = "hardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<String>,
    #[doc = "Maximum Backup retention in days for the performance tier edition"]
    #[serde(rename = "maxBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i32>,
    #[doc = "Minimum Backup retention in days for the performance tier edition"]
    #[serde(rename = "minBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i32>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "maxStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "minStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
}
impl PerformanceTierServiceLevelObjectives {
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
#[doc = "The recoverable server's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableServerProperties {
    #[doc = "The last available backup date time."]
    #[serde(rename = "lastAvailableBackupDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_available_backup_date_time: Option<String>,
    #[doc = "The service level objective"]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[doc = "Edition of the performance tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "vCore associated with the service level objective"]
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i32>,
    #[doc = "Hardware generation associated with the service level objective"]
    #[serde(rename = "hardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<String>,
    #[doc = "The MariaDB version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl RecoverableServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A recoverable server resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableServerResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The recoverable server's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableServerProperties>,
}
impl RecoverableServerResource {
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled."]
    pub state: security_alert_policy_properties::State,
    #[doc = "Specifies an array of alerts that are disabled. Allowed values are: Sql_Injection, Sql_Injection_Vulnerability, Access_Anomaly"]
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[doc = "Specifies an array of e-mail addresses to which the alert is sent."]
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the Threat Detection audit storage account."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the Threat Detection audit logs."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}
impl SecurityAlertPolicyProperties {
    pub fn new(state: security_alert_policy_properties::State) -> Self {
        Self {
            state,
            disabled_alerts: Vec::new(),
            email_addresses: Vec::new(),
            email_account_admins: None,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
        }
    }
}
pub mod security_alert_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "Represents a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "Billing information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}
impl Server {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "Represents a server to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerForCreate {
    #[doc = "Billing information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties used to create a new server."]
    pub properties: ServerPropertiesForCreate,
    #[doc = "The location the resource resides in."]
    pub location: String,
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServerForCreate {
    pub fn new(properties: ServerPropertiesForCreate, location: String) -> Self {
        Self {
            sku: None,
            properties,
            location,
            tags: None,
        }
    }
}
#[doc = "A list of servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerListResult {
    #[doc = "The list of servers"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,
}
impl azure_core::Continuable for ServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
    #[doc = "The version of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[doc = "Enable ssl enforcement or not when connect to server."]
    #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[doc = "Enforce a minimal Tls version for the server."]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[doc = "A state of a server that is visible to user."]
    #[serde(rename = "userVisibleState", default, skip_serializing_if = "Option::is_none")]
    pub user_visible_state: Option<server_properties::UserVisibleState>,
    #[doc = "The fully qualified domain name of a server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[doc = "Earliest restore point creation time (ISO8601 format)"]
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "Storage Profile properties of a server"]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "The replication role of the server."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<String>,
    #[doc = "The master server id of a replica server."]
    #[serde(rename = "masterServerId", default, skip_serializing_if = "Option::is_none")]
    pub master_server_id: Option<String>,
    #[doc = "The maximum number of replicas that a master server can have."]
    #[serde(rename = "replicaCapacity", default, skip_serializing_if = "Option::is_none")]
    pub replica_capacity: Option<i32>,
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
    #[serde(remote = "UserVisibleState")]
    pub enum UserVisibleState {
        Ready,
        Dropping,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserVisibleState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserVisibleState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserVisibleState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ready => serializer.serialize_unit_variant("UserVisibleState", 0u32, "Ready"),
                Self::Dropping => serializer.serialize_unit_variant("UserVisibleState", 1u32, "Dropping"),
                Self::Disabled => serializer.serialize_unit_variant("UserVisibleState", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties used to create a new server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForCreate {
    #[doc = "The version of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[doc = "Enable ssl enforcement or not when connect to server."]
    #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[doc = "Enforce a minimal Tls version for the server."]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[doc = "Storage Profile properties of a server"]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "The mode to create a new server."]
    #[serde(rename = "createMode")]
    pub create_mode: server_properties_for_create::CreateMode,
}
impl ServerPropertiesForCreate {
    pub fn new(create_mode: server_properties_for_create::CreateMode) -> Self {
        Self {
            version: None,
            ssl_enforcement: None,
            minimal_tls_version: None,
            storage_profile: None,
            create_mode,
        }
    }
}
pub mod server_properties_for_create {
    use super::*;
    #[doc = "The mode to create a new server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        GeoRestore,
        Replica,
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
                Self::GeoRestore => serializer.serialize_unit_variant("CreateMode", 2u32, "GeoRestore"),
                Self::Replica => serializer.serialize_unit_variant("CreateMode", 3u32, "Replica"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties used to create a new server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForDefaultCreate {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[doc = "The administrator's login name of a server. Can only be specified when the server is being created (and is required for creation)."]
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[doc = "The password of the administrator login."]
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
}
impl ServerPropertiesForDefaultCreate {
    pub fn new(
        server_properties_for_create: ServerPropertiesForCreate,
        administrator_login: String,
        administrator_login_password: String,
    ) -> Self {
        Self {
            server_properties_for_create,
            administrator_login,
            administrator_login_password,
        }
    }
}
#[doc = "The properties used to create a new server by restoring to a different region from a geo replicated backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForGeoRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[doc = "The source server id to restore from."]
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
impl ServerPropertiesForGeoRestore {
    pub fn new(server_properties_for_create: ServerPropertiesForCreate, source_server_id: String) -> Self {
        Self {
            server_properties_for_create,
            source_server_id,
        }
    }
}
#[doc = "The properties to create a new replica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForReplica {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[doc = "The master server id to create replica from."]
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
impl ServerPropertiesForReplica {
    pub fn new(server_properties_for_create: ServerPropertiesForCreate, source_server_id: String) -> Self {
        Self {
            server_properties_for_create,
            source_server_id,
        }
    }
}
#[doc = "The properties used to create a new server by restoring from a backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[doc = "The source server id to restore from."]
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
    #[doc = "Restore point creation time (ISO8601 format), specifying the time to restore from."]
    #[serde(rename = "restorePointInTime", with = "azure_core::date::rfc3339")]
    pub restore_point_in_time: time::OffsetDateTime,
}
impl ServerPropertiesForRestore {
    pub fn new(
        server_properties_for_create: ServerPropertiesForCreate,
        source_server_id: String,
        restore_point_in_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            server_properties_for_create,
            source_server_id,
            restore_point_in_time,
        }
    }
}
#[doc = "A server security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
}
impl ServerSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the server's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters allowed to update for a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUpdateParameters {
    #[doc = "Billing information related properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties that can be updated for a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<server_update_parameters::Properties>,
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServerUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_update_parameters {
    use super::*;
    #[doc = "The properties that can be updated for a server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Storage Profile properties of a server"]
        #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
        pub storage_profile: Option<StorageProfile>,
        #[doc = "The password of the administrator login."]
        #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
        pub administrator_login_password: Option<String>,
        #[doc = "The version of a server."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<ServerVersion>,
        #[doc = "Enable ssl enforcement or not when connect to server."]
        #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
        pub ssl_enforcement: Option<SslEnforcement>,
        #[doc = "Enforce a minimal Tls version for the server."]
        #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
        pub minimal_tls_version: Option<MinimalTlsVersion>,
        #[doc = "The replication role of the server."]
        #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
        pub replication_role: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The version of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerVersion")]
pub enum ServerVersion {
    #[serde(rename = "10.2")]
    N10_2,
    #[serde(rename = "10.3")]
    N10_3,
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
            Self::N10_2 => serializer.serialize_unit_variant("ServerVersion", 0u32, "10.2"),
            Self::N10_3 => serializer.serialize_unit_variant("ServerVersion", 1u32, "10.3"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Billing information related properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the sku, typically, tier + family + cores, e.g. B_Gen4_1, GP_Gen5_8."]
    pub name: String,
    #[doc = "The tier of the particular SKU, e.g. Basic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[doc = "The scale up/out capacity, representing server's compute units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "The size code, to be interpreted by resource as appropriate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The family of hardware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
            size: None,
            family: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The tier of the particular SKU, e.g. Basic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Basic,
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
                Self::Basic => serializer.serialize_unit_variant("Tier", 0u32, "Basic"),
                Self::GeneralPurpose => serializer.serialize_unit_variant("Tier", 1u32, "GeneralPurpose"),
                Self::MemoryOptimized => serializer.serialize_unit_variant("Tier", 2u32, "MemoryOptimized"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Enable ssl enforcement or not when connect to server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SslEnforcement {
    Enabled,
    Disabled,
}
#[doc = "Storage Profile properties of a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Backup retention days for the server."]
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[doc = "Enable Geo-redundant or not for server backup."]
    #[serde(rename = "geoRedundantBackup", default, skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<storage_profile::GeoRedundantBackup>,
    #[doc = "Max storage allowed for a server."]
    #[serde(rename = "storageMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_mb: Option<i32>,
    #[doc = "Enable Storage Auto Grow."]
    #[serde(rename = "storageAutogrow", default, skip_serializing_if = "Option::is_none")]
    pub storage_autogrow: Option<storage_profile::StorageAutogrow>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_profile {
    use super::*;
    #[doc = "Enable Geo-redundant or not for server backup."]
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
    #[doc = "Enable Storage Auto Grow."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAutogrow")]
    pub enum StorageAutogrow {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAutogrow {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAutogrow {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAutogrow {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("StorageAutogrow", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("StorageAutogrow", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "A virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
impl VirtualNetworkRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of virtual network rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[doc = "The ARM resource id of the virtual network subnet."]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub virtual_network_subnet_id: String,
    #[doc = "Create firewall rule before the virtual network has vnet service endpoint enabled."]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
    #[doc = "Virtual Network Rule State"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<virtual_network_rule_properties::State>,
}
impl VirtualNetworkRuleProperties {
    pub fn new(virtual_network_subnet_id: String) -> Self {
        Self {
            virtual_network_subnet_id,
            ignore_missing_vnet_service_endpoint: None,
            state: None,
        }
    }
}
pub mod virtual_network_rule_properties {
    use super::*;
    #[doc = "Virtual Network Rule State"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Initializing,
        InProgress,
        Ready,
        Deleting,
        Unknown,
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
                Self::Initializing => serializer.serialize_unit_variant("State", 0u32, "Initializing"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Ready => serializer.serialize_unit_variant("State", 2u32, "Ready"),
                Self::Deleting => serializer.serialize_unit_variant("State", 3u32, "Deleting"),
                Self::Unknown => serializer.serialize_unit_variant("State", 4u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
