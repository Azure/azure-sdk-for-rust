#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "the aggregation type of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggregationType {
    None,
    Average,
    Count,
    Minimum,
    Maximum,
    Total,
}
#[doc = "Represents a baseline metadata value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaselineMetadata {
    #[doc = "Name of the baseline metadata."]
    pub name: String,
    #[doc = "Value of the baseline metadata."]
    pub value: String,
}
impl BaselineMetadata {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Type of operation: get, read, delete, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[doc = "Name of dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of dimension."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Legacy usage, should not set."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
    #[doc = "When set, the dimension is hidden from the customer, used in conjunction with the defaultDimensionValues field below"]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "Default dimension value to be sent down for the hidden dimension during query"]
    #[serde(rename = "defaultDimensionValues", default, skip_serializing_if = "Option::is_none")]
    pub default_dimension_values: Option<serde_json::Value>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Criterion for dynamic threshold."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicMetricCriteria {
    #[serde(flatten)]
    pub multi_metric_criteria: MultiMetricCriteria,
    #[doc = "The operator used to compare the metric value against the threshold."]
    pub operator: dynamic_metric_criteria::Operator,
    #[doc = "The extent of deviation required to trigger an alert. This will affect how tight the threshold is to the metric series pattern."]
    #[serde(rename = "alertSensitivity")]
    pub alert_sensitivity: dynamic_metric_criteria::AlertSensitivity,
    #[doc = "The minimum number of violations required within the selected lookback time window required to raise an alert."]
    #[serde(rename = "failingPeriods")]
    pub failing_periods: DynamicThresholdFailingPeriods,
    #[doc = "Use this option to set the date from which to start learning the metric historical data and calculate the dynamic thresholds (in ISO8601 format)"]
    #[serde(rename = "ignoreDataBefore", default, with = "azure_core::date::rfc3339::option")]
    pub ignore_data_before: Option<time::OffsetDateTime>,
}
impl DynamicMetricCriteria {
    pub fn new(
        multi_metric_criteria: MultiMetricCriteria,
        operator: dynamic_metric_criteria::Operator,
        alert_sensitivity: dynamic_metric_criteria::AlertSensitivity,
        failing_periods: DynamicThresholdFailingPeriods,
    ) -> Self {
        Self {
            multi_metric_criteria,
            operator,
            alert_sensitivity,
            failing_periods,
            ignore_data_before: None,
        }
    }
}
pub mod dynamic_metric_criteria {
    use super::*;
    #[doc = "The operator used to compare the metric value against the threshold."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        GreaterThan,
        LessThan,
        GreaterOrLessThan,
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
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 0u32, "GreaterThan"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 1u32, "LessThan"),
                Self::GreaterOrLessThan => serializer.serialize_unit_variant("Operator", 2u32, "GreaterOrLessThan"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The extent of deviation required to trigger an alert. This will affect how tight the threshold is to the metric series pattern."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlertSensitivity")]
    pub enum AlertSensitivity {
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlertSensitivity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlertSensitivity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlertSensitivity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("AlertSensitivity", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("AlertSensitivity", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("AlertSensitivity", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The minimum number of violations required within the selected lookback time window required to raise an alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicThresholdFailingPeriods {
    #[doc = "The number of aggregated lookback points. The lookback time window is calculated based on the aggregation granularity (windowSize) and the selected number of aggregated points."]
    #[serde(rename = "numberOfEvaluationPeriods")]
    pub number_of_evaluation_periods: f64,
    #[doc = "The number of violations to trigger an alert. Should be smaller or equal to numberOfEvaluationPeriods."]
    #[serde(rename = "minFailingPeriodsToAlert")]
    pub min_failing_periods_to_alert: f64,
}
impl DynamicThresholdFailingPeriods {
    pub fn new(number_of_evaluation_periods: f64, min_failing_periods_to_alert: f64) -> Self {
        Self {
            number_of_evaluation_periods,
            min_failing_periods_to_alert,
        }
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorContract {
    #[doc = "Describes the format of Error response."]
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
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[doc = "The localizable string class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    #[doc = "the invariant value."]
    pub value: String,
    #[doc = "the locale specific value."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizableString {
    pub fn new(value: String) -> Self {
        Self {
            value,
            localized_value: None,
        }
    }
}
#[doc = "Log specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of log specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of specification."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a metric metadata value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataValue {
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[doc = "the value of the metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetadataValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result data of a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[doc = "the metric Id."]
    pub id: String,
    #[doc = "the resource type of the metric resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The localizable string class."]
    pub name: LocalizableString,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "'Success' or the error details on query failures for this metric."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message encountered querying this specific metric."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The unit of the metric."]
    pub unit: Unit,
    #[doc = "the time series returned when a data query is performed."]
    pub timeseries: Vec<TimeSeriesElement>,
}
impl Metric {
    pub fn new(id: String, type_: String, name: LocalizableString, unit: Unit, timeseries: Vec<TimeSeriesElement>) -> Self {
        Self {
            id,
            type_,
            name,
            display_description: None,
            error_code: None,
            error_message: None,
            unit,
            timeseries,
        }
    }
}
#[doc = "the aggregation type of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricAggregationType")]
pub enum MetricAggregationType {
    None,
    Average,
    Count,
    Minimum,
    Maximum,
    Total,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricAggregationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricAggregationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricAggregationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("MetricAggregationType", 0u32, "None"),
            Self::Average => serializer.serialize_unit_variant("MetricAggregationType", 1u32, "Average"),
            Self::Count => serializer.serialize_unit_variant("MetricAggregationType", 2u32, "Count"),
            Self::Minimum => serializer.serialize_unit_variant("MetricAggregationType", 3u32, "Minimum"),
            Self::Maximum => serializer.serialize_unit_variant("MetricAggregationType", 4u32, "Maximum"),
            Self::Total => serializer.serialize_unit_variant("MetricAggregationType", 5u32, "Total"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An alert action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertAction {
    #[doc = "the id of the action group to use."]
    #[serde(rename = "actionGroupId", default, skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[doc = "This field allows specifying custom properties, which would be appended to the alert payload sent as input to the webhook."]
    #[serde(rename = "webHookProperties", default, skip_serializing_if = "Option::is_none")]
    pub web_hook_properties: Option<serde_json::Value>,
}
impl MetricAlertAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "specifies the type of the alert criteria."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "odata.type")]
pub enum MetricAlertCriteriaUnion {
    #[serde(rename = "Microsoft.Azure.Monitor.MultipleResourceMultipleMetricCriteria")]
    MicrosoftAzureMonitorMultipleResourceMultipleMetricCriteria(MetricAlertMultipleResourceMultipleMetricCriteria),
    #[serde(rename = "Microsoft.Azure.Monitor.SingleResourceMultipleMetricCriteria")]
    MicrosoftAzureMonitorSingleResourceMultipleMetricCriteria(MetricAlertSingleResourceMultipleMetricCriteria),
    #[serde(rename = "Microsoft.Azure.Monitor.WebtestLocationAvailabilityCriteria")]
    MicrosoftAzureMonitorWebtestLocationAvailabilityCriteria(WebtestLocationAvailabilityCriteria),
}
#[doc = "Specifies the metric alert criteria for multiple resource that has multiple metric criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertMultipleResourceMultipleMetricCriteria {
    #[doc = "the list of multiple metric criteria for this 'all of' operation. "]
    #[serde(
        rename = "allOf",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_of: Vec<MultiMetricCriteriaUnion>,
}
impl MetricAlertMultipleResourceMultipleMetricCriteria {
    pub fn new() -> Self {
        Self { all_of: Vec::new() }
    }
}
#[doc = "An alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertProperties {
    #[doc = "the description of the metric alert that will be included in the alert email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Alert severity {0, 1, 2, 3, 4}"]
    pub severity: i32,
    #[doc = "the flag that indicates whether the metric alert is enabled."]
    pub enabled: bool,
    #[doc = "the list of resource id's that this metric alert is scoped to."]
    pub scopes: Vec<String>,
    #[doc = "how often the metric alert is evaluated represented in ISO 8601 duration format."]
    #[serde(rename = "evaluationFrequency")]
    pub evaluation_frequency: String,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold."]
    #[serde(rename = "windowSize")]
    pub window_size: String,
    #[doc = "the resource type of the target resource(s) on which the alert is created/updated. Mandatory if the scope contains a subscription, resource group, or more than one resource."]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "the region of the target resource(s) on which the alert is created/updated. Mandatory if the scope contains a subscription, resource group, or more than one resource."]
    #[serde(rename = "targetResourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
    #[doc = "The rule criteria that defines the conditions of the alert rule."]
    pub criteria: MetricAlertCriteriaUnion,
    #[doc = "the flag that indicates whether the alert should be auto resolved or not. The default is true."]
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[doc = "the array of actions that are performed when the alert rule becomes active, and when an alert condition is resolved."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<MetricAlertAction>,
    #[doc = "Last time the rule was updated in ISO8601 format."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "the value indicating whether this alert rule is migrated."]
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
}
impl MetricAlertProperties {
    pub fn new(
        severity: i32,
        enabled: bool,
        scopes: Vec<String>,
        evaluation_frequency: String,
        window_size: String,
        criteria: MetricAlertCriteriaUnion,
    ) -> Self {
        Self {
            description: None,
            severity,
            enabled,
            scopes,
            evaluation_frequency,
            window_size,
            target_resource_type: None,
            target_resource_region: None,
            criteria,
            auto_mitigate: None,
            actions: Vec::new(),
            last_updated_time: None,
            is_migrated: None,
        }
    }
}
#[doc = "An alert rule properties for patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertPropertiesPatch {
    #[doc = "the description of the metric alert that will be included in the alert email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Alert severity {0, 1, 2, 3, 4}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[doc = "the flag that indicates whether the metric alert is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "the list of resource id's that this metric alert is scoped to."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
    #[doc = "how often the metric alert is evaluated represented in ISO 8601 duration format."]
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[doc = "the period of time (in ISO 8601 duration format) that is used to monitor alert activity based on the threshold."]
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[doc = "the resource type of the target resource(s) on which the alert is created/updated. Mandatory for MultipleResourceMultipleMetricCriteria."]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "the region of the target resource(s) on which the alert is created/updated. Mandatory for MultipleResourceMultipleMetricCriteria."]
    #[serde(rename = "targetResourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
    #[doc = "The rule criteria that defines the conditions of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<MetricAlertCriteriaUnion>,
    #[doc = "the flag that indicates whether the alert should be auto resolved or not. The default is true."]
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[doc = "the array of actions that are performed when the alert rule becomes active, and when an alert condition is resolved."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<MetricAlertAction>,
    #[doc = "Last time the rule was updated in ISO8601 format."]
    #[serde(rename = "lastUpdatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "the value indicating whether this alert rule is migrated."]
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
}
impl MetricAlertPropertiesPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric alert resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An alert rule."]
    pub properties: MetricAlertProperties,
}
impl MetricAlertResource {
    pub fn new(resource: Resource, properties: MetricAlertProperties) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertResourceCollection {
    #[doc = "the values for the alert rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MetricAlertResource>,
}
impl azure_core::Continuable for MetricAlertResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricAlertResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric alert resource for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An alert rule properties for patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertPropertiesPatch>,
}
impl MetricAlertResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the metric alert criteria for a single resource that has multiple metric criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertSingleResourceMultipleMetricCriteria {
    #[doc = "The list of metric criteria for this 'all of' operation. "]
    #[serde(
        rename = "allOf",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub all_of: Vec<MetricCriteria>,
}
impl MetricAlertSingleResourceMultipleMetricCriteria {
    pub fn new() -> Self {
        Self { all_of: Vec::new() }
    }
}
#[doc = "An alert status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertStatus {
    #[doc = "The status name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The alert rule arm id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The extended resource type name."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "An alert status properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertStatusProperties>,
}
impl MetricAlertStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertStatusCollection {
    #[doc = "the values for the alert rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MetricAlertStatus>,
}
impl MetricAlertStatusCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAlertStatusProperties {
    #[doc = "An object describing the type of the dimensions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<serde_json::Value>,
    #[doc = "status value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "UTC time when the status was checked."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl MetricAlertStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric availability specifies the time grain (aggregation interval or frequency) and the retention period for that time grain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[doc = "the time grain specifies the aggregation interval for the metric. Expressed as a duration 'PT1M', 'P1D', etc."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "the retention period for the metric at the specified timegrain.  Expressed as a duration 'PT1M', 'P1D', etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a metric baselines query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBaselinesProperties {
    #[doc = "The timespan for which the data was retrieved. Its value consists of two datetimes concatenated, separated by '/'.  This may be adjusted in the future and returned back from what was originally requested."]
    pub timespan: String,
    #[doc = "The interval (window size) for which the metric data was returned in.  This may be adjusted in the future and returned back from what was originally requested.  This is not present if a metadata request was made."]
    pub interval: String,
    #[doc = "The namespace of the metrics been queried."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The baseline for each time series that was queried."]
    pub baselines: Vec<TimeSeriesBaseline>,
}
impl MetricBaselinesProperties {
    pub fn new(timespan: String, interval: String, baselines: Vec<TimeSeriesBaseline>) -> Self {
        Self {
            timespan,
            interval,
            namespace: None,
            baselines,
        }
    }
}
#[doc = "A list of metric baselines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricBaselinesResponse {
    #[doc = "The list of metric baselines."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SingleMetricBaseline>,
}
impl azure_core::Continuable for MetricBaselinesResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricBaselinesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricClass")]
pub enum MetricClass {
    Availability,
    Transactions,
    Errors,
    Latency,
    Saturation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricClass {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricClass {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricClass {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Availability => serializer.serialize_unit_variant("MetricClass", 0u32, "Availability"),
            Self::Transactions => serializer.serialize_unit_variant("MetricClass", 1u32, "Transactions"),
            Self::Errors => serializer.serialize_unit_variant("MetricClass", 2u32, "Errors"),
            Self::Latency => serializer.serialize_unit_variant("MetricClass", 3u32, "Latency"),
            Self::Saturation => serializer.serialize_unit_variant("MetricClass", 4u32, "Saturation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Criterion to filter metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCriteria {
    #[serde(flatten)]
    pub multi_metric_criteria: MultiMetricCriteria,
    #[doc = "the criteria operator."]
    pub operator: metric_criteria::Operator,
    #[doc = "the criteria threshold value that activates the alert."]
    pub threshold: f64,
}
impl MetricCriteria {
    pub fn new(multi_metric_criteria: MultiMetricCriteria, operator: metric_criteria::Operator, threshold: f64) -> Self {
        Self {
            multi_metric_criteria,
            operator,
            threshold,
        }
    }
}
pub mod metric_criteria {
    use super::*;
    #[doc = "the criteria operator."]
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
}
#[doc = "Metric definition class specifies the metadata for a metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[doc = "Flag to indicate whether the dimension is required."]
    #[serde(rename = "isDimensionRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_dimension_required: Option<bool>,
    #[doc = "the resource identifier of the resource that emitted the metric."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "the namespace the metric belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Custom category name for this metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The class of the metric."]
    #[serde(rename = "metricClass", default, skip_serializing_if = "Option::is_none")]
    pub metric_class: Option<MetricClass>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<MetricUnit>,
    #[doc = "the aggregation type of the metric."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<AggregationType>,
    #[doc = "the collection of what aggregation types are supported."]
    #[serde(
        rename = "supportedAggregationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_aggregation_types: Vec<AggregationType>,
    #[doc = "the collection of what aggregation intervals are available to be queried."]
    #[serde(
        rename = "metricAvailabilities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[doc = "the resource identifier of the metric definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the name and the display name of the dimension, i.e. it is a localizable string."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<LocalizableString>,
}
impl MetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents collection of metric definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionCollection {
    #[doc = "the values for the metric definitions."]
    pub value: Vec<MetricDefinition>,
}
impl azure_core::Continuable for MetricDefinitionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricDefinitionCollection {
    pub fn new(value: Vec<MetricDefinition>) -> Self {
        Self { value }
    }
}
#[doc = "Specifies a metric dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    #[doc = "Name of the dimension."]
    pub name: String,
    #[doc = "the dimension operator. Only 'Include' and 'Exclude' are supported"]
    pub operator: String,
    #[doc = "list of dimension values."]
    pub values: Vec<String>,
}
impl MetricDimension {
    pub fn new(name: String, operator: String, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
#[doc = "Metric namespace class specifies the metadata for a metric namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNamespace {
    #[doc = "The ID of the metric namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the namespace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The escaped name of the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Kind of namespace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<NamespaceClassification>,
    #[doc = "The fully qualified metric namespace name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricNamespaceName>,
}
impl MetricNamespace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents collection of metric namespaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricNamespaceCollection {
    #[doc = "The values for the metric namespaces."]
    pub value: Vec<MetricNamespace>,
}
impl azure_core::Continuable for MetricNamespaceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricNamespaceCollection {
    pub fn new(value: Vec<MetricNamespace>) -> Self {
        Self { value }
    }
}
#[doc = "The fully qualified metric namespace name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNamespaceName {
    #[doc = "The metric namespace name."]
    #[serde(rename = "metricNamespaceName", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace_name: Option<String>,
}
impl MetricNamespaceName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric dimension name and value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSingleDimension {
    #[doc = "Name of the dimension."]
    pub name: String,
    #[doc = "Value of the dimension."]
    pub value: String,
}
impl MetricSingleDimension {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Metric specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Display description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The metric unit. Possible values include: Count,Bytes,Seconds,Percent,CountPerSecond,BytesPerSecond,MilliSeconds,ByteSeconds,Unspecified,BitsPerSecond,Cores,MilliCores,NanoCores"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The default metric aggregation type. Possible values include: Total,Average,Maximum,Minimum,Count"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "The supported aggregation types for the metrics."]
    #[serde(
        rename = "supportedAggregationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The supported time grain types for the metrics."]
    #[serde(
        rename = "supportedTimeGrainTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "The supported time grain types for the metrics."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availabilities: Vec<String>,
    #[doc = "The metric lock aggregation type."]
    #[serde(rename = "lockAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub lock_aggregation_type: Option<String>,
    #[doc = "Category or type of metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The dimensions of metric."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<DimensionProperties>,
    #[doc = "Property to specify whether to fill empty gaps with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "The internal metric name."]
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricUnit")]
pub enum MetricUnit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
    ByteSeconds,
    Unspecified,
    Cores,
    MilliCores,
    NanoCores,
    BitsPerSecond,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricUnit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count => serializer.serialize_unit_variant("MetricUnit", 0u32, "Count"),
            Self::Bytes => serializer.serialize_unit_variant("MetricUnit", 1u32, "Bytes"),
            Self::Seconds => serializer.serialize_unit_variant("MetricUnit", 2u32, "Seconds"),
            Self::CountPerSecond => serializer.serialize_unit_variant("MetricUnit", 3u32, "CountPerSecond"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("MetricUnit", 4u32, "BytesPerSecond"),
            Self::Percent => serializer.serialize_unit_variant("MetricUnit", 5u32, "Percent"),
            Self::MilliSeconds => serializer.serialize_unit_variant("MetricUnit", 6u32, "MilliSeconds"),
            Self::ByteSeconds => serializer.serialize_unit_variant("MetricUnit", 7u32, "ByteSeconds"),
            Self::Unspecified => serializer.serialize_unit_variant("MetricUnit", 8u32, "Unspecified"),
            Self::Cores => serializer.serialize_unit_variant("MetricUnit", 9u32, "Cores"),
            Self::MilliCores => serializer.serialize_unit_variant("MetricUnit", 10u32, "MilliCores"),
            Self::NanoCores => serializer.serialize_unit_variant("MetricUnit", 11u32, "NanoCores"),
            Self::BitsPerSecond => serializer.serialize_unit_variant("MetricUnit", 12u32, "BitsPerSecond"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a metric value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    #[doc = "the timestamp for the metric value in ISO 8601 format."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: time::OffsetDateTime,
    #[doc = "the average value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "the least value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "the greatest value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[doc = "the sum of all of the values in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[doc = "the number of samples in the time range. Can be used to determine the number of values that contributed to the average value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
}
impl MetricValue {
    pub fn new(time_stamp: time::OffsetDateTime) -> Self {
        Self {
            time_stamp,
            average: None,
            minimum: None,
            maximum: None,
            total: None,
            count: None,
        }
    }
}
#[doc = "The types of conditions for a multi resource alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiMetricCriteria {
    #[doc = "Name of the criteria."]
    pub name: String,
    #[doc = "Name of the metric."]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "Namespace of the metric."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[doc = "the criteria time aggregation types."]
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: multi_metric_criteria::TimeAggregation,
    #[doc = "List of dimension conditions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "Allows creating an alert rule on a custom metric that isn't yet emitted, by causing the metric validation to be skipped."]
    #[serde(rename = "skipMetricValidation", default, skip_serializing_if = "Option::is_none")]
    pub skip_metric_validation: Option<bool>,
}
impl MultiMetricCriteria {
    pub fn new(name: String, metric_name: String, time_aggregation: multi_metric_criteria::TimeAggregation) -> Self {
        Self {
            name,
            metric_name,
            metric_namespace: None,
            time_aggregation,
            dimensions: Vec::new(),
            skip_metric_validation: None,
        }
    }
}
pub mod multi_metric_criteria {
    use super::*;
    #[doc = "the criteria time aggregation types."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeAggregation")]
    pub enum TimeAggregation {
        Average,
        Count,
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
                Self::Average => serializer.serialize_unit_variant("TimeAggregation", 0u32, "Average"),
                Self::Count => serializer.serialize_unit_variant("TimeAggregation", 1u32, "Count"),
                Self::Minimum => serializer.serialize_unit_variant("TimeAggregation", 2u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("TimeAggregation", 3u32, "Maximum"),
                Self::Total => serializer.serialize_unit_variant("TimeAggregation", 4u32, "Total"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the type of threshold criteria"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "criterionType")]
pub enum MultiMetricCriteriaUnion {
    DynamicThresholdCriterion(DynamicMetricCriteria),
    StaticThresholdCriterion(MetricCriteria),
}
#[doc = "Kind of namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NamespaceClassification")]
pub enum NamespaceClassification {
    Platform,
    Custom,
    Qos,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NamespaceClassification {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NamespaceClassification {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NamespaceClassification {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Platform => serializer.serialize_unit_variant("NamespaceClassification", 0u32, "Platform"),
            Self::Custom => serializer.serialize_unit_variant("NamespaceClassification", 1u32, "Custom"),
            Self::Qos => serializer.serialize_unit_variant("NamespaceClassification", 2u32, "Qos"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Microsoft Insights API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Property to specify whether the action is a data action."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The publisher of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub publisher: Option<String>,
        #[doc = "Service provider: Microsoft.Insights"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: AlertRules, Autoscale, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Microsoft.Insights operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Insights provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "One property of operation, include log specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "The response to a metrics query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[doc = "The integer value representing the relative cost of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[doc = "The timespan for which the data was retrieved. Its value consists of two datetimes concatenated, separated by '/'.  This may be adjusted in the future and returned back from what was originally requested."]
    pub timespan: String,
    #[doc = "The interval (window size) for which the metric data was returned in.  This may be adjusted in the future and returned back from what was originally requested.  This is not present if a metadata request was made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The namespace of the metrics being queried"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The region of the resource being queried for metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceregion: Option<String>,
    #[doc = "the value of the collection."]
    pub value: Vec<Metric>,
}
impl Response {
    pub fn new(timespan: String, value: Vec<Metric>) -> Self {
        Self {
            cost: None,
            timespan,
            interval: None,
            namespace: None,
            resourceregion: None,
            value,
        }
    }
}
#[doc = "One property of operation, include log specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Log specifications of operation."]
    #[serde(
        rename = "logSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Metric specifications of operation."]
    #[serde(
        rename = "metricSpecifications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_specifications: Vec<MetricSpecification>,
    #[doc = "Legacy Metric specifications for operation. Deprecated, do not use."]
    #[serde(rename = "legacyMetricSpecifications", default, skip_serializing_if = "Option::is_none")]
    pub legacy_metric_specifications: Option<serde_json::Value>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The baseline values for a single sensitivity value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleBaseline {
    #[doc = "the sensitivity of the baseline."]
    pub sensitivity: single_baseline::Sensitivity,
    #[doc = "The low thresholds of the baseline."]
    #[serde(rename = "lowThresholds")]
    pub low_thresholds: Vec<f64>,
    #[doc = "The high thresholds of the baseline."]
    #[serde(rename = "highThresholds")]
    pub high_thresholds: Vec<f64>,
}
impl SingleBaseline {
    pub fn new(sensitivity: single_baseline::Sensitivity, low_thresholds: Vec<f64>, high_thresholds: Vec<f64>) -> Self {
        Self {
            sensitivity,
            low_thresholds,
            high_thresholds,
        }
    }
}
pub mod single_baseline {
    use super::*;
    #[doc = "the sensitivity of the baseline."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Sensitivity")]
    pub enum Sensitivity {
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Sensitivity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Sensitivity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Sensitivity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("Sensitivity", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("Sensitivity", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("Sensitivity", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The baseline results of a single metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleMetricBaseline {
    #[doc = "The metric baseline Id."]
    pub id: String,
    #[doc = "The resource type of the metric baseline resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The name of the metric for which the baselines were retrieved."]
    pub name: String,
    #[doc = "The response to a metric baselines query."]
    pub properties: MetricBaselinesProperties,
}
impl SingleMetricBaseline {
    pub fn new(id: String, type_: String, name: String, properties: MetricBaselinesProperties) -> Self {
        Self {
            id,
            type_,
            name,
            properties,
        }
    }
}
#[doc = "The result data of a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionScopeMetric {
    #[doc = "the metric Id."]
    pub id: String,
    #[doc = "the resource type of the metric resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The localizable string class."]
    pub name: LocalizableString,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "'Success' or the error details on query failures for this metric."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message encountered querying this specific metric."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The unit of the metric."]
    pub unit: MetricUnit,
    #[doc = "the time series returned when a data query is performed."]
    pub timeseries: Vec<TimeSeriesElement>,
}
impl SubscriptionScopeMetric {
    pub fn new(id: String, type_: String, name: LocalizableString, unit: MetricUnit, timeseries: Vec<TimeSeriesElement>) -> Self {
        Self {
            id,
            type_,
            name,
            display_description: None,
            error_code: None,
            error_message: None,
            unit,
            timeseries,
        }
    }
}
#[doc = "Metric definition class specifies the metadata for a metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionScopeMetricDefinition {
    #[doc = "Flag to indicate whether the dimension is required."]
    #[serde(rename = "isDimensionRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_dimension_required: Option<bool>,
    #[doc = "the resource identifier of the resource that emitted the metric."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "the namespace the metric belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Custom category name for this metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The class of the metric."]
    #[serde(rename = "metricClass", default, skip_serializing_if = "Option::is_none")]
    pub metric_class: Option<MetricClass>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<MetricUnit>,
    #[doc = "the aggregation type of the metric."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<MetricAggregationType>,
    #[doc = "the collection of what aggregation types are supported."]
    #[serde(
        rename = "supportedAggregationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_aggregation_types: Vec<MetricAggregationType>,
    #[doc = "the collection of what aggregation intervals are available to be queried."]
    #[serde(
        rename = "metricAvailabilities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[doc = "the resource identifier of the metric definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the name and the display name of the dimension, i.e. it is a localizable string."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<LocalizableString>,
}
impl SubscriptionScopeMetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents collection of metric definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionScopeMetricDefinitionCollection {
    #[doc = "The values for the metric definitions."]
    pub value: Vec<SubscriptionScopeMetricDefinition>,
}
impl azure_core::Continuable for SubscriptionScopeMetricDefinitionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SubscriptionScopeMetricDefinitionCollection {
    pub fn new(value: Vec<SubscriptionScopeMetricDefinition>) -> Self {
        Self { value }
    }
}
#[doc = "The response to a subscription scope metrics query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionScopeMetricResponse {
    #[doc = "The integer value representing the relative cost of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[doc = "The timespan for which the data was retrieved. Its value consists of two datetimes concatenated, separated by '/'.  This may be adjusted in the future and returned back from what was originally requested."]
    pub timespan: String,
    #[doc = "The interval (window size) for which the metric data was returned in.  This may be adjusted in the future and returned back from what was originally requested.  This is not present if a metadata request was made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The namespace of the metrics being queried"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The region of the resource being queried for metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceregion: Option<String>,
    #[doc = "the value of the collection."]
    pub value: Vec<SubscriptionScopeMetric>,
}
impl SubscriptionScopeMetricResponse {
    pub fn new(timespan: String, value: Vec<SubscriptionScopeMetric>) -> Self {
        Self {
            cost: None,
            timespan,
            interval: None,
            namespace: None,
            resourceregion: None,
            value,
        }
    }
}
#[doc = "Query parameters can also be specified in the body, specifying the same parameter in both the body and query parameters will result in an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionScopeMetricsRequestBodyParameters {
    #[doc = "The timespan of the query. It is a string with the following format 'startDateTime_ISO/endDateTime_ISO'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,
    #[doc = "The interval (i.e. timegrain) of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The names of the metrics (comma separated) to retrieve."]
    #[serde(rename = "metricNames", default, skip_serializing_if = "Option::is_none")]
    pub metric_names: Option<String>,
    #[doc = "The list of aggregation types (comma separated) to retrieve."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[doc = "The **$filter** is used to reduce the set of metric data returned.<br>Example:<br>Metric contains metadata A, B and C.<br>- Return all time series of C where A = a1 and B = b1 or b2<br>**$filter=A eq ‘a1’ and B eq ‘b1’ or B eq ‘b2’ and C eq ‘*’**<br>- Invalid variant:<br>**$filter=A eq ‘a1’ and B eq ‘b1’ and C eq ‘*’ or B = ‘b2’**<br>This is invalid because the logical or operator cannot separate two different metadata names.<br>- Return all time series where A = a1, B = b1 and C = c1:<br>**$filter=A eq ‘a1’ and B eq ‘b1’ and C eq ‘c1’**<br>- Return all time series where A = a1<br>**$filter=A eq ‘a1’ and B eq ‘*’ and C eq ‘*’**."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "The maximum number of records to retrieve.\nValid only if $filter is specified.\nDefaults to 10."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[doc = "The aggregation to use for sorting results and the direction of the sort.\nOnly one order can be specified.\nExamples: sum asc."]
    #[serde(rename = "orderBy", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[doc = "Dimension name(s) to rollup results by. For example if you only want to see metric values with a filter like 'City eq Seattle or City eq Tacoma' but don't want to see separate values for each city, you can specify 'RollUpBy=City' to see the results for Seattle and Tacoma rolled up into one timeseries."]
    #[serde(rename = "rollUpBy", default, skip_serializing_if = "Option::is_none")]
    pub roll_up_by: Option<String>,
    #[doc = "Reduces the set of data collected. The syntax allowed depends on the operation. See the operation's description for details."]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<subscription_scope_metrics_request_body_parameters::ResultType>,
    #[doc = "Metric namespace where the metrics you want reside."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[doc = "When set to true, if the timespan passed in is not supported by this metric, the API will return the result using the closest supported timespan. When set to false, an error is returned for invalid timespan parameters. Defaults to false."]
    #[serde(rename = "autoAdjustTimegrain", default, skip_serializing_if = "Option::is_none")]
    pub auto_adjust_timegrain: Option<bool>,
    #[doc = "When set to false, invalid filter parameter values will be ignored. When set to true, an error is returned for invalid filter parameters. Defaults to true."]
    #[serde(rename = "validateDimensions", default, skip_serializing_if = "Option::is_none")]
    pub validate_dimensions: Option<bool>,
}
impl SubscriptionScopeMetricsRequestBodyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_scope_metrics_request_body_parameters {
    use super::*;
    #[doc = "Reduces the set of data collected. The syntax allowed depends on the operation. See the operation's description for details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResultType")]
    pub enum ResultType {
        Data,
        Metadata,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResultType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResultType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResultType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Data => serializer.serialize_unit_variant("ResultType", 0u32, "Data"),
                Self::Metadata => serializer.serialize_unit_variant("ResultType", 1u32, "Metadata"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The baseline values for a single time series."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesBaseline {
    #[doc = "The aggregation type of the metric."]
    pub aggregation: String,
    #[doc = "The dimensions of this time series."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<MetricSingleDimension>,
    #[doc = "The list of timestamps of the baselines."]
    pub timestamps: Vec<time::OffsetDateTime>,
    #[doc = "The baseline values for each sensitivity."]
    pub data: Vec<SingleBaseline>,
    #[doc = "The baseline metadata values."]
    #[serde(
        rename = "metadataValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_values: Vec<BaselineMetadata>,
}
impl TimeSeriesBaseline {
    pub fn new(aggregation: String, timestamps: Vec<time::OffsetDateTime>, data: Vec<SingleBaseline>) -> Self {
        Self {
            aggregation,
            dimensions: Vec::new(),
            timestamps,
            data,
            metadata_values: Vec::new(),
        }
    }
}
#[doc = "A time series result type. The discriminator value is always TimeSeries in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesElement {
    #[doc = "the metadata values returned if $filter was specified in the call."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadatavalues: Vec<MetadataValue>,
    #[doc = "An array of data points representing the metric values.  This is only returned if a result type of data is specified."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<MetricValue>,
}
impl TimeSeriesElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
    ByteSeconds,
    Unspecified,
    Cores,
    MilliCores,
    NanoCores,
    BitsPerSecond,
}
#[doc = "Specifies the metric alert rule criteria for a web test resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestLocationAvailabilityCriteria {
    #[doc = "The Application Insights web test Id."]
    #[serde(rename = "webTestId")]
    pub web_test_id: String,
    #[doc = "The Application Insights resource Id."]
    #[serde(rename = "componentId")]
    pub component_id: String,
    #[doc = "The number of failed locations."]
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: f64,
}
impl WebtestLocationAvailabilityCriteria {
    pub fn new(web_test_id: String, component_id: String, failed_location_count: f64) -> Self {
        Self {
            web_test_id,
            component_id,
            failed_location_count,
        }
    }
}
