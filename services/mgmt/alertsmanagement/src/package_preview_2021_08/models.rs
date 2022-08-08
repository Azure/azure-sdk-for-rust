#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Action to be applied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[doc = "Action that should be applied."]
    #[serde(rename = "actionType")]
    pub action_type: action::ActionType,
}
impl Action {
    pub fn new(action_type: action::ActionType) -> Self {
        Self { action_type }
    }
}
pub mod action {
    use super::*;
    #[doc = "Action that should be applied."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        AddActionGroups,
        RemoveAllActionGroups,
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
                Self::AddActionGroups => serializer.serialize_unit_variant("ActionType", 0u32, "AddActionGroups"),
                Self::RemoveAllActionGroups => serializer.serialize_unit_variant("ActionType", 1u32, "RemoveAllActionGroups"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Add action groups to alert processing rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddActionGroups {
    #[serde(flatten)]
    pub action: Action,
    #[doc = "List of action group Ids to add to alert processing rule."]
    #[serde(rename = "actionGroupIds")]
    pub action_group_ids: Vec<String>,
}
impl AddActionGroups {
    pub fn new(action: Action, action_group_ids: Vec<String>) -> Self {
        Self { action, action_group_ids }
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
    pub actions: Vec<Action>,
    #[doc = "Description of alert processing rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates if the given alert processing rule is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AlertProcessingRuleProperties {
    pub fn new(scopes: Scopes, actions: Vec<Action>) -> Self {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertProcessingRule>,
}
impl azure_core::Continuable for AlertProcessingRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Details of a monitor service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorServiceDetails {
    #[doc = "Monitor service name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Monitor service display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl MonitorServiceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitor service details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorServiceList {
    #[serde(flatten)]
    pub alerts_meta_data_properties: AlertsMetaDataProperties,
    #[doc = "Array of operations"]
    pub data: Vec<MonitorServiceDetails>,
}
impl MonitorServiceList {
    pub fn new(alerts_meta_data_properties: AlertsMetaDataProperties, data: Vec<MonitorServiceDetails>) -> Self {
        Self {
            alerts_meta_data_properties,
            data,
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
    #[doc = "Specifies when the recurrence should be applied."]
    #[serde(rename = "recurrenceType")]
    pub recurrence_type: recurrence::RecurrenceType,
    #[doc = "Start time for recurrence."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time for recurrence."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
impl Recurrence {
    pub fn new(recurrence_type: recurrence::RecurrenceType) -> Self {
        Self {
            recurrence_type,
            start_time: None,
            end_time: None,
        }
    }
}
pub mod recurrence {
    use super::*;
    #[doc = "Specifies when the recurrence should be applied."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecurrenceType")]
    pub enum RecurrenceType {
        Daily,
        Weekly,
        Monthly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecurrenceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecurrenceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecurrenceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("RecurrenceType", 0u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("RecurrenceType", 1u32, "Weekly"),
                Self::Monthly => serializer.serialize_unit_variant("RecurrenceType", 2u32, "Monthly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Indicates if all action groups should be removed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveAllActionGroups {
    #[serde(flatten)]
    pub action: Action,
}
impl RemoveAllActionGroups {
    pub fn new(action: Action) -> Self {
        Self { action }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrences: Vec<Recurrence>,
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
#[doc = "Action status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionStatus {
    #[doc = "Value indicating whether alert is suppressed."]
    #[serde(rename = "isSuppressed", default, skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
}
impl ActionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert created in alert management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Alert property bag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information specific to the monitor service that gives more contextual details about the alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertContext {}
impl AlertContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert Modification details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the alert modification item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertModificationProperties>,
}
impl AlertModification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert modification item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertModificationItem {
    #[doc = "Reason for the modification"]
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<alert_modification_item::ModificationEvent>,
    #[doc = "Old value"]
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[doc = "New value"]
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[doc = "Modified date and time"]
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[doc = "Modified user details (Principal client name)"]
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[doc = "Modification comments"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Description of the modification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AlertModificationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_modification_item {
    use super::*;
    #[doc = "Reason for the modification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        AlertCreated,
        StateChange,
        MonitorConditionChange,
        SeverityChange,
        ActionRuleTriggered,
        ActionRuleSuppressed,
        ActionsTriggered,
        ActionsSuppressed,
        ActionsFailed,
    }
}
#[doc = "Properties of the alert modification item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertModificationProperties {
    #[doc = "Unique Id of the alert for which the history is being retrieved"]
    #[serde(rename = "alertId", default, skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<String>,
    #[doc = "Modification details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifications: Vec<AlertModificationItem>,
}
impl AlertModificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert property bag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "This object contains consistent fields across different monitor services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub essentials: Option<Essentials>,
    #[doc = "Information specific to the monitor service that gives more contextual details about the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<AlertContext>,
    #[doc = "Config which would be used for displaying the data in portal."]
    #[serde(rename = "egressConfig", default, skip_serializing_if = "Option::is_none")]
    pub egress_config: Option<EgressConfig>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List the alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsList {
    #[doc = "URL to fetch the next set of alerts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of alerts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
}
impl azure_core::Continuable for AlertsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AlertsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "alert meta data information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsMetaData {
    #[doc = "alert meta data property bag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsMetaDataProperties>,
}
impl AlertsMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "alert meta data property bag"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsMetaDataProperties {
    #[doc = "Identification of the information to be retrieved by API call"]
    #[serde(rename = "metadataIdentifier")]
    pub metadata_identifier: alerts_meta_data_properties::MetadataIdentifier,
}
impl AlertsMetaDataProperties {
    pub fn new(metadata_identifier: alerts_meta_data_properties::MetadataIdentifier) -> Self {
        Self { metadata_identifier }
    }
}
pub mod alerts_meta_data_properties {
    use super::*;
    #[doc = "Identification of the information to be retrieved by API call"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MetadataIdentifier")]
    pub enum MetadataIdentifier {
        MonitorServiceList,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MetadataIdentifier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MetadataIdentifier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MetadataIdentifier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MonitorServiceList => serializer.serialize_unit_variant("MetadataIdentifier", 0u32, "MonitorServiceList"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Summary of alerts based on the input filters and 'groupby' parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Group the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsSummaryGroup>,
}
impl AlertsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Group the result set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSummaryGroup {
    #[doc = "Total count of the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[doc = "Total count of the smart groups."]
    #[serde(rename = "smartGroupsCount", default, skip_serializing_if = "Option::is_none")]
    pub smart_groups_count: Option<i64>,
    #[doc = "Name of the field aggregated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupedby: Option<String>,
    #[doc = "List of the items"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<AlertsSummaryGroupItem>,
}
impl AlertsSummaryGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alerts summary group item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSummaryGroupItem {
    #[doc = "Value of the aggregated field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Count of the aggregated field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Name of the field aggregated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupedby: Option<String>,
    #[doc = "List of the items"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<AlertsSummaryGroupItem>,
}
impl AlertsSummaryGroupItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Config which would be used for displaying the data in portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EgressConfig {}
impl EgressConfig {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This object contains consistent fields across different monitor services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Essentials {
    #[doc = "Severity of alert Sev0 being highest and Sev4 being lowest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<essentials::Severity>,
    #[doc = "The type of signal the alert is based on, which could be metrics, logs or activity logs."]
    #[serde(rename = "signalType", default, skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<essentials::SignalType>,
    #[doc = "Alert object state, which can be modified by the user."]
    #[serde(rename = "alertState", default, skip_serializing_if = "Option::is_none")]
    pub alert_state: Option<essentials::AlertState>,
    #[doc = "Condition of the rule at the monitor service. It represents whether the underlying conditions have crossed the defined alert rule thresholds."]
    #[serde(rename = "monitorCondition", default, skip_serializing_if = "Option::is_none")]
    pub monitor_condition: Option<essentials::MonitorCondition>,
    #[doc = "Target ARM resource, on which alert got created."]
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<String>,
    #[doc = "Name of the target ARM resource name, on which alert got created."]
    #[serde(rename = "targetResourceName", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_name: Option<String>,
    #[doc = "Resource group of target ARM resource, on which alert got created."]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[doc = "Resource type of target ARM resource, on which alert got created."]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "Monitor service on which the rule(monitor) is set."]
    #[serde(rename = "monitorService", default, skip_serializing_if = "Option::is_none")]
    pub monitor_service: Option<essentials::MonitorService>,
    #[doc = "Rule(monitor) which fired alert instance. Depending on the monitor service,  this would be ARM id or name of the rule."]
    #[serde(rename = "alertRule", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule: Option<String>,
    #[doc = "Unique Id created by monitor service for each alert instance. This could be used to track the issue at the monitor service, in case of Nagios, Zabbix, SCOM etc."]
    #[serde(rename = "sourceCreatedId", default, skip_serializing_if = "Option::is_none")]
    pub source_created_id: Option<String>,
    #[doc = "Unique Id of the smart group"]
    #[serde(rename = "smartGroupId", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_id: Option<String>,
    #[doc = "Verbose reason describing the reason why this alert instance is added to a smart group"]
    #[serde(rename = "smartGroupingReason", default, skip_serializing_if = "Option::is_none")]
    pub smart_grouping_reason: Option<String>,
    #[doc = "Creation time(ISO-8601 format) of alert instance."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Last modification time(ISO-8601 format) of alert instance."]
    #[serde(rename = "lastModifiedDateTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Resolved time(ISO-8601 format) of alert instance. This will be updated when monitor service resolves the alert instance because the rule condition is no longer met."]
    #[serde(rename = "monitorConditionResolvedDateTime", with = "azure_core::date::rfc3339::option")]
    pub monitor_condition_resolved_date_time: Option<time::OffsetDateTime>,
    #[doc = "User who last modified the alert, in case of monitor service updates user would be 'system', otherwise name of the user."]
    #[serde(rename = "lastModifiedUserName", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user_name: Option<String>,
    #[doc = "Action status"]
    #[serde(rename = "actionStatus", default, skip_serializing_if = "Option::is_none")]
    pub action_status: Option<ActionStatus>,
    #[doc = "Alert description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Essentials {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod essentials {
    use super::*;
    #[doc = "Severity of alert Sev0 being highest and Sev4 being lowest."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
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
                Self::Sev0 => serializer.serialize_unit_variant("Severity", 0u32, "Sev0"),
                Self::Sev1 => serializer.serialize_unit_variant("Severity", 1u32, "Sev1"),
                Self::Sev2 => serializer.serialize_unit_variant("Severity", 2u32, "Sev2"),
                Self::Sev3 => serializer.serialize_unit_variant("Severity", 3u32, "Sev3"),
                Self::Sev4 => serializer.serialize_unit_variant("Severity", 4u32, "Sev4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of signal the alert is based on, which could be metrics, logs or activity logs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SignalType")]
    pub enum SignalType {
        Metric,
        Log,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SignalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SignalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SignalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Metric => serializer.serialize_unit_variant("SignalType", 0u32, "Metric"),
                Self::Log => serializer.serialize_unit_variant("SignalType", 1u32, "Log"),
                Self::Unknown => serializer.serialize_unit_variant("SignalType", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Alert object state, which can be modified by the user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlertState")]
    pub enum AlertState {
        New,
        Acknowledged,
        Closed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlertState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlertState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlertState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::New => serializer.serialize_unit_variant("AlertState", 0u32, "New"),
                Self::Acknowledged => serializer.serialize_unit_variant("AlertState", 1u32, "Acknowledged"),
                Self::Closed => serializer.serialize_unit_variant("AlertState", 2u32, "Closed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Condition of the rule at the monitor service. It represents whether the underlying conditions have crossed the defined alert rule thresholds."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MonitorCondition")]
    pub enum MonitorCondition {
        Fired,
        Resolved,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MonitorCondition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MonitorCondition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MonitorCondition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Fired => serializer.serialize_unit_variant("MonitorCondition", 0u32, "Fired"),
                Self::Resolved => serializer.serialize_unit_variant("MonitorCondition", 1u32, "Resolved"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Monitor service on which the rule(monitor) is set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MonitorService")]
    pub enum MonitorService {
        #[serde(rename = "Application Insights")]
        ApplicationInsights,
        #[serde(rename = "ActivityLog Administrative")]
        ActivityLogAdministrative,
        #[serde(rename = "ActivityLog Security")]
        ActivityLogSecurity,
        #[serde(rename = "ActivityLog Recommendation")]
        ActivityLogRecommendation,
        #[serde(rename = "ActivityLog Policy")]
        ActivityLogPolicy,
        #[serde(rename = "ActivityLog Autoscale")]
        ActivityLogAutoscale,
        #[serde(rename = "Log Analytics")]
        LogAnalytics,
        Nagios,
        Platform,
        #[serde(rename = "SCOM")]
        Scom,
        ServiceHealth,
        SmartDetector,
        #[serde(rename = "VM Insights")]
        VmInsights,
        Zabbix,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MonitorService {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MonitorService {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MonitorService {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationInsights => serializer.serialize_unit_variant("MonitorService", 0u32, "Application Insights"),
                Self::ActivityLogAdministrative => serializer.serialize_unit_variant("MonitorService", 1u32, "ActivityLog Administrative"),
                Self::ActivityLogSecurity => serializer.serialize_unit_variant("MonitorService", 2u32, "ActivityLog Security"),
                Self::ActivityLogRecommendation => serializer.serialize_unit_variant("MonitorService", 3u32, "ActivityLog Recommendation"),
                Self::ActivityLogPolicy => serializer.serialize_unit_variant("MonitorService", 4u32, "ActivityLog Policy"),
                Self::ActivityLogAutoscale => serializer.serialize_unit_variant("MonitorService", 5u32, "ActivityLog Autoscale"),
                Self::LogAnalytics => serializer.serialize_unit_variant("MonitorService", 6u32, "Log Analytics"),
                Self::Nagios => serializer.serialize_unit_variant("MonitorService", 7u32, "Nagios"),
                Self::Platform => serializer.serialize_unit_variant("MonitorService", 8u32, "Platform"),
                Self::Scom => serializer.serialize_unit_variant("MonitorService", 9u32, "SCOM"),
                Self::ServiceHealth => serializer.serialize_unit_variant("MonitorService", 10u32, "ServiceHealth"),
                Self::SmartDetector => serializer.serialize_unit_variant("MonitorService", 11u32, "SmartDetector"),
                Self::VmInsights => serializer.serialize_unit_variant("MonitorService", 12u32, "VM Insights"),
                Self::Zabbix => serializer.serialize_unit_variant("MonitorService", 13u32, "Zabbix"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Operation provided by provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Properties of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations available in the AlertsManagement RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "URL to fetch the next set of alerts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of operations"]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsList {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Set of related alerts grouped together smartly by AMS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of smart group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmartGroupProperties>,
}
impl SmartGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Aggregated property of each type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroupAggregatedProperty {
    #[doc = "Name of the type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Total number of items of type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl SmartGroupAggregatedProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert Modification details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroupModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the smartGroup modification item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmartGroupModificationProperties>,
}
impl SmartGroupModification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "smartGroup modification item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroupModificationItem {
    #[doc = "Reason for the modification"]
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<smart_group_modification_item::ModificationEvent>,
    #[doc = "Old value"]
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[doc = "New value"]
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[doc = "Modified date and time"]
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[doc = "Modified user details (Principal client name)"]
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[doc = "Modification comments"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Description of the modification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SmartGroupModificationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod smart_group_modification_item {
    use super::*;
    #[doc = "Reason for the modification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        SmartGroupCreated,
        StateChange,
        AlertAdded,
        AlertRemoved,
    }
}
#[doc = "Properties of the smartGroup modification item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroupModificationProperties {
    #[doc = "Unique Id of the smartGroup for which the history is being retrieved"]
    #[serde(rename = "smartGroupId", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_id: Option<String>,
    #[doc = "Modification details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifications: Vec<SmartGroupModificationItem>,
    #[doc = "URL to fetch the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SmartGroupModificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of smart group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroupProperties {
    #[doc = "Total number of alerts in smart group"]
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
    #[doc = "Smart group state"]
    #[serde(rename = "smartGroupState", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_state: Option<smart_group_properties::SmartGroupState>,
    #[doc = "Severity of smart group is the highest(Sev0 >... > Sev4) severity of all the alerts in the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<smart_group_properties::Severity>,
    #[doc = "Creation time of smart group. Date-Time in ISO-8601 format."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Last updated time of smart group. Date-Time in ISO-8601 format."]
    #[serde(rename = "lastModifiedDateTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Last modified by user name."]
    #[serde(rename = "lastModifiedUserName", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user_name: Option<String>,
    #[doc = "Summary of target resources in the smart group"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<SmartGroupAggregatedProperty>,
    #[doc = "Summary of target resource types in the smart group"]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<SmartGroupAggregatedProperty>,
    #[doc = "Summary of target resource groups in the smart group"]
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<SmartGroupAggregatedProperty>,
    #[doc = "Summary of monitorServices in the smart group"]
    #[serde(rename = "monitorServices", default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_services: Vec<SmartGroupAggregatedProperty>,
    #[doc = "Summary of monitorConditions in the smart group"]
    #[serde(rename = "monitorConditions", default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_conditions: Vec<SmartGroupAggregatedProperty>,
    #[doc = "Summary of alertStates in the smart group"]
    #[serde(rename = "alertStates", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_states: Vec<SmartGroupAggregatedProperty>,
    #[doc = "Summary of alertSeverities in the smart group"]
    #[serde(rename = "alertSeverities", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_severities: Vec<SmartGroupAggregatedProperty>,
    #[doc = "The URI to fetch the next page of alerts. Call ListNext() with this URI to fetch the next page alerts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SmartGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod smart_group_properties {
    use super::*;
    #[doc = "Smart group state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SmartGroupState")]
    pub enum SmartGroupState {
        New,
        Acknowledged,
        Closed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SmartGroupState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SmartGroupState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SmartGroupState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::New => serializer.serialize_unit_variant("SmartGroupState", 0u32, "New"),
                Self::Acknowledged => serializer.serialize_unit_variant("SmartGroupState", 1u32, "Acknowledged"),
                Self::Closed => serializer.serialize_unit_variant("SmartGroupState", 2u32, "Closed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Severity of smart group is the highest(Sev0 >... > Sev4) severity of all the alerts in the group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
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
                Self::Sev0 => serializer.serialize_unit_variant("Severity", 0u32, "Sev0"),
                Self::Sev1 => serializer.serialize_unit_variant("Severity", 1u32, "Sev1"),
                Self::Sev2 => serializer.serialize_unit_variant("Severity", 2u32, "Sev2"),
                Self::Sev3 => serializer.serialize_unit_variant("Severity", 3u32, "Sev3"),
                Self::Sev4 => serializer.serialize_unit_variant("Severity", 4u32, "Sev4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List the alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartGroupsList {
    #[doc = "URL to fetch the next set of alerts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of alerts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SmartGroup>,
}
impl azure_core::Continuable for SmartGroupsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SmartGroupsList {
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
