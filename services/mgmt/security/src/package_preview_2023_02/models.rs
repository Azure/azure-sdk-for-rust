#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Baseline details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Baseline {
    #[doc = "Expected results."]
    #[serde(
        rename = "expectedResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expected_results: Vec<Vec<String>>,
    #[doc = "Baseline update time (UTC)."]
    #[serde(rename = "updatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time: Option<time::OffsetDateTime>,
}
impl Baseline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule result adjusted with baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaselineAdjustedResult {
    #[doc = "Baseline details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<Baseline>,
    #[doc = "The rule result status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    #[doc = "Results the are not in baseline."]
    #[serde(
        rename = "resultsNotInBaseline",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results_not_in_baseline: Vec<Vec<String>>,
    #[doc = "Results the are in baseline."]
    #[serde(
        rename = "resultsOnlyInBaseline",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results_only_in_baseline: Vec<Vec<String>>,
}
impl BaselineAdjustedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The benchmark references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenchmarkReference {
    #[doc = "The benchmark name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    #[doc = "The benchmark reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
impl BenchmarkReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
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
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
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
#[doc = "The health report resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthReport {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of the health report"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthReportProperties>,
}
impl HealthReport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of the health report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthReportProperties {
    #[doc = "The resource details of the health report"]
    #[serde(rename = "resourceDetails", default, skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
    #[doc = "The environment details of the resource"]
    #[serde(rename = "environmentDetails", default, skip_serializing_if = "Option::is_none")]
    pub environment_details: Option<EnvironmentDetails>,
    #[doc = "The classification of the health report"]
    #[serde(rename = "healthDataClassification", default, skip_serializing_if = "Option::is_none")]
    pub health_data_classification: Option<HealthDataClassification>,
    #[doc = "The status of the health report"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The affected defenders plans by unhealthy report"]
    #[serde(
        rename = "affectedDefendersPlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub affected_defenders_plans: Vec<String>,
    #[doc = "A collection of the issues in the report"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<Issue>,
}
impl HealthReportProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of health reports list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthReportsList {
    #[doc = "Collection of health reports in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<HealthReport>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HealthReportsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HealthReportsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule query details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryCheck {
    #[doc = "The rule query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "Expected result."]
    #[serde(
        rename = "expectedResult",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expected_result: Vec<Vec<String>>,
    #[doc = "Column names of expected result."]
    #[serde(
        rename = "columnNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_names: Vec<String>,
}
impl QueryCheck {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Remediation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Remediation {
    #[doc = "Remediation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Remediation script."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scripts: Vec<String>,
    #[doc = "Is remediation automated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated: Option<bool>,
    #[doc = "Optional link to remediate in Azure Portal."]
    #[serde(rename = "portalLink", default, skip_serializing_if = "Option::is_none")]
    pub portal_link: Option<String>,
}
impl Remediation {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Rule results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleResults {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Rule results properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RuleResultsProperties>,
}
impl RuleResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule results input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleResultsInput {
    #[doc = "Take results from latest scan."]
    #[serde(rename = "latestScan", default, skip_serializing_if = "Option::is_none")]
    pub latest_scan: Option<bool>,
    #[doc = "Expected results to be inserted into the baseline.\r\nLeave this field empty it LatestScan == true."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<Vec<String>>,
}
impl RuleResultsInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule results properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleResultsProperties {
    #[doc = "Expected results in the baseline."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<Vec<String>>,
}
impl RuleResultsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule severity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleSeverity")]
pub enum RuleSeverity {
    High,
    Medium,
    Low,
    Informational,
    Obsolete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleSeverity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleSeverity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleSeverity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("RuleSeverity", 0u32, "High"),
            Self::Medium => serializer.serialize_unit_variant("RuleSeverity", 1u32, "Medium"),
            Self::Low => serializer.serialize_unit_variant("RuleSeverity", 2u32, "Low"),
            Self::Informational => serializer.serialize_unit_variant("RuleSeverity", 3u32, "Informational"),
            Self::Obsolete => serializer.serialize_unit_variant("RuleSeverity", 4u32, "Obsolete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The rule result status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleStatus")]
pub enum RuleStatus {
    NonFinding,
    Finding,
    InternalError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NonFinding => serializer.serialize_unit_variant("RuleStatus", 0u32, "NonFinding"),
            Self::Finding => serializer.serialize_unit_variant("RuleStatus", 1u32, "Finding"),
            Self::InternalError => serializer.serialize_unit_variant("RuleStatus", 2u32, "InternalError"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The rule type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleType")]
pub enum RuleType {
    Binary,
    BaselineExpected,
    PositiveList,
    NegativeList,
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
            Self::Binary => serializer.serialize_unit_variant("RuleType", 0u32, "Binary"),
            Self::BaselineExpected => serializer.serialize_unit_variant("RuleType", 1u32, "BaselineExpected"),
            Self::PositiveList => serializer.serialize_unit_variant("RuleType", 2u32, "PositiveList"),
            Self::NegativeList => serializer.serialize_unit_variant("RuleType", 3u32, "NegativeList"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A list of rules results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesResults {
    #[doc = "List of rule results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RuleResults>,
}
impl RulesResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rules results input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesResultsInput {
    #[doc = "Take results from latest scan."]
    #[serde(rename = "latestScan", default, skip_serializing_if = "Option::is_none")]
    pub latest_scan: Option<bool>,
    #[doc = "Expected results to be inserted into the baseline.\r\nLeave this field empty it LatestScan == true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<serde_json::Value>,
}
impl RulesResultsInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scan {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A vulnerability assessment scan record properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScanProperties>,
}
impl Scan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanProperties {
    #[doc = "The scan trigger type."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<ScanTriggerType>,
    #[doc = "The scan status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ScanState>,
    #[doc = "The server name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The database name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The SQL version."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
    #[doc = "The scan start time (UTC)."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Scan results are valid until end time (UTC)."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The number of failed rules with high severity."]
    #[serde(rename = "highSeverityFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub high_severity_failed_rules_count: Option<i32>,
    #[doc = "The number of failed rules with medium severity."]
    #[serde(rename = "mediumSeverityFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub medium_severity_failed_rules_count: Option<i32>,
    #[doc = "The number of failed rules with low severity."]
    #[serde(rename = "lowSeverityFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub low_severity_failed_rules_count: Option<i32>,
    #[doc = "The number of total passed rules."]
    #[serde(rename = "totalPassedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub total_passed_rules_count: Option<i32>,
    #[doc = "The number of total failed rules."]
    #[serde(rename = "totalFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub total_failed_rules_count: Option<i32>,
    #[doc = "The number of total rules assessed."]
    #[serde(rename = "totalRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub total_rules_count: Option<i32>,
    #[doc = "Baseline created for this database, and has one or more rules."]
    #[serde(rename = "isBaselineApplied", default, skip_serializing_if = "Option::is_none")]
    pub is_baseline_applied: Option<bool>,
    #[doc = "Last scan time."]
    #[serde(rename = "lastScanTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_scan_time: Option<time::OffsetDateTime>,
}
impl ScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan result for a single rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A vulnerability assessment scan result properties for a single rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScanResultProperties>,
}
impl ScanResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan result properties for a single rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResultProperties {
    #[doc = "The rule Id."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "The rule result status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    #[doc = "Indicated whether the results specified here are trimmed."]
    #[serde(rename = "isTrimmed", default, skip_serializing_if = "Option::is_none")]
    pub is_trimmed: Option<bool>,
    #[doc = "The results of the query that was run."]
    #[serde(
        rename = "queryResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub query_results: Vec<Vec<String>>,
    #[doc = "Remediation details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[doc = "The rule result adjusted with baseline."]
    #[serde(rename = "baselineAdjustedResult", default, skip_serializing_if = "Option::is_none")]
    pub baseline_adjusted_result: Option<BaselineAdjustedResult>,
    #[doc = "vulnerability assessment rule metadata details."]
    #[serde(rename = "ruleMetadata", default, skip_serializing_if = "Option::is_none")]
    pub rule_metadata: Option<VaRule>,
}
impl ScanResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of vulnerability assessment scan results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResults {
    #[doc = "List of vulnerability assessment scan results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScanResult>,
}
impl ScanResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The scan status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScanState")]
pub enum ScanState {
    Failed,
    FailedToRun,
    InProgress,
    Passed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScanState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScanState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScanState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Failed => serializer.serialize_unit_variant("ScanState", 0u32, "Failed"),
            Self::FailedToRun => serializer.serialize_unit_variant("ScanState", 1u32, "FailedToRun"),
            Self::InProgress => serializer.serialize_unit_variant("ScanState", 2u32, "InProgress"),
            Self::Passed => serializer.serialize_unit_variant("ScanState", 3u32, "Passed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The scan trigger type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScanTriggerType")]
pub enum ScanTriggerType {
    OnDemand,
    Recurring,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScanTriggerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScanTriggerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScanTriggerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OnDemand => serializer.serialize_unit_variant("ScanTriggerType", 0u32, "OnDemand"),
            Self::Recurring => serializer.serialize_unit_variant("ScanTriggerType", 1u32, "Recurring"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A list of vulnerability assessment scan records."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scans {
    #[doc = "List of vulnerability assessment scan records."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Scan>,
}
impl Scans {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "vulnerability assessment rule metadata details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaRule {
    #[doc = "The rule Id."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "The rule severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<RuleSeverity>,
    #[doc = "The rule category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The rule type."]
    #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<RuleType>,
    #[doc = "The rule title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The rule rationale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[doc = "The rule query details."]
    #[serde(rename = "queryCheck", default, skip_serializing_if = "Option::is_none")]
    pub query_check: Option<QueryCheck>,
    #[doc = "The benchmark references."]
    #[serde(
        rename = "benchmarkReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub benchmark_references: Vec<BenchmarkReference>,
}
impl VaRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The environment details of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDetails {
    #[doc = "The native resource id of the resource (in case of Azure - the resource Id, in case of MC - the native resource id)"]
    #[serde(rename = "nativeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub native_resource_id: Option<String>,
    #[doc = "The hierarchy id of the connector (in case of Azure - the subscription Id, in case of MC - the hierarchyId id)"]
    #[serde(rename = "environmentHierarchyId", default, skip_serializing_if = "Option::is_none")]
    pub environment_hierarchy_id: Option<String>,
    #[doc = "The organizational hierarchy id of the connector (in case of Azure - the subscription Id, in case of MC - the organizational hierarchyId id)"]
    #[serde(rename = "organizationalHierarchyId", default, skip_serializing_if = "Option::is_none")]
    pub organizational_hierarchy_id: Option<String>,
    #[doc = "The subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl EnvironmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The classification of the health report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthDataClassification {
    #[doc = "The component describes the name of the agent/service that scans the issue"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[doc = "The scenario describes the health scenario issue of the component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
    #[doc = "The resource scope of the health report"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<health_data_classification::Scope>,
}
impl HealthDataClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_data_classification {
    use super::*;
    #[doc = "The resource scope of the health report"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        Connectors,
        Clusters,
        VirtualMachines,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connectors => serializer.serialize_unit_variant("Scope", 0u32, "Connectors"),
                Self::Clusters => serializer.serialize_unit_variant("Scope", 1u32, "Clusters"),
                Self::VirtualMachines => serializer.serialize_unit_variant("Scope", 2u32, "VirtualMachines"),
                Self::Unknown => serializer.serialize_unit_variant("Scope", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The issue that caused the resource to by unhealthy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    #[doc = "The unique issue key"]
    #[serde(rename = "issueKey")]
    pub issue_key: String,
    #[doc = "The issue name"]
    #[serde(rename = "issueName", default, skip_serializing_if = "Option::is_none")]
    pub issue_name: Option<String>,
    #[doc = "The affected security values that MDC offers that will be affected by the issue, for example: recommendations, alerts, etc"]
    #[serde(
        rename = "securityValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub security_values: Vec<String>,
    #[doc = "The issue description"]
    #[serde(rename = "issueDescription", default, skip_serializing_if = "Option::is_none")]
    pub issue_description: Option<String>,
    #[doc = "Human readable description of what you should do to mitigate this health issue"]
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[doc = "The remediation script to solve this issue"]
    #[serde(rename = "remediationScript", default, skip_serializing_if = "Option::is_none")]
    pub remediation_script: Option<String>,
    #[doc = "Additional data for the given issue. The additional data depends on the issue type"]
    #[serde(rename = "issueAdditionalData", default, skip_serializing_if = "Option::is_none")]
    pub issue_additional_data: Option<serde_json::Value>,
}
impl Issue {
    pub fn new(issue_key: String) -> Self {
        Self {
            issue_key,
            issue_name: None,
            security_values: Vec::new(),
            issue_description: None,
            remediation_steps: None,
            remediation_script: None,
            issue_additional_data: None,
        }
    }
}
#[doc = "The resource details of the health report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDetails {
    #[doc = "The status of the health report"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<resource_details::Source>,
    #[doc = "The azure id of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The id of the connector"]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
}
impl ResourceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_details {
    use super::*;
    #[doc = "The status of the health report"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        Aws,
        Gcp,
        Azure,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aws => serializer.serialize_unit_variant("Source", 0u32, "Aws"),
                Self::Gcp => serializer.serialize_unit_variant("Source", 1u32, "Gcp"),
                Self::Azure => serializer.serialize_unit_variant("Source", 2u32, "Azure"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of the health report"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Status {
    #[doc = "The status of the health report"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<status::Code>,
    #[doc = "The date of when the status of the health report was changed in the last time"]
    #[serde(rename = "statusChangeDate", default, with = "azure_core::date::rfc3339::option")]
    pub status_change_date: Option<time::OffsetDateTime>,
    #[doc = "The date of when the resource of the health report was scanned in the first time"]
    #[serde(rename = "firstEvaluationDate", default, with = "azure_core::date::rfc3339::option")]
    pub first_evaluation_date: Option<time::OffsetDateTime>,
}
impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod status {
    use super::*;
    #[doc = "The status of the health report"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Healthy,
        NotHealthy,
        NotApplicable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("Code", 0u32, "Healthy"),
                Self::NotHealthy => serializer.serialize_unit_variant("Code", 1u32, "NotHealthy"),
                Self::NotApplicable => serializer.serialize_unit_variant("Code", 2u32, "NotApplicable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
