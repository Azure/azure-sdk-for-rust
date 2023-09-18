#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Actions to invoke when the alert fires."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Actions {
    #[doc = "Action Group resource Ids to invoke when the alert fires."]
    #[serde(
        rename = "actionGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_groups: Vec<String>,
    #[doc = "The properties of an alert payload."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
}
impl Actions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A condition of the scheduled query rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    #[doc = "Log query alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "Aggregation type. Relevant and required only for rules of the kind LogAlert."]
    #[serde(rename = "timeAggregation", default, skip_serializing_if = "Option::is_none")]
    pub time_aggregation: Option<condition::TimeAggregation>,
    #[doc = "The column containing the metric measure number. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "metricMeasureColumn", default, skip_serializing_if = "Option::is_none")]
    pub metric_measure_column: Option<String>,
    #[doc = "The column containing the resource id. The content of the column must be a uri formatted as resource id. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "resourceIdColumn", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_column: Option<String>,
    #[doc = "List of Dimensions conditions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<Dimension>,
    #[doc = "The criteria operator. Relevant and required only for rules of the kind LogAlert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<condition::Operator>,
    #[doc = "the criteria threshold value that activates the alert. Relevant and required only for rules of the kind LogAlert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[doc = "The minimum number of violations required within the selected lookback time window required to raise an alert. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "failingPeriods", default, skip_serializing_if = "Option::is_none")]
    pub failing_periods: Option<condition::FailingPeriods>,
    #[doc = "The name of the metric to be sent. Relevant and required only for rules of the kind LogToMetric."]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}
