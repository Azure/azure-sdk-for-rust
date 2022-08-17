#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Apply Update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties for apply update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplyUpdateProperties>,
}
impl ApplyUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for apply update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyUpdateProperties {
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<apply_update_properties::Status>,
    #[doc = "The resourceId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Last Update time"]
    #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_time: Option<time::OffsetDateTime>,
}
impl ApplyUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod apply_update_properties {
    use super::*;
    #[doc = "The status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::RetryNow => serializer.serialize_unit_variant("Status", 3u32, "RetryNow"),
                Self::RetryLater => serializer.serialize_unit_variant("Status", 4u32, "RetryLater"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Configuration Assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for configuration assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationAssignmentProperties>,
}
impl ConfigurationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for configuration assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationAssignmentProperties {
    #[doc = "The maintenance configuration Id"]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "The unique resourceId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ConfigurationAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response details received from the Azure Maintenance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ConfigurationAssignments list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConfigurationAssignmentsResult {
    #[doc = "The list of configuration Assignments"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationAssignment>,
}
impl azure_core::Continuable for ListConfigurationAssignmentsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListConfigurationAssignmentsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for MaintenanceConfigurations list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListMaintenanceConfigurationsResult {
    #[doc = "The list of maintenance Configurations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MaintenanceConfiguration>,
}
impl azure_core::Continuable for ListMaintenanceConfigurationsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListMaintenanceConfigurationsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for Updates list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUpdatesResult {
    #[doc = "The pending updates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Update>,
}
impl azure_core::Continuable for ListUpdatesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListUpdatesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance configuration record type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Gets or sets location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets tags of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties for maintenance configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceConfigurationProperties>,
}
impl MaintenanceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for maintenance configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfigurationProperties {
    #[doc = "Gets or sets namespace of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Gets or sets extensionProperties of the maintenanceConfiguration"]
    #[serde(rename = "extensionProperties", default, skip_serializing_if = "Option::is_none")]
    pub extension_properties: Option<serde_json::Value>,
    #[doc = "Gets or sets maintenanceScope of the configuration"]
    #[serde(rename = "maintenanceScope", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<maintenance_configuration_properties::MaintenanceScope>,
    #[doc = "Definition of a MaintenanceWindow"]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[doc = "Gets or sets the visibility of the configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<maintenance_configuration_properties::Visibility>,
}
impl MaintenanceConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_configuration_properties {
    use super::*;
    #[doc = "Gets or sets maintenanceScope of the configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MaintenanceScope")]
    pub enum MaintenanceScope {
        All,
        Host,
        Resource,
        InResource,
        #[serde(rename = "OSImage")]
        OsImage,
        Extension,
        InGuestPatch,
        #[serde(rename = "SQLDB")]
        Sqldb,
        #[serde(rename = "SQLManagedInstance")]
        SqlManagedInstance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MaintenanceScope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MaintenanceScope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MaintenanceScope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("MaintenanceScope", 0u32, "All"),
                Self::Host => serializer.serialize_unit_variant("MaintenanceScope", 1u32, "Host"),
                Self::Resource => serializer.serialize_unit_variant("MaintenanceScope", 2u32, "Resource"),
                Self::InResource => serializer.serialize_unit_variant("MaintenanceScope", 3u32, "InResource"),
                Self::OsImage => serializer.serialize_unit_variant("MaintenanceScope", 4u32, "OSImage"),
                Self::Extension => serializer.serialize_unit_variant("MaintenanceScope", 5u32, "Extension"),
                Self::InGuestPatch => serializer.serialize_unit_variant("MaintenanceScope", 6u32, "InGuestPatch"),
                Self::Sqldb => serializer.serialize_unit_variant("MaintenanceScope", 7u32, "SQLDB"),
                Self::SqlManagedInstance => serializer.serialize_unit_variant("MaintenanceScope", 8u32, "SQLManagedInstance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the visibility of the configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Visibility")]
    pub enum Visibility {
        Custom,
        Public,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Visibility {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Visibility {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Visibility {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Custom => serializer.serialize_unit_variant("Visibility", 0u32, "Custom"),
                Self::Public => serializer.serialize_unit_variant("Visibility", 1u32, "Public"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An error response received from the Azure Maintenance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceError {
    #[doc = "An error response details received from the Azure Maintenance service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl azure_core::Continuable for MaintenanceError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MaintenanceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a MaintenanceWindow"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindow {
    #[doc = "Effective start date of the maintenance window in YYYY-MM-DD hh:mm format. The start date can be set to either the current date or future date. The window will be created in the time zone provided and adjusted to daylight savings according to that time zone."]
    #[serde(rename = "startDateTime", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[doc = "Effective expiration date of the maintenance window in YYYY-MM-DD hh:mm format. The window will be created in the time zone provided and adjusted to daylight savings according to that time zone. Expiration date must be set to a future date. If not provided, it will be set to the maximum datetime 9999-12-31 23:59:59."]
    #[serde(rename = "expirationDateTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[doc = "Duration of the maintenance window in HH:mm format. If not provided, default value will be used based on maintenance scope provided. Example: 05:00."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Name of the timezone. List of timezones can be obtained by executing [System.TimeZoneInfo]::GetSystemTimeZones() in PowerShell. Example: Pacific Standard Time, UTC, W. Europe Standard Time, Korea Standard Time, Cen. Australia Standard Time."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Rate at which a Maintenance window is expected to recur. The rate can be expressed as daily, weekly, or monthly schedules. Daily schedule are formatted as recurEvery: [Frequency as integer]['Day(s)']. If no frequency is provided, the default frequency is 1. Daily schedule examples are recurEvery: Day, recurEvery: 3Days.  Weekly schedule are formatted as recurEvery: [Frequency as integer]['Week(s)'] [Optional comma separated list of weekdays Monday-Sunday]. Weekly schedule examples are recurEvery: 3Weeks, recurEvery: Week Saturday,Sunday. Monthly schedules are formatted as [Frequency as integer]['Month(s)'] [Comma separated list of month days] or [Frequency as integer]['Month(s)'] [Week of Month (First, Second, Third, Fourth, Last)] [Weekday Monday-Sunday]. Monthly schedule examples are recurEvery: Month, recurEvery: 2Months, recurEvery: Month day23,day24, recurEvery: Month Last Sunday, recurEvery: Month Fourth Monday."]
    #[serde(rename = "recurEvery", default, skip_serializing_if = "Option::is_none")]
    pub recur_every: Option<String>,
}
impl MaintenanceWindow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an operation returned by the GetOperations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Information about an operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationInfo {
    #[doc = "Name of the provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Operations operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "A collection of operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified identifier of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance update on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Update {
    #[doc = "The impact area"]
    #[serde(rename = "maintenanceScope", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<update::MaintenanceScope>,
    #[doc = "The impact type"]
    #[serde(rename = "impactType", default, skip_serializing_if = "Option::is_none")]
    pub impact_type: Option<update::ImpactType>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update::Status>,
    #[doc = "Duration of impact in seconds"]
    #[serde(rename = "impactDurationInSec", default, skip_serializing_if = "Option::is_none")]
    pub impact_duration_in_sec: Option<i32>,
    #[doc = "Time when Azure will start force updates if not self-updated by customer before this time"]
    #[serde(rename = "notBefore", default, with = "azure_core::date::rfc3339::option")]
    pub not_before: Option<time::OffsetDateTime>,
    #[doc = "Properties for update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateProperties>,
}
impl Update {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update {
    use super::*;
    #[doc = "The impact area"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MaintenanceScope")]
    pub enum MaintenanceScope {
        All,
        Host,
        Resource,
        InResource,
        #[serde(rename = "OSImage")]
        OsImage,
        Extension,
        InGuestPatch,
        #[serde(rename = "SQLDB")]
        Sqldb,
        #[serde(rename = "SQLManagedInstance")]
        SqlManagedInstance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MaintenanceScope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MaintenanceScope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MaintenanceScope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("MaintenanceScope", 0u32, "All"),
                Self::Host => serializer.serialize_unit_variant("MaintenanceScope", 1u32, "Host"),
                Self::Resource => serializer.serialize_unit_variant("MaintenanceScope", 2u32, "Resource"),
                Self::InResource => serializer.serialize_unit_variant("MaintenanceScope", 3u32, "InResource"),
                Self::OsImage => serializer.serialize_unit_variant("MaintenanceScope", 4u32, "OSImage"),
                Self::Extension => serializer.serialize_unit_variant("MaintenanceScope", 5u32, "Extension"),
                Self::InGuestPatch => serializer.serialize_unit_variant("MaintenanceScope", 6u32, "InGuestPatch"),
                Self::Sqldb => serializer.serialize_unit_variant("MaintenanceScope", 7u32, "SQLDB"),
                Self::SqlManagedInstance => serializer.serialize_unit_variant("MaintenanceScope", 8u32, "SQLManagedInstance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The impact type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImpactType")]
    pub enum ImpactType {
        None,
        Freeze,
        Restart,
        Redeploy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImpactType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImpactType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImpactType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ImpactType", 0u32, "None"),
                Self::Freeze => serializer.serialize_unit_variant("ImpactType", 1u32, "Freeze"),
                Self::Restart => serializer.serialize_unit_variant("ImpactType", 2u32, "Restart"),
                Self::Redeploy => serializer.serialize_unit_variant("ImpactType", 3u32, "Redeploy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::RetryNow => serializer.serialize_unit_variant("Status", 3u32, "RetryNow"),
                Self::RetryLater => serializer.serialize_unit_variant("Status", 4u32, "RetryLater"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties for update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProperties {
    #[doc = "The resourceId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl UpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
