#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Governance rule's condition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "The governance rule Condition's Property, e.g. Severity or AssessmentKey, see examples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[doc = "The governance rule Condition's Value like severity Low, High or assessments keys, see examples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The governance rule Condition's Operator, for example Equals for severity or In for list of assessments, see examples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<condition::Operator>,
}
impl Condition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod condition {
    use super::*;
    #[doc = "The governance rule Condition's Operator, for example Equals for severity or In for list of assessments, see examples"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        In,
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
                Self::In => serializer.serialize_unit_variant("Operator", 1u32, "In"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type Conditions = Vec<Condition>;
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
#[doc = "Governance rule execution parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExecuteGovernanceRuleParams {
    #[doc = "Describe if governance rule should be override"]
    #[serde(rename = "override", default, skip_serializing_if = "Option::is_none")]
    pub override_: Option<bool>,
}
impl ExecuteGovernanceRuleParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Execute status of Security GovernanceRule over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExecuteRuleStatus {
    #[doc = "Unique key for the execution of GovernanceRule"]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}
impl ExecuteRuleStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security GovernanceAssignment over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an governance assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GovernanceAssignmentProperties>,
}
impl GovernanceAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describe the additional data of GovernanceAssignment - optional"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceAssignmentAdditionalData {
    #[doc = "Ticket number associated with this GovernanceAssignment"]
    #[serde(rename = "ticketNumber", default, skip_serializing_if = "Option::is_none")]
    pub ticket_number: Option<i32>,
    #[doc = "Ticket link associated with this GovernanceAssignment - for example: https://snow.com"]
    #[serde(rename = "ticketLink", default, skip_serializing_if = "Option::is_none")]
    pub ticket_link: Option<String>,
    #[doc = "The ticket status associated with this GovernanceAssignment - for example: Active"]
    #[serde(rename = "ticketStatus", default, skip_serializing_if = "Option::is_none")]
    pub ticket_status: Option<String>,
}
impl GovernanceAssignmentAdditionalData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of an governance assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GovernanceAssignmentProperties {
    #[doc = "The Owner for the governance assignment - e.g. user@contoso.com - see example"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The remediation due-date - after this date Secure Score will be affected (in case of  active grace-period)"]
    #[serde(rename = "remediationDueDate", with = "azure_core::date::rfc3339")]
    pub remediation_due_date: time::OffsetDateTime,
    #[doc = "The ETA (estimated time of arrival) for remediation"]
    #[serde(rename = "remediationEta", default, skip_serializing_if = "Option::is_none")]
    pub remediation_eta: Option<RemediationEta>,
    #[doc = "Defines whether there is a grace period on the governance assignment"]
    #[serde(rename = "isGracePeriod", default, skip_serializing_if = "Option::is_none")]
    pub is_grace_period: Option<bool>,
    #[doc = "The governance email weekly notification configuration."]
    #[serde(rename = "governanceEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub governance_email_notification: Option<GovernanceEmailNotification>,
    #[doc = "Describe the additional data of GovernanceAssignment - optional"]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<GovernanceAssignmentAdditionalData>,
}
impl GovernanceAssignmentProperties {
    pub fn new(remediation_due_date: time::OffsetDateTime) -> Self {
        Self {
            owner: None,
            remediation_due_date,
            remediation_eta: None,
            is_grace_period: None,
            governance_email_notification: None,
            additional_data: None,
        }
    }
}
#[doc = "Page of a security governance assignments list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceAssignmentsList {
    #[doc = "Collection of governance assignments in this page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GovernanceAssignment>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GovernanceAssignmentsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GovernanceAssignmentsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The governance email weekly notification configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceEmailNotification {
    #[doc = "Exclude manager from weekly email notification."]
    #[serde(rename = "disableManagerEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub disable_manager_email_notification: Option<bool>,
    #[doc = "Exclude  owner from weekly email notification."]
    #[serde(rename = "disableOwnerEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub disable_owner_email_notification: Option<bool>,
}
impl GovernanceEmailNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security GovernanceRule over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an governanceRule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GovernanceRuleProperties>,
}
impl GovernanceRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of governance rule's condition sets - OR between ConditionSets, AND between conditions in a set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleConditionSets {}
impl GovernanceRuleConditionSets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The governance email weekly notification configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleEmailNotification {
    #[doc = "Defines whether manager email notifications are disabled."]
    #[serde(rename = "disableManagerEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub disable_manager_email_notification: Option<bool>,
    #[doc = "Defines whether owner email notifications are disabled."]
    #[serde(rename = "disableOwnerEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub disable_owner_email_notification: Option<bool>,
}
impl GovernanceRuleEmailNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a security governanceRules list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleList {
    #[doc = "Collection of governanceRules in this page"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GovernanceRule>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GovernanceRuleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GovernanceRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describe the owner source of governance rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleOwnerSource {
    #[doc = "The owner type for the governance rule owner source"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<governance_rule_owner_source::Type>,
    #[doc = "The source value e.g. tag key like owner name or email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GovernanceRuleOwnerSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod governance_rule_owner_source {
    use super::*;
    #[doc = "The owner type for the governance rule owner source"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        ByTag,
        Manually,
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
                Self::ByTag => serializer.serialize_unit_variant("Type", 0u32, "ByTag"),
                Self::Manually => serializer.serialize_unit_variant("Type", 1u32, "Manually"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes properties of an governanceRule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GovernanceRuleProperties {
    #[doc = "display name of the governanceRule"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "description of the governanceRule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Governance rule remediation timeframe - this is the time that will affect on the grace-period duration e.g. 7.00:00:00 - means 7 days"]
    #[serde(rename = "remediationTimeframe", default, skip_serializing_if = "Option::is_none")]
    pub remediation_timeframe: Option<String>,
    #[doc = "Defines whether there is a grace period on the governance rule"]
    #[serde(rename = "isGracePeriod", default, skip_serializing_if = "Option::is_none")]
    pub is_grace_period: Option<bool>,
    #[doc = "The governance rule priority, priority to the lower number. Rules with the same priority on the same subscription will not be allowed"]
    #[serde(rename = "rulePriority")]
    pub rule_priority: i32,
    #[doc = "Defines whether the rule is active/inactive"]
    #[serde(rename = "isDisabled", default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[doc = "The rule type of the governance rule, defines the source of the rule e.g. Integrated"]
    #[serde(rename = "ruleType")]
    pub rule_type: governance_rule_properties::RuleType,
    #[doc = "The governance rule source, what the rule affects, e.g. Assessments"]
    #[serde(rename = "sourceResourceType")]
    pub source_resource_type: governance_rule_properties::SourceResourceType,
    #[doc = "The governance rule conditionSets - see examples"]
    #[serde(rename = "conditionSets")]
    pub condition_sets: Vec<GovernanceRuleConditionSets>,
    #[doc = "Describe the owner source of governance rule"]
    #[serde(rename = "ownerSource")]
    pub owner_source: GovernanceRuleOwnerSource,
    #[doc = "The governance email weekly notification configuration."]
    #[serde(rename = "governanceEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub governance_email_notification: Option<GovernanceRuleEmailNotification>,
}
impl GovernanceRuleProperties {
    pub fn new(
        display_name: String,
        rule_priority: i32,
        rule_type: governance_rule_properties::RuleType,
        source_resource_type: governance_rule_properties::SourceResourceType,
        condition_sets: Vec<GovernanceRuleConditionSets>,
        owner_source: GovernanceRuleOwnerSource,
    ) -> Self {
        Self {
            display_name,
            description: None,
            remediation_timeframe: None,
            is_grace_period: None,
            rule_priority,
            is_disabled: None,
            rule_type,
            source_resource_type,
            condition_sets,
            owner_source,
            governance_email_notification: None,
        }
    }
}
pub mod governance_rule_properties {
    use super::*;
    #[doc = "The rule type of the governance rule, defines the source of the rule e.g. Integrated"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RuleType")]
    pub enum RuleType {
        Integrated,
        ServiceNow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RuleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RuleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RuleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Integrated => serializer.serialize_unit_variant("RuleType", 0u32, "Integrated"),
                Self::ServiceNow => serializer.serialize_unit_variant("RuleType", 1u32, "ServiceNow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The governance rule source, what the rule affects, e.g. Assessments"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceResourceType")]
    pub enum SourceResourceType {
        Assessments,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceResourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceResourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceResourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Assessments => serializer.serialize_unit_variant("SourceResourceType", 0u32, "Assessments"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The ETA (estimated time of arrival) for remediation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemediationEta {
    #[doc = "ETA for remediation."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub eta: time::OffsetDateTime,
    #[doc = "Justification for change of Eta."]
    pub justification: String,
}
impl RemediationEta {
    pub fn new(eta: time::OffsetDateTime, justification: String) -> Self {
        Self { eta, justification }
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
