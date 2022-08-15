#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Citus version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CitusVersion")]
pub enum CitusVersion {
    #[serde(rename = "8.3")]
    N8_3,
    #[serde(rename = "9.0")]
    N9_0,
    #[serde(rename = "9.1")]
    N9_1,
    #[serde(rename = "9.2")]
    N9_2,
    #[serde(rename = "9.3")]
    N9_3,
    #[serde(rename = "9.4")]
    N9_4,
    #[serde(rename = "9.5")]
    N9_5,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CitusVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CitusVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CitusVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N8_3 => serializer.serialize_unit_variant("CitusVersion", 0u32, "8.3"),
            Self::N9_0 => serializer.serialize_unit_variant("CitusVersion", 1u32, "9.0"),
            Self::N9_1 => serializer.serialize_unit_variant("CitusVersion", 2u32, "9.1"),
            Self::N9_2 => serializer.serialize_unit_variant("CitusVersion", 3u32, "9.2"),
            Self::N9_3 => serializer.serialize_unit_variant("CitusVersion", 4u32, "9.3"),
            Self::N9_4 => serializer.serialize_unit_variant("CitusVersion", 5u32, "9.4"),
            Self::N9_5 => serializer.serialize_unit_variant("CitusVersion", 6u32, "9.5"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
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
#[doc = "Represents a server group firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of a server group firewall rule."]
    pub properties: FirewallRuleProperties,
}
impl FirewallRule {
    pub fn new(properties: FirewallRuleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "A list of firewall rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[doc = "The list of firewall rules in a server group."]
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
#[doc = "The properties of a server group firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[doc = "The start IP address of the server group firewall rule. Must be IPv4 format."]
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[doc = "The end IP address of the server group firewall rule. Must be IPv4 format."]
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
pub type FullyQualifiedDomainName = String;
#[doc = "Maintenance window of a server group."]
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
    #[serde(rename = "type")]
    pub type_: name_availability_request::Type,
}
impl NameAvailabilityRequest {
    pub fn new(name: String, type_: name_availability_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod name_availability_request {
    use super::*;
    #[doc = "Resource type used for verification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DBforPostgreSQL/serverGroupsv2")]
        MicrosoftDBforPostgreSqlServerGroupsv2,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The PostgreSQL version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PostgreSqlVersion")]
pub enum PostgreSqlVersion {
    #[serde(rename = "11")]
    N11,
    #[serde(rename = "12")]
    N12,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PostgreSqlVersion {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PostgreSqlVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PostgreSqlVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N11 => serializer.serialize_unit_variant("PostgreSqlVersion", 0u32, "11"),
            Self::N12 => serializer.serialize_unit_variant("PostgreSqlVersion", 1u32, "12"),
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
#[doc = "Represents a server group role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Role {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of a server group role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleProperties>,
}
impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of roles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleListResult {
    #[doc = "The list of roles in a server group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Role>,
}
impl azure_core::Continuable for RoleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RoleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server group role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleProperties {
    #[doc = "The password of the server group role."]
    pub password: String,
}
impl RoleProperties {
    pub fn new(password: String) -> Self {
        Self { password }
    }
}
#[doc = "Represents a configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of a configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerConfigurationProperties>,
}
impl ServerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfigurationListResult {
    #[doc = "The list of server configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerConfiguration>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfigurationProperties {
    #[doc = "Value of the configuration."]
    pub value: String,
    #[doc = "Source of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Description of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Default value of the configuration."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "Data type of the configuration."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<server_configuration_properties::DataType>,
    #[doc = "Allowed values of the configuration."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
}
impl ServerConfigurationProperties {
    pub fn new(value: String) -> Self {
        Self {
            value,
            source: None,
            description: None,
            default_value: None,
            data_type: None,
            allowed_values: None,
        }
    }
}
pub mod server_configuration_properties {
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
#[doc = "Represents a server group for create."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties used to create a new server group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupProperties>,
}
impl ServerGroup {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "Represents the configuration list of server role groups in a server group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of server group configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupConfigurationProperties>,
}
impl ServerGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server group configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupConfigurationListResult {
    #[doc = "The list of server group configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerGroupConfiguration>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerGroupConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerGroupConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of server group configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupConfigurationProperties {
    #[doc = "Description of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Data type of the configuration."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<server_group_configuration_properties::DataType>,
    #[doc = "Allowed values of the configuration."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[doc = "The list of server role group configuration values."]
    #[serde(rename = "serverRoleGroupConfigurations")]
    pub server_role_group_configurations: Vec<ServerRoleGroupConfiguration>,
}
impl ServerGroupConfigurationProperties {
    pub fn new(server_role_group_configurations: Vec<ServerRoleGroupConfiguration>) -> Self {
        Self {
            description: None,
            data_type: None,
            allowed_values: None,
            server_role_group_configurations,
        }
    }
}
pub mod server_group_configuration_properties {
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
#[doc = "Represents a server group for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupForUpdate {
    #[doc = "The location the resource resides in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The properties used to update a server group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupPropertiesForUpdate>,
    #[doc = "Application-specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServerGroupForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupListResult {
    #[doc = "The list of server groups"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerGroup>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties used to create a new server group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupProperties {
    #[doc = "The mode to create a new server group."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_group_properties::CreateMode>,
    #[doc = "The administrator's login name of servers in server group. Can only be specified when the server is being created (and is required for creation)."]
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[doc = "The password of the administrator login."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The backup retention days for server group."]
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[doc = "The PostgreSQL version."]
    #[serde(rename = "postgresqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub postgresql_version: Option<PostgreSqlVersion>,
    #[doc = "The Citus version."]
    #[serde(rename = "citusVersion", default, skip_serializing_if = "Option::is_none")]
    pub citus_version: Option<CitusVersion>,
    #[doc = "If Citus MX is enabled or not for the server group."]
    #[serde(rename = "enableMx", default, skip_serializing_if = "Option::is_none")]
    pub enable_mx: Option<bool>,
    #[doc = "If ZFS compression is enabled or not for the server group."]
    #[serde(rename = "enableZfs", default, skip_serializing_if = "Option::is_none")]
    pub enable_zfs: Option<bool>,
    #[doc = "If shards on coordinator is enabled or not for the server group."]
    #[serde(rename = "enableShardsOnCoordinator", default, skip_serializing_if = "Option::is_none")]
    pub enable_shards_on_coordinator: Option<bool>,
    #[doc = "A state of a server group/server that is visible to user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ServerState>,
    #[doc = "The earliest restore point time (ISO8601 format) for server group."]
    #[serde(rename = "earliestRestoreTime", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_time: Option<time::OffsetDateTime>,
    #[doc = "The resource provider type of server group."]
    #[serde(rename = "resourceProviderType", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_type: Option<server_group_properties::ResourceProviderType>,
    #[doc = "The list of server role groups."]
    #[serde(rename = "serverRoleGroups", default, skip_serializing_if = "Option::is_none")]
    pub server_role_groups: Option<ServerRoleGroupList>,
    #[doc = "Maintenance window of a server group."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "Availability Zone information of the server group."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "Standby Availability Zone information of the server group."]
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
    #[doc = "The delegated subnet arguments for a server group."]
    #[serde(rename = "delegatedSubnetArguments", default, skip_serializing_if = "Option::is_none")]
    pub delegated_subnet_arguments: Option<server_group_properties::DelegatedSubnetArguments>,
    #[doc = "The private dns zone arguments for a server group."]
    #[serde(rename = "privateDnsZoneArguments", default, skip_serializing_if = "Option::is_none")]
    pub private_dns_zone_arguments: Option<server_group_properties::PrivateDnsZoneArguments>,
    #[doc = "The array of read replica server groups."]
    #[serde(rename = "readReplicas", default, skip_serializing_if = "Vec::is_empty")]
    pub read_replicas: Vec<String>,
    #[doc = "The source server group id for read replica server groups."]
    #[serde(rename = "sourceServerGroup", default, skip_serializing_if = "Option::is_none")]
    pub source_server_group: Option<String>,
    #[doc = "The source subscription id to restore from. It's required when 'createMode' is 'PointInTimeRestore' or 'ReadReplica'"]
    #[serde(rename = "sourceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub source_subscription_id: Option<String>,
    #[doc = "The source resource group name to restore from. It's required when 'createMode' is 'PointInTimeRestore' or 'ReadReplica'"]
    #[serde(rename = "sourceResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_group_name: Option<String>,
    #[doc = "The source server group name to restore from. It's required when 'createMode' is 'PointInTimeRestore' or 'ReadReplica'"]
    #[serde(rename = "sourceServerGroupName", default, skip_serializing_if = "Option::is_none")]
    pub source_server_group_name: Option<String>,
    #[doc = "The source server group location to restore from. It's required when 'createMode' is 'PointInTimeRestore' or 'ReadReplica'"]
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[doc = "Restore point creation time (ISO8601 format), specifying the time to restore from. It's required when 'createMode' is 'PointInTimeRestore'"]
    #[serde(rename = "pointInTimeUTC", with = "azure_core::date::rfc3339::option")]
    pub point_in_time_utc: Option<time::OffsetDateTime>,
}
impl ServerGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_group_properties {
    use super::*;
    #[doc = "The mode to create a new server group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        ReadReplica,
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
                Self::ReadReplica => serializer.serialize_unit_variant("CreateMode", 2u32, "ReadReplica"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The resource provider type of server group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceProviderType")]
    pub enum ResourceProviderType {
        Meru,
        Marlin,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceProviderType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceProviderType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceProviderType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Meru => serializer.serialize_unit_variant("ResourceProviderType", 0u32, "Meru"),
                Self::Marlin => serializer.serialize_unit_variant("ResourceProviderType", 1u32, "Marlin"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The delegated subnet arguments for a server group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DelegatedSubnetArguments {
        #[doc = "delegated subnet arm resource id."]
        #[serde(rename = "subnetArmResourceId", default, skip_serializing_if = "Option::is_none")]
        pub subnet_arm_resource_id: Option<String>,
    }
    impl DelegatedSubnetArguments {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The private dns zone arguments for a server group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PrivateDnsZoneArguments {
        #[doc = "private dns zone arm resource id."]
        #[serde(rename = "privateDnsZoneArmResourceId", default, skip_serializing_if = "Option::is_none")]
        pub private_dns_zone_arm_resource_id: Option<String>,
    }
    impl PrivateDnsZoneArguments {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The properties used to update a server group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupPropertiesForUpdate {
    #[doc = "The password of the administrator login."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The backup retention days for server group."]
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[doc = "The PostgreSQL version."]
    #[serde(rename = "postgresqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub postgresql_version: Option<PostgreSqlVersion>,
    #[doc = "The Citus version."]
    #[serde(rename = "citusVersion", default, skip_serializing_if = "Option::is_none")]
    pub citus_version: Option<CitusVersion>,
    #[doc = "If shards on coordinator is enabled or not for the server group."]
    #[serde(rename = "enableShardsOnCoordinator", default, skip_serializing_if = "Option::is_none")]
    pub enable_shards_on_coordinator: Option<bool>,
    #[doc = "The list of server role groups."]
    #[serde(rename = "serverRoleGroups", default, skip_serializing_if = "Option::is_none")]
    pub server_role_groups: Option<ServerRoleGroupList>,
    #[doc = "Maintenance window of a server group."]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "Availability Zone information of the server group."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "Standby Availability Zone information of the server group."]
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
}
impl ServerGroupPropertiesForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a server in a server group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of a server in server group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerGroupServerProperties>,
}
impl ServerGroupServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of servers in a server group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupServerListResult {
    #[doc = "The list of servers in a server group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerGroupServer>,
}
impl azure_core::Continuable for ServerGroupServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServerGroupServerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server in server group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerGroupServerProperties {
    #[serde(flatten)]
    pub server_properties: ServerProperties,
    #[doc = "The fully qualified domain name of a server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<FullyQualifiedDomainName>,
    #[doc = "The role of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ServerRole>,
    #[doc = "A state of a server group/server that is visible to user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ServerState>,
    #[doc = "A state of a server role group/server that is visible to user for HA feature."]
    #[serde(rename = "haState", default, skip_serializing_if = "Option::is_none")]
    pub ha_state: Option<ServerHaState>,
    #[doc = "The administrator's login name of a servers in server group."]
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[doc = "The PostgreSQL version."]
    #[serde(rename = "postgresqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub postgresql_version: Option<PostgreSqlVersion>,
    #[doc = "The Citus version."]
    #[serde(rename = "citusVersion", default, skip_serializing_if = "Option::is_none")]
    pub citus_version: Option<CitusVersion>,
    #[doc = "Availability Zone information of the server group."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "Standby Availability Zone information of the server group."]
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
}
impl ServerGroupServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A state of a server role group/server that is visible to user for HA feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerHaState")]
pub enum ServerHaState {
    NotEnabled,
    CreatingStandby,
    ReplicatingData,
    FailingOver,
    Healthy,
    RemovingStandby,
    NotSync,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerHaState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerHaState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerHaState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotEnabled => serializer.serialize_unit_variant("ServerHaState", 0u32, "NotEnabled"),
            Self::CreatingStandby => serializer.serialize_unit_variant("ServerHaState", 1u32, "CreatingStandby"),
            Self::ReplicatingData => serializer.serialize_unit_variant("ServerHaState", 2u32, "ReplicatingData"),
            Self::FailingOver => serializer.serialize_unit_variant("ServerHaState", 3u32, "FailingOver"),
            Self::Healthy => serializer.serialize_unit_variant("ServerHaState", 4u32, "Healthy"),
            Self::RemovingStandby => serializer.serialize_unit_variant("ServerHaState", 5u32, "RemovingStandby"),
            Self::NotSync => serializer.serialize_unit_variant("ServerHaState", 6u32, "NotSync"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The name object for a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerNameItem {
    #[doc = "The name of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The fully qualified domain name of a server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<FullyQualifiedDomainName>,
}
impl ServerNameItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[doc = "The edition of a server (default: GeneralPurpose)."]
    #[serde(rename = "serverEdition", default, skip_serializing_if = "Option::is_none")]
    pub server_edition: Option<server_properties::ServerEdition>,
    #[doc = "The storage of a server in MB (max: 2097152 = 2TiB)."]
    #[serde(rename = "storageQuotaInMb", default, skip_serializing_if = "Option::is_none")]
    pub storage_quota_in_mb: Option<i64>,
    #[doc = "The vCores count of a server (max: 64)."]
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i64>,
    #[doc = "If high availability is enabled or not for the server."]
    #[serde(rename = "enableHa", default, skip_serializing_if = "Option::is_none")]
    pub enable_ha: Option<bool>,
    #[doc = "If public IP is requested or not for a server."]
    #[serde(rename = "enablePublicIp", default, skip_serializing_if = "Option::is_none")]
    pub enable_public_ip: Option<bool>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_properties {
    use super::*;
    #[doc = "The edition of a server (default: GeneralPurpose)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerEdition")]
    pub enum ServerEdition {
        GeneralPurpose,
        MemoryOptimized,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerEdition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerEdition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerEdition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GeneralPurpose => serializer.serialize_unit_variant("ServerEdition", 0u32, "GeneralPurpose"),
                Self::MemoryOptimized => serializer.serialize_unit_variant("ServerEdition", 1u32, "MemoryOptimized"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The role of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerRole")]
pub enum ServerRole {
    Coordinator,
    Worker,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Coordinator => serializer.serialize_unit_variant("ServerRole", 0u32, "Coordinator"),
            Self::Worker => serializer.serialize_unit_variant("ServerRole", 1u32, "Worker"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a server role group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerRoleGroup {
    #[serde(flatten)]
    pub server_properties: ServerProperties,
    #[doc = "The name of the server role group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ServerRole>,
    #[doc = "The number of servers in the server role group."]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[doc = "The list of server names in the server role group."]
    #[serde(rename = "serverNames", default, skip_serializing_if = "Vec::is_empty")]
    pub server_names: Vec<ServerNameItem>,
}
impl ServerRoleGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents server role group configuration value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerRoleGroupConfiguration {
    #[doc = "The role of a server."]
    pub role: ServerRole,
    #[doc = "Value of the configuration."]
    pub value: String,
    #[doc = "Default value of the configuration."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "Source of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl ServerRoleGroupConfiguration {
    pub fn new(role: ServerRole, value: String) -> Self {
        Self {
            role,
            value,
            default_value: None,
            source: None,
        }
    }
}
pub type ServerRoleGroupList = Vec<ServerRoleGroup>;
#[doc = "A state of a server group/server that is visible to user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerState")]
pub enum ServerState {
    Ready,
    Dropping,
    Disabled,
    Starting,
    Stopping,
    Stopped,
    Updating,
    Provisioning,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ready => serializer.serialize_unit_variant("ServerState", 0u32, "Ready"),
            Self::Dropping => serializer.serialize_unit_variant("ServerState", 1u32, "Dropping"),
            Self::Disabled => serializer.serialize_unit_variant("ServerState", 2u32, "Disabled"),
            Self::Starting => serializer.serialize_unit_variant("ServerState", 3u32, "Starting"),
            Self::Stopping => serializer.serialize_unit_variant("ServerState", 4u32, "Stopping"),
            Self::Stopped => serializer.serialize_unit_variant("ServerState", 5u32, "Stopped"),
            Self::Updating => serializer.serialize_unit_variant("ServerState", 6u32, "Updating"),
            Self::Provisioning => serializer.serialize_unit_variant("ServerState", 7u32, "Provisioning"),
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
    #[doc = "The timestamp of resource last modification (UTC)"]
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