impl Condition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod condition {
    use super::*;
    #[doc = "Aggregation type. Relevant and required only for rules of the kind LogAlert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeAggregation")]
    pub enum TimeAggregation {
        Count,
        Average,
        Minimum,
        Maximum,
        Total,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TimeAggregation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TimeAggregation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TimeAggregation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("TimeAggregation", 0u32, "Count"),
                Self::Average => serializer.serialize_unit_variant("TimeAggregation", 1u32, "Average"),
                Self::Minimum => serializer.serialize_unit_variant("TimeAggregation", 2u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("TimeAggregation", 3u32, "Maximum"),
                Self::Total => serializer.serialize_unit_variant("TimeAggregation", 4u32, "Total"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The criteria operator. Relevant and required only for rules of the kind LogAlert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
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
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 1u32, "GreaterThan"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("Operator", 2u32, "GreaterThanOrEqual"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 3u32, "LessThan"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("Operator", 4u32, "LessThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The minimum number of violations required within the selected lookback time window required to raise an alert. Relevant only for rules of the kind LogAlert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct FailingPeriods {
        #[doc = "The number of aggregated lookback points. The lookback time window is calculated based on the aggregation granularity (windowSize) and the selected number of aggregated points. Default value is 1"]
        #[serde(rename = "numberOfEvaluationPeriods", default, skip_serializing_if = "Option::is_none")]
        pub number_of_evaluation_periods: Option<i64>,
        #[doc = "The number of violations to trigger an alert. Should be smaller or equal to numberOfEvaluationPeriods. Default value is 1"]
        #[serde(rename = "minFailingPeriodsToAlert", default, skip_serializing_if = "Option::is_none")]
        pub min_failing_periods_to_alert: Option<i64>,
    }
    impl FailingPeriods {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Dimension splitting and filtering definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[doc = "Name of the dimension"]
    pub name: String,
    #[doc = "Operator for dimension values"]
    pub operator: dimension::Operator,
    #[doc = "List of dimension values"]
    pub values: Vec<String>,
}
impl Dimension {
    pub fn new(name: String, operator: dimension::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod dimension {
    use super::*;
    #[doc = "Operator for dimension values"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Include,
        Exclude,
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
                Self::Include => serializer.serialize_unit_variant("Operator", 0u32, "Include"),
                Self::Exclude => serializer.serialize_unit_variant("Operator", 1u32, "Exclude"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorContract {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for ErrorContract {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorContract {
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
#[doc = "The rule criteria that defines the conditions of the scheduled query rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledQueryRuleCriteria {
    #[doc = "A list of conditions to evaluate against the specified scopes"]
    #[serde(
        rename = "allOf",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_of: Vec<Condition>,
}
impl ScheduledQueryRuleCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "scheduled query rule Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledQueryRuleProperties {
    #[doc = "The api-version used when creating this alert rule"]
    #[serde(rename = "createdWithApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub created_with_api_version: Option<String>,
    #[doc = "True if alert rule is legacy Log Analytic rule"]
    #[serde(rename = "isLegacyLogAnalyticsRule", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy_log_analytics_rule: Option<bool>,
    #[doc = "The description of the scheduled query rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the alert rule"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Severity of the alert. Should be an integer between [0-4]. Value of 0 is severest. Relevant and required only for rules of the kind LogAlert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<scheduled_query_rule_properties::Severity>,
    #[doc = "The flag which indicates whether this scheduled query rule is enabled. Value should be true or false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The list of resource id's that this scheduled query rule is scoped to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
    #[doc = "How often the scheduled query rule is evaluated represented in ISO 8601 duration format. Relevant and required only for rules of the kind LogAlert."]
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[doc = "The period of time (in ISO 8601 duration format) on which the Alert query will be executed (bin size). Relevant and required only for rules of the kind LogAlert."]
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[doc = "If specified then overrides the query time range (default is WindowSize*NumberOfEvaluationPeriods). Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "overrideQueryTimeRange", default, skip_serializing_if = "Option::is_none")]
    pub override_query_time_range: Option<String>,
    #[doc = "List of resource type of the target resource(s) on which the alert is created/updated. For example if the scope is a resource group and targetResourceTypes is Microsoft.Compute/virtualMachines, then a different alert will be fired for each virtual machine in the resource group which meet the alert criteria. Relevant only for rules of the kind LogAlert"]
    #[serde(
        rename = "targetResourceTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_resource_types: Vec<String>,
    #[doc = "The rule criteria that defines the conditions of the scheduled query rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<ScheduledQueryRuleCriteria>,
    #[doc = "Mute actions for the chosen period of time (in ISO 8601 duration format) after the alert is fired. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "muteActionsDuration", default, skip_serializing_if = "Option::is_none")]
    pub mute_actions_duration: Option<String>,
    #[doc = "Actions to invoke when the alert fires."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,
    #[doc = "The flag which indicates whether this scheduled query rule has been configured to be stored in the customer's storage. The default is false."]
    #[serde(rename = "isWorkspaceAlertsStorageConfigured", default, skip_serializing_if = "Option::is_none")]
    pub is_workspace_alerts_storage_configured: Option<bool>,
    #[doc = "The flag which indicates whether this scheduled query rule should be stored in the customer's storage. The default is false. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "checkWorkspaceAlertsStorageConfigured", default, skip_serializing_if = "Option::is_none")]
    pub check_workspace_alerts_storage_configured: Option<bool>,
    #[doc = "The flag which indicates whether the provided query should be validated or not. The default is false. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "skipQueryValidation", default, skip_serializing_if = "Option::is_none")]
    pub skip_query_validation: Option<bool>,
    #[doc = "The flag that indicates whether the alert should be automatically resolved or not. The default is true. Relevant only for rules of the kind LogAlert."]
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
}
impl ScheduledQueryRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod scheduled_query_rule_properties {
    use super::*;
    #[doc = "Severity of the alert. Should be an integer between [0-4]. Value of 0 is severest. Relevant and required only for rules of the kind LogAlert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The scheduled query rule resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledQueryRuleResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Indicates the type of scheduled query rule. The default is LogAlert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<scheduled_query_rule_resource::Kind>,
    #[doc = "The etag field is *not* required. If it is provided in the response body, it must also be provided as a header per the normal etag convention.  Entity tags are used for comparing two or more entities from the same requested resource. HTTP/1.1 uses entity tags in the etag (section 14.19), If-Match (section 14.24), If-None-Match (section 14.26), and If-Range (section 14.27) header fields. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "scheduled query rule Definition"]
    pub properties: ScheduledQueryRuleProperties,
}
impl ScheduledQueryRuleResource {
    pub fn new(tracked_resource: TrackedResource, properties: ScheduledQueryRuleProperties) -> Self {
        Self {
            tracked_resource,
            kind: None,
            etag: None,
            system_data: None,
            properties,
        }
    }
}
pub mod scheduled_query_rule_resource {
    use super::*;
    #[doc = "Indicates the type of scheduled query rule. The default is LogAlert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        LogAlert,
        LogToMetric,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LogAlert => serializer.serialize_unit_variant("Kind", 0u32, "LogAlert"),
                Self::LogToMetric => serializer.serialize_unit_variant("Kind", 1u32, "LogToMetric"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a collection of scheduled query rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledQueryRuleResourceCollection {
    #[doc = "The values for the scheduled query rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScheduledQueryRuleResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduledQueryRuleResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScheduledQueryRuleResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The scheduled query rule resource for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledQueryRuleResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "scheduled query rule Definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledQueryRuleProperties>,
}
impl ScheduledQueryRuleResourcePatch {
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
