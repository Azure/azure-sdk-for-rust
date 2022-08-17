#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A list of Analysis Results. It will only contain one element as all the data will be nested in a singleton object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisResultListResult {
    #[doc = "The list of Analysis Results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AnalysisResultSingletonResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AnalysisResultListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AnalysisResultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Analysis Result Singleton Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisResultSingletonResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of Analysis Result resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisResultSingletonResourceProperties>,
}
impl AnalysisResultSingletonResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of Analysis Result resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResultSingletonResourceProperties {
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[doc = "Type of the Analysis Result."]
    #[serde(rename = "analysisResultType")]
    pub analysis_result_type: analysis_result_singleton_resource_properties::AnalysisResultType,
}
impl AnalysisResultSingletonResourceProperties {
    pub fn new(analysis_result_type: analysis_result_singleton_resource_properties::AnalysisResultType) -> Self {
        Self {
            grade: None,
            analysis_result_type,
        }
    }
}
pub mod analysis_result_singleton_resource_properties {
    use super::*;
    #[doc = "Type of the Analysis Result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AnalysisResultType")]
    pub enum AnalysisResultType {
        ScriptExecution,
        Reliability,
        #[serde(rename = "CPUUtilization")]
        CpuUtilization,
        MemoryUtilization,
        #[serde(rename = "CPURegression")]
        CpuRegression,
        MemoryRegression,
        TestAnalysis,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AnalysisResultType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AnalysisResultType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AnalysisResultType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ScriptExecution => serializer.serialize_unit_variant("AnalysisResultType", 0u32, "ScriptExecution"),
                Self::Reliability => serializer.serialize_unit_variant("AnalysisResultType", 1u32, "Reliability"),
                Self::CpuUtilization => serializer.serialize_unit_variant("AnalysisResultType", 2u32, "CPUUtilization"),
                Self::MemoryUtilization => serializer.serialize_unit_variant("AnalysisResultType", 3u32, "MemoryUtilization"),
                Self::CpuRegression => serializer.serialize_unit_variant("AnalysisResultType", 4u32, "CPURegression"),
                Self::MemoryRegression => serializer.serialize_unit_variant("AnalysisResultType", 5u32, "MemoryRegression"),
                Self::TestAnalysis => serializer.serialize_unit_variant("AnalysisResultType", 6u32, "TestAnalysis"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of available OSs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOsListResult {
    #[doc = "The list of available OSs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableOsResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableOsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableOsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Available OS properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOsProperties {
    #[doc = "The Id of an Available OS of a Test Base Account."]
    #[serde(rename = "osId", default, skip_serializing_if = "Option::is_none")]
    pub os_id: Option<String>,
    #[doc = "The name of an Available OS of a Test Base Account."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of an Available OS of a Test Base Account."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "The insider channel of an Available OS of a Test Base Account."]
    #[serde(rename = "insiderChannel", default, skip_serializing_if = "Option::is_none")]
    pub insider_channel: Option<String>,
    #[doc = "The OS update type of an Available OS of a Test Base Account."]
    #[serde(rename = "osUpdateType", default, skip_serializing_if = "Option::is_none")]
    pub os_update_type: Option<String>,
    #[doc = "The Platform of an Available OS of a Test Base Account."]
    #[serde(rename = "osPlatform", default, skip_serializing_if = "Option::is_none")]
    pub os_platform: Option<String>,
}
impl AvailableOsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Available OS resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOsResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The Available OS properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableOsProperties>,
}
impl AvailableOsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubExecutionUsageDetail {
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "executionRequestId", default, skip_serializing_if = "Option::is_none")]
    pub execution_request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "startTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_stamp: Option<time::OffsetDateTime>,
    #[serde(rename = "endTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_stamp: Option<time::OffsetDateTime>,
    #[serde(rename = "osBuild", default, skip_serializing_if = "Option::is_none")]
    pub os_build: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(rename = "testType", default, skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
    #[serde(rename = "updateType", default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
    #[serde(rename = "usedFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub used_free_hours: Option<f64>,
    #[serde(rename = "usedBillableHours", default, skip_serializing_if = "Option::is_none")]
    pub used_billable_hours: Option<f64>,
    #[serde(rename = "billedCharges", default, skip_serializing_if = "Option::is_none")]
    pub billed_charges: Option<f64>,
}
impl BillingHubExecutionUsageDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubFreeHourIncrementEntry {
    #[serde(rename = "createTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub create_time_stamp: Option<time::OffsetDateTime>,
    #[serde(rename = "expirationTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time_stamp: Option<time::OffsetDateTime>,
    #[serde(rename = "incrementalFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub incremental_free_hours: Option<f64>,
    #[serde(rename = "remainingFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub remaining_free_hours: Option<f64>,
}
impl BillingHubFreeHourIncrementEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubGetFreeHourBalanceResponse {
    #[serde(rename = "totalRemainingFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub total_remaining_free_hours: Option<f64>,
    #[serde(rename = "incrementEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub increment_entries: Vec<BillingHubFreeHourIncrementEntry>,
}
impl BillingHubGetFreeHourBalanceResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingHubGetUsageRequest {
    #[serde(rename = "startTimeStamp", with = "azure_core::date::rfc3339")]
    pub start_time_stamp: time::OffsetDateTime,
    #[serde(rename = "endTimeStamp", with = "azure_core::date::rfc3339")]
    pub end_time_stamp: time::OffsetDateTime,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "pageIndex", default, skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i32>,
}
impl BillingHubGetUsageRequest {
    pub fn new(start_time_stamp: time::OffsetDateTime, end_time_stamp: time::OffsetDateTime) -> Self {
        Self {
            start_time_stamp,
            end_time_stamp,
            page_size: None,
            page_index: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubGetUsageResponse {
    #[serde(rename = "totalUsedFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_free_hours: Option<f64>,
    #[serde(rename = "totalUsedBillableHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_billable_hours: Option<f64>,
    #[serde(rename = "totalCharges", default, skip_serializing_if = "Option::is_none")]
    pub total_charges: Option<f64>,
    #[serde(rename = "packageUsageEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub package_usage_entries: Vec<BillingHubPackageUsage>,
    #[serde(rename = "nextRequest", default, skip_serializing_if = "Option::is_none")]
    pub next_request: Option<BillingHubGetUsageRequest>,
}
impl BillingHubGetUsageResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubPackageUsage {
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "azureResourceUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_uri: Option<String>,
    #[serde(rename = "totalUsedFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_free_hours: Option<f64>,
    #[serde(rename = "totalUsedBillableHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_billable_hours: Option<f64>,
    #[serde(rename = "totalCharges", default, skip_serializing_if = "Option::is_none")]
    pub total_charges: Option<f64>,
    #[serde(rename = "usageEntriesGroupedByUpdateType", default, skip_serializing_if = "Vec::is_empty")]
    pub usage_entries_grouped_by_update_type: Vec<BillingHubUsageGroupedByUpdateType>,
}
impl BillingHubPackageUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubUsage {
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "azureResourceUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_uri: Option<String>,
    #[serde(rename = "totalUsedFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_free_hours: Option<f64>,
    #[serde(rename = "totalUsedBillableHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_billable_hours: Option<f64>,
    #[serde(rename = "totalCharges", default, skip_serializing_if = "Option::is_none")]
    pub total_charges: Option<f64>,
    #[serde(rename = "usageGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub usage_groups: Vec<BillingHubUsageGroup>,
}
impl BillingHubUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubUsageGroup {
    #[serde(rename = "totalUsedFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_free_hours: Option<f64>,
    #[serde(rename = "totalUsedBillableHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_billable_hours: Option<f64>,
    #[serde(rename = "totalCharges", default, skip_serializing_if = "Option::is_none")]
    pub total_charges: Option<f64>,
    #[serde(rename = "releaseBuildRevision", default, skip_serializing_if = "Option::is_none")]
    pub release_build_revision: Option<i64>,
    #[serde(rename = "releaseBuildNumber", default, skip_serializing_if = "Option::is_none")]
    pub release_build_number: Option<i64>,
    #[serde(rename = "releaseBuildDate", default, with = "azure_core::date::rfc3339::option")]
    pub release_build_date: Option<time::OffsetDateTime>,
    #[serde(rename = "osBuild", default, skip_serializing_if = "Option::is_none")]
    pub os_build: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(rename = "testType", default, skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
    #[serde(rename = "productFamily", default, skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    #[serde(rename = "executionUsageDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub execution_usage_details: Vec<BillingHubExecutionUsageDetail>,
}
impl BillingHubUsageGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingHubUsageGroupedByUpdateType {
    #[serde(rename = "updateType", default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
    #[serde(rename = "totalUsedFreeHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_free_hours: Option<f64>,
    #[serde(rename = "totalUsedBillableHours", default, skip_serializing_if = "Option::is_none")]
    pub total_used_billable_hours: Option<f64>,
    #[serde(rename = "totalCharges", default, skip_serializing_if = "Option::is_none")]
    pub total_charges: Option<f64>,
    #[serde(rename = "usageGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub usage_groups: Vec<BillingHubUsageGroup>,
}
impl BillingHubUsageGroupedByUpdateType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of CPU Regression Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuRegressionResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The result array data."]
    #[serde(rename = "cpuRegressionResults", default, skip_serializing_if = "Vec::is_empty")]
    pub cpu_regression_results: Vec<RegressionResult>,
}
impl CpuRegressionResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            cpu_regression_results: Vec::new(),
        }
    }
}
#[doc = "The properties of CPU Utilization Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuUtilizationResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The result array data."]
    #[serde(rename = "cpuUtilizationResults", default, skip_serializing_if = "Vec::is_empty")]
    pub cpu_utilization_results: Vec<UtilizationResult>,
}
impl CpuUtilizationResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            cpu_utilization_results: Vec::new(),
        }
    }
}
#[doc = "Description of a Check Name availability response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Value indicating the availability of the name: true if the name is available; otherwise, false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason for unavailability of a name. Required if nameAvailable == false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "The detailed info regarding the reason associated with the name. Required if nameAvailable == false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "The reason for unavailability of a name. Required if nameAvailable == false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The command used in the test"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Command {
    #[doc = "The name of the command."]
    pub name: String,
    #[doc = "The action of the command."]
    pub action: command::Action,
    #[doc = "The type of command content."]
    #[serde(rename = "contentType")]
    pub content_type: command::ContentType,
    #[doc = "The content of the command. The content depends on source type."]
    pub content: String,
    #[doc = "Specifies whether to run the command as administrator."]
    #[serde(rename = "runElevated", default, skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[doc = "Specifies whether to restart the VM after the command executed."]
    #[serde(rename = "restartAfter", default, skip_serializing_if = "Option::is_none")]
    pub restart_after: Option<bool>,
    #[doc = "Specifies the max run time of the command."]
    #[serde(rename = "maxRunTime", default, skip_serializing_if = "Option::is_none")]
    pub max_run_time: Option<i32>,
    #[doc = "Specifies whether to run the command in interactive mode."]
    #[serde(rename = "runAsInteractive", default, skip_serializing_if = "Option::is_none")]
    pub run_as_interactive: Option<bool>,
    #[doc = "Specifies whether to run the command even if a previous command is failed."]
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[doc = "Specifies whether to apply update before the command."]
    #[serde(rename = "applyUpdateBefore", default, skip_serializing_if = "Option::is_none")]
    pub apply_update_before: Option<bool>,
}
impl Command {
    pub fn new(name: String, action: command::Action, content_type: command::ContentType, content: String) -> Self {
        Self {
            name,
            action,
            content_type,
            content,
            run_elevated: None,
            restart_after: None,
            max_run_time: None,
            run_as_interactive: None,
            always_run: None,
            apply_update_before: None,
        }
    }
}
pub mod command {
    use super::*;
    #[doc = "The action of the command."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Install,
        Launch,
        Close,
        Uninstall,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Install => serializer.serialize_unit_variant("Action", 0u32, "Install"),
                Self::Launch => serializer.serialize_unit_variant("Action", 1u32, "Launch"),
                Self::Close => serializer.serialize_unit_variant("Action", 2u32, "Close"),
                Self::Uninstall => serializer.serialize_unit_variant("Action", 3u32, "Uninstall"),
                Self::Custom => serializer.serialize_unit_variant("Action", 4u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of command content."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContentType")]
    pub enum ContentType {
        Inline,
        File,
        Path,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ContentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inline => serializer.serialize_unit_variant("ContentType", 0u32, "Inline"),
                Self::File => serializer.serialize_unit_variant("ContentType", 1u32, "File"),
                Self::Path => serializer.serialize_unit_variant("ContentType", 2u32, "Path"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of Test Base Customer Events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerEventListResult {
    #[doc = "The list of Test Base Customer Events."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomerEventResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomerEventListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomerEventListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A notification events subscribed to be received by customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerEventProperties {
    #[doc = "The name of the event subscribed to."]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[doc = "The notification event receivers."]
    pub receivers: Vec<NotificationEventReceiver>,
}
impl CustomerEventProperties {
    pub fn new(event_name: String, receivers: Vec<NotificationEventReceiver>) -> Self {
        Self { event_name, receivers }
    }
}
#[doc = "The Customer Notification Event resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerEventResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "A notification events subscribed to be received by customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomerEventProperties>,
}
impl CustomerEventResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The user object receiver value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributionGroupListReceiverValue {
    #[doc = "The list of distribution groups."]
    #[serde(rename = "distributionGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub distribution_groups: Vec<String>,
}
impl DistributionGroupListReceiverValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of getting a download URL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrlResponse {
    #[doc = "The download URL."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[doc = "Expiry date of the download URL."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
}
impl DownloadUrlResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of email events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailEventListResult {
    #[doc = "The list of email events."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EmailEventResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EmailEventListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EmailEventListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Email Event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailEventProperties {
    #[doc = "The identifier of the email event."]
    #[serde(rename = "eventId", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[doc = "The name of the email event."]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[doc = "The display name of the email event."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl EmailEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The email event resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailEventResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The Email Event properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmailEventProperties>,
}
impl EmailEventResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error response send when an operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "A list of favorite processes for a package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteProcessListResult {
    #[doc = "The list of favorite processes for a package."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FavoriteProcessResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FavoriteProcessListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FavoriteProcessListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a favorite process identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteProcessProperties {
    #[doc = "The actual name of the favorite process. It will be equal to resource name except for the scenario that the process name contains characters that are not allowed in the resource name."]
    #[serde(rename = "actualProcessName")]
    pub actual_process_name: String,
}
impl FavoriteProcessProperties {
    pub fn new(actual_process_name: String) -> Self {
        Self { actual_process_name }
    }
}
#[doc = "A favorite process identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FavoriteProcessResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a favorite process identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FavoriteProcessProperties>,
}
impl FavoriteProcessResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The URL response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileUploadUrlResponse {
    #[doc = "The URL used for uploading the package."]
    #[serde(rename = "uploadUrl", default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[doc = "The blob path of the uploaded package. It will be used as the 'blobPath' property of PackageResource."]
    #[serde(rename = "blobPath", default, skip_serializing_if = "Option::is_none")]
    pub blob_path: Option<String>,
}
impl FileUploadUrlResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of flighting rings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlightingRingListResult {
    #[doc = "The list of flighting rings."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FlightingRingResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FlightingRingListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FlightingRingListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Flighting Ring properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlightingRingProperties {
    #[doc = "The actual name of a flighting ring of a Test Base Account."]
    #[serde(rename = "actualFlightingRingName", default, skip_serializing_if = "Option::is_none")]
    pub actual_flighting_ring_name: Option<String>,
}
impl FlightingRingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The flighting ring resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlightingRingResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The Flighting Ring properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FlightingRingProperties>,
}
impl FlightingRingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the Test Base Account GetFileUploadURL action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetFileUploadUrlParameters {
    #[doc = "The custom file name of the uploaded blob."]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
}
impl GetFileUploadUrlParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The failure identified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentifiedFailure {
    #[doc = "The Failure Id."]
    #[serde(rename = "failureId", default, skip_serializing_if = "Option::is_none")]
    pub failure_id: Option<String>,
    #[doc = "The category of the failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<identified_failure::Category>,
    #[doc = "Message that shows information about the failure."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Guidance that shows what the customer needs to do for this failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
}
impl IdentifiedFailure {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identified_failure {
    use super::*;
    #[doc = "The category of the failure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        None,
        Unidentified,
        Package,
        #[serde(rename = "OSUpdate")]
        OsUpdate,
        Infrastructure,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Category", 0u32, "None"),
                Self::Unidentified => serializer.serialize_unit_variant("Category", 1u32, "Unidentified"),
                Self::Package => serializer.serialize_unit_variant("Category", 2u32, "Package"),
                Self::OsUpdate => serializer.serialize_unit_variant("Category", 3u32, "OSUpdate"),
                Self::Infrastructure => serializer.serialize_unit_variant("Category", 4u32, "Infrastructure"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of Memory Regression Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryRegressionResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The result array data."]
    #[serde(rename = "memoryRegressionResults", default, skip_serializing_if = "Vec::is_empty")]
    pub memory_regression_results: Vec<RegressionResult>,
}
impl MemoryRegressionResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            memory_regression_results: Vec::new(),
        }
    }
}
#[doc = "The properties of Memory Utilization Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryUtilizationResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The result array data."]
    #[serde(rename = "memoryUtilizationResults", default, skip_serializing_if = "Vec::is_empty")]
    pub memory_utilization_results: Vec<UtilizationResult>,
}
impl MemoryUtilizationResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            memory_utilization_results: Vec::new(),
        }
    }
}
#[doc = "A notification event receivers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationEventReceiver {
    #[doc = "The type of the notification event receiver."]
    #[serde(rename = "receiverType", default, skip_serializing_if = "Option::is_none")]
    pub receiver_type: Option<String>,
    #[doc = "A notification event receiver value."]
    #[serde(rename = "receiverValue", default, skip_serializing_if = "Option::is_none")]
    pub receiver_value: Option<NotificationReceiverValue>,
}
impl NotificationEventReceiver {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A notification event receiver value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationReceiverValue {
    #[doc = "The user object receiver value."]
    #[serde(rename = "userObjectReceiverValue", default, skip_serializing_if = "Option::is_none")]
    pub user_object_receiver_value: Option<UserObjectReceiverValue>,
    #[doc = "The subscription role receiver value."]
    #[serde(rename = "subscriptionReceiverValue", default, skip_serializing_if = "Option::is_none")]
    pub subscription_receiver_value: Option<SubscriptionReceiverValue>,
    #[doc = "The user object receiver value."]
    #[serde(rename = "distributionGroupListReceiverValue", default, skip_serializing_if = "Option::is_none")]
    pub distribution_group_list_receiver_value: Option<DistributionGroupListReceiverValue>,
}
impl NotificationReceiverValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of OS Updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsUpdateListResult {
    #[doc = "The list of OS Updates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OsUpdateResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OsUpdateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OsUpdateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an OS Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsUpdateProperties {
    #[doc = "The name of the OS."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The name of tested release."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[doc = "The flighting ring, only for release of feature updates."]
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[doc = "The build version of the tested release (OS update)."]
    #[serde(rename = "buildVersion", default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
    #[doc = "The build revision of the tested release (OS update)"]
    #[serde(rename = "buildRevision", default, skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    #[doc = "The type of this release (OS update)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<os_update_properties::Type>,
    #[doc = "The release version date the tested release (OS update)"]
    #[serde(rename = "releaseVersionDate", default, with = "azure_core::date::rfc3339::option")]
    pub release_version_date: Option<time::OffsetDateTime>,
}
impl OsUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_update_properties {
    use super::*;
    #[doc = "The type of this release (OS update)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SecurityUpdate,
        FeatureUpdate,
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
                Self::SecurityUpdate => serializer.serialize_unit_variant("Type", 0u32, "SecurityUpdate"),
                Self::FeatureUpdate => serializer.serialize_unit_variant("Type", 1u32, "FeatureUpdate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An OS Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsUpdateResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of an OS Update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OsUpdateProperties>,
}
impl OsUpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The summary of a test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsUpdateTestSummary {
    #[doc = "The operating system name. e.g. Windows 10 1809."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The name of the tested release (OS update)."]
    #[serde(rename = "releaseName", default, skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[doc = "The build version of the tested release (OS update)"]
    #[serde(rename = "buildVersion", default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
    #[doc = "The build revision of the tested release (OS update)"]
    #[serde(rename = "buildRevision", default, skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    #[doc = "The release version date."]
    #[serde(rename = "releaseVersionDate", default, with = "azure_core::date::rfc3339::option")]
    pub release_version_date: Option<time::OffsetDateTime>,
    #[doc = "The flighting ring, only for release of feature updates."]
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[doc = "The execution status of a test."]
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[doc = "The status of a test."]
    #[serde(rename = "testStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_status: Option<TestStatus>,
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[doc = "The run time of the test."]
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[doc = "The test type of the package"]
    #[serde(rename = "testType", default, skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
}
impl OsUpdateTestSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The summary of some tests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsUpdatesTestSummary {
    #[doc = "The execution status of a test."]
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[doc = "The status of a test."]
    #[serde(rename = "testStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_status: Option<TestStatus>,
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[doc = "The run time of the last test."]
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[doc = "Detailed summary for each OS update"]
    #[serde(rename = "osUpdateTestSummaries", default, skip_serializing_if = "Vec::is_empty")]
    pub os_update_test_summaries: Vec<OsUpdateTestSummary>,
}
impl OsUpdatesTestSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The object that describes the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Operation properties."]
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
    #[doc = "The object that describes the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider name: Microsoft.TestBase."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Operation type: read, write, delete, listPackages, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Resource type on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Friendly name of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of operations supported by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Test Base resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters body to pass for Test Base Package name availability check."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageCheckNameAvailabilityParameters {
    #[doc = "Resource name to verify."]
    pub name: String,
    #[doc = "Application name to verify."]
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[doc = "Version name to verify."]
    pub version: String,
    #[doc = "fully qualified resource type which includes provider namespace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PackageCheckNameAvailabilityParameters {
    pub fn new(name: String, application_name: String, version: String) -> Self {
        Self {
            name,
            application_name,
            version,
            type_: None,
        }
    }
}
#[doc = "A list of Test Base Packages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageListResult {
    #[doc = "The list of Test Base Packages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PackageResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PackageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PackageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Test Base Package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageProperties {
    #[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Application name"]
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[doc = "Application version"]
    pub version: String,
    #[doc = "OOB, functional or both. Mapped to the data in 'tests' property."]
    #[serde(rename = "testTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub test_types: Vec<TestType>,
    #[doc = "Specifies the target OSs of specific OS Update types."]
    #[serde(rename = "targetOSList")]
    pub target_os_list: Vec<TargetOsInfo>,
    #[doc = "The status of the package."]
    #[serde(rename = "packageStatus", default, skip_serializing_if = "Option::is_none")]
    pub package_status: Option<package_properties::PackageStatus>,
    #[doc = "The UTC timestamp when the package was last modified."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The flighting ring for feature update."]
    #[serde(rename = "flightingRing")]
    pub flighting_ring: String,
    #[doc = "Flag showing that whether the package is enabled. It doesn't schedule test for package which is not enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "The file path of the package."]
    #[serde(rename = "blobPath")]
    pub blob_path: String,
    #[doc = "The validation results. There's validation on package when it's created or updated."]
    #[serde(rename = "validationResults", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_results: Vec<PackageValidationResult>,
    #[doc = "The detailed test information."]
    pub tests: Vec<Test>,
}
impl PackageProperties {
    pub fn new(
        application_name: String,
        version: String,
        target_os_list: Vec<TargetOsInfo>,
        flighting_ring: String,
        blob_path: String,
        tests: Vec<Test>,
    ) -> Self {
        Self {
            provisioning_state: None,
            application_name,
            version,
            test_types: Vec::new(),
            target_os_list,
            package_status: None,
            last_modified_time: None,
            flighting_ring,
            is_enabled: None,
            blob_path,
            validation_results: Vec::new(),
            tests,
        }
    }
}
pub mod package_properties {
    use super::*;
    #[doc = "The status of the package."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PackageStatus")]
    pub enum PackageStatus {
        Unknown,
        Registered,
        Ready,
        Error,
        ValidatingPackage,
        PreValidationCheckPass,
        Deleted,
        ValidationLongerThanUsual,
        VerifyingPackage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PackageStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PackageStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PackageStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PackageStatus", 0u32, "Unknown"),
                Self::Registered => serializer.serialize_unit_variant("PackageStatus", 1u32, "Registered"),
                Self::Ready => serializer.serialize_unit_variant("PackageStatus", 2u32, "Ready"),
                Self::Error => serializer.serialize_unit_variant("PackageStatus", 3u32, "Error"),
                Self::ValidatingPackage => serializer.serialize_unit_variant("PackageStatus", 4u32, "ValidatingPackage"),
                Self::PreValidationCheckPass => serializer.serialize_unit_variant("PackageStatus", 5u32, "PreValidationCheckPass"),
                Self::Deleted => serializer.serialize_unit_variant("PackageStatus", 6u32, "Deleted"),
                Self::ValidationLongerThanUsual => serializer.serialize_unit_variant("PackageStatus", 7u32, "ValidationLongerThanUsual"),
                Self::VerifyingPackage => serializer.serialize_unit_variant("PackageStatus", 8u32, "VerifyingPackage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Test Base Package resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of the Test Base Package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PackageProperties>,
}
impl PackageResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "The parameters supplied to the Test Base Package to start a Test Run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageRunTestParameters {
    #[doc = "The test type."]
    #[serde(rename = "testType")]
    pub test_type: TestType,
    #[doc = "Specifies the OS update type to test against."]
    #[serde(rename = "osUpdateType", default, skip_serializing_if = "Option::is_none")]
    pub os_update_type: Option<package_run_test_parameters::OsUpdateType>,
    #[doc = "The operating system name. e.g. Windows 10 1809."]
    #[serde(rename = "osName")]
    pub os_name: String,
    #[doc = "The name of the tested release (OS update)."]
    #[serde(rename = "releaseName", default, skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[doc = "The flighting ring, only for release of feature updates."]
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
}
impl PackageRunTestParameters {
    pub fn new(test_type: TestType, os_name: String) -> Self {
        Self {
            test_type,
            os_update_type: None,
            os_name,
            release_name: None,
            flighting_ring: None,
        }
    }
}
pub mod package_run_test_parameters {
    use super::*;
    #[doc = "Specifies the OS update type to test against."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsUpdateType")]
    pub enum OsUpdateType {
        SecurityUpdate,
        FeatureUpdate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsUpdateType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsUpdateType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsUpdateType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SecurityUpdate => serializer.serialize_unit_variant("OsUpdateType", 0u32, "SecurityUpdate"),
                Self::FeatureUpdate => serializer.serialize_unit_variant("OsUpdateType", 1u32, "FeatureUpdate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for OsUpdateType {
        fn default() -> Self {
            Self::SecurityUpdate
        }
    }
}
#[doc = "Parameters supplied to update a Test Base Package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageUpdateParameterProperties {
    #[doc = "Specifies the target OSs of specific OS Update types."]
    #[serde(rename = "targetOSList", default, skip_serializing_if = "Vec::is_empty")]
    pub target_os_list: Vec<TargetOsInfo>,
    #[doc = "The flighting ring for feature update."]
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[doc = "Specifies whether the package is enabled. It doesn't schedule test for package which is not enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "The file name of the package."]
    #[serde(rename = "blobPath", default, skip_serializing_if = "Option::is_none")]
    pub blob_path: Option<String>,
    #[doc = "The detailed test information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tests: Vec<Test>,
}
impl PackageUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to update a Test Base Package."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageUpdateParameters {
    #[doc = "Parameters supplied to update a Test Base Package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PackageUpdateParameterProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl PackageUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The validation results. There's validation on package when it's created or updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageValidationResult {
    #[doc = "Validation name."]
    #[serde(rename = "validationName", default, skip_serializing_if = "Option::is_none")]
    pub validation_name: Option<String>,
    #[doc = "Indicates whether the package passed the validation."]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Error information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}
impl PackageValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Cancelled,
    Creating,
    Deleting,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
            Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Cancelled"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for an ARM proxy resource. It will have everything other than required location and tags"]
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
#[doc = "The regression result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegressionResult {
    #[doc = "FileName of the binary being analyzed."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[doc = "Message that facilitates debugging a particular regression, if any was inferred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "Difference between the two datasets being analyzed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<f64>,
    #[doc = "Indicates if a regression was inferred."]
    #[serde(rename = "isRegressed", default, skip_serializing_if = "Option::is_none")]
    pub is_regressed: Option<bool>,
}
impl RegressionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a regression test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegressionTestDetails {
    #[doc = "Difference between the two datasets being analyzed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<f64>,
    #[doc = "Indicates if a regression was inferred."]
    #[serde(rename = "isRegressed", default, skip_serializing_if = "Option::is_none")]
    pub is_regressed: Option<bool>,
}
impl RegressionTestDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Reliability Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReliabilityResult {
    #[doc = "File name."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Count of number of launches."]
    #[serde(rename = "launchCount", default, skip_serializing_if = "Option::is_none")]
    pub launch_count: Option<i32>,
    #[doc = "Count of number of crashes."]
    #[serde(rename = "crashCount", default, skip_serializing_if = "Option::is_none")]
    pub crash_count: Option<i32>,
    #[doc = "Count of number of hangs."]
    #[serde(rename = "hangCount", default, skip_serializing_if = "Option::is_none")]
    pub hang_count: Option<i32>,
    #[doc = "The grade of a test."]
    #[serde(rename = "regressionGrade", default, skip_serializing_if = "Option::is_none")]
    pub regression_grade: Option<TestGrade>,
    #[doc = "The grade of a test."]
    #[serde(rename = "crashRegressionGrade", default, skip_serializing_if = "Option::is_none")]
    pub crash_regression_grade: Option<TestGrade>,
    #[doc = "The details of a regression test."]
    #[serde(rename = "crashRegressionTestDetails", default, skip_serializing_if = "Option::is_none")]
    pub crash_regression_test_details: Option<RegressionTestDetails>,
    #[doc = "The grade of a test."]
    #[serde(rename = "hangRegressionGrade", default, skip_serializing_if = "Option::is_none")]
    pub hang_regression_grade: Option<TestGrade>,
    #[doc = "The details of a regression test."]
    #[serde(rename = "hangRegressionTestDetails", default, skip_serializing_if = "Option::is_none")]
    pub hang_regression_test_details: Option<RegressionTestDetails>,
}
impl ReliabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of Reliability Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReliabilityResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The result array data."]
    #[serde(rename = "reliabilityResults", default, skip_serializing_if = "Vec::is_empty")]
    pub reliability_results: Vec<ReliabilityResult>,
}
impl ReliabilityResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            reliability_results: Vec::new(),
        }
    }
}
#[doc = "The Resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Script Execution Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptExecutionResult {
    #[doc = "Name of script."]
    #[serde(rename = "scriptName", default, skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    #[doc = "Start time of script execution."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of script execution."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Exit code."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[doc = "Whether the script execution is timed out."]
    #[serde(rename = "timedOut", default, skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    #[doc = "The stdout log file name."]
    #[serde(rename = "stdoutLogFileName", default, skip_serializing_if = "Option::is_none")]
    pub stdout_log_file_name: Option<String>,
    #[doc = "The stderr log file name."]
    #[serde(rename = "stderrLogFileName", default, skip_serializing_if = "Option::is_none")]
    pub stderr_log_file_name: Option<String>,
}
impl ScriptExecutionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of Script Execution Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptExecutionResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The result array data."]
    #[serde(rename = "scriptExecutionResults", default, skip_serializing_if = "Vec::is_empty")]
    pub script_execution_results: Vec<ScriptExecutionResult>,
}
impl ScriptExecutionResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            script_execution_results: Vec::new(),
        }
    }
}
#[doc = "The subscription role receiver value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionReceiverValue {
    #[doc = "The subscription id of the notification receiver."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The subscription name of the notification receiver."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "The role of the notification receiver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl SubscriptionReceiverValue {
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
    #[doc = "The type of identity that last modified the resource."]
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
#[doc = "Resource tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information of the target OS to be tested."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetOsInfo {
    #[doc = "Specifies the OS update type to test against, e.g., 'Security updates' or 'Feature updates'."]
    #[serde(rename = "osUpdateType")]
    pub os_update_type: String,
    #[doc = "Specifies the target OSs to be tested."]
    #[serde(rename = "targetOSs")]
    pub target_o_ss: Vec<String>,
    #[doc = "Specifies the baseline OSs to be tested."]
    #[serde(rename = "baselineOSs", default, skip_serializing_if = "Vec::is_empty")]
    pub baseline_o_ss: Vec<String>,
}
impl TargetOsInfo {
    pub fn new(os_update_type: String, target_o_ss: Vec<String>) -> Self {
        Self {
            os_update_type,
            target_o_ss,
            baseline_o_ss: Vec::new(),
        }
    }
}
#[doc = "The definition of a Test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Test {
    #[doc = "The test type."]
    #[serde(rename = "testType")]
    pub test_type: TestType,
    #[doc = "The status of the validation run of the package."]
    #[serde(rename = "validationRunStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_run_status: Option<test::ValidationRunStatus>,
    #[doc = "Resource identifier of the validation test result."]
    #[serde(rename = "validationResultId", default, skip_serializing_if = "Option::is_none")]
    pub validation_result_id: Option<String>,
    #[doc = "Indicates if this test is active.It doesn't schedule test for not active Test."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The commands used in the test."]
    pub commands: Vec<Command>,
}
impl Test {
    pub fn new(test_type: TestType, commands: Vec<Command>) -> Self {
        Self {
            test_type,
            validation_run_status: None,
            validation_result_id: None,
            is_active: None,
            commands,
        }
    }
}
pub mod test {
    use super::*;
    #[doc = "The status of the validation run of the package."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationRunStatus")]
    pub enum ValidationRunStatus {
        Unknown,
        Pending,
        Passed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationRunStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationRunStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationRunStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ValidationRunStatus", 0u32, "Unknown"),
                Self::Pending => serializer.serialize_unit_variant("ValidationRunStatus", 1u32, "Pending"),
                Self::Passed => serializer.serialize_unit_variant("ValidationRunStatus", 2u32, "Passed"),
                Self::Failed => serializer.serialize_unit_variant("ValidationRunStatus", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The test analysis result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAnalysisResult {
    #[doc = "The data to provide more failure analysis information."]
    #[serde(rename = "blobData", default, skip_serializing_if = "Option::is_none")]
    pub blob_data: Option<String>,
    #[doc = "The status of the analysis."]
    #[serde(rename = "testAnalysisStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_analysis_status: Option<test_analysis_result::TestAnalysisStatus>,
    #[doc = "The array of identified failures."]
    #[serde(rename = "identifiedFailures", default, skip_serializing_if = "Vec::is_empty")]
    pub identified_failures: Vec<IdentifiedFailure>,
}
impl TestAnalysisResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_analysis_result {
    use super::*;
    #[doc = "The status of the analysis."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestAnalysisStatus")]
    pub enum TestAnalysisStatus {
        None,
        Analyzing,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestAnalysisStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestAnalysisStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestAnalysisStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("TestAnalysisStatus", 0u32, "None"),
                Self::Analyzing => serializer.serialize_unit_variant("TestAnalysisStatus", 1u32, "Analyzing"),
                Self::Completed => serializer.serialize_unit_variant("TestAnalysisStatus", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("TestAnalysisStatus", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of Test Analysis Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestAnalysisResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[doc = "The test analysis result."]
    #[serde(rename = "testAnalysisResult", default, skip_serializing_if = "Option::is_none")]
    pub test_analysis_result: Option<TestAnalysisResult>,
}
impl TestAnalysisResultSingletonResourceProperties {
    pub fn new(analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties) -> Self {
        Self {
            analysis_result_singleton_resource_properties,
            test_analysis_result: None,
        }
    }
}
#[doc = "A list of Test Base Accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountListResult {
    #[doc = "The list of Test Base Accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestBaseAccountResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestBaseAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TestBaseAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Test Base Account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of a Test Base Account resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestBaseAccountResourceProperties>,
}
impl TestBaseAccountResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "The properties of a Test Base Account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountResourceProperties {
    #[doc = "ARM provisioning state, see https://github.com/Azure/azure-resource-manager-rpc/blob/master/v1.0/Addendum.md#provisioningstate-property"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Describes a Test Base Account SKU."]
    pub sku: TestBaseAccountSku,
    #[doc = "The access level of the Test Base Account."]
    #[serde(rename = "accessLevel", default, skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
}
impl TestBaseAccountResourceProperties {
    pub fn new(sku: TestBaseAccountSku) -> Self {
        Self {
            provisioning_state: None,
            sku,
            access_level: None,
        }
    }
}
#[doc = "Describes a Test Base Account SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountSku {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of the SKU. This is typically a letter + number code, such as B0 or S0."]
    pub name: String,
    #[doc = "The tier of this particular SKU."]
    pub tier: test_base_account_sku::Tier,
    #[doc = "The capabilities of a SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<TestBaseAccountSkuCapability>,
    #[doc = "The locations that the SKU is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
}
impl TestBaseAccountSku {
    pub fn new(name: String, tier: test_base_account_sku::Tier) -> Self {
        Self {
            resource_type: None,
            name,
            tier,
            capabilities: Vec::new(),
            locations: Vec::new(),
        }
    }
}
pub mod test_base_account_sku {
    use super::*;
    #[doc = "The tier of this particular SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
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
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the Test Base Account SKU Capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountSkuCapability {
    #[doc = "An invariant to describe the feature, such as 'SLA'."]
    pub name: String,
    #[doc = "An invariant if the feature is measured by quantity, such as 99.9%."]
    pub value: String,
}
impl TestBaseAccountSkuCapability {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "A list of Test Base Account SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountSkuListResult {
    #[doc = "The list of Test Base Account SKUs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestBaseAccountSku>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestBaseAccountSkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TestBaseAccountSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to update a Test Base Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountUpdateParameterProperties {
    #[doc = "Describes a Test Base Account SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<TestBaseAccountSku>,
}
impl TestBaseAccountUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to update a Test Base Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountUpdateParameters {
    #[doc = "Parameters supplied to update a Test Base Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestBaseAccountUpdateParameterProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl TestBaseAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The usage data of a Test Base Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountUsageData {
    #[doc = "Fully qualified ARM resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Localizable String object containing the name and a localized value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<TestBaseAccountUsageName>,
    #[doc = "Representing the units of the usage quota. Possible values are: Count, Bytes, Seconds, Percent, CountPerSecond, BytesPerSecond."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The maximum permitted value for the usage quota. If there is no limit, this value will be -1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "Current value for the usage quota."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
}
impl TestBaseAccountUsageData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Test Base Account usage data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountUsageDataList {
    #[doc = "The list of Test Base Account usage data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestBaseAccountUsageData>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestBaseAccountUsageDataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TestBaseAccountUsageDataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localizable String object containing the name and a localized value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestBaseAccountUsageName {
    #[doc = "The identifier of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized name of the usage."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl TestBaseAccountUsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The execution status of a test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestExecutionStatus")]
pub enum TestExecutionStatus {
    None,
    InProgress,
    Processing,
    Completed,
    NotExecuted,
    Incomplete,
    Failed,
    Succeeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestExecutionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestExecutionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestExecutionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("TestExecutionStatus", 0u32, "None"),
            Self::InProgress => serializer.serialize_unit_variant("TestExecutionStatus", 1u32, "InProgress"),
            Self::Processing => serializer.serialize_unit_variant("TestExecutionStatus", 2u32, "Processing"),
            Self::Completed => serializer.serialize_unit_variant("TestExecutionStatus", 3u32, "Completed"),
            Self::NotExecuted => serializer.serialize_unit_variant("TestExecutionStatus", 4u32, "NotExecuted"),
            Self::Incomplete => serializer.serialize_unit_variant("TestExecutionStatus", 5u32, "Incomplete"),
            Self::Failed => serializer.serialize_unit_variant("TestExecutionStatus", 6u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("TestExecutionStatus", 7u32, "Succeeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The grade of a test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestGrade")]
pub enum TestGrade {
    None,
    NotAvailable,
    Pass,
    Fail,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestGrade {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestGrade {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestGrade {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("TestGrade", 0u32, "None"),
            Self::NotAvailable => serializer.serialize_unit_variant("TestGrade", 1u32, "NotAvailable"),
            Self::Pass => serializer.serialize_unit_variant("TestGrade", 2u32, "Pass"),
            Self::Fail => serializer.serialize_unit_variant("TestGrade", 3u32, "Fail"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The summary of a Test Analysis Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultAnalysisSummary {
    #[doc = "Metric name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The analysis status."]
    #[serde(rename = "analysisStatus", default, skip_serializing_if = "Option::is_none")]
    pub analysis_status: Option<test_result_analysis_summary::AnalysisStatus>,
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
}
impl TestResultAnalysisSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_result_analysis_summary {
    use super::*;
    #[doc = "The analysis status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AnalysisStatus")]
    pub enum AnalysisStatus {
        None,
        Completed,
        InProgress,
        Failed,
        Succeeded,
        Available,
        NotAvailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AnalysisStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AnalysisStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AnalysisStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("AnalysisStatus", 0u32, "None"),
                Self::Completed => serializer.serialize_unit_variant("AnalysisStatus", 1u32, "Completed"),
                Self::InProgress => serializer.serialize_unit_variant("AnalysisStatus", 2u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("AnalysisStatus", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("AnalysisStatus", 4u32, "Succeeded"),
                Self::Available => serializer.serialize_unit_variant("AnalysisStatus", 5u32, "Available"),
                Self::NotAvailable => serializer.serialize_unit_variant("AnalysisStatus", 6u32, "NotAvailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters body to pass for getting the download URL of the test execution console log file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResultConsoleLogDownloadUrlParameters {
    #[doc = "The log file name corresponding to the download URL."]
    #[serde(rename = "logFileName")]
    pub log_file_name: String,
}
impl TestResultConsoleLogDownloadUrlParameters {
    pub fn new(log_file_name: String) -> Self {
        Self { log_file_name }
    }
}
#[doc = "A list of Test Results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultListResult {
    #[doc = "The list of Test Results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestResultResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestResultListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TestResultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a Test Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultProperties {
    #[doc = "Azure Id of the baseline test result."]
    #[serde(rename = "baselineTestResultId", default, skip_serializing_if = "Option::is_none")]
    pub baseline_test_result_id: Option<String>,
    #[doc = "Resource Id of the package."]
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[doc = "Application name."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "Application version."]
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[doc = "The operating system name, e.g. Windows 10 1809."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The name of the tested release (OS update)."]
    #[serde(rename = "releaseName", default, skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[doc = "The release version date of the tested release."]
    #[serde(rename = "releaseVersionDate", default, with = "azure_core::date::rfc3339::option")]
    pub release_version_date: Option<time::OffsetDateTime>,
    #[doc = "The flighting ring, only for release of feature updates."]
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[doc = "The build version of the tested release (OS update)."]
    #[serde(rename = "buildVersion", default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
    #[doc = "The build revision of the tested release (OS update)."]
    #[serde(rename = "buildRevision", default, skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    #[doc = "Test type. E.g. 'Out of box test' or 'Functional test'."]
    #[serde(rename = "testType", default, skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
    #[doc = "The run time of the test."]
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[doc = "Whether download data is available."]
    #[serde(rename = "isDownloadDataAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_download_data_available: Option<bool>,
    #[doc = "Whether video data is available."]
    #[serde(rename = "isVideoAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_video_available: Option<bool>,
    #[doc = "The execution status of a test."]
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[doc = "The status of a test."]
    #[serde(rename = "testStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_status: Option<TestStatus>,
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[doc = "KB number."]
    #[serde(rename = "kbNumber", default, skip_serializing_if = "Option::is_none")]
    pub kb_number: Option<String>,
    #[doc = "Interop media type."]
    #[serde(rename = "interopMediaType", default, skip_serializing_if = "Option::is_none")]
    pub interop_media_type: Option<String>,
    #[doc = "Interop media version."]
    #[serde(rename = "interopMediaVersion", default, skip_serializing_if = "Option::is_none")]
    pub interop_media_version: Option<String>,
    #[doc = "The version of the Windows update package."]
    #[serde(rename = "packageVersion", default, skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[doc = "List of analysis summaries."]
    #[serde(rename = "analysisSummaries", default, skip_serializing_if = "Vec::is_empty")]
    pub analysis_summaries: Vec<TestResultAnalysisSummary>,
}
impl TestResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Test Result Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestResultResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of a Test Result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestResultProperties>,
}
impl TestResultResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of a test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestStatus")]
pub enum TestStatus {
    None,
    TestExecutionInProgress,
    DataProcessing,
    TestFailure,
    UpdateFailure,
    TestAndUpdateFailure,
    InfrastructureFailure,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("TestStatus", 0u32, "None"),
            Self::TestExecutionInProgress => serializer.serialize_unit_variant("TestStatus", 1u32, "TestExecutionInProgress"),
            Self::DataProcessing => serializer.serialize_unit_variant("TestStatus", 2u32, "DataProcessing"),
            Self::TestFailure => serializer.serialize_unit_variant("TestStatus", 3u32, "TestFailure"),
            Self::UpdateFailure => serializer.serialize_unit_variant("TestStatus", 4u32, "UpdateFailure"),
            Self::TestAndUpdateFailure => serializer.serialize_unit_variant("TestStatus", 5u32, "TestAndUpdateFailure"),
            Self::InfrastructureFailure => serializer.serialize_unit_variant("TestStatus", 6u32, "InfrastructureFailure"),
            Self::Completed => serializer.serialize_unit_variant("TestStatus", 7u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A list of Test Summaries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSummaryListResult {
    #[doc = "The list of Test Summaries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestSummaryResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestSummaryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TestSummaryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Test Summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSummaryProperties {
    #[doc = "The Id of the current Test Summary."]
    #[serde(rename = "testSummaryId", default, skip_serializing_if = "Option::is_none")]
    pub test_summary_id: Option<String>,
    #[doc = "The Azure resource Id of package."]
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[doc = "Application name."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "Application version."]
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[doc = "The execution status of a test."]
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[doc = "The status of a test."]
    #[serde(rename = "testStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_status: Option<TestStatus>,
    #[doc = "The grade of a test."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[doc = "The run time of the last test."]
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[doc = "The summary of some tests."]
    #[serde(rename = "featureUpdatesTestSummary", default, skip_serializing_if = "Option::is_none")]
    pub feature_updates_test_summary: Option<OsUpdatesTestSummary>,
    #[doc = "The summary of some tests."]
    #[serde(rename = "securityUpdatesTestSummary", default, skip_serializing_if = "Option::is_none")]
    pub security_updates_test_summary: Option<OsUpdatesTestSummary>,
    #[doc = "Resource tags."]
    #[serde(rename = "packageTags", default, skip_serializing_if = "Option::is_none")]
    pub package_tags: Option<Tags>,
}
impl TestSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of a Test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestSummaryResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a Test Summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestSummaryProperties>,
}
impl TestSummaryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestType")]
pub enum TestType {
    OutOfBoxTest,
    FunctionalTest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OutOfBoxTest => serializer.serialize_unit_variant("TestType", 0u32, "OutOfBoxTest"),
            Self::FunctionalTest => serializer.serialize_unit_variant("TestType", 1u32, "FunctionalTest"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A list of test types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTypeListResult {
    #[doc = "The list of test types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestTypeResource>,
    #[doc = "Link to the next set of results. Not empty if value contains incomplete list of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TestTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Test Type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTypeProperties {
    #[doc = "The actual name of a test type of a Test Base Account."]
    #[serde(rename = "actualTestTypeName", default, skip_serializing_if = "Option::is_none")]
    pub actual_test_type_name: Option<String>,
}
impl TestTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The test type resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestTypeResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The Test Type properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestTypeProperties>,
}
impl TestTypeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
            etag: None,
        }
    }
}
#[doc = "The user object receiver value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserObjectReceiverValue {
    #[doc = "user object ids."]
    #[serde(rename = "userObjectIds", default, skip_serializing_if = "Vec::is_empty")]
    pub user_object_ids: Vec<String>,
}
impl UserObjectReceiverValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The bound of a utilization result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UtilizationBound {
    #[doc = "The percentile of the bound."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentile: Option<f64>,
    #[doc = "The value of the bound."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl UtilizationBound {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The utilization entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UtilizationEntry {
    #[doc = "The timestamp."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl UtilizationEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Utilization Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UtilizationResult {
    #[doc = "Process name, or '_total' for all processes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[doc = "The bound of a utilization result."]
    #[serde(rename = "upperBound", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<UtilizationBound>,
    #[doc = "The bound of a utilization result."]
    #[serde(rename = "lowerBound", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<UtilizationBound>,
    #[doc = "Utilization data"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilization: Vec<UtilizationEntry>,
}
impl UtilizationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
