#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Action rule with action group configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(flatten)]
    pub action_rule_properties: ActionRuleProperties,
    #[doc = "Action group to trigger if action rule matches"]
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
}
impl ActionGroup {
    pub fn new(action_rule_properties: ActionRuleProperties, action_group_id: String) -> Self {
        Self {
            action_rule_properties,
            action_group_id,
        }
    }
}
#[doc = "The Action Groups information, used by the alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupsInformation {
    #[doc = "An optional custom email subject to use in email notifications."]
    #[serde(rename = "customEmailSubject", default, skip_serializing_if = "Option::is_none")]
    pub custom_email_subject: Option<String>,
    #[doc = "An optional custom web-hook payload to use in web-hook notifications."]
    #[serde(rename = "customWebhookPayload", default, skip_serializing_if = "Option::is_none")]
    pub custom_webhook_payload: Option<String>,
    #[doc = "The Action Group resource IDs."]
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
}
impl ActionGroupsInformation {
    pub fn new(group_ids: Vec<String>) -> Self {
        Self {
            custom_email_subject: None,
            custom_webhook_payload: None,
            group_ids,
        }
    }
}
#[doc = "Action rule object containing target scope, conditions and suppression logic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRule {
    #[serde(flatten)]
    pub managed_resource: ManagedResource,
    #[doc = "Action rule properties defining scope, conditions, suppression logic for action rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionRuleProperties>,
}
impl ActionRule {
    pub fn new(managed_resource: ManagedResource) -> Self {
        Self {
            managed_resource,
            properties: None,
        }
    }
}
#[doc = "Action rule properties defining scope, conditions, suppression logic for action rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRuleProperties {
    #[doc = "Target scope for a given action rule. By default scope will be the subscription. User can also provide list of resource groups or list of resources from the scope subscription as well."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[doc = "Conditions in alert instance to be matched for a given action rule. Default value is all. Multiple values could be provided with comma separation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
    #[doc = "Description of action rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Creation time of action rule. Date-Time in ISO-8601 format."]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "Last updated time of action rule. Date-Time in ISO-8601 format."]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
    #[doc = "Created by user name."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Last modified by user name."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Indicates if the given action rule is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<action_rule_properties::Status>,
    #[doc = "Indicates type of action rule"]
    #[serde(rename = "type")]
    pub type_: action_rule_properties::Type,
}
impl ActionRuleProperties {
    pub fn new(type_: action_rule_properties::Type) -> Self {
        Self {
            scope: None,
            conditions: None,
            description: None,
            created_at: None,
            last_modified_at: None,
            created_by: None,
            last_modified_by: None,
            status: None,
            type_,
        }
    }
}
pub mod action_rule_properties {
    use super::*;
    #[doc = "Indicates if the given action rule is enabled or disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates type of action rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Suppression,
        ActionGroup,
        Diagnostics,
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
                Self::Suppression => serializer.serialize_unit_variant("Type", 0u32, "Suppression"),
                Self::ActionGroup => serializer.serialize_unit_variant("Type", 1u32, "ActionGroup"),
                Self::Diagnostics => serializer.serialize_unit_variant("Type", 2u32, "Diagnostics"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of action rules"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionRulesList {
    #[doc = "URL to fetch the next set of action rules"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of action rules"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionRule>,
}
impl azure_core::Continuable for ActionRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ActionRulesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert rule information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRule {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[doc = "The alert rule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRuleProperties>,
}
impl AlertRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert rule patch information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulePatchObject {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The alert rule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRulePatchProperties>,
}
impl AlertRulePatchObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert rule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulePatchProperties {
    #[doc = "The alert rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The alert rule state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<alert_rule_patch_properties::State>,
    #[doc = "The alert rule severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<alert_rule_patch_properties::Severity>,
    #[doc = "The alert rule frequency in ISO8601 format. The time granularity must be in minutes and minimum value is 5 minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[doc = "The Action Groups information, used by the alert rule."]
    #[serde(rename = "actionGroups", default, skip_serializing_if = "Option::is_none")]
    pub action_groups: Option<ActionGroupsInformation>,
    #[doc = "Optional throttling information for the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttling: Option<ThrottlingInformation>,
}
impl AlertRulePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_rule_patch_properties {
    use super::*;
    #[doc = "The alert rule state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The alert rule severity."]
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
#[doc = "The alert rule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleProperties {
    #[doc = "The alert rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The alert rule state."]
    pub state: alert_rule_properties::State,
    #[doc = "The alert rule severity."]
    pub severity: alert_rule_properties::Severity,
    #[doc = "The alert rule frequency in ISO8601 format. The time granularity must be in minutes and minimum value is 5 minutes."]
    pub frequency: String,
    #[doc = "The detector information. By default this is not populated, unless it's specified in expandDetector"]
    pub detector: Detector,
    #[doc = "The alert rule resources scope."]
    pub scope: Vec<String>,
    #[doc = "The Action Groups information, used by the alert rule."]
    #[serde(rename = "actionGroups")]
    pub action_groups: ActionGroupsInformation,
    #[doc = "Optional throttling information for the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttling: Option<ThrottlingInformation>,
}
impl AlertRuleProperties {
    pub fn new(
        state: alert_rule_properties::State,
        severity: alert_rule_properties::Severity,
        frequency: String,
        detector: Detector,
        scope: Vec<String>,
        action_groups: ActionGroupsInformation,
    ) -> Self {
        Self {
            description: None,
            state,
            severity,
            frequency,
            detector,
            scope,
            action_groups,
            throttling: None,
        }
    }
}
pub mod alert_rule_properties {
    use super::*;
    #[doc = "The alert rule state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The alert rule severity."]
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
#[doc = "List of Smart Detector alert rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulesList {
    #[doc = "List of Smart Detector alert rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertRule>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AlertRulesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "condition to trigger an action rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "operator for a given condition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<condition::Operator>,
    #[doc = "list of values to match for a given condition."]
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
    #[doc = "operator for a given condition"]
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
#[doc = "Conditions in alert instance to be matched for a given action rule. Default value is all. Multiple values could be provided with comma separation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Conditions {
    #[doc = "condition to trigger an action rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Condition>,
    #[doc = "condition to trigger an action rule"]
    #[serde(rename = "monitorService", default, skip_serializing_if = "Option::is_none")]
    pub monitor_service: Option<Condition>,
    #[doc = "condition to trigger an action rule"]
    #[serde(rename = "monitorCondition", default, skip_serializing_if = "Option::is_none")]
    pub monitor_condition: Option<Condition>,
    #[doc = "condition to trigger an action rule"]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<Condition>,
    #[doc = "condition to trigger an action rule"]
    #[serde(rename = "alertRuleId", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule_id: Option<Condition>,
    #[doc = "condition to trigger an action rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Condition>,
    #[doc = "condition to trigger an action rule"]
    #[serde(rename = "alertContext", default, skip_serializing_if = "Option::is_none")]
    pub alert_context: Option<Condition>,
}
impl Conditions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The detector information. By default this is not populated, unless it's specified in expandDetector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Detector {
    #[doc = "The detector id."]
    pub id: String,
    #[doc = "The detector's parameters.'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The Smart Detector name. By default this is not populated, unless it's specified in expandDetector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Smart Detector description. By default this is not populated, unless it's specified in expandDetector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Smart Detector supported resource types. By default this is not populated, unless it's specified in expandDetector"]
    #[serde(rename = "supportedResourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_resource_types: Vec<String>,
    #[doc = "The Smart Detector image path. By default this is not populated, unless it's specified in expandDetector"]
    #[serde(rename = "imagePaths", default, skip_serializing_if = "Vec::is_empty")]
    pub image_paths: Vec<String>,
}
impl Detector {
    pub fn new(id: String) -> Self {
        Self {
            id,
            parameters: None,
            name: None,
            description: None,
            supported_resource_types: Vec::new(),
            image_paths: Vec::new(),
        }
    }
}
#[doc = "Action rule with diagnostics configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diagnostics {
    #[serde(flatten)]
    pub action_rule_properties: ActionRuleProperties,
}
impl Diagnostics {
    pub fn new(action_rule_properties: ActionRuleProperties) -> Self {
        Self { action_rule_properties }
    }
}
#[doc = "An azure managed resource object"]
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
#[doc = "Data contract for patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchObject {
    #[doc = "Action rule properties supported by patch"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
    #[doc = "tags to be updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action rule properties supported by patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[doc = "Indicates if the given action rule is enabled or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<patch_properties::Status>,
}
impl PatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_properties {
    use super::*;
    #[doc = "Indicates if the given action rule is enabled or disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Target scope for a given action rule. By default scope will be the subscription. User can also provide list of resource groups or list of resources from the scope subscription as well."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scope {
    #[doc = "type of target scope"]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<scope::ScopeType>,
    #[doc = "list of ARM IDs of the given scope type which will be the target of the given action rule."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
impl Scope {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scope {
    use super::*;
    #[doc = "type of target scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScopeType")]
    pub enum ScopeType {
        ResourceGroup,
        Resource,
        Subscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScopeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScopeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScopeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ResourceGroup => serializer.serialize_unit_variant("ScopeType", 0u32, "ResourceGroup"),
                Self::Resource => serializer.serialize_unit_variant("ScopeType", 1u32, "Resource"),
                Self::Subscription => serializer.serialize_unit_variant("ScopeType", 2u32, "Subscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describe the format of an Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartDetectorErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for SmartDetectorErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SmartDetectorErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action rule with suppression configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Suppression {
    #[serde(flatten)]
    pub action_rule_properties: ActionRuleProperties,
    #[doc = "Suppression logic for a given action rule"]
    #[serde(rename = "suppressionConfig")]
    pub suppression_config: SuppressionConfig,
}
impl Suppression {
    pub fn new(action_rule_properties: ActionRuleProperties, suppression_config: SuppressionConfig) -> Self {
        Self {
            action_rule_properties,
            suppression_config,
        }
    }
}
#[doc = "Suppression logic for a given action rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionConfig {
    #[doc = "Specifies when the suppression should be applied"]
    #[serde(rename = "recurrenceType")]
    pub recurrence_type: suppression_config::RecurrenceType,
    #[doc = "Schedule for a given suppression configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<SuppressionSchedule>,
}
impl SuppressionConfig {
    pub fn new(recurrence_type: suppression_config::RecurrenceType) -> Self {
        Self {
            recurrence_type,
            schedule: None,
        }
    }
}
pub mod suppression_config {
    use super::*;
    #[doc = "Specifies when the suppression should be applied"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecurrenceType")]
    pub enum RecurrenceType {
        Always,
        Once,
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
                Self::Always => serializer.serialize_unit_variant("RecurrenceType", 0u32, "Always"),
                Self::Once => serializer.serialize_unit_variant("RecurrenceType", 1u32, "Once"),
                Self::Daily => serializer.serialize_unit_variant("RecurrenceType", 2u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("RecurrenceType", 3u32, "Weekly"),
                Self::Monthly => serializer.serialize_unit_variant("RecurrenceType", 4u32, "Monthly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schedule for a given suppression configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuppressionSchedule {
    #[doc = "Start date for suppression"]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "End date for suppression"]
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "Start time for suppression"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End date for suppression"]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Specifies the values for recurrence pattern"]
    #[serde(rename = "recurrenceValues", default, skip_serializing_if = "Vec::is_empty")]
    pub recurrence_values: Vec<i64>,
}
impl SuppressionSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional throttling information for the alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThrottlingInformation {
    #[doc = "The required duration (in ISO8601 format) to wait before notifying on the alert rule again. The time granularity must be in minutes and minimum value is 0 minutes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl ThrottlingInformation {
    pub fn new() -> Self {
        Self::default()
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
