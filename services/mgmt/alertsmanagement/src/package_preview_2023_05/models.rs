#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Action that should be applied."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "actionType")]
pub enum ActionUnion {
    AddActionGroups(AddActionGroups),
    CorrelateAlerts(CorrelateAlerts),
    RemoveAllActionGroups(RemoveAllActionGroups),
}
#[doc = "Add action groups to alert processing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddActionGroups {
    #[doc = "List of action group Ids to add to alert processing rule."]
    #[serde(rename = "actionGroupIds")]
    pub action_group_ids: Vec<String>,
}
impl AddActionGroups {
    pub fn new(action_group_ids: Vec<String>) -> Self {
        Self { action_group_ids }
    }
}
#[doc = "Alert processing rule object containing target scopes, conditions and scheduling logic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProcessingRule {
    #[serde(flatten)]
    pub managed_resource: ManagedResource,
    #[doc = "Alert processing rule properties defining scopes, conditions and scheduling logic for alert processing rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProcessingRuleProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AlertProcessingRule {
    pub fn new(managed_resource: ManagedResource) -> Self {
        Self {
            managed_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Alert processing rule properties defining scopes, conditions and scheduling logic for alert processing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProcessingRuleProperties {
    #[doc = "List of ARM IDs which will be the target of the given alert processing rule."]
    pub scopes: Scopes,
    #[doc = "Conditions in alert instance to be matched for a given alert processing rule. Default value is all. Multiple values could be provided with comma separation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
    #[doc = "Scheduling configuration for a given alert processing rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[doc = "Actions to be applied."]
    pub actions: Vec<ActionUnion>,
    #[doc = "Description of alert processing rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates if the given alert processing rule is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AlertProcessingRuleProperties {
    pub fn new(scopes: Scopes, actions: Vec<ActionUnion>) -> Self {
        Self {
            scopes,
            conditions: None,
            schedule: None,
            actions,
            description: None,
            enabled: None,
        }
    }
}
#[doc = "List of alert processing rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProcessingRulesList {
    #[doc = "URL to fetch the next set of alert processing rules."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of alert processing rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AlertProcessingRule>,
}
impl azure_core::Continuable for AlertProcessingRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertProcessingRulesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Condition to trigger an alert processing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "Field for a given condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<condition::Field>,
    #[doc = "Operator for a given condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<condition::Operator>,
    #[doc = "List of values to match for a given condition."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl Condition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod condition {
    use super::*;
    #[doc = "Field for a given condition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Field")]
    pub enum Field {
        Severity,
        MonitorService,
        MonitorCondition,
        SignalType,
        TargetResourceType,
        TargetResource,
        TargetResourceGroup,
        AlertRuleId,
        AlertRuleName,
        Description,
        AlertContext,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Field {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Field {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Severity => serializer.serialize_unit_variant("Field", 0u32, "Severity"),
                Self::MonitorService => serializer.serialize_unit_variant("Field", 1u32, "MonitorService"),
                Self::MonitorCondition => serializer.serialize_unit_variant("Field", 2u32, "MonitorCondition"),
                Self::SignalType => serializer.serialize_unit_variant("Field", 3u32, "SignalType"),
                Self::TargetResourceType => serializer.serialize_unit_variant("Field", 4u32, "TargetResourceType"),
                Self::TargetResource => serializer.serialize_unit_variant("Field", 5u32, "TargetResource"),
                Self::TargetResourceGroup => serializer.serialize_unit_variant("Field", 6u32, "TargetResourceGroup"),
                Self::AlertRuleId => serializer.serialize_unit_variant("Field", 7u32, "AlertRuleId"),
                Self::AlertRuleName => serializer.serialize_unit_variant("Field", 8u32, "AlertRuleName"),
                Self::Description => serializer.serialize_unit_variant("Field", 9u32, "Description"),
                Self::AlertContext => serializer.serialize_unit_variant("Field", 10u32, "AlertContext"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Operator for a given condition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        NotEquals,
        Contains,
        DoesNotContain,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equals => serializer.serialize_unit_variant("Operator", 0u32, "Equals"),
                Self::NotEquals => serializer.serialize_unit_variant("Operator", 1u32, "NotEquals"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 2u32, "Contains"),
                Self::DoesNotContain => serializer.serialize_unit_variant("Operator", 3u32, "DoesNotContain"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type Conditions = Vec<Condition>;
#[doc = "Add logic for alerts correlation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorrelateAlerts {
    #[doc = "The list of conditions for the alerts correlations."]
    #[serde(rename = "correlateBy")]
    pub correlate_by: Vec<CorrelateBy>,
    #[doc = "The required duration (in ISO8601 format) for the alerts correlation."]
    #[serde(rename = "correlationInterval")]
    pub correlation_interval: String,
    #[doc = "The priority of this correlation."]
    pub priority: i32,
    #[doc = "Indicates how to handle child alerts notifications."]
    #[serde(rename = "notificationsForCorrelatedAlerts", default, skip_serializing_if = "Option::is_none")]
    pub notifications_for_correlated_alerts: Option<correlate_alerts::NotificationsForCorrelatedAlerts>,
}
impl CorrelateAlerts {
    pub fn new(correlate_by: Vec<CorrelateBy>, correlation_interval: String, priority: i32) -> Self {
        Self {
            correlate_by,
            correlation_interval,
            priority,
            notifications_for_correlated_alerts: None,
        }
    }
}
pub mod correlate_alerts {
    use super::*;
    #[doc = "Indicates how to handle child alerts notifications."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotificationsForCorrelatedAlerts")]
    pub enum NotificationsForCorrelatedAlerts {
        NotifyAlways,
        SuppressAlways,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotificationsForCorrelatedAlerts {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotificationsForCorrelatedAlerts {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotificationsForCorrelatedAlerts {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotifyAlways => serializer.serialize_unit_variant("NotificationsForCorrelatedAlerts", 0u32, "NotifyAlways"),
                Self::SuppressAlways => serializer.serialize_unit_variant("NotificationsForCorrelatedAlerts", 1u32, "SuppressAlways"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NotificationsForCorrelatedAlerts {
        fn default() -> Self {
            Self::SuppressAlways
        }
    }
}
#[doc = "The logic for the correlation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CorrelateBy {
    #[doc = "The JPath of the property that the alerts should be correlated by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}
impl CorrelateBy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Daily recurrence object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DailyRecurrence {
    #[serde(flatten)]
    pub recurrence: Recurrence,
}
impl DailyRecurrence {
    pub fn new(recurrence: Recurrence) -> Self {
        Self { recurrence }
    }
}
#[doc = "Days of week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DaysOfWeek")]
pub enum DaysOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DaysOfWeek {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DaysOfWeek {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DaysOfWeek {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sunday => serializer.serialize_unit_variant("DaysOfWeek", 0u32, "Sunday"),
            Self::Monday => serializer.serialize_unit_variant("DaysOfWeek", 1u32, "Monday"),
            Self::Tuesday => serializer.serialize_unit_variant("DaysOfWeek", 2u32, "Tuesday"),
            Self::Wednesday => serializer.serialize_unit_variant("DaysOfWeek", 3u32, "Wednesday"),
            Self::Thursday => serializer.serialize_unit_variant("DaysOfWeek", 4u32, "Thursday"),
            Self::Friday => serializer.serialize_unit_variant("DaysOfWeek", 5u32, "Friday"),
            Self::Saturday => serializer.serialize_unit_variant("DaysOfWeek", 6u32, "Saturday"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An azure managed resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Monthly recurrence object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyRecurrence {
    #[serde(flatten)]
    pub recurrence: Recurrence,
    #[doc = "Specifies the values for monthly recurrence pattern."]
    #[serde(rename = "daysOfMonth")]
    pub days_of_month: Vec<i32>,
}
impl MonthlyRecurrence {
    pub fn new(recurrence: Recurrence, days_of_month: Vec<i32>) -> Self {
        Self { recurrence, days_of_month }
    }
}
#[doc = "Data contract for patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchObject {
    #[doc = "Alert processing rule properties supported by patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
    #[doc = "Tags to be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert processing rule properties supported by patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[doc = "Indicates if the given alert processing rule is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl PatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recurrence object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    #[doc = "Start time for recurrence."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time for recurrence."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
impl Recurrence {
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
        }
    }
}
#[doc = "Specifies when the recurrence should be applied."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "recurrenceType")]
pub enum RecurrenceUnion {
    Daily(DailyRecurrence),
    Monthly(MonthlyRecurrence),
    Weekly(WeeklyRecurrence),
}
#[doc = "Indicates if all action groups should be removed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveAllActionGroups {}
impl RemoveAllActionGroups {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scheduling configuration for a given alert processing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[doc = "Scheduling effective from time. Date-Time in ISO-8601 format without timezone suffix."]
    #[serde(rename = "effectiveFrom", default, skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    #[doc = "Scheduling effective until time. Date-Time in ISO-8601 format without timezone suffix."]
    #[serde(rename = "effectiveUntil", default, skip_serializing_if = "Option::is_none")]
    pub effective_until: Option<String>,
    #[doc = "Scheduling time zone."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "List of recurrences."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recurrences: Vec<RecurrenceUnion>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Scopes = Vec<String>;
#[doc = "Weekly recurrence object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeeklyRecurrence {
    #[serde(flatten)]
    pub recurrence: Recurrence,
    #[doc = "Specifies the values for weekly recurrence pattern."]
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Vec<DaysOfWeek>,
}
impl WeeklyRecurrence {
    pub fn new(recurrence: Recurrence, days_of_week: Vec<DaysOfWeek>) -> Self {
        Self { recurrence, days_of_week }
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Details of error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
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
#[doc = "Details of error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Error code, intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error, intended for display in user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the particular error, for example name of the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
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
