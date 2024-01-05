#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A list of active directory administrators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerAzureAdAdministrator>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AdministratorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a active directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdministratorProperties {
    #[doc = "Type of the sever administrator."]
    #[serde(rename = "administratorType")]
    pub administrator_type: administrator_properties::AdministratorType,
    #[doc = "Login name of the server administrator."]
    pub login: String,
    #[doc = "SID (object ID) of the server administrator."]
    pub sid: String,
    #[doc = "Tenant ID of the administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Azure Active Directory only Authentication enabled."]
    #[serde(rename = "azureADOnlyAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_ad_only_authentication: Option<bool>,
}
impl AdministratorProperties {
    pub fn new(administrator_type: administrator_properties::AdministratorType, login: String, sid: String) -> Self {
        Self {
            administrator_type,
            login,
            sid,
            tenant_id: None,
            azure_ad_only_authentication: None,
        }
    }
}
pub mod administrator_properties {
    use super::*;
    #[doc = "Type of the sever administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AdministratorType")]
    pub enum AdministratorType {
        ActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AdministratorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AdministratorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AdministratorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("AdministratorType", 0u32, "ActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of an Advanced Threat Protection state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvancedThreatProtectionProperties {
    #[doc = "Specifies the state of the Advanced Threat Protection, whether it is enabled or disabled or a state has not been applied yet on the specific database or server."]
    pub state: advanced_threat_protection_properties::State,
    #[doc = "Specifies the UTC creation time of the policy."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
}
impl AdvancedThreatProtectionProperties {
    pub fn new(state: advanced_threat_protection_properties::State) -> Self {
        Self {
            state,
            creation_time: None,
        }
    }
}
pub mod advanced_threat_protection_properties {
    use super::*;
    #[doc = "Specifies the state of the Advanced Threat Protection, whether it is enabled or disabled or a state has not been applied yet on the specific database or server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        New,
        Enabled,
        Disabled,
    }
}
#[doc = "Database, Server or Elastic Pool Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Advisor {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for a Database, Server or Elastic Pool Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvisorProperties>,
}
impl Advisor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a Database, Server or Elastic Pool Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvisorProperties {
    #[doc = "Gets the status of availability of this advisor to customers. Possible values are 'GA', 'PublicPreview', 'LimitedPublicPreview' and 'PrivatePreview'."]
    #[serde(rename = "advisorStatus", default, skip_serializing_if = "Option::is_none")]
    pub advisor_status: Option<advisor_properties::AdvisorStatus>,
    #[doc = "Gets the auto-execute status (whether to let the system execute the recommendations) of this advisor. Possible values are 'Enabled' and 'Disabled'"]
    #[serde(rename = "autoExecuteStatus")]
    pub auto_execute_status: advisor_properties::AutoExecuteStatus,
    #[doc = "Gets the resource from which current value of auto-execute status is inherited. Auto-execute status can be set on (and inherited from) different levels in the resource hierarchy. Possible values are 'Subscription', 'Server', 'ElasticPool', 'Database' and 'Default' (when status is not explicitly set on any level)."]
    #[serde(rename = "autoExecuteStatusInheritedFrom", default, skip_serializing_if = "Option::is_none")]
    pub auto_execute_status_inherited_from: Option<advisor_properties::AutoExecuteStatusInheritedFrom>,
    #[doc = "Gets that status of recommendations for this advisor and reason for not having any recommendations. Possible values include, but are not limited to, 'Ok' (Recommendations available),LowActivity (not enough workload to analyze), 'DbSeemsTuned' (Database is doing well), etc."]
    #[serde(rename = "recommendationsStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_status: Option<String>,
    #[doc = "Gets the time when the current resource was analyzed for recommendations by this advisor."]
    #[serde(rename = "lastChecked", default, with = "azure_core::date::rfc3339::option")]
    pub last_checked: Option<time::OffsetDateTime>,
    #[doc = "Gets the recommended actions for this advisor."]
    #[serde(
        rename = "recommendedActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommended_actions: Vec<RecommendedAction>,
}
impl AdvisorProperties {
    pub fn new(auto_execute_status: advisor_properties::AutoExecuteStatus) -> Self {
        Self {
            advisor_status: None,
            auto_execute_status,
            auto_execute_status_inherited_from: None,
            recommendations_status: None,
            last_checked: None,
            recommended_actions: Vec::new(),
        }
    }
}
pub mod advisor_properties {
    use super::*;
    #[doc = "Gets the status of availability of this advisor to customers. Possible values are 'GA', 'PublicPreview', 'LimitedPublicPreview' and 'PrivatePreview'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AdvisorStatus {
        #[serde(rename = "GA")]
        Ga,
        PublicPreview,
        LimitedPublicPreview,
        PrivatePreview,
    }
    #[doc = "Gets the auto-execute status (whether to let the system execute the recommendations) of this advisor. Possible values are 'Enabled' and 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AutoExecuteStatus {
        Enabled,
        Disabled,
        Default,
    }
    #[doc = "Gets the resource from which current value of auto-execute status is inherited. Auto-execute status can be set on (and inherited from) different levels in the resource hierarchy. Possible values are 'Subscription', 'Server', 'ElasticPool', 'Database' and 'Default' (when status is not explicitly set on any level)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AutoExecuteStatusInheritedFrom {
        Default,
        Subscription,
        Server,
        ElasticPool,
        Database,
    }
}
#[doc = "Supported auto pause delay time range"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoPauseDelayTimeRange {
    #[doc = "Minimum value"]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[doc = "Maximum value"]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[doc = "Step value for discrete values between the minimum value and the maximum value."]
    #[serde(rename = "stepSize", default, skip_serializing_if = "Option::is_none")]
    pub step_size: Option<i32>,
    #[doc = "Default value is no value is provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Unit of time that delay is expressed in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<auto_pause_delay_time_range::Unit>,
    #[doc = "Value that is used to not pause (infinite delay before pause)"]
    #[serde(rename = "doNotPauseValue", default, skip_serializing_if = "Option::is_none")]
    pub do_not_pause_value: Option<i32>,
}
impl AutoPauseDelayTimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_pause_delay_time_range {
    use super::*;
    #[doc = "Unit of time that delay is expressed in"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Minutes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Minutes => serializer.serialize_unit_variant("Unit", 0u32, "Minutes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Automatic tuning properties for individual advisors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticTuningOptions {
    #[doc = "Automatic tuning option desired state."]
    #[serde(rename = "desiredState", default, skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<automatic_tuning_options::DesiredState>,
    #[doc = "Automatic tuning option actual state."]
    #[serde(rename = "actualState", default, skip_serializing_if = "Option::is_none")]
    pub actual_state: Option<automatic_tuning_options::ActualState>,
    #[doc = "Reason code if desired and actual state are different."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<i32>,
    #[doc = "Reason description if desired and actual state are different."]
    #[serde(rename = "reasonDesc", default, skip_serializing_if = "Option::is_none")]
    pub reason_desc: Option<automatic_tuning_options::ReasonDesc>,
}
impl AutomaticTuningOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automatic_tuning_options {
    use super::*;
    #[doc = "Automatic tuning option desired state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesiredState {
        Off,
        On,
        Default,
    }
    #[doc = "Automatic tuning option actual state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActualState {
        Off,
        On,
    }
    #[doc = "Reason description if desired and actual state are different."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonDesc {
        Default,
        Disabled,
        AutoConfigured,
        InheritedFromServer,
        QueryStoreOff,
        QueryStoreReadOnly,
        NotSupported,
    }
}
#[doc = "Automatic tuning properties for individual advisors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticTuningServerOptions {
    #[doc = "Automatic tuning option desired state."]
    #[serde(rename = "desiredState", default, skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<automatic_tuning_server_options::DesiredState>,
    #[doc = "Automatic tuning option actual state."]
    #[serde(rename = "actualState", default, skip_serializing_if = "Option::is_none")]
    pub actual_state: Option<automatic_tuning_server_options::ActualState>,
    #[doc = "Reason code if desired and actual state are different."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<i32>,
    #[doc = "Reason description if desired and actual state are different."]
    #[serde(rename = "reasonDesc", default, skip_serializing_if = "Option::is_none")]
    pub reason_desc: Option<automatic_tuning_server_options::ReasonDesc>,
}
impl AutomaticTuningServerOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automatic_tuning_server_options {
    use super::*;
    #[doc = "Automatic tuning option desired state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesiredState {
        Off,
        On,
        Default,
    }
    #[doc = "Automatic tuning option actual state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActualState {
        Off,
        On,
    }
    #[doc = "Reason description if desired and actual state are different."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonDesc {
        Default,
        Disabled,
        AutoConfigured,
    }
}
#[doc = "Server-level Automatic Tuning properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticTuningServerProperties {
    #[doc = "Automatic tuning desired state."]
    #[serde(rename = "desiredState", default, skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<automatic_tuning_server_properties::DesiredState>,
    #[doc = "Automatic tuning actual state."]
    #[serde(rename = "actualState", default, skip_serializing_if = "Option::is_none")]
    pub actual_state: Option<automatic_tuning_server_properties::ActualState>,
    #[doc = "Automatic tuning options definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl AutomaticTuningServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automatic_tuning_server_properties {
    use super::*;
    #[doc = "Automatic tuning desired state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesiredState {
        Custom,
        Auto,
        Unspecified,
    }
    #[doc = "Automatic tuning actual state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActualState {
        Custom,
        Auto,
        Unspecified,
    }
}
#[doc = "A list of active directory only authentications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAdOnlyAuthListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerAzureAdOnlyAuthentication>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureAdOnlyAuthListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureAdOnlyAuthListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a active directory only authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAdOnlyAuthProperties {
    #[doc = "Azure Active Directory only Authentication enabled."]
    #[serde(rename = "azureADOnlyAuthentication")]
    pub azure_ad_only_authentication: bool,
}
impl AzureAdOnlyAuthProperties {
    pub fn new(azure_ad_only_authentication: bool) -> Self {
        Self {
            azure_ad_only_authentication,
        }
    }
}
#[doc = "A short term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupShortTermRetentionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a short term retention policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupShortTermRetentionPolicyProperties>,
}
impl BackupShortTermRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of short term retention policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupShortTermRetentionPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BackupShortTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BackupShortTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BackupShortTermRetentionPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a short term retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupShortTermRetentionPolicyProperties {
    #[doc = "The backup retention period in days. This is how many days Point-in-Time Restore will be supported."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "The differential backup interval in hours. This is how many interval hours between each differential backup will be supported. This is only applicable to live databases but not dropped databases."]
    #[serde(rename = "diffBackupIntervalInHours", default, skip_serializing_if = "Option::is_none")]
    pub diff_backup_interval_in_hours: Option<backup_short_term_retention_policy_properties::DiffBackupIntervalInHours>,
}
impl BackupShortTermRetentionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_short_term_retention_policy_properties {
    use super::*;
    #[doc = "The differential backup interval in hours. This is how many interval hours between each differential backup will be supported. This is only applicable to live databases but not dropped databases."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiffBackupIntervalInHours")]
    pub enum DiffBackupIntervalInHours {
        #[serde(rename = "12")]
        N12,
        #[serde(rename = "24")]
        N24,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiffBackupIntervalInHours {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiffBackupIntervalInHours {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiffBackupIntervalInHours {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N12 => serializer.serialize_unit_variant("DiffBackupIntervalInHours", 0u32, "12"),
                Self::N24 => serializer.serialize_unit_variant("DiffBackupIntervalInHours", 1u32, "24"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SQL Vulnerability Assessment baseline Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Baseline {
    #[doc = "SQL Vulnerability Assessment baseline expected results"]
    #[serde(
        rename = "expectedResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expected_results: Vec<Vec<String>>,
    #[doc = "SQL Vulnerability Assessment baseline update time (UTC)"]
    #[serde(rename = "updatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time: Option<time::OffsetDateTime>,
}
impl Baseline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Vulnerability Assessment baseline adjusted results"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaselineAdjustedResult {
    #[doc = "SQL Vulnerability Assessment baseline Details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<Baseline>,
    #[doc = "SQL Vulnerability Assessment baseline status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<baseline_adjusted_result::Status>,
    #[doc = "SQL Vulnerability Assessment results that are not in baseline"]
    #[serde(
        rename = "resultsNotInBaseline",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results_not_in_baseline: Vec<Vec<String>>,
    #[doc = "SQL Vulnerability Assessment results that are in baseline."]
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
pub mod baseline_adjusted_result {
    use super::*;
    #[doc = "SQL Vulnerability Assessment baseline status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NonFinding,
        Finding,
        InternalError,
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
                Self::NonFinding => serializer.serialize_unit_variant("Status", 0u32, "NonFinding"),
                Self::Finding => serializer.serialize_unit_variant("Status", 1u32, "Finding"),
                Self::InternalError => serializer.serialize_unit_variant("Status", 2u32, "InternalError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SQL Vulnerability Assessment benchmark reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenchmarkReference {
    #[doc = "SQL Vulnerability Assessment benchmark name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    #[doc = "SQL Vulnerability Assessment benchmark reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
impl BenchmarkReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateInfo {
    #[doc = "The certificate name"]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[doc = "The certificate expiry date"]
    #[serde(rename = "expiryDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date: Option<time::OffsetDateTime>,
}
impl CertificateInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information necessary to change long term retention backup access tier and related operation mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeLongTermRetentionBackupAccessTierParameters {
    #[doc = "The long term retention backup storage access tier"]
    #[serde(rename = "backupStorageAccessTier")]
    pub backup_storage_access_tier: String,
    #[doc = "The operation mode when updating ltr backup storage access tier"]
    #[serde(rename = "operationMode")]
    pub operation_mode: String,
}
impl ChangeLongTermRetentionBackupAccessTierParameters {
    pub fn new(backup_storage_access_tier: String, operation_mode: String) -> Self {
        Self {
            backup_storage_access_tier,
            operation_mode,
        }
    }
}
#[doc = "A request to check whether the specified name for a resource is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_request::Type,
}
impl CheckNameAvailabilityRequest {
    pub fn new(name: String, type_: check_name_availability_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Sql/servers")]
        MicrosoftSqlServers,
    }
}
#[doc = "The result of a name availability check."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "The name whose availability was checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "True if the name is available, otherwise false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[doc = "The reason code explaining why the name is unavailable. Will be undefined if the name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "A message explaining why the name is unavailable. Will be undefined if the name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason code explaining why the name is unavailable. Will be undefined if the name is available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "Contains the information necessary to perform a complete database restore operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompleteDatabaseRestoreDefinition {
    #[doc = "The last backup name to apply"]
    #[serde(rename = "lastBackupName")]
    pub last_backup_name: String,
}
impl CompleteDatabaseRestoreDefinition {
    pub fn new(last_backup_name: String) -> Self {
        Self { last_backup_name }
    }
}
#[doc = "Contains the information necessary to perform long term retention backup copy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CopyLongTermRetentionBackupParameters {
    #[doc = "Contains the properties to perform long term retention backup copy operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CopyLongTermRetentionBackupParametersProperties>,
}
impl CopyLongTermRetentionBackupParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the properties to perform long term retention backup copy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CopyLongTermRetentionBackupParametersProperties {
    #[doc = "The subscription that owns the target server"]
    #[serde(rename = "targetSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub target_subscription_id: Option<String>,
    #[doc = "The resource group that owns the target server"]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[doc = "The resource Id of the target server that owns the database"]
    #[serde(rename = "targetServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_server_resource_id: Option<String>,
    #[doc = "The fully qualified domain name of the target server"]
    #[serde(rename = "targetServerFullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub target_server_fully_qualified_domain_name: Option<String>,
    #[doc = "The name of the database owns the copied backup."]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "The storage redundancy type of the copied backup"]
    #[serde(rename = "targetBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub target_backup_storage_redundancy: Option<copy_long_term_retention_backup_parameters_properties::TargetBackupStorageRedundancy>,
}
impl CopyLongTermRetentionBackupParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod copy_long_term_retention_backup_parameters_properties {
    use super::*;
    #[doc = "The storage redundancy type of the copied backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetBackupStorageRedundancy")]
    pub enum TargetBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains the information necessary to perform a create database restore point operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDatabaseRestorePointDefinition {
    #[doc = "The restore point label to apply"]
    #[serde(rename = "restorePointLabel")]
    pub restore_point_label: String,
}
impl CreateDatabaseRestorePointDefinition {
    pub fn new(restore_point_label: String) -> Self {
        Self { restore_point_label }
    }
}
#[doc = "A database data masking policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The location of the data masking policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of Data Masking Policy. Metadata, used for Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The properties of a database data masking policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMaskingPolicyProperties>,
}
impl DataMaskingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database data masking policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMaskingPolicyProperties {
    #[doc = "The state of the data masking policy."]
    #[serde(rename = "dataMaskingState")]
    pub data_masking_state: data_masking_policy_properties::DataMaskingState,
    #[doc = "The list of the exempt principals. Specifies the semicolon-separated list of database users for which the data masking policy does not apply. The specified users receive data results without masking for all of the database queries."]
    #[serde(rename = "exemptPrincipals", default, skip_serializing_if = "Option::is_none")]
    pub exempt_principals: Option<String>,
    #[doc = "The list of the application principals. This is a legacy parameter and is no longer used."]
    #[serde(rename = "applicationPrincipals", default, skip_serializing_if = "Option::is_none")]
    pub application_principals: Option<String>,
    #[doc = "The masking level. This is a legacy parameter and is no longer used."]
    #[serde(rename = "maskingLevel", default, skip_serializing_if = "Option::is_none")]
    pub masking_level: Option<String>,
}
impl DataMaskingPolicyProperties {
    pub fn new(data_masking_state: data_masking_policy_properties::DataMaskingState) -> Self {
        Self {
            data_masking_state,
            exempt_principals: None,
            application_principals: None,
            masking_level: None,
        }
    }
}
pub mod data_masking_policy_properties {
    use super::*;
    #[doc = "The state of the data masking policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataMaskingState {
        Enabled,
        Disabled,
    }
}
#[doc = "A database data masking rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The location of the data masking rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of Data Masking Rule. Metadata, used for Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The properties of a database data masking rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMaskingRuleProperties>,
}
impl DataMaskingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of database data masking rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingRuleListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataMaskingRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataMaskingRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataMaskingRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database data masking rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMaskingRuleProperties {
    #[doc = "The rule Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The rule state. Used to delete a rule. To delete an existing rule, specify the schemaName, tableName, columnName, maskingFunction, and specify ruleState as disabled. However, if the rule doesn't already exist, the rule will be created with ruleState set to enabled, regardless of the provided value of ruleState."]
    #[serde(rename = "ruleState", default, skip_serializing_if = "Option::is_none")]
    pub rule_state: Option<data_masking_rule_properties::RuleState>,
    #[doc = "The schema name on which the data masking rule is applied."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    #[doc = "The table name on which the data masking rule is applied."]
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[doc = "The column name on which the data masking rule is applied."]
    #[serde(rename = "columnName")]
    pub column_name: String,
    #[doc = "The alias name. This is a legacy parameter and is no longer used."]
    #[serde(rename = "aliasName", default, skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[doc = "The masking function that is used for the data masking rule."]
    #[serde(rename = "maskingFunction")]
    pub masking_function: data_masking_rule_properties::MaskingFunction,
    #[doc = "The numberFrom property of the masking rule. Required if maskingFunction is set to Number, otherwise this parameter will be ignored."]
    #[serde(rename = "numberFrom", default, skip_serializing_if = "Option::is_none")]
    pub number_from: Option<String>,
    #[doc = "The numberTo property of the data masking rule. Required if maskingFunction is set to Number, otherwise this parameter will be ignored."]
    #[serde(rename = "numberTo", default, skip_serializing_if = "Option::is_none")]
    pub number_to: Option<String>,
    #[doc = "If maskingFunction is set to Text, the number of characters to show unmasked in the beginning of the string. Otherwise, this parameter will be ignored."]
    #[serde(rename = "prefixSize", default, skip_serializing_if = "Option::is_none")]
    pub prefix_size: Option<String>,
    #[doc = "If maskingFunction is set to Text, the number of characters to show unmasked at the end of the string. Otherwise, this parameter will be ignored."]
    #[serde(rename = "suffixSize", default, skip_serializing_if = "Option::is_none")]
    pub suffix_size: Option<String>,
    #[doc = "If maskingFunction is set to Text, the character to use for masking the unexposed part of the string. Otherwise, this parameter will be ignored."]
    #[serde(rename = "replacementString", default, skip_serializing_if = "Option::is_none")]
    pub replacement_string: Option<String>,
}
impl DataMaskingRuleProperties {
    pub fn new(
        schema_name: String,
        table_name: String,
        column_name: String,
        masking_function: data_masking_rule_properties::MaskingFunction,
    ) -> Self {
        Self {
            id: None,
            rule_state: None,
            schema_name,
            table_name,
            column_name,
            alias_name: None,
            masking_function,
            number_from: None,
            number_to: None,
            prefix_size: None,
            suffix_size: None,
            replacement_string: None,
        }
    }
}
pub mod data_masking_rule_properties {
    use super::*;
    #[doc = "The rule state. Used to delete a rule. To delete an existing rule, specify the schemaName, tableName, columnName, maskingFunction, and specify ruleState as disabled. However, if the rule doesn't already exist, the rule will be created with ruleState set to enabled, regardless of the provided value of ruleState."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RuleState {
        Enabled,
        Disabled,
    }
    #[doc = "The masking function that is used for the data masking rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaskingFunction {
        Default,
        #[serde(rename = "CCN")]
        Ccn,
        Email,
        Number,
        #[serde(rename = "SSN")]
        Ssn,
        Text,
    }
}
#[doc = "User activities of a data warehouse"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataWarehouseUserActivities {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "User activities of a data warehouse. This currently includes the count of running or suspended queries. For more information, please view the sys.dm_pdw_exec_requests dynamic management view (DMV)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataWarehouseUserActivitiesProperties>,
}
impl DataWarehouseUserActivities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User activities of a data warehouse"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataWarehouseUserActivitiesListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataWarehouseUserActivities>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataWarehouseUserActivitiesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DataWarehouseUserActivitiesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User activities of a data warehouse. This currently includes the count of running or suspended queries. For more information, please view the sys.dm_pdw_exec_requests dynamic management view (DMV)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataWarehouseUserActivitiesProperties {
    #[doc = "Count of running and suspended queries."]
    #[serde(rename = "activeQueriesCount", default, skip_serializing_if = "Option::is_none")]
    pub active_queries_count: Option<i32>,
}
impl DataWarehouseUserActivitiesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Kind of database. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource that manages the database."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DatabaseIdentity>,
    #[doc = "The database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl Database {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            kind: None,
            managed_by: None,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "A database Advanced Threat Protection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAdvancedThreatProtection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of an Advanced Threat Protection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvancedThreatProtectionProperties>,
}
impl DatabaseAdvancedThreatProtection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the database's Advanced Threat Protection configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAdvancedThreatProtectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseAdvancedThreatProtection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseAdvancedThreatProtectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseAdvancedThreatProtectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database-level Automatic Tuning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAutomaticTuning {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Database-level Automatic Tuning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseAutomaticTuningProperties>,
}
impl DatabaseAutomaticTuning {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database-level Automatic Tuning properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAutomaticTuningProperties {
    #[doc = "Automatic tuning desired state."]
    #[serde(rename = "desiredState", default, skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<database_automatic_tuning_properties::DesiredState>,
    #[doc = "Automatic tuning actual state."]
    #[serde(rename = "actualState", default, skip_serializing_if = "Option::is_none")]
    pub actual_state: Option<database_automatic_tuning_properties::ActualState>,
    #[doc = "Automatic tuning options definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl DatabaseAutomaticTuningProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_automatic_tuning_properties {
    use super::*;
    #[doc = "Automatic tuning desired state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DesiredState {
        Inherit,
        Custom,
        Auto,
        Unspecified,
    }
    #[doc = "Automatic tuning actual state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActualState {
        Inherit,
        Custom,
        Auto,
        Unspecified,
    }
}
#[doc = "A database blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties of a database blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseBlobAuditingPolicyProperties>,
}
impl DatabaseBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of database auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseBlobAuditingPolicyProperties {
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\nLEDGER_OPERATION_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(
        rename = "auditActionsAndGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled' and 'IsAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
    #[doc = "Specifies whether Managed Identity is used to access blob storage"]
    #[serde(rename = "isManagedIdentityInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_managed_identity_in_use: Option<bool>,
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: database_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
}
impl DatabaseBlobAuditingPolicyProperties {
    pub fn new(state: database_blob_auditing_policy_properties::State) -> Self {
        Self {
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
            is_managed_identity_in_use: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            storage_account_subscription_id: None,
        }
    }
}
pub mod database_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A database column resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseColumn {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Database column properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseColumnProperties>,
}
impl DatabaseColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of database columns."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseColumnListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseColumn>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseColumnListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseColumnListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database column properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseColumnProperties {
    #[doc = "The column data type."]
    #[serde(rename = "columnType", default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<database_column_properties::ColumnType>,
    #[doc = "The table temporal type."]
    #[serde(rename = "temporalType", default, skip_serializing_if = "Option::is_none")]
    pub temporal_type: Option<database_column_properties::TemporalType>,
    #[doc = "Whether or not the column belongs to a memory optimized table."]
    #[serde(rename = "memoryOptimized", default, skip_serializing_if = "Option::is_none")]
    pub memory_optimized: Option<bool>,
    #[doc = "Whether or not the column is computed."]
    #[serde(rename = "isComputed", default, skip_serializing_if = "Option::is_none")]
    pub is_computed: Option<bool>,
}
impl DatabaseColumnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_column_properties {
    use super::*;
    #[doc = "The column data type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ColumnType")]
    pub enum ColumnType {
        #[serde(rename = "image")]
        Image,
        #[serde(rename = "text")]
        Text,
        #[serde(rename = "uniqueidentifier")]
        Uniqueidentifier,
        #[serde(rename = "date")]
        Date,
        #[serde(rename = "time")]
        Time,
        #[serde(rename = "datetime2")]
        Datetime2,
        #[serde(rename = "datetimeoffset")]
        Datetimeoffset,
        #[serde(rename = "tinyint")]
        Tinyint,
        #[serde(rename = "smallint")]
        Smallint,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "smalldatetime")]
        Smalldatetime,
        #[serde(rename = "real")]
        Real,
        #[serde(rename = "money")]
        Money,
        #[serde(rename = "datetime")]
        Datetime,
        #[serde(rename = "float")]
        Float,
        #[serde(rename = "sql_variant")]
        SqlVariant,
        #[serde(rename = "ntext")]
        Ntext,
        #[serde(rename = "bit")]
        Bit,
        #[serde(rename = "decimal")]
        Decimal,
        #[serde(rename = "numeric")]
        Numeric,
        #[serde(rename = "smallmoney")]
        Smallmoney,
        #[serde(rename = "bigint")]
        Bigint,
        #[serde(rename = "hierarchyid")]
        Hierarchyid,
        #[serde(rename = "geometry")]
        Geometry,
        #[serde(rename = "geography")]
        Geography,
        #[serde(rename = "varbinary")]
        Varbinary,
        #[serde(rename = "varchar")]
        Varchar,
        #[serde(rename = "binary")]
        Binary,
        #[serde(rename = "char")]
        Char,
        #[serde(rename = "timestamp")]
        Timestamp,
        #[serde(rename = "nvarchar")]
        Nvarchar,
        #[serde(rename = "nchar")]
        Nchar,
        #[serde(rename = "xml")]
        Xml,
        #[serde(rename = "sysname")]
        Sysname,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ColumnType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ColumnType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ColumnType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Image => serializer.serialize_unit_variant("ColumnType", 0u32, "image"),
                Self::Text => serializer.serialize_unit_variant("ColumnType", 1u32, "text"),
                Self::Uniqueidentifier => serializer.serialize_unit_variant("ColumnType", 2u32, "uniqueidentifier"),
                Self::Date => serializer.serialize_unit_variant("ColumnType", 3u32, "date"),
                Self::Time => serializer.serialize_unit_variant("ColumnType", 4u32, "time"),
                Self::Datetime2 => serializer.serialize_unit_variant("ColumnType", 5u32, "datetime2"),
                Self::Datetimeoffset => serializer.serialize_unit_variant("ColumnType", 6u32, "datetimeoffset"),
                Self::Tinyint => serializer.serialize_unit_variant("ColumnType", 7u32, "tinyint"),
                Self::Smallint => serializer.serialize_unit_variant("ColumnType", 8u32, "smallint"),
                Self::Int => serializer.serialize_unit_variant("ColumnType", 9u32, "int"),
                Self::Smalldatetime => serializer.serialize_unit_variant("ColumnType", 10u32, "smalldatetime"),
                Self::Real => serializer.serialize_unit_variant("ColumnType", 11u32, "real"),
                Self::Money => serializer.serialize_unit_variant("ColumnType", 12u32, "money"),
                Self::Datetime => serializer.serialize_unit_variant("ColumnType", 13u32, "datetime"),
                Self::Float => serializer.serialize_unit_variant("ColumnType", 14u32, "float"),
                Self::SqlVariant => serializer.serialize_unit_variant("ColumnType", 15u32, "sql_variant"),
                Self::Ntext => serializer.serialize_unit_variant("ColumnType", 16u32, "ntext"),
                Self::Bit => serializer.serialize_unit_variant("ColumnType", 17u32, "bit"),
                Self::Decimal => serializer.serialize_unit_variant("ColumnType", 18u32, "decimal"),
                Self::Numeric => serializer.serialize_unit_variant("ColumnType", 19u32, "numeric"),
                Self::Smallmoney => serializer.serialize_unit_variant("ColumnType", 20u32, "smallmoney"),
                Self::Bigint => serializer.serialize_unit_variant("ColumnType", 21u32, "bigint"),
                Self::Hierarchyid => serializer.serialize_unit_variant("ColumnType", 22u32, "hierarchyid"),
                Self::Geometry => serializer.serialize_unit_variant("ColumnType", 23u32, "geometry"),
                Self::Geography => serializer.serialize_unit_variant("ColumnType", 24u32, "geography"),
                Self::Varbinary => serializer.serialize_unit_variant("ColumnType", 25u32, "varbinary"),
                Self::Varchar => serializer.serialize_unit_variant("ColumnType", 26u32, "varchar"),
                Self::Binary => serializer.serialize_unit_variant("ColumnType", 27u32, "binary"),
                Self::Char => serializer.serialize_unit_variant("ColumnType", 28u32, "char"),
                Self::Timestamp => serializer.serialize_unit_variant("ColumnType", 29u32, "timestamp"),
                Self::Nvarchar => serializer.serialize_unit_variant("ColumnType", 30u32, "nvarchar"),
                Self::Nchar => serializer.serialize_unit_variant("ColumnType", 31u32, "nchar"),
                Self::Xml => serializer.serialize_unit_variant("ColumnType", 32u32, "xml"),
                Self::Sysname => serializer.serialize_unit_variant("ColumnType", 33u32, "sysname"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The table temporal type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TemporalType")]
    pub enum TemporalType {
        NonTemporalTable,
        HistoryTable,
        SystemVersionedTemporalTable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TemporalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TemporalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TemporalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NonTemporalTable => serializer.serialize_unit_variant("TemporalType", 0u32, "NonTemporalTable"),
                Self::HistoryTable => serializer.serialize_unit_variant("TemporalType", 1u32, "HistoryTable"),
                Self::SystemVersionedTemporalTable => {
                    serializer.serialize_unit_variant("TemporalType", 2u32, "SystemVersionedTemporalTable")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An Import, Export, or PolybaseImport resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseExtensions {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the database information after a successful Import, Export, or PolybaseImport"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseExtensionsProperties>,
}
impl DatabaseExtensions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the database information after a successful Import, Export, or PolybaseImport"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseExtensionsProperties {
    #[doc = "Operation mode of the operation: Import, Export, or PolybaseImport."]
    #[serde(rename = "operationMode")]
    pub operation_mode: database_extensions_properties::OperationMode,
    #[doc = "Storage key type: StorageAccessKey or SharedAccessKey."]
    #[serde(rename = "storageKeyType")]
    pub storage_key_type: database_extensions_properties::StorageKeyType,
    #[doc = "Storage key for the storage account."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage Uri for the storage account."]
    #[serde(rename = "storageUri")]
    pub storage_uri: String,
    #[doc = "Administrator login name."]
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[doc = "Administrator login password."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "Authentication type: SQL authentication or AD password."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "Database edition for the newly created database in the case of an import operation."]
    #[serde(rename = "databaseEdition", default, skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<String>,
    #[doc = "Database service level objective for the newly created database in the case of an import operation."]
    #[serde(rename = "serviceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub service_objective_name: Option<String>,
    #[doc = "Database max size in bytes for the newly created database in the case of an import operation."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<String>,
    #[doc = "Contains the ARM resources for which to create private endpoint connection."]
    #[serde(rename = "networkIsolation", default, skip_serializing_if = "Option::is_none")]
    pub network_isolation: Option<NetworkIsolationSettings>,
}
impl DatabaseExtensionsProperties {
    pub fn new(
        operation_mode: database_extensions_properties::OperationMode,
        storage_key_type: database_extensions_properties::StorageKeyType,
        storage_key: String,
        storage_uri: String,
    ) -> Self {
        Self {
            operation_mode,
            storage_key_type,
            storage_key,
            storage_uri,
            administrator_login: None,
            administrator_login_password: None,
            authentication_type: None,
            database_edition: None,
            service_objective_name: None,
            max_size_bytes: None,
            network_isolation: None,
        }
    }
}
pub mod database_extensions_properties {
    use super::*;
    #[doc = "Operation mode of the operation: Import, Export, or PolybaseImport."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationMode")]
    pub enum OperationMode {
        PolybaseImport,
        Import,
        Export,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PolybaseImport => serializer.serialize_unit_variant("OperationMode", 0u32, "PolybaseImport"),
                Self::Import => serializer.serialize_unit_variant("OperationMode", 1u32, "Import"),
                Self::Export => serializer.serialize_unit_variant("OperationMode", 2u32, "Export"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Storage key type: StorageAccessKey or SharedAccessKey."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageKeyType")]
    pub enum StorageKeyType {
        SharedAccessKey,
        StorageAccessKey,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SharedAccessKey => serializer.serialize_unit_variant("StorageKeyType", 0u32, "SharedAccessKey"),
                Self::StorageAccessKey => serializer.serialize_unit_variant("StorageKeyType", 1u32, "StorageAccessKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseIdentity {
    #[doc = "The identity type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<database_identity::Type>,
    #[doc = "The Azure Active Directory tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl DatabaseIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_identity {
    use super::*;
    #[doc = "The identity type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        UserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Database level key used for encryption at rest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseKey {
    #[doc = "The database key type. Only supported value is 'AzureKeyVault'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<database_key::Type>,
    #[doc = "Thumbprint of the database key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The database key creation date."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Subregion of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subregion: Option<String>,
}
impl DatabaseKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_key {
    use super::*;
    #[doc = "The database key type. Only supported value is 'AzureKeyVault'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        AzureKeyVault,
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
                Self::AzureKeyVault => serializer.serialize_unit_variant("Type", 0u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Database>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseOperationProperties>,
}
impl DatabaseOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list database operations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseOperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseOperationProperties {
    #[doc = "The name of the database the operation is being performed on."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The name of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The friendly name of operation."]
    #[serde(rename = "operationFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub operation_friendly_name: Option<String>,
    #[doc = "The percentage of the operation completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The name of the server."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The operation start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<database_operation_properties::State>,
    #[doc = "The operation error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The operation error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The operation error severity."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "Whether or not the error is a user error."]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
    #[doc = "The estimated completion time of the operation."]
    #[serde(rename = "estimatedCompletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub estimated_completion_time: Option<time::OffsetDateTime>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the operation can be cancelled."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
    #[doc = "The phase details properties of a database operation."]
    #[serde(rename = "operationPhaseDetails", default, skip_serializing_if = "Option::is_none")]
    pub operation_phase_details: Option<PhaseDetails>,
}
impl DatabaseOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_operation_properties {
    use super::*;
    #[doc = "The operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        InProgress,
        Succeeded,
        Failed,
        CancelInProgress,
        Cancelled,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::CancelInProgress => serializer.serialize_unit_variant("State", 4u32, "CancelInProgress"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 5u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: regular database creation.\r\n\r\nCopy: creates a database as a copy of an existing database. sourceDatabaseId must be specified as the resource ID of the source database.\r\n\r\nSecondary: creates a database as a secondary replica of an existing database. sourceDatabaseId must be specified as the resource ID of the existing primary database.\r\n\r\nPointInTimeRestore: Creates a database by restoring a point in time backup of an existing database. sourceDatabaseId must be specified as the resource ID of the existing database, and restorePointInTime must be specified.\r\n\r\nRecovery: Creates a database by restoring a geo-replicated backup. sourceDatabaseId must be specified as the recoverable database resource ID to restore.\r\n\r\nRestore: Creates a database by restoring a backup of a deleted database. sourceDatabaseId must be specified. If sourceDatabaseId is the database's original resource ID, then sourceDatabaseDeletionDate must be specified. Otherwise sourceDatabaseId must be the restorable dropped database resource ID and sourceDatabaseDeletionDate is ignored. restorePointInTime may also be specified to restore from an earlier point in time.\r\n\r\nRestoreLongTermRetentionBackup: Creates a database by restoring from a long term retention vault. recoveryServicesRecoveryPointResourceId must be specified as the recovery point resource ID.\r\n\r\nCopy, Secondary, and RestoreLongTermRetentionBackup are not supported for DataWarehouse edition."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<database_properties::CreateMode>,
    #[doc = "The collation of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "The max size of the database expressed in bytes."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "The name of the sample schema to apply when creating this database."]
    #[serde(rename = "sampleName", default, skip_serializing_if = "Option::is_none")]
    pub sample_name: Option<database_properties::SampleName>,
    #[doc = "The resource identifier of the elastic pool containing this database."]
    #[serde(rename = "elasticPoolId", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_id: Option<String>,
    #[doc = "The resource identifier of the source database associated with create operation of this database."]
    #[serde(rename = "sourceDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub source_database_id: Option<String>,
    #[doc = "The status of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<database_properties::Status>,
    #[doc = "The ID of the database."]
    #[serde(rename = "databaseId", default, skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[doc = "The creation date of the database (ISO8601 format)."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The current service level objective name of the database."]
    #[serde(rename = "currentServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub current_service_objective_name: Option<String>,
    #[doc = "The requested service level objective name of the database."]
    #[serde(rename = "requestedServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub requested_service_objective_name: Option<String>,
    #[doc = "The default secondary region for this database."]
    #[serde(rename = "defaultSecondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub default_secondary_location: Option<String>,
    #[doc = "Failover Group resource identifier that this database belongs to."]
    #[serde(rename = "failoverGroupId", default, skip_serializing_if = "Option::is_none")]
    pub failover_group_id: Option<String>,
    #[doc = "Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database."]
    #[serde(rename = "restorePointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the time that the database was deleted."]
    #[serde(rename = "sourceDatabaseDeletionDate", default, with = "azure_core::date::rfc3339::option")]
    pub source_database_deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The resource identifier of the recovery point associated with create operation of this database."]
    #[serde(rename = "recoveryServicesRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_recovery_point_id: Option<String>,
    #[doc = "The resource identifier of the long term retention backup associated with create operation of this database."]
    #[serde(rename = "longTermRetentionBackupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub long_term_retention_backup_resource_id: Option<String>,
    #[doc = "The resource identifier of the recoverable database associated with create operation of this database."]
    #[serde(rename = "recoverableDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_database_id: Option<String>,
    #[doc = "The resource identifier of the restorable dropped database associated with create operation of this database."]
    #[serde(rename = "restorableDroppedDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub restorable_dropped_database_id: Option<String>,
    #[doc = "Collation of the metadata catalog."]
    #[serde(rename = "catalogCollation", default, skip_serializing_if = "Option::is_none")]
    pub catalog_collation: Option<database_properties::CatalogCollation>,
    #[doc = "Whether or not this database is zone redundant, which means the replicas of this database will be spread across multiple availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The license type to apply for this database. `LicenseIncluded` if you need a license, or `BasePrice` if you have a license and are eligible for the Azure Hybrid Benefit."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<database_properties::LicenseType>,
    #[doc = "The max log size for this database."]
    #[serde(rename = "maxLogSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_log_size_bytes: Option<i64>,
    #[doc = "This records the earliest start date and time that restore is available for this database (ISO8601 format)."]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region. Not applicable to a Hyperscale database within an elastic pool."]
    #[serde(rename = "readScale", default, skip_serializing_if = "Option::is_none")]
    pub read_scale: Option<database_properties::ReadScale>,
    #[doc = "The number of secondary replicas associated with the database that are used to provide high availability. Not applicable to a Hyperscale database within an elastic pool."]
    #[serde(rename = "highAvailabilityReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_replica_count: Option<i32>,
    #[doc = "The secondary type of the database if it is a secondary.  Valid values are Geo, Named and Standby."]
    #[serde(rename = "secondaryType", default, skip_serializing_if = "Option::is_none")]
    pub secondary_type: Option<database_properties::SecondaryType>,
    #[doc = "An ARM Resource SKU."]
    #[serde(rename = "currentSku", default, skip_serializing_if = "Option::is_none")]
    pub current_sku: Option<Sku>,
    #[doc = "Time in minutes after which database is automatically paused. A value of -1 means that automatic pause is disabled"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
    #[doc = "The storage account type used to store backups for this database."]
    #[serde(rename = "currentBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub current_backup_storage_redundancy: Option<database_properties::CurrentBackupStorageRedundancy>,
    #[doc = "The storage account type to be used to store backups for this database."]
    #[serde(rename = "requestedBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub requested_backup_storage_redundancy: Option<database_properties::RequestedBackupStorageRedundancy>,
    #[doc = "Minimal capacity that database will always have allocated, if not paused"]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "The date when database was paused by user configuration or action(ISO8601 format). Null if the database is ready."]
    #[serde(rename = "pausedDate", default, with = "azure_core::date::rfc3339::option")]
    pub paused_date: Option<time::OffsetDateTime>,
    #[doc = "The date when database was resumed by user action or database login (ISO8601 format). Null if the database is paused."]
    #[serde(rename = "resumedDate", default, with = "azure_core::date::rfc3339::option")]
    pub resumed_date: Option<time::OffsetDateTime>,
    #[doc = "Maintenance configuration id assigned to the database. This configuration defines the period when the maintenance updates will occur."]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "Whether or not this database is a ledger database, which means all tables in the database are ledger tables. Note: the value of this property cannot be changed after the database has been created."]
    #[serde(rename = "isLedgerOn", default, skip_serializing_if = "Option::is_none")]
    pub is_ledger_on: Option<bool>,
    #[doc = "Infra encryption is enabled for this database."]
    #[serde(rename = "isInfraEncryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_infra_encryption_enabled: Option<bool>,
    #[doc = "The Client id used for cross tenant per database CMK scenario"]
    #[serde(rename = "federatedClientId", default, skip_serializing_if = "Option::is_none")]
    pub federated_client_id: Option<String>,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<serde_json::Value>,
    #[doc = "The azure key vault URI of the database if it's configured with per Database Customer Managed Keys."]
    #[serde(rename = "encryptionProtector", default, skip_serializing_if = "Option::is_none")]
    pub encryption_protector: Option<String>,
    #[doc = "Type of enclave requested on the database i.e. Default or VBS enclaves."]
    #[serde(rename = "preferredEnclaveType", default, skip_serializing_if = "Option::is_none")]
    pub preferred_enclave_type: Option<database_properties::PreferredEnclaveType>,
    #[doc = "Whether or not the database uses free monthly limits. Allowed on one database in a subscription."]
    #[serde(rename = "useFreeLimit", default, skip_serializing_if = "Option::is_none")]
    pub use_free_limit: Option<bool>,
    #[doc = "Specifies the behavior when monthly free limits are exhausted for the free database.\r\n\r\nAutoPause: The database will be auto paused upon exhaustion of free limits for remainder of the month.\r\n\r\nBillForUsage: The database will continue to be online upon exhaustion of free limits and any overage will be billed."]
    #[serde(rename = "freeLimitExhaustionBehavior", default, skip_serializing_if = "Option::is_none")]
    pub free_limit_exhaustion_behavior: Option<database_properties::FreeLimitExhaustionBehavior>,
    #[doc = "The resource identifier of the source associated with the create operation of this database.\r\n\r\nThis property is only supported for DataWarehouse edition and allows to restore across subscriptions.\r\n\r\nWhen sourceResourceId is specified, sourceDatabaseId, recoverableDatabaseId, restorableDroppedDatabaseId and sourceDatabaseDeletionDate must not be specified and CreateMode must be PointInTimeRestore, Restore or Recover.\r\n\r\nWhen createMode is PointInTimeRestore, sourceResourceId must be the resource ID of the existing database or existing sql pool, and restorePointInTime must be specified.\r\n\r\nWhen createMode is Restore, sourceResourceId must be the resource ID of restorable dropped database or restorable dropped sql pool.\r\n\r\nWhen createMode is Recover, sourceResourceId must be the resource ID of recoverable database or recoverable sql pool.\r\n\r\nWhen source subscription belongs to a different tenant than target subscription, “x-ms-authorization-auxiliary” header must contain authentication token for the source tenant. For more details about “x-ms-authorization-auxiliary” header see https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/authenticate-multi-tenant "]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "Whether or not customer controlled manual cutover needs to be done during Update Database operation to Hyperscale tier.\r\n\r\nThis property is only applicable when scaling database from Business Critical/General Purpose/Premium/Standard tier to Hyperscale tier.\r\n\r\nWhen manualCutover is specified, the scaling operation will wait for user input to trigger cutover to Hyperscale database.\r\n\r\nTo trigger cutover, please provide 'performCutover' parameter when the Scaling operation is in Waiting state."]
    #[serde(rename = "manualCutover", default, skip_serializing_if = "Option::is_none")]
    pub manual_cutover: Option<bool>,
    #[doc = "To trigger customer controlled manual cutover during the wait state while Scaling operation is in progress.\r\n\r\nThis property parameter is only applicable for scaling operations that are initiated along with 'manualCutover' parameter.\r\n\r\nThis property is only applicable when scaling database from Business Critical/General Purpose/Premium/Standard tier to Hyperscale tier is already in progress.\r\n\r\nWhen performCutover is specified, the scaling operation will trigger cutover and perform role-change to Hyperscale database."]
    #[serde(rename = "performCutover", default, skip_serializing_if = "Option::is_none")]
    pub perform_cutover: Option<bool>,
    #[doc = "Specifies the availability zone the database is pinned to."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<database_properties::AvailabilityZone>,
    #[doc = "The flag to enable or disable auto rotation of database encryption protector AKV key."]
    #[serde(rename = "encryptionProtectorAutoRotation", default, skip_serializing_if = "Option::is_none")]
    pub encryption_protector_auto_rotation: Option<bool>,
}
impl DatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_properties {
    use super::*;
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: regular database creation.\r\n\r\nCopy: creates a database as a copy of an existing database. sourceDatabaseId must be specified as the resource ID of the source database.\r\n\r\nSecondary: creates a database as a secondary replica of an existing database. sourceDatabaseId must be specified as the resource ID of the existing primary database.\r\n\r\nPointInTimeRestore: Creates a database by restoring a point in time backup of an existing database. sourceDatabaseId must be specified as the resource ID of the existing database, and restorePointInTime must be specified.\r\n\r\nRecovery: Creates a database by restoring a geo-replicated backup. sourceDatabaseId must be specified as the recoverable database resource ID to restore.\r\n\r\nRestore: Creates a database by restoring a backup of a deleted database. sourceDatabaseId must be specified. If sourceDatabaseId is the database's original resource ID, then sourceDatabaseDeletionDate must be specified. Otherwise sourceDatabaseId must be the restorable dropped database resource ID and sourceDatabaseDeletionDate is ignored. restorePointInTime may also be specified to restore from an earlier point in time.\r\n\r\nRestoreLongTermRetentionBackup: Creates a database by restoring from a long term retention vault. recoveryServicesRecoveryPointResourceId must be specified as the recovery point resource ID.\r\n\r\nCopy, Secondary, and RestoreLongTermRetentionBackup are not supported for DataWarehouse edition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        Copy,
        Secondary,
        PointInTimeRestore,
        Restore,
        Recovery,
        RestoreExternalBackup,
        RestoreExternalBackupSecondary,
        RestoreLongTermRetentionBackup,
        OnlineSecondary,
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
                Self::Copy => serializer.serialize_unit_variant("CreateMode", 1u32, "Copy"),
                Self::Secondary => serializer.serialize_unit_variant("CreateMode", 2u32, "Secondary"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 3u32, "PointInTimeRestore"),
                Self::Restore => serializer.serialize_unit_variant("CreateMode", 4u32, "Restore"),
                Self::Recovery => serializer.serialize_unit_variant("CreateMode", 5u32, "Recovery"),
                Self::RestoreExternalBackup => serializer.serialize_unit_variant("CreateMode", 6u32, "RestoreExternalBackup"),
                Self::RestoreExternalBackupSecondary => {
                    serializer.serialize_unit_variant("CreateMode", 7u32, "RestoreExternalBackupSecondary")
                }
                Self::RestoreLongTermRetentionBackup => {
                    serializer.serialize_unit_variant("CreateMode", 8u32, "RestoreLongTermRetentionBackup")
                }
                Self::OnlineSecondary => serializer.serialize_unit_variant("CreateMode", 9u32, "OnlineSecondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The name of the sample schema to apply when creating this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SampleName")]
    pub enum SampleName {
        #[serde(rename = "AdventureWorksLT")]
        AdventureWorksLt,
        WideWorldImportersStd,
        WideWorldImportersFull,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SampleName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SampleName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SampleName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AdventureWorksLt => serializer.serialize_unit_variant("SampleName", 0u32, "AdventureWorksLT"),
                Self::WideWorldImportersStd => serializer.serialize_unit_variant("SampleName", 1u32, "WideWorldImportersStd"),
                Self::WideWorldImportersFull => serializer.serialize_unit_variant("SampleName", 2u32, "WideWorldImportersFull"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Online,
        Restoring,
        RecoveryPending,
        Recovering,
        Suspect,
        Offline,
        Standby,
        Shutdown,
        EmergencyMode,
        AutoClosed,
        Copying,
        Creating,
        Inaccessible,
        OfflineSecondary,
        Pausing,
        Paused,
        Resuming,
        Scaling,
        OfflineChangingDwPerformanceTiers,
        OnlineChangingDwPerformanceTiers,
        Disabled,
        Stopping,
        Stopped,
        Starting,
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
                Self::Online => serializer.serialize_unit_variant("Status", 0u32, "Online"),
                Self::Restoring => serializer.serialize_unit_variant("Status", 1u32, "Restoring"),
                Self::RecoveryPending => serializer.serialize_unit_variant("Status", 2u32, "RecoveryPending"),
                Self::Recovering => serializer.serialize_unit_variant("Status", 3u32, "Recovering"),
                Self::Suspect => serializer.serialize_unit_variant("Status", 4u32, "Suspect"),
                Self::Offline => serializer.serialize_unit_variant("Status", 5u32, "Offline"),
                Self::Standby => serializer.serialize_unit_variant("Status", 6u32, "Standby"),
                Self::Shutdown => serializer.serialize_unit_variant("Status", 7u32, "Shutdown"),
                Self::EmergencyMode => serializer.serialize_unit_variant("Status", 8u32, "EmergencyMode"),
                Self::AutoClosed => serializer.serialize_unit_variant("Status", 9u32, "AutoClosed"),
                Self::Copying => serializer.serialize_unit_variant("Status", 10u32, "Copying"),
                Self::Creating => serializer.serialize_unit_variant("Status", 11u32, "Creating"),
                Self::Inaccessible => serializer.serialize_unit_variant("Status", 12u32, "Inaccessible"),
                Self::OfflineSecondary => serializer.serialize_unit_variant("Status", 13u32, "OfflineSecondary"),
                Self::Pausing => serializer.serialize_unit_variant("Status", 14u32, "Pausing"),
                Self::Paused => serializer.serialize_unit_variant("Status", 15u32, "Paused"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 16u32, "Resuming"),
                Self::Scaling => serializer.serialize_unit_variant("Status", 17u32, "Scaling"),
                Self::OfflineChangingDwPerformanceTiers => {
                    serializer.serialize_unit_variant("Status", 18u32, "OfflineChangingDwPerformanceTiers")
                }
                Self::OnlineChangingDwPerformanceTiers => {
                    serializer.serialize_unit_variant("Status", 19u32, "OnlineChangingDwPerformanceTiers")
                }
                Self::Disabled => serializer.serialize_unit_variant("Status", 20u32, "Disabled"),
                Self::Stopping => serializer.serialize_unit_variant("Status", 21u32, "Stopping"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 22u32, "Stopped"),
                Self::Starting => serializer.serialize_unit_variant("Status", 23u32, "Starting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Collation of the metadata catalog."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CatalogCollation")]
    pub enum CatalogCollation {
        #[serde(rename = "DATABASE_DEFAULT")]
        DatabaseDefault,
        #[serde(rename = "SQL_Latin1_General_CP1_CI_AS")]
        SqlLatin1GeneralCp1CiAs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CatalogCollation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CatalogCollation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CatalogCollation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DatabaseDefault => serializer.serialize_unit_variant("CatalogCollation", 0u32, "DATABASE_DEFAULT"),
                Self::SqlLatin1GeneralCp1CiAs => {
                    serializer.serialize_unit_variant("CatalogCollation", 1u32, "SQL_Latin1_General_CP1_CI_AS")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The license type to apply for this database. `LicenseIncluded` if you need a license, or `BasePrice` if you have a license and are eligible for the Azure Hybrid Benefit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        LicenseIncluded,
        BasePrice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 0u32, "LicenseIncluded"),
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 1u32, "BasePrice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region. Not applicable to a Hyperscale database within an elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReadScale")]
    pub enum ReadScale {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReadScale {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReadScale {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReadScale {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ReadScale", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ReadScale", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The secondary type of the database if it is a secondary.  Valid values are Geo, Named and Standby."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecondaryType")]
    pub enum SecondaryType {
        Geo,
        Named,
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecondaryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecondaryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecondaryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("SecondaryType", 0u32, "Geo"),
                Self::Named => serializer.serialize_unit_variant("SecondaryType", 1u32, "Named"),
                Self::Standby => serializer.serialize_unit_variant("SecondaryType", 2u32, "Standby"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentBackupStorageRedundancy")]
    pub enum CurrentBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type to be used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestedBackupStorageRedundancy")]
    pub enum RequestedBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestedBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestedBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestedBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of enclave requested on the database i.e. Default or VBS enclaves."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredEnclaveType")]
    pub enum PreferredEnclaveType {
        Default,
        #[serde(rename = "VBS")]
        Vbs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredEnclaveType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredEnclaveType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredEnclaveType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("PreferredEnclaveType", 0u32, "Default"),
                Self::Vbs => serializer.serialize_unit_variant("PreferredEnclaveType", 1u32, "VBS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the behavior when monthly free limits are exhausted for the free database.\r\n\r\nAutoPause: The database will be auto paused upon exhaustion of free limits for remainder of the month.\r\n\r\nBillForUsage: The database will continue to be online upon exhaustion of free limits and any overage will be billed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FreeLimitExhaustionBehavior")]
    pub enum FreeLimitExhaustionBehavior {
        AutoPause,
        BillOverUsage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FreeLimitExhaustionBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FreeLimitExhaustionBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FreeLimitExhaustionBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoPause => serializer.serialize_unit_variant("FreeLimitExhaustionBehavior", 0u32, "AutoPause"),
                Self::BillOverUsage => serializer.serialize_unit_variant("FreeLimitExhaustionBehavior", 1u32, "BillOverUsage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the availability zone the database is pinned to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityZone")]
    pub enum AvailabilityZone {
        NoPreference,
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
        #[serde(rename = "3")]
        N3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityZone {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityZone {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityZone {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoPreference => serializer.serialize_unit_variant("AvailabilityZone", 0u32, "NoPreference"),
                Self::N1 => serializer.serialize_unit_variant("AvailabilityZone", 1u32, "1"),
                Self::N2 => serializer.serialize_unit_variant("AvailabilityZone", 2u32, "2"),
                Self::N3 => serializer.serialize_unit_variant("AvailabilityZone", 3u32, "3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A database schema resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSchema {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl DatabaseSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of database schemas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSchemaListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseSchema>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseSchemaListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseSchemaListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the database's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSecurityAlertListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseSecurityAlertListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseSecurityAlertListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertsPolicyProperties>,
}
impl DatabaseSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database sql vulnerability assessment baseline set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSqlVulnerabilityAssessmentBaselineSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a database Sql Vulnerability Assessment baseline set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseSqlVulnerabilityAssessmentBaselineSetProperties>,
}
impl DatabaseSqlVulnerabilityAssessmentBaselineSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SQL Vulnerability Assessments baseline set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSqlVulnerabilityAssessmentBaselineSetListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseSqlVulnerabilityAssessmentBaselineSet>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseSqlVulnerabilityAssessmentBaselineSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseSqlVulnerabilityAssessmentBaselineSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database Sql Vulnerability Assessment baseline set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSqlVulnerabilityAssessmentBaselineSetProperties {
    #[doc = "The baseline set result"]
    pub results: serde_json::Value,
}
impl DatabaseSqlVulnerabilityAssessmentBaselineSetProperties {
    pub fn new(results: serde_json::Value) -> Self {
        Self { results }
    }
}
#[doc = "A database sql vulnerability assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaseline {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a database Sql Vulnerability Assessment rule baseline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseSqlVulnerabilityAssessmentRuleBaselineProperties>,
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaseline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database sql vulnerability assessment rule baseline input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaselineInput {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a database Sql Vulnerability Assessment rule baseline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseSqlVulnerabilityAssessmentRuleBaselineInputProperties>,
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaselineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database Sql Vulnerability Assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaselineInputProperties {
    #[doc = "The latest scan flag"]
    #[serde(rename = "latestScan")]
    pub latest_scan: bool,
    #[doc = "The rule baseline result"]
    pub results: Vec<Vec<String>>,
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaselineInputProperties {
    pub fn new(latest_scan: bool, results: Vec<Vec<String>>) -> Self {
        Self { latest_scan, results }
    }
}
#[doc = "A database sql vulnerability assessment rule baseline list input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaselineListInput {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a database Sql Vulnerability Assessment rule baseline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseSqlVulnerabilityAssessmentRuleBaselineListInputProperties>,
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaselineListInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database Sql Vulnerability Assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaselineListInputProperties {
    #[doc = "The latest scan flag"]
    #[serde(rename = "latestScan")]
    pub latest_scan: bool,
    #[doc = "The rule baseline result list"]
    pub results: serde_json::Value,
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaselineListInputProperties {
    pub fn new(latest_scan: bool, results: serde_json::Value) -> Self {
        Self { latest_scan, results }
    }
}
#[doc = "A list of SQL Vulnerability Assessments rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaselineListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseSqlVulnerabilityAssessmentRuleBaseline>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseSqlVulnerabilityAssessmentRuleBaselineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaselineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database Sql Vulnerability Assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSqlVulnerabilityAssessmentRuleBaselineProperties {
    #[doc = "The rule baseline result"]
    pub results: Vec<Vec<String>>,
}
impl DatabaseSqlVulnerabilityAssessmentRuleBaselineProperties {
    pub fn new(results: Vec<Vec<String>>) -> Self {
        Self { results }
    }
}
#[doc = "A database table resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseTable {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Database table properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseTableProperties>,
}
impl DatabaseTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of database tables."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseTableListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseTable>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseTableListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseTableListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database table properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseTableProperties {
    #[doc = "The table temporal type."]
    #[serde(rename = "temporalType", default, skip_serializing_if = "Option::is_none")]
    pub temporal_type: Option<database_table_properties::TemporalType>,
    #[doc = "Whether or not the table is memory optimized."]
    #[serde(rename = "memoryOptimized", default, skip_serializing_if = "Option::is_none")]
    pub memory_optimized: Option<bool>,
}
impl DatabaseTableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_table_properties {
    use super::*;
    #[doc = "The table temporal type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TemporalType")]
    pub enum TemporalType {
        NonTemporalTable,
        HistoryTable,
        SystemVersionedTemporalTable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TemporalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TemporalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TemporalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NonTemporalTable => serializer.serialize_unit_variant("TemporalType", 0u32, "NonTemporalTable"),
                Self::HistoryTable => serializer.serialize_unit_variant("TemporalType", 1u32, "HistoryTable"),
                Self::SystemVersionedTemporalTable => {
                    serializer.serialize_unit_variant("TemporalType", 2u32, "SystemVersionedTemporalTable")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A database update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUpdate {
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DatabaseIdentity>,
    #[doc = "A database update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUpdateProperties {
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: regular database creation.\r\n\r\nCopy: creates a database as a copy of an existing database. sourceDatabaseId must be specified as the resource ID of the source database.\r\n\r\nSecondary: creates a database as a secondary replica of an existing database. sourceDatabaseId must be specified as the resource ID of the existing primary database.\r\n\r\nPointInTimeRestore: Creates a database by restoring a point in time backup of an existing database. sourceDatabaseId must be specified as the resource ID of the existing database, and restorePointInTime must be specified.\r\n\r\nRecovery: Creates a database by restoring a geo-replicated backup. sourceDatabaseId must be specified as the recoverable database resource ID to restore.\r\n\r\nRestore: Creates a database by restoring a backup of a deleted database. sourceDatabaseId must be specified. If sourceDatabaseId is the database's original resource ID, then sourceDatabaseDeletionDate must be specified. Otherwise sourceDatabaseId must be the restorable dropped database resource ID and sourceDatabaseDeletionDate is ignored. restorePointInTime may also be specified to restore from an earlier point in time.\r\n\r\nRestoreLongTermRetentionBackup: Creates a database by restoring from a long term retention vault. recoveryServicesRecoveryPointResourceId must be specified as the recovery point resource ID.\r\n\r\nCopy, Secondary, and RestoreLongTermRetentionBackup are not supported for DataWarehouse edition."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<database_update_properties::CreateMode>,
    #[doc = "The collation of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "The max size of the database expressed in bytes."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "The name of the sample schema to apply when creating this database."]
    #[serde(rename = "sampleName", default, skip_serializing_if = "Option::is_none")]
    pub sample_name: Option<database_update_properties::SampleName>,
    #[doc = "The resource identifier of the elastic pool containing this database."]
    #[serde(rename = "elasticPoolId", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_id: Option<String>,
    #[doc = "The resource identifier of the source database associated with create operation of this database."]
    #[serde(rename = "sourceDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub source_database_id: Option<String>,
    #[doc = "The status of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<database_update_properties::Status>,
    #[doc = "The ID of the database."]
    #[serde(rename = "databaseId", default, skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[doc = "The creation date of the database (ISO8601 format)."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The current service level objective name of the database."]
    #[serde(rename = "currentServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub current_service_objective_name: Option<String>,
    #[doc = "The requested service level objective name of the database."]
    #[serde(rename = "requestedServiceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub requested_service_objective_name: Option<String>,
    #[doc = "The default secondary region for this database."]
    #[serde(rename = "defaultSecondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub default_secondary_location: Option<String>,
    #[doc = "Failover Group resource identifier that this database belongs to."]
    #[serde(rename = "failoverGroupId", default, skip_serializing_if = "Option::is_none")]
    pub failover_group_id: Option<String>,
    #[doc = "Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database."]
    #[serde(rename = "restorePointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the time that the database was deleted."]
    #[serde(rename = "sourceDatabaseDeletionDate", default, with = "azure_core::date::rfc3339::option")]
    pub source_database_deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The resource identifier of the recovery point associated with create operation of this database."]
    #[serde(rename = "recoveryServicesRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_recovery_point_id: Option<String>,
    #[doc = "The resource identifier of the long term retention backup associated with create operation of this database."]
    #[serde(rename = "longTermRetentionBackupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub long_term_retention_backup_resource_id: Option<String>,
    #[doc = "The resource identifier of the recoverable database associated with create operation of this database."]
    #[serde(rename = "recoverableDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_database_id: Option<String>,
    #[doc = "The resource identifier of the restorable dropped database associated with create operation of this database."]
    #[serde(rename = "restorableDroppedDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub restorable_dropped_database_id: Option<String>,
    #[doc = "Collation of the metadata catalog."]
    #[serde(rename = "catalogCollation", default, skip_serializing_if = "Option::is_none")]
    pub catalog_collation: Option<database_update_properties::CatalogCollation>,
    #[doc = "Whether or not this database is zone redundant, which means the replicas of this database will be spread across multiple availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The license type to apply for this database. `LicenseIncluded` if you need a license, or `BasePrice` if you have a license and are eligible for the Azure Hybrid Benefit."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<database_update_properties::LicenseType>,
    #[doc = "The max log size for this database."]
    #[serde(rename = "maxLogSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_log_size_bytes: Option<i64>,
    #[doc = "This records the earliest start date and time that restore is available for this database (ISO8601 format)."]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region. Not applicable to a Hyperscale database within an elastic pool."]
    #[serde(rename = "readScale", default, skip_serializing_if = "Option::is_none")]
    pub read_scale: Option<database_update_properties::ReadScale>,
    #[doc = "The number of secondary replicas associated with the database that are used to provide high availability. Not applicable to a Hyperscale database within an elastic pool."]
    #[serde(rename = "highAvailabilityReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_replica_count: Option<i32>,
    #[doc = "The secondary type of the database if it is a secondary.  Valid values are Geo, Named and Standby."]
    #[serde(rename = "secondaryType", default, skip_serializing_if = "Option::is_none")]
    pub secondary_type: Option<database_update_properties::SecondaryType>,
    #[doc = "An ARM Resource SKU."]
    #[serde(rename = "currentSku", default, skip_serializing_if = "Option::is_none")]
    pub current_sku: Option<Sku>,
    #[doc = "Time in minutes after which database is automatically paused. A value of -1 means that automatic pause is disabled"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
    #[doc = "The storage account type used to store backups for this database."]
    #[serde(rename = "currentBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub current_backup_storage_redundancy: Option<database_update_properties::CurrentBackupStorageRedundancy>,
    #[doc = "The storage account type to be used to store backups for this database."]
    #[serde(rename = "requestedBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub requested_backup_storage_redundancy: Option<database_update_properties::RequestedBackupStorageRedundancy>,
    #[doc = "Minimal capacity that database will always have allocated, if not paused"]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "The date when database was paused by user configuration or action(ISO8601 format). Null if the database is ready."]
    #[serde(rename = "pausedDate", default, with = "azure_core::date::rfc3339::option")]
    pub paused_date: Option<time::OffsetDateTime>,
    #[doc = "The date when database was resumed by user action or database login (ISO8601 format). Null if the database is paused."]
    #[serde(rename = "resumedDate", default, with = "azure_core::date::rfc3339::option")]
    pub resumed_date: Option<time::OffsetDateTime>,
    #[doc = "Maintenance configuration id assigned to the database. This configuration defines the period when the maintenance updates will occur."]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "Whether or not this database is a ledger database, which means all tables in the database are ledger tables. Note: the value of this property cannot be changed after the database has been created."]
    #[serde(rename = "isLedgerOn", default, skip_serializing_if = "Option::is_none")]
    pub is_ledger_on: Option<bool>,
    #[doc = "Infra encryption is enabled for this database."]
    #[serde(rename = "isInfraEncryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_infra_encryption_enabled: Option<bool>,
    #[doc = "The Client id used for cross tenant per database CMK scenario"]
    #[serde(rename = "federatedClientId", default, skip_serializing_if = "Option::is_none")]
    pub federated_client_id: Option<String>,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<serde_json::Value>,
    #[doc = "The azure key vault URI of the database if it's configured with per Database Customer Managed Keys."]
    #[serde(rename = "encryptionProtector", default, skip_serializing_if = "Option::is_none")]
    pub encryption_protector: Option<String>,
    #[doc = "Type of enclave requested on the database i.e. Default or VBS enclaves."]
    #[serde(rename = "preferredEnclaveType", default, skip_serializing_if = "Option::is_none")]
    pub preferred_enclave_type: Option<database_update_properties::PreferredEnclaveType>,
    #[doc = "Whether or not the database uses free monthly limits. Allowed on one database in a subscription."]
    #[serde(rename = "useFreeLimit", default, skip_serializing_if = "Option::is_none")]
    pub use_free_limit: Option<bool>,
    #[doc = "Specifies the behavior when monthly free limits are exhausted for the free database.\r\n\r\nAutoPause: The database will be auto paused upon exhaustion of free limits for remainder of the month.\r\n\r\nBillForUsage: The database will continue to be online upon exhaustion of free limits and any overage will be billed."]
    #[serde(rename = "freeLimitExhaustionBehavior", default, skip_serializing_if = "Option::is_none")]
    pub free_limit_exhaustion_behavior: Option<database_update_properties::FreeLimitExhaustionBehavior>,
    #[doc = "Whether or not customer controlled manual cutover needs to be done during Update Database operation to Hyperscale tier.\r\n\r\nThis property is only applicable when scaling database from Business Critical/General Purpose/Premium/Standard tier to Hyperscale tier.\r\n\r\nWhen manualCutover is specified, the scaling operation will wait for user input to trigger cutover to Hyperscale database.\r\n\r\nTo trigger cutover, please provide 'performCutover' parameter when the Scaling operation is in Waiting state."]
    #[serde(rename = "manualCutover", default, skip_serializing_if = "Option::is_none")]
    pub manual_cutover: Option<bool>,
    #[doc = "To trigger customer controlled manual cutover during the wait state while Scaling operation is in progress.\r\n\r\nThis property parameter is only applicable for scaling operations that are initiated along with 'manualCutover' parameter.\r\n\r\nThis property is only applicable when scaling database from Business Critical/General Purpose/Premium/Standard tier to Hyperscale tier is already in progress.\r\n\r\nWhen performCutover is specified, the scaling operation will trigger cutover and perform role-change to Hyperscale database."]
    #[serde(rename = "performCutover", default, skip_serializing_if = "Option::is_none")]
    pub perform_cutover: Option<bool>,
    #[doc = "The flag to enable or disable auto rotation of database encryption protector AKV key."]
    #[serde(rename = "encryptionProtectorAutoRotation", default, skip_serializing_if = "Option::is_none")]
    pub encryption_protector_auto_rotation: Option<bool>,
}
impl DatabaseUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_update_properties {
    use super::*;
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: regular database creation.\r\n\r\nCopy: creates a database as a copy of an existing database. sourceDatabaseId must be specified as the resource ID of the source database.\r\n\r\nSecondary: creates a database as a secondary replica of an existing database. sourceDatabaseId must be specified as the resource ID of the existing primary database.\r\n\r\nPointInTimeRestore: Creates a database by restoring a point in time backup of an existing database. sourceDatabaseId must be specified as the resource ID of the existing database, and restorePointInTime must be specified.\r\n\r\nRecovery: Creates a database by restoring a geo-replicated backup. sourceDatabaseId must be specified as the recoverable database resource ID to restore.\r\n\r\nRestore: Creates a database by restoring a backup of a deleted database. sourceDatabaseId must be specified. If sourceDatabaseId is the database's original resource ID, then sourceDatabaseDeletionDate must be specified. Otherwise sourceDatabaseId must be the restorable dropped database resource ID and sourceDatabaseDeletionDate is ignored. restorePointInTime may also be specified to restore from an earlier point in time.\r\n\r\nRestoreLongTermRetentionBackup: Creates a database by restoring from a long term retention vault. recoveryServicesRecoveryPointResourceId must be specified as the recovery point resource ID.\r\n\r\nCopy, Secondary, and RestoreLongTermRetentionBackup are not supported for DataWarehouse edition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        Copy,
        Secondary,
        PointInTimeRestore,
        Restore,
        Recovery,
        RestoreExternalBackup,
        RestoreExternalBackupSecondary,
        RestoreLongTermRetentionBackup,
        OnlineSecondary,
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
                Self::Copy => serializer.serialize_unit_variant("CreateMode", 1u32, "Copy"),
                Self::Secondary => serializer.serialize_unit_variant("CreateMode", 2u32, "Secondary"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 3u32, "PointInTimeRestore"),
                Self::Restore => serializer.serialize_unit_variant("CreateMode", 4u32, "Restore"),
                Self::Recovery => serializer.serialize_unit_variant("CreateMode", 5u32, "Recovery"),
                Self::RestoreExternalBackup => serializer.serialize_unit_variant("CreateMode", 6u32, "RestoreExternalBackup"),
                Self::RestoreExternalBackupSecondary => {
                    serializer.serialize_unit_variant("CreateMode", 7u32, "RestoreExternalBackupSecondary")
                }
                Self::RestoreLongTermRetentionBackup => {
                    serializer.serialize_unit_variant("CreateMode", 8u32, "RestoreLongTermRetentionBackup")
                }
                Self::OnlineSecondary => serializer.serialize_unit_variant("CreateMode", 9u32, "OnlineSecondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The name of the sample schema to apply when creating this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SampleName")]
    pub enum SampleName {
        #[serde(rename = "AdventureWorksLT")]
        AdventureWorksLt,
        WideWorldImportersStd,
        WideWorldImportersFull,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SampleName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SampleName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SampleName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AdventureWorksLt => serializer.serialize_unit_variant("SampleName", 0u32, "AdventureWorksLT"),
                Self::WideWorldImportersStd => serializer.serialize_unit_variant("SampleName", 1u32, "WideWorldImportersStd"),
                Self::WideWorldImportersFull => serializer.serialize_unit_variant("SampleName", 2u32, "WideWorldImportersFull"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Online,
        Restoring,
        RecoveryPending,
        Recovering,
        Suspect,
        Offline,
        Standby,
        Shutdown,
        EmergencyMode,
        AutoClosed,
        Copying,
        Creating,
        Inaccessible,
        OfflineSecondary,
        Pausing,
        Paused,
        Resuming,
        Scaling,
        OfflineChangingDwPerformanceTiers,
        OnlineChangingDwPerformanceTiers,
        Disabled,
        Stopping,
        Stopped,
        Starting,
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
                Self::Online => serializer.serialize_unit_variant("Status", 0u32, "Online"),
                Self::Restoring => serializer.serialize_unit_variant("Status", 1u32, "Restoring"),
                Self::RecoveryPending => serializer.serialize_unit_variant("Status", 2u32, "RecoveryPending"),
                Self::Recovering => serializer.serialize_unit_variant("Status", 3u32, "Recovering"),
                Self::Suspect => serializer.serialize_unit_variant("Status", 4u32, "Suspect"),
                Self::Offline => serializer.serialize_unit_variant("Status", 5u32, "Offline"),
                Self::Standby => serializer.serialize_unit_variant("Status", 6u32, "Standby"),
                Self::Shutdown => serializer.serialize_unit_variant("Status", 7u32, "Shutdown"),
                Self::EmergencyMode => serializer.serialize_unit_variant("Status", 8u32, "EmergencyMode"),
                Self::AutoClosed => serializer.serialize_unit_variant("Status", 9u32, "AutoClosed"),
                Self::Copying => serializer.serialize_unit_variant("Status", 10u32, "Copying"),
                Self::Creating => serializer.serialize_unit_variant("Status", 11u32, "Creating"),
                Self::Inaccessible => serializer.serialize_unit_variant("Status", 12u32, "Inaccessible"),
                Self::OfflineSecondary => serializer.serialize_unit_variant("Status", 13u32, "OfflineSecondary"),
                Self::Pausing => serializer.serialize_unit_variant("Status", 14u32, "Pausing"),
                Self::Paused => serializer.serialize_unit_variant("Status", 15u32, "Paused"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 16u32, "Resuming"),
                Self::Scaling => serializer.serialize_unit_variant("Status", 17u32, "Scaling"),
                Self::OfflineChangingDwPerformanceTiers => {
                    serializer.serialize_unit_variant("Status", 18u32, "OfflineChangingDwPerformanceTiers")
                }
                Self::OnlineChangingDwPerformanceTiers => {
                    serializer.serialize_unit_variant("Status", 19u32, "OnlineChangingDwPerformanceTiers")
                }
                Self::Disabled => serializer.serialize_unit_variant("Status", 20u32, "Disabled"),
                Self::Stopping => serializer.serialize_unit_variant("Status", 21u32, "Stopping"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 22u32, "Stopped"),
                Self::Starting => serializer.serialize_unit_variant("Status", 23u32, "Starting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Collation of the metadata catalog."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CatalogCollation")]
    pub enum CatalogCollation {
        #[serde(rename = "DATABASE_DEFAULT")]
        DatabaseDefault,
        #[serde(rename = "SQL_Latin1_General_CP1_CI_AS")]
        SqlLatin1GeneralCp1CiAs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CatalogCollation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CatalogCollation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CatalogCollation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DatabaseDefault => serializer.serialize_unit_variant("CatalogCollation", 0u32, "DATABASE_DEFAULT"),
                Self::SqlLatin1GeneralCp1CiAs => {
                    serializer.serialize_unit_variant("CatalogCollation", 1u32, "SQL_Latin1_General_CP1_CI_AS")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The license type to apply for this database. `LicenseIncluded` if you need a license, or `BasePrice` if you have a license and are eligible for the Azure Hybrid Benefit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        LicenseIncluded,
        BasePrice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 0u32, "LicenseIncluded"),
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 1u32, "BasePrice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region. Not applicable to a Hyperscale database within an elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReadScale")]
    pub enum ReadScale {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReadScale {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReadScale {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReadScale {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ReadScale", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ReadScale", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The secondary type of the database if it is a secondary.  Valid values are Geo, Named and Standby."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecondaryType")]
    pub enum SecondaryType {
        Geo,
        Named,
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecondaryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecondaryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecondaryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("SecondaryType", 0u32, "Geo"),
                Self::Named => serializer.serialize_unit_variant("SecondaryType", 1u32, "Named"),
                Self::Standby => serializer.serialize_unit_variant("SecondaryType", 2u32, "Standby"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentBackupStorageRedundancy")]
    pub enum CurrentBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type to be used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestedBackupStorageRedundancy")]
    pub enum RequestedBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestedBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestedBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestedBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of enclave requested on the database i.e. Default or VBS enclaves."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredEnclaveType")]
    pub enum PreferredEnclaveType {
        Default,
        #[serde(rename = "VBS")]
        Vbs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredEnclaveType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredEnclaveType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredEnclaveType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("PreferredEnclaveType", 0u32, "Default"),
                Self::Vbs => serializer.serialize_unit_variant("PreferredEnclaveType", 1u32, "VBS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the behavior when monthly free limits are exhausted for the free database.\r\n\r\nAutoPause: The database will be auto paused upon exhaustion of free limits for remainder of the month.\r\n\r\nBillForUsage: The database will continue to be online upon exhaustion of free limits and any overage will be billed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FreeLimitExhaustionBehavior")]
    pub enum FreeLimitExhaustionBehavior {
        AutoPause,
        BillOverUsage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FreeLimitExhaustionBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FreeLimitExhaustionBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FreeLimitExhaustionBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoPause => serializer.serialize_unit_variant("FreeLimitExhaustionBehavior", 0u32, "AutoPause"),
                Self::BillOverUsage => serializer.serialize_unit_variant("FreeLimitExhaustionBehavior", 1u32, "BillOverUsage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Usage metric of a database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUsage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a database usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseUsageProperties>,
}
impl DatabaseUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of database usage metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUsageListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseUsage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUsageProperties {
    #[doc = "User-readable name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "Boundary value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl DatabaseUsageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUserIdentity {
    #[doc = "The Azure Active Directory principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Azure Active Directory client id."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl DatabaseUserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database vulnerability assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVulnerabilityAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a database Vulnerability Assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseVulnerabilityAssessmentProperties>,
}
impl DatabaseVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the database's vulnerability assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVulnerabilityAssessmentListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DatabaseVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DatabaseVulnerabilityAssessmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database Vulnerability Assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVulnerabilityAssessmentProperties {
    #[doc = "A blob storage container path to hold the scan results (e.g. https://myStorage.blob.core.windows.net/VaScans/).  It is required if server level vulnerability assessment policy doesn't set"]
    #[serde(rename = "storageContainerPath", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_path: Option<String>,
    #[doc = "A shared access signature (SAS Key) that has write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required. Applies only if the storage account is not behind a Vnet or a firewall"]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required. Applies only if the storage account is not behind a Vnet or a firewall"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Properties of a Vulnerability Assessment recurring scans."]
    #[serde(rename = "recurringScans", default, skip_serializing_if = "Option::is_none")]
    pub recurring_scans: Option<VulnerabilityAssessmentRecurringScansProperties>,
}
impl DatabaseVulnerabilityAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database vulnerability assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVulnerabilityAssessmentRuleBaseline {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a database Vulnerability Assessment rule baseline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseVulnerabilityAssessmentRuleBaselineProperties>,
}
impl DatabaseVulnerabilityAssessmentRuleBaseline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for an Azure SQL Database Vulnerability Assessment rule baseline's result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseVulnerabilityAssessmentRuleBaselineItem {
    #[doc = "The rule baseline result"]
    pub result: Vec<String>,
}
impl DatabaseVulnerabilityAssessmentRuleBaselineItem {
    pub fn new(result: Vec<String>) -> Self {
        Self { result }
    }
}
#[doc = "Properties of a database Vulnerability Assessment rule baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseVulnerabilityAssessmentRuleBaselineProperties {
    #[doc = "The rule baseline result"]
    #[serde(rename = "baselineResults")]
    pub baseline_results: Vec<DatabaseVulnerabilityAssessmentRuleBaselineItem>,
}
impl DatabaseVulnerabilityAssessmentRuleBaselineProperties {
    pub fn new(baseline_results: Vec<DatabaseVulnerabilityAssessmentRuleBaselineItem>) -> Self {
        Self { baseline_results }
    }
}
#[doc = "Properties of the export operation's result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVulnerabilityAssessmentScanExportProperties {
    #[doc = "Location of the exported report (e.g. https://myStorage.blob.core.windows.net/VaScans/scans/serverName/databaseName/scan_scanId.xlsx)."]
    #[serde(rename = "exportedReportLocation", default, skip_serializing_if = "Option::is_none")]
    pub exported_report_location: Option<String>,
}
impl DatabaseVulnerabilityAssessmentScanExportProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database Vulnerability Assessment scan export resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseVulnerabilityAssessmentScansExport {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the export operation's result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseVulnerabilityAssessmentScanExportProperties>,
}
impl DatabaseVulnerabilityAssessmentScansExport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A deleted server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a deleted server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedServerProperties>,
}
impl DeletedServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of deleted servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedServerListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedServer>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeletedServerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a deleted server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedServerProperties {
    #[doc = "The version of the deleted server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The deletion time of the deleted server."]
    #[serde(rename = "deletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub deletion_time: Option<time::OffsetDateTime>,
    #[doc = "The original ID of the server before deletion."]
    #[serde(rename = "originalId", default, skip_serializing_if = "Option::is_none")]
    pub original_id: Option<String>,
    #[doc = "The fully qualified domain name of the server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
}
impl DeletedServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Distributed availability group between box and Sql Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a distributed availability group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DistributedAvailabilityGroupProperties>,
}
impl DistributedAvailabilityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database specific information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroupDatabase {
    #[doc = "The name of the database in link"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Managed instance replica id"]
    #[serde(rename = "instanceReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub instance_replica_id: Option<String>,
    #[doc = "SQL server replica id"]
    #[serde(rename = "partnerReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub partner_replica_id: Option<String>,
    #[doc = "Current link state"]
    #[serde(rename = "replicaState", default, skip_serializing_if = "Option::is_none")]
    pub replica_state: Option<String>,
    #[doc = "Link health state"]
    #[serde(rename = "synchronizationHealth", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_health: Option<distributed_availability_group_database::SynchronizationHealth>,
    #[doc = "Link connected state"]
    #[serde(rename = "connectedState", default, skip_serializing_if = "Option::is_none")]
    pub connected_state: Option<distributed_availability_group_database::ConnectedState>,
    #[doc = "Last received LSN"]
    #[serde(rename = "lastReceivedLsn", default, skip_serializing_if = "Option::is_none")]
    pub last_received_lsn: Option<String>,
    #[doc = "Last received LSN time"]
    #[serde(rename = "lastReceivedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_received_time: Option<time::OffsetDateTime>,
    #[doc = "Last sent LSN"]
    #[serde(rename = "lastSentLsn", default, skip_serializing_if = "Option::is_none")]
    pub last_sent_lsn: Option<String>,
    #[doc = "Last sent LSN time"]
    #[serde(rename = "lastSentTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_sent_time: Option<time::OffsetDateTime>,
    #[doc = "Last commit LSN"]
    #[serde(rename = "lastCommitLsn", default, skip_serializing_if = "Option::is_none")]
    pub last_commit_lsn: Option<String>,
    #[doc = "Last commit LSN time"]
    #[serde(rename = "lastCommitTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_commit_time: Option<time::OffsetDateTime>,
    #[doc = "Last hardened LSN"]
    #[serde(rename = "lastHardenedLsn", default, skip_serializing_if = "Option::is_none")]
    pub last_hardened_lsn: Option<String>,
    #[doc = "Last hardened LSN time"]
    #[serde(rename = "lastHardenedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_hardened_time: Option<time::OffsetDateTime>,
    #[doc = "Last backup LSN"]
    #[serde(rename = "lastBackupLsn", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_lsn: Option<String>,
    #[doc = "Last backup LSN time"]
    #[serde(rename = "lastBackupTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_backup_time: Option<time::OffsetDateTime>,
    #[doc = "The most recent link connection error description"]
    #[serde(rename = "mostRecentLinkError", default, skip_serializing_if = "Option::is_none")]
    pub most_recent_link_error: Option<String>,
    #[doc = "Certificate information"]
    #[serde(rename = "partnerAuthCertValidity", default, skip_serializing_if = "Option::is_none")]
    pub partner_auth_cert_validity: Option<CertificateInfo>,
    #[doc = "Replication lag when Managed Instance link side is primary"]
    #[serde(rename = "instanceSendReplicationLagSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_send_replication_lag_seconds: Option<i32>,
    #[doc = "Redo lag when Managed Instance link side is primary"]
    #[serde(rename = "instanceRedoReplicationLagSeconds", default, skip_serializing_if = "Option::is_none")]
    pub instance_redo_replication_lag_seconds: Option<i32>,
}
impl DistributedAvailabilityGroupDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod distributed_availability_group_database {
    use super::*;
    #[doc = "Link health state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SynchronizationHealth")]
    pub enum SynchronizationHealth {
        #[serde(rename = "NOT_HEALTHY")]
        NotHealthy,
        #[serde(rename = "PARTIALLY_HEALTHY")]
        PartiallyHealthy,
        #[serde(rename = "HEALTHY")]
        Healthy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SynchronizationHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SynchronizationHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SynchronizationHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotHealthy => serializer.serialize_unit_variant("SynchronizationHealth", 0u32, "NOT_HEALTHY"),
                Self::PartiallyHealthy => serializer.serialize_unit_variant("SynchronizationHealth", 1u32, "PARTIALLY_HEALTHY"),
                Self::Healthy => serializer.serialize_unit_variant("SynchronizationHealth", 2u32, "HEALTHY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Link connected state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectedState")]
    pub enum ConnectedState {
        #[serde(rename = "DISCONNECTED")]
        Disconnected,
        #[serde(rename = "CONNECTED")]
        Connected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectedState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectedState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectedState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disconnected => serializer.serialize_unit_variant("ConnectedState", 0u32, "DISCONNECTED"),
                Self::Connected => serializer.serialize_unit_variant("ConnectedState", 1u32, "CONNECTED"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a distributed availability group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroupProperties {
    #[doc = "Name of the distributed availability group"]
    #[serde(rename = "distributedAvailabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub distributed_availability_group_name: Option<String>,
    #[doc = "ID of the distributed availability group"]
    #[serde(rename = "distributedAvailabilityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub distributed_availability_group_id: Option<String>,
    #[doc = "Replication mode of the link"]
    #[serde(rename = "replicationMode", default, skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<distributed_availability_group_properties::ReplicationMode>,
    #[doc = "SQL server side link role"]
    #[serde(rename = "partnerLinkRole", default, skip_serializing_if = "Option::is_none")]
    pub partner_link_role: Option<distributed_availability_group_properties::PartnerLinkRole>,
    #[doc = "SQL server side availability group name"]
    #[serde(rename = "partnerAvailabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub partner_availability_group_name: Option<String>,
    #[doc = "SQL server side endpoint - IP or DNS resolvable name"]
    #[serde(rename = "partnerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub partner_endpoint: Option<String>,
    #[doc = "Managed instance side link role"]
    #[serde(rename = "instanceLinkRole", default, skip_serializing_if = "Option::is_none")]
    pub instance_link_role: Option<distributed_availability_group_properties::InstanceLinkRole>,
    #[doc = "Managed instance side availability group name"]
    #[serde(rename = "instanceAvailabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub instance_availability_group_name: Option<String>,
    #[doc = "The link failover mode - can be Manual if intended to be used for two-way failover with a supported SQL Server, or None for one-way failover to Azure."]
    #[serde(rename = "failoverMode", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<distributed_availability_group_properties::FailoverMode>,
    #[doc = "Database seeding mode – can be Automatic (default), or Manual for supported scenarios."]
    #[serde(rename = "seedingMode", default, skip_serializing_if = "Option::is_none")]
    pub seeding_mode: Option<distributed_availability_group_properties::SeedingMode>,
    #[doc = "Databases in the distributed availability group"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<DistributedAvailabilityGroupDatabase>,
}
impl DistributedAvailabilityGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod distributed_availability_group_properties {
    use super::*;
    #[doc = "Replication mode of the link"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationMode")]
    pub enum ReplicationMode {
        Async,
        Sync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Async => serializer.serialize_unit_variant("ReplicationMode", 0u32, "Async"),
                Self::Sync => serializer.serialize_unit_variant("ReplicationMode", 1u32, "Sync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL server side link role"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PartnerLinkRole")]
    pub enum PartnerLinkRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PartnerLinkRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PartnerLinkRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PartnerLinkRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("PartnerLinkRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("PartnerLinkRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Managed instance side link role"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstanceLinkRole")]
    pub enum InstanceLinkRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstanceLinkRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstanceLinkRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstanceLinkRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("InstanceLinkRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("InstanceLinkRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The link failover mode - can be Manual if intended to be used for two-way failover with a supported SQL Server, or None for one-way failover to Azure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverMode")]
    pub enum FailoverMode {
        None,
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("FailoverMode", 0u32, "None"),
                Self::Manual => serializer.serialize_unit_variant("FailoverMode", 1u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Database seeding mode – can be Automatic (default), or Manual for supported scenarios."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SeedingMode")]
    pub enum SeedingMode {
        Automatic,
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SeedingMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SeedingMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SeedingMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("SeedingMode", 0u32, "Automatic"),
                Self::Manual => serializer.serialize_unit_variant("SeedingMode", 1u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Distributed availability group failover request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributedAvailabilityGroupSetRole {
    #[doc = "New role of managed instance in a distributed availability group, can be Primary or Secondary."]
    #[serde(rename = "instanceRole")]
    pub instance_role: distributed_availability_group_set_role::InstanceRole,
    #[doc = "The type of the role change, can be Planned or Forced."]
    #[serde(rename = "roleChangeType")]
    pub role_change_type: distributed_availability_group_set_role::RoleChangeType,
}
impl DistributedAvailabilityGroupSetRole {
    pub fn new(
        instance_role: distributed_availability_group_set_role::InstanceRole,
        role_change_type: distributed_availability_group_set_role::RoleChangeType,
    ) -> Self {
        Self {
            instance_role,
            role_change_type,
        }
    }
}
pub mod distributed_availability_group_set_role {
    use super::*;
    #[doc = "New role of managed instance in a distributed availability group, can be Primary or Secondary."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstanceRole")]
    pub enum InstanceRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstanceRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstanceRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstanceRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("InstanceRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("InstanceRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the role change, can be Planned or Forced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleChangeType")]
    pub enum RoleChangeType {
        Forced,
        Planned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleChangeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleChangeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleChangeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Forced => serializer.serialize_unit_variant("RoleChangeType", 0u32, "Forced"),
                Self::Planned => serializer.serialize_unit_variant("RoleChangeType", 1u32, "Planned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of distributed availability groups in instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroupsListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DistributedAvailabilityGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DistributedAvailabilityGroupsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DistributedAvailabilityGroupsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The edition capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EditionCapability {
    #[doc = "The database edition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported service objectives for the edition."]
    #[serde(
        rename = "supportedServiceLevelObjectives",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_service_level_objectives: Vec<ServiceObjectiveCapability>,
    #[doc = "Whether or not zone redundancy is supported for the edition."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The read scale capability."]
    #[serde(rename = "readScale", default, skip_serializing_if = "Option::is_none")]
    pub read_scale: Option<ReadScaleCapability>,
    #[doc = "The list of supported storage capabilities for this edition"]
    #[serde(
        rename = "supportedStorageCapabilities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_capabilities: Vec<StorageCapability>,
    #[doc = "Whether or not zone pinning is supported for the edition."]
    #[serde(rename = "zonePinning", default, skip_serializing_if = "Option::is_none")]
    pub zone_pinning: Option<bool>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<edition_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl EditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod edition_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "An elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Kind of elastic pool. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties of an elastic pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ElasticPoolProperties>,
}
impl ElasticPool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            kind: None,
            properties: None,
        }
    }
}
#[doc = "The elastic pool edition capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolEditionCapability {
    #[doc = "The elastic pool edition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported elastic pool DTU levels for the edition."]
    #[serde(
        rename = "supportedElasticPoolPerformanceLevels",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_elastic_pool_performance_levels: Vec<ElasticPoolPerformanceLevelCapability>,
    #[doc = "Whether or not zone redundancy is supported for the edition."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "Whether or not zone pinning is supported for the edition."]
    #[serde(rename = "zonePinning", default, skip_serializing_if = "Option::is_none")]
    pub zone_pinning: Option<bool>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<elastic_pool_edition_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ElasticPoolEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_edition_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The result of an elastic pool list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ElasticPool>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ElasticPoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A elastic pool operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a elastic pool operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ElasticPoolOperationProperties>,
}
impl ElasticPoolOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list elastic pool operations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolOperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ElasticPoolOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticPoolOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ElasticPoolOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a elastic pool operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolOperationProperties {
    #[doc = "The name of the elastic pool the operation is being performed on."]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The name of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The friendly name of operation."]
    #[serde(rename = "operationFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub operation_friendly_name: Option<String>,
    #[doc = "The percentage of the operation completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The name of the server."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The operation start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The operation error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The operation error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The operation error severity."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "Whether or not the error is a user error."]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
    #[doc = "The estimated completion time of the operation."]
    #[serde(rename = "estimatedCompletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub estimated_completion_time: Option<time::OffsetDateTime>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the operation can be cancelled."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
}
impl ElasticPoolOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The max per-database performance level capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolPerDatabaseMaxPerformanceLevelCapability {
    #[doc = "The maximum performance level per database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Unit type used to measure performance level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<elastic_pool_per_database_max_performance_level_capability::Unit>,
    #[doc = "The list of supported min database performance levels."]
    #[serde(
        rename = "supportedPerDatabaseMinPerformanceLevels",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_per_database_min_performance_levels: Vec<ElasticPoolPerDatabaseMinPerformanceLevelCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<elastic_pool_per_database_max_performance_level_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ElasticPoolPerDatabaseMaxPerformanceLevelCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_per_database_max_performance_level_capability {
    use super::*;
    #[doc = "Unit type used to measure performance level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        #[serde(rename = "DTU")]
        Dtu,
        VCores,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dtu => serializer.serialize_unit_variant("Unit", 0u32, "DTU"),
                Self::VCores => serializer.serialize_unit_variant("Unit", 1u32, "VCores"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The minimum per-database performance level capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolPerDatabaseMinPerformanceLevelCapability {
    #[doc = "The minimum performance level per database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Unit type used to measure performance level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<elastic_pool_per_database_min_performance_level_capability::Unit>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<elastic_pool_per_database_min_performance_level_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ElasticPoolPerDatabaseMinPerformanceLevelCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_per_database_min_performance_level_capability {
    use super::*;
    #[doc = "Unit type used to measure performance level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        #[serde(rename = "DTU")]
        Dtu,
        VCores,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dtu => serializer.serialize_unit_variant("Unit", 0u32, "DTU"),
                Self::VCores => serializer.serialize_unit_variant("Unit", 1u32, "VCores"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "Per database settings of an elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolPerDatabaseSettings {
    #[doc = "The minimum capacity all databases are guaranteed."]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "The maximum capacity any one database can consume."]
    #[serde(rename = "maxCapacity", default, skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[doc = "Auto Pause Delay for per database within pool"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
}
impl ElasticPoolPerDatabaseSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Elastic Pool performance level capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolPerformanceLevelCapability {
    #[doc = "The performance level capability."]
    #[serde(rename = "performanceLevel", default, skip_serializing_if = "Option::is_none")]
    pub performance_level: Option<PerformanceLevelCapability>,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "List of supported license types."]
    #[serde(
        rename = "supportedLicenseTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "The maximum number of databases supported."]
    #[serde(rename = "maxDatabaseCount", default, skip_serializing_if = "Option::is_none")]
    pub max_database_count: Option<i32>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "includedMaxSize", default, skip_serializing_if = "Option::is_none")]
    pub included_max_size: Option<MaxSizeCapability>,
    #[doc = "The list of supported max sizes."]
    #[serde(
        rename = "supportedMaxSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_max_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "The list of supported per database max sizes."]
    #[serde(
        rename = "supportedPerDatabaseMaxSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_per_database_max_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "The list of supported per database max performance levels."]
    #[serde(
        rename = "supportedPerDatabaseMaxPerformanceLevels",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_per_database_max_performance_levels: Vec<ElasticPoolPerDatabaseMaxPerformanceLevelCapability>,
    #[doc = "Whether or not zone redundancy is supported for the performance level."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "List of supported maintenance configurations"]
    #[serde(
        rename = "supportedMaintenanceConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_maintenance_configurations: Vec<MaintenanceConfigurationCapability>,
    #[doc = "List of supported min capacities"]
    #[serde(
        rename = "supportedMinCapacities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_min_capacities: Vec<MinCapacityCapability>,
    #[doc = "Supported auto pause delay time range"]
    #[serde(rename = "supportedAutoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub supported_auto_pause_delay: Option<AutoPauseDelayTimeRange>,
    #[doc = "Supported auto pause delay time range"]
    #[serde(rename = "supportedPerDatabaseAutoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub supported_per_database_auto_pause_delay: Option<PerDatabaseAutoPauseDelayTimeRange>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<elastic_pool_performance_level_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ElasticPoolPerformanceLevelCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_performance_level_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "Properties of an elastic pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolProperties {
    #[doc = "The state of the elastic pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<elastic_pool_properties::State>,
    #[doc = "The creation date of the elastic pool (ISO8601 format)."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The storage limit for the database elastic pool in bytes."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "Minimal capacity that serverless pool will not shrink below, if not paused"]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "Per database settings of an elastic pool."]
    #[serde(rename = "perDatabaseSettings", default, skip_serializing_if = "Option::is_none")]
    pub per_database_settings: Option<ElasticPoolPerDatabaseSettings>,
    #[doc = "Whether or not this elastic pool is zone redundant, which means the replicas of this elastic pool will be spread across multiple availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The license type to apply for this elastic pool."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<elastic_pool_properties::LicenseType>,
    #[doc = "Maintenance configuration id assigned to the elastic pool. This configuration defines the period when the maintenance updates will will occur."]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "The number of secondary replicas associated with the elastic pool that are used to provide high availability. Applicable only to Hyperscale elastic pools."]
    #[serde(rename = "highAvailabilityReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_replica_count: Option<i32>,
    #[doc = "Time in minutes after which elastic pool is automatically paused. A value of -1 means that automatic pause is disabled"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
    #[doc = "Type of enclave requested on the elastic pool."]
    #[serde(rename = "preferredEnclaveType", default, skip_serializing_if = "Option::is_none")]
    pub preferred_enclave_type: Option<elastic_pool_properties::PreferredEnclaveType>,
    #[doc = "Specifies the availability zone the pool's primary replica is pinned to."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<elastic_pool_properties::AvailabilityZone>,
}
impl ElasticPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_properties {
    use super::*;
    #[doc = "The state of the elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Creating,
        Ready,
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
                Self::Creating => serializer.serialize_unit_variant("State", 0u32, "Creating"),
                Self::Ready => serializer.serialize_unit_variant("State", 1u32, "Ready"),
                Self::Disabled => serializer.serialize_unit_variant("State", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The license type to apply for this elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        LicenseIncluded,
        BasePrice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 0u32, "LicenseIncluded"),
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 1u32, "BasePrice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of enclave requested on the elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredEnclaveType")]
    pub enum PreferredEnclaveType {
        Default,
        #[serde(rename = "VBS")]
        Vbs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredEnclaveType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredEnclaveType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredEnclaveType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("PreferredEnclaveType", 0u32, "Default"),
                Self::Vbs => serializer.serialize_unit_variant("PreferredEnclaveType", 1u32, "VBS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the availability zone the pool's primary replica is pinned to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityZone")]
    pub enum AvailabilityZone {
        NoPreference,
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
        #[serde(rename = "3")]
        N3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityZone {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityZone {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityZone {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoPreference => serializer.serialize_unit_variant("AvailabilityZone", 0u32, "NoPreference"),
                Self::N1 => serializer.serialize_unit_variant("AvailabilityZone", 1u32, "1"),
                Self::N2 => serializer.serialize_unit_variant("AvailabilityZone", 2u32, "2"),
                Self::N3 => serializer.serialize_unit_variant("AvailabilityZone", 3u32, "3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An elastic pool update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolUpdate {
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of an elastic pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ElasticPoolUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ElasticPoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an elastic pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolUpdateProperties {
    #[doc = "The storage limit for the database elastic pool in bytes."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "Minimal capacity that serverless pool will not shrink below, if not paused"]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "Per database settings of an elastic pool."]
    #[serde(rename = "perDatabaseSettings", default, skip_serializing_if = "Option::is_none")]
    pub per_database_settings: Option<ElasticPoolPerDatabaseSettings>,
    #[doc = "Whether or not this elastic pool is zone redundant, which means the replicas of this elastic pool will be spread across multiple availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The license type to apply for this elastic pool."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<elastic_pool_update_properties::LicenseType>,
    #[doc = "Maintenance configuration id assigned to the elastic pool. This configuration defines the period when the maintenance updates will will occur."]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "The number of secondary replicas associated with the elastic pool that are used to provide high availability. Applicable only to Hyperscale elastic pools."]
    #[serde(rename = "highAvailabilityReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub high_availability_replica_count: Option<i32>,
    #[doc = "Time in minutes after which elastic pool is automatically paused. A value of -1 means that automatic pause is disabled"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
    #[doc = "Type of enclave requested on the elastic pool."]
    #[serde(rename = "preferredEnclaveType", default, skip_serializing_if = "Option::is_none")]
    pub preferred_enclave_type: Option<elastic_pool_update_properties::PreferredEnclaveType>,
    #[doc = "Specifies the availability zone the pool's primary replica is pinned to."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<elastic_pool_update_properties::AvailabilityZone>,
}
impl ElasticPoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod elastic_pool_update_properties {
    use super::*;
    #[doc = "The license type to apply for this elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        LicenseIncluded,
        BasePrice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 0u32, "LicenseIncluded"),
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 1u32, "BasePrice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of enclave requested on the elastic pool."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredEnclaveType")]
    pub enum PreferredEnclaveType {
        Default,
        #[serde(rename = "VBS")]
        Vbs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredEnclaveType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredEnclaveType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredEnclaveType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("PreferredEnclaveType", 0u32, "Default"),
                Self::Vbs => serializer.serialize_unit_variant("PreferredEnclaveType", 1u32, "VBS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the availability zone the pool's primary replica is pinned to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityZone")]
    pub enum AvailabilityZone {
        NoPreference,
        #[serde(rename = "1")]
        N1,
        #[serde(rename = "2")]
        N2,
        #[serde(rename = "3")]
        N3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityZone {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityZone {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityZone {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NoPreference => serializer.serialize_unit_variant("AvailabilityZone", 0u32, "NoPreference"),
                Self::N1 => serializer.serialize_unit_variant("AvailabilityZone", 1u32, "1"),
                Self::N2 => serializer.serialize_unit_variant("AvailabilityZone", 2u32, "2"),
                Self::N3 => serializer.serialize_unit_variant("AvailabilityZone", 3u32, "3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The server encryption protector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProtector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Kind of encryption protector. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for an encryption protector execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EncryptionProtectorProperties>,
}
impl EncryptionProtector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server encryption protectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProtectorListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EncryptionProtector>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EncryptionProtectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EncryptionProtectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for an encryption protector execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionProtectorProperties {
    #[doc = "Subregion of the encryption protector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subregion: Option<String>,
    #[doc = "The name of the server key."]
    #[serde(rename = "serverKeyName", default, skip_serializing_if = "Option::is_none")]
    pub server_key_name: Option<String>,
    #[doc = "The encryption protector type like 'ServiceManaged', 'AzureKeyVault'."]
    #[serde(rename = "serverKeyType")]
    pub server_key_type: encryption_protector_properties::ServerKeyType,
    #[doc = "The URI of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Thumbprint of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Key auto rotation opt-in flag. Either true or false."]
    #[serde(rename = "autoRotationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_rotation_enabled: Option<bool>,
}
impl EncryptionProtectorProperties {
    pub fn new(server_key_type: encryption_protector_properties::ServerKeyType) -> Self {
        Self {
            subregion: None,
            server_key_name: None,
            server_key_type,
            uri: None,
            thumbprint: None,
            auto_rotation_enabled: None,
        }
    }
}
pub mod encryption_protector_properties {
    use super::*;
    #[doc = "The encryption protector type like 'ServiceManaged', 'AzureKeyVault'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerKeyType")]
    pub enum ServerKeyType {
        ServiceManaged,
        AzureKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceManaged => serializer.serialize_unit_variant("ServerKeyType", 0u32, "ServiceManaged"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("ServerKeyType", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Certificate used on an endpoint on the Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointCertificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an endpoint certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointCertificateProperties>,
}
impl EndpointCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of endpoint certificates on the target instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointCertificateListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EndpointCertificate>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EndpointCertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EndpointCertificateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an endpoint certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointCertificateProperties {
    #[doc = "The certificate public blob"]
    #[serde(rename = "publicBlob", default, skip_serializing_if = "Option::is_none")]
    pub public_blob: Option<String>,
}
impl EndpointCertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name that the managed instance service needs to communicate with, along with additional details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The IP Addresses and Ports used when connecting to DomainName."]
    #[serde(
        rename = "endpointDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name that the managed instance service needs to communicate with, along with additional details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl EndpointDetail {
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
#[doc = "Contains the information necessary to perform export database operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDatabaseDefinition {
    #[doc = "Storage key type."]
    #[serde(rename = "storageKeyType")]
    pub storage_key_type: export_database_definition::StorageKeyType,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage Uri."]
    #[serde(rename = "storageUri")]
    pub storage_uri: String,
    #[doc = "Administrator login name."]
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[doc = "Administrator login password."]
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
    #[doc = "Authentication type."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "Contains the ARM resources for which to create private endpoint connection."]
    #[serde(rename = "networkIsolation", default, skip_serializing_if = "Option::is_none")]
    pub network_isolation: Option<NetworkIsolationSettings>,
}
impl ExportDatabaseDefinition {
    pub fn new(
        storage_key_type: export_database_definition::StorageKeyType,
        storage_key: String,
        storage_uri: String,
        administrator_login: String,
        administrator_login_password: String,
    ) -> Self {
        Self {
            storage_key_type,
            storage_key,
            storage_uri,
            administrator_login,
            administrator_login_password,
            authentication_type: None,
            network_isolation: None,
        }
    }
}
pub mod export_database_definition {
    use super::*;
    #[doc = "Storage key type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageKeyType")]
    pub enum StorageKeyType {
        SharedAccessKey,
        StorageAccessKey,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SharedAccessKey => serializer.serialize_unit_variant("StorageKeyType", 0u32, "SharedAccessKey"),
                Self::StorageAccessKey => serializer.serialize_unit_variant("StorageKeyType", 1u32, "StorageAccessKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An extended database blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedDatabaseBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an extended database blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedDatabaseBlobAuditingPolicyProperties>,
}
impl ExtendedDatabaseBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of database extended auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedDatabaseBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExtendedDatabaseBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtendedDatabaseBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ExtendedDatabaseBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an extended database blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedDatabaseBlobAuditingPolicyProperties {
    #[doc = "Specifies condition of where clause when creating an audit."]
    #[serde(rename = "predicateExpression", default, skip_serializing_if = "Option::is_none")]
    pub predicate_expression: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\nLEDGER_OPERATION_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(
        rename = "auditActionsAndGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled' and 'IsAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
    #[doc = "Specifies whether Managed Identity is used to access blob storage"]
    #[serde(rename = "isManagedIdentityInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_managed_identity_in_use: Option<bool>,
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: extended_database_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
}
impl ExtendedDatabaseBlobAuditingPolicyProperties {
    pub fn new(state: extended_database_blob_auditing_policy_properties::State) -> Self {
        Self {
            predicate_expression: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
            is_managed_identity_in_use: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            storage_account_subscription_id: None,
        }
    }
}
pub mod extended_database_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "An extended server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedServerBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an extended server blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedServerBlobAuditingPolicyProperties>,
}
impl ExtendedServerBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server extended auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedServerBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExtendedServerBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtendedServerBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ExtendedServerBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an extended server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedServerBlobAuditingPolicyProperties {
    #[doc = "Specifies the state of devops audit. If state is Enabled, devops logs will be sent to Azure Monitor.\r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled', 'IsAzureMonitorTargetEnabled' as true and 'IsDevopsAuditEnabled' as true\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'DevOpsOperationsAudit' diagnostic logs category on the master database should also be created.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/master/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isDevopsAuditEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_devops_audit_enabled: Option<bool>,
    #[doc = "Specifies condition of where clause when creating an audit."]
    #[serde(rename = "predicateExpression", default, skip_serializing_if = "Option::is_none")]
    pub predicate_expression: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\nLEDGER_OPERATION_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(
        rename = "auditActionsAndGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled' and 'IsAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
    #[doc = "Specifies whether Managed Identity is used to access blob storage"]
    #[serde(rename = "isManagedIdentityInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_managed_identity_in_use: Option<bool>,
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: extended_server_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
}
impl ExtendedServerBlobAuditingPolicyProperties {
    pub fn new(state: extended_server_blob_auditing_policy_properties::State) -> Self {
        Self {
            is_devops_audit_enabled: None,
            predicate_expression: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
            is_managed_identity_in_use: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            storage_account_subscription_id: None,
        }
    }
}
pub mod extended_server_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of a failover group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FailoverGroupProperties>,
}
impl FailoverGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of failover groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FailoverGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FailoverGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FailoverGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverGroupProperties {
    #[doc = "Read-write endpoint of the failover group instance."]
    #[serde(rename = "readWriteEndpoint")]
    pub read_write_endpoint: FailoverGroupReadWriteEndpoint,
    #[doc = "Read-only endpoint of the failover group instance."]
    #[serde(rename = "readOnlyEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub read_only_endpoint: Option<FailoverGroupReadOnlyEndpoint>,
    #[doc = "Local replication role of the failover group instance."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<failover_group_properties::ReplicationRole>,
    #[doc = "Replication state of the failover group instance."]
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<String>,
    #[doc = "List of partner server information for the failover group."]
    #[serde(rename = "partnerServers")]
    pub partner_servers: Vec<PartnerInfo>,
    #[doc = "List of databases in the failover group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "Databases secondary type on partner server."]
    #[serde(rename = "secondaryType", default, skip_serializing_if = "Option::is_none")]
    pub secondary_type: Option<failover_group_properties::SecondaryType>,
}
impl FailoverGroupProperties {
    pub fn new(read_write_endpoint: FailoverGroupReadWriteEndpoint, partner_servers: Vec<PartnerInfo>) -> Self {
        Self {
            read_write_endpoint,
            read_only_endpoint: None,
            replication_role: None,
            replication_state: None,
            partner_servers,
            databases: Vec::new(),
            secondary_type: None,
        }
    }
}
pub mod failover_group_properties {
    use super::*;
    #[doc = "Local replication role of the failover group instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationRole")]
    pub enum ReplicationRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ReplicationRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("ReplicationRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Databases secondary type on partner server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecondaryType")]
    pub enum SecondaryType {
        Geo,
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecondaryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecondaryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecondaryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("SecondaryType", 0u32, "Geo"),
                Self::Standby => serializer.serialize_unit_variant("SecondaryType", 1u32, "Standby"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Read-only endpoint of the failover group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroupReadOnlyEndpoint {
    #[doc = "Failover policy of the read-only endpoint for the failover group."]
    #[serde(rename = "failoverPolicy", default, skip_serializing_if = "Option::is_none")]
    pub failover_policy: Option<failover_group_read_only_endpoint::FailoverPolicy>,
    #[doc = "The target partner server where the read-only endpoint points to."]
    #[serde(rename = "targetServer", default, skip_serializing_if = "Option::is_none")]
    pub target_server: Option<String>,
}
impl FailoverGroupReadOnlyEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod failover_group_read_only_endpoint {
    use super::*;
    #[doc = "Failover policy of the read-only endpoint for the failover group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverPolicy")]
    pub enum FailoverPolicy {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("FailoverPolicy", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("FailoverPolicy", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Read-write endpoint of the failover group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverGroupReadWriteEndpoint {
    #[doc = "Failover policy of the read-write endpoint for the failover group. If failoverPolicy is Automatic then failoverWithDataLossGracePeriodMinutes is required."]
    #[serde(rename = "failoverPolicy")]
    pub failover_policy: failover_group_read_write_endpoint::FailoverPolicy,
    #[doc = "Grace period before failover with data loss is attempted for the read-write endpoint. If failoverPolicy is Automatic then failoverWithDataLossGracePeriodMinutes is required."]
    #[serde(
        rename = "failoverWithDataLossGracePeriodMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failover_with_data_loss_grace_period_minutes: Option<i32>,
}
impl FailoverGroupReadWriteEndpoint {
    pub fn new(failover_policy: failover_group_read_write_endpoint::FailoverPolicy) -> Self {
        Self {
            failover_policy,
            failover_with_data_loss_grace_period_minutes: None,
        }
    }
}
pub mod failover_group_read_write_endpoint {
    use super::*;
    #[doc = "Failover policy of the read-write endpoint for the failover group. If failoverPolicy is Automatic then failoverWithDataLossGracePeriodMinutes is required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverPolicy")]
    pub enum FailoverPolicy {
        Manual,
        Automatic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("FailoverPolicy", 0u32, "Manual"),
                Self::Automatic => serializer.serialize_unit_variant("FailoverPolicy", 1u32, "Automatic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A failover group update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroupUpdate {
    #[doc = "Properties of a failover group update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FailoverGroupUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl FailoverGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a failover group update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroupUpdateProperties {
    #[doc = "Read-write endpoint of the failover group instance."]
    #[serde(rename = "readWriteEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub read_write_endpoint: Option<FailoverGroupReadWriteEndpoint>,
    #[doc = "Read-only endpoint of the failover group instance."]
    #[serde(rename = "readOnlyEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub read_only_endpoint: Option<FailoverGroupReadOnlyEndpoint>,
    #[doc = "List of databases in the failover group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "List of partner server information for the failover group."]
    #[serde(
        rename = "partnerServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub partner_servers: Vec<PartnerInfo>,
    #[doc = "Databases secondary type on partner server."]
    #[serde(rename = "secondaryType", default, skip_serializing_if = "Option::is_none")]
    pub secondary_type: Option<failover_group_update_properties::SecondaryType>,
}
impl FailoverGroupUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod failover_group_update_properties {
    use super::*;
    #[doc = "Databases secondary type on partner server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecondaryType")]
    pub enum SecondaryType {
        Geo,
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecondaryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecondaryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecondaryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("SecondaryType", 0u32, "Geo"),
                Self::Standby => serializer.serialize_unit_variant("SecondaryType", 1u32, "Standby"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource_with_writable_name: ProxyResourceWithWritableName,
    #[doc = "The properties of a server firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerFirewallRuleProperties>,
}
impl FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server firewall rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<FirewallRule>,
}
impl FirewallRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list firewall rules request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FirewallRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported free limit exhaustion behavior options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FreeLimitExhaustionBehaviorCapability {
    #[doc = "Free limit exhaustion behavior type"]
    #[serde(rename = "exhaustionBehaviorType", default, skip_serializing_if = "Option::is_none")]
    pub exhaustion_behavior_type: Option<free_limit_exhaustion_behavior_capability::ExhaustionBehaviorType>,
    #[doc = "Free limit exhaustion behavior status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<free_limit_exhaustion_behavior_capability::Status>,
}
impl FreeLimitExhaustionBehaviorCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod free_limit_exhaustion_behavior_capability {
    use super::*;
    #[doc = "Free limit exhaustion behavior type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExhaustionBehaviorType")]
    pub enum ExhaustionBehaviorType {
        AutoPause,
        BillOverUsage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExhaustionBehaviorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExhaustionBehaviorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExhaustionBehaviorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoPause => serializer.serialize_unit_variant("ExhaustionBehaviorType", 0u32, "AutoPause"),
                Self::BillOverUsage => serializer.serialize_unit_variant("ExhaustionBehaviorType", 1u32, "BillOverUsage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Free limit exhaustion behavior status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A Geo backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoBackupPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Backup policy location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Kind of geo backup policy.  This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The properties of the geo backup policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GeoBackupPolicyProperties>,
}
impl GeoBackupPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of geo backup policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoBackupPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GeoBackupPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GeoBackupPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GeoBackupPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the geo backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoBackupPolicyProperties {
    #[doc = "The state of the geo backup policy."]
    pub state: geo_backup_policy_properties::State,
    #[doc = "The storage type of the geo backup policy."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}
impl GeoBackupPolicyProperties {
    pub fn new(state: geo_backup_policy_properties::State) -> Self {
        Self { state, storage_type: None }
    }
}
pub mod geo_backup_policy_properties {
    use super::*;
    #[doc = "The state of the geo backup policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "An IPv6 server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv6FirewallRule {
    #[serde(flatten)]
    pub proxy_resource_with_writable_name: ProxyResourceWithWritableName,
    #[doc = "The properties of an IPv6 server firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IPv6ServerFirewallRuleProperties>,
}
impl IPv6FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list IPv6 firewall rules request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv6FirewallRuleListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IPv6FirewallRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IPv6FirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IPv6FirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an IPv6 server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv6ServerFirewallRuleProperties {
    #[doc = "The start IP address of the firewall rule. Must be IPv6 format."]
    #[serde(rename = "startIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub start_i_pv6_address: Option<String>,
    #[doc = "The end IP address of the firewall rule. Must be IPv6 format. Must be greater than or equal to startIpAddress."]
    #[serde(rename = "endIPv6Address", default, skip_serializing_if = "Option::is_none")]
    pub end_i_pv6_address: Option<String>,
}
impl IPv6ServerFirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information necessary to perform import operation for existing database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportExistingDatabaseDefinition {
    #[doc = "Storage key type."]
    #[serde(rename = "storageKeyType")]
    pub storage_key_type: import_existing_database_definition::StorageKeyType,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage Uri."]
    #[serde(rename = "storageUri")]
    pub storage_uri: String,
    #[doc = "Administrator login name."]
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[doc = "Administrator login password."]
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
    #[doc = "Authentication type."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "Contains the ARM resources for which to create private endpoint connection."]
    #[serde(rename = "networkIsolation", default, skip_serializing_if = "Option::is_none")]
    pub network_isolation: Option<NetworkIsolationSettings>,
}
impl ImportExistingDatabaseDefinition {
    pub fn new(
        storage_key_type: import_existing_database_definition::StorageKeyType,
        storage_key: String,
        storage_uri: String,
        administrator_login: String,
        administrator_login_password: String,
    ) -> Self {
        Self {
            storage_key_type,
            storage_key,
            storage_uri,
            administrator_login,
            administrator_login_password,
            authentication_type: None,
            network_isolation: None,
        }
    }
}
pub mod import_existing_database_definition {
    use super::*;
    #[doc = "Storage key type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageKeyType")]
    pub enum StorageKeyType {
        SharedAccessKey,
        StorageAccessKey,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SharedAccessKey => serializer.serialize_unit_variant("StorageKeyType", 0u32, "SharedAccessKey"),
                Self::StorageAccessKey => serializer.serialize_unit_variant("StorageKeyType", 1u32, "StorageAccessKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Import export operation extensions list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportExportExtensionsOperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ImportExportExtensionsOperationResult>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImportExportExtensionsOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImportExportExtensionsOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Extension operation result resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportExportExtensionsOperationResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the operation result properties for import/export operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportExportExtensionsOperationResultProperties>,
}
impl ImportExportExtensionsOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the operation result properties for import/export operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportExportExtensionsOperationResultProperties {
    #[doc = "Request Id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Request type."]
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[doc = "Last modified time."]
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[doc = "Server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Queued time."]
    #[serde(rename = "queuedTime", default, skip_serializing_if = "Option::is_none")]
    pub queued_time: Option<String>,
    #[doc = "Blob URI."]
    #[serde(rename = "blobUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_uri: Option<String>,
    #[doc = "Gets the status of private endpoints associated with this request."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionRequestStatus>,
}
impl ImportExportExtensionsOperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An ImportExport operation result resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportExportOperationResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the operation result properties for import/export operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportExportOperationResultProperties>,
}
impl ImportExportOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the operation result properties for import/export operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportExportOperationResultProperties {
    #[doc = "Request Id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Request type."]
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[doc = "Queued time."]
    #[serde(rename = "queuedTime", default, skip_serializing_if = "Option::is_none")]
    pub queued_time: Option<String>,
    #[doc = "Last modified time."]
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[doc = "Blob Uri."]
    #[serde(rename = "blobUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_uri: Option<String>,
    #[doc = "Server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Gets the status of private endpoints associated with this request."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionRequestStatus>,
}
impl ImportExportOperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information necessary to perform import operation for new database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportNewDatabaseDefinition {
    #[doc = "Name of the import database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Edition of the import database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "Service level objective name of the import database."]
    #[serde(rename = "serviceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub service_objective_name: Option<String>,
    #[doc = "Max size in bytes for the import database."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<String>,
    #[doc = "Storage key type."]
    #[serde(rename = "storageKeyType")]
    pub storage_key_type: import_new_database_definition::StorageKeyType,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage Uri."]
    #[serde(rename = "storageUri")]
    pub storage_uri: String,
    #[doc = "Administrator login name."]
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[doc = "Administrator login password."]
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
    #[doc = "Authentication type."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "Contains the ARM resources for which to create private endpoint connection."]
    #[serde(rename = "networkIsolation", default, skip_serializing_if = "Option::is_none")]
    pub network_isolation: Option<NetworkIsolationSettings>,
}
impl ImportNewDatabaseDefinition {
    pub fn new(
        storage_key_type: import_new_database_definition::StorageKeyType,
        storage_key: String,
        storage_uri: String,
        administrator_login: String,
        administrator_login_password: String,
    ) -> Self {
        Self {
            database_name: None,
            edition: None,
            service_objective_name: None,
            max_size_bytes: None,
            storage_key_type,
            storage_key,
            storage_uri,
            administrator_login,
            administrator_login_password,
            authentication_type: None,
            network_isolation: None,
        }
    }
}
pub mod import_new_database_definition {
    use super::*;
    #[doc = "Storage key type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageKeyType")]
    pub enum StorageKeyType {
        SharedAccessKey,
        StorageAccessKey,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SharedAccessKey => serializer.serialize_unit_variant("StorageKeyType", 0u32, "SharedAccessKey"),
                Self::StorageAccessKey => serializer.serialize_unit_variant("StorageKeyType", 1u32, "StorageAccessKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An instance failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceFailoverGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a instance failover group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InstanceFailoverGroupProperties>,
}
impl InstanceFailoverGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of instance failover groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceFailoverGroupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InstanceFailoverGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InstanceFailoverGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InstanceFailoverGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a instance failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceFailoverGroupProperties {
    #[doc = "Type of the geo-secondary instance. Set 'Standby' if the instance is used as a DR option only."]
    #[serde(rename = "secondaryType", default, skip_serializing_if = "Option::is_none")]
    pub secondary_type: Option<instance_failover_group_properties::SecondaryType>,
    #[doc = "Read-write endpoint of the failover group instance."]
    #[serde(rename = "readWriteEndpoint")]
    pub read_write_endpoint: InstanceFailoverGroupReadWriteEndpoint,
    #[doc = "Read-only endpoint of the failover group instance."]
    #[serde(rename = "readOnlyEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub read_only_endpoint: Option<InstanceFailoverGroupReadOnlyEndpoint>,
    #[doc = "Local replication role of the failover group instance."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<instance_failover_group_properties::ReplicationRole>,
    #[doc = "Replication state of the failover group instance."]
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<String>,
    #[doc = "Partner region information for the failover group."]
    #[serde(rename = "partnerRegions")]
    pub partner_regions: Vec<PartnerRegionInfo>,
    #[doc = "List of managed instance pairs in the failover group."]
    #[serde(rename = "managedInstancePairs")]
    pub managed_instance_pairs: Vec<ManagedInstancePairInfo>,
}
impl InstanceFailoverGroupProperties {
    pub fn new(
        read_write_endpoint: InstanceFailoverGroupReadWriteEndpoint,
        partner_regions: Vec<PartnerRegionInfo>,
        managed_instance_pairs: Vec<ManagedInstancePairInfo>,
    ) -> Self {
        Self {
            secondary_type: None,
            read_write_endpoint,
            read_only_endpoint: None,
            replication_role: None,
            replication_state: None,
            partner_regions,
            managed_instance_pairs,
        }
    }
}
pub mod instance_failover_group_properties {
    use super::*;
    #[doc = "Type of the geo-secondary instance. Set 'Standby' if the instance is used as a DR option only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecondaryType")]
    pub enum SecondaryType {
        Geo,
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecondaryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecondaryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecondaryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("SecondaryType", 0u32, "Geo"),
                Self::Standby => serializer.serialize_unit_variant("SecondaryType", 1u32, "Standby"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Local replication role of the failover group instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationRole")]
    pub enum ReplicationRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ReplicationRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("ReplicationRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Read-only endpoint of the failover group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceFailoverGroupReadOnlyEndpoint {
    #[doc = "Failover policy of the read-only endpoint for the failover group."]
    #[serde(rename = "failoverPolicy", default, skip_serializing_if = "Option::is_none")]
    pub failover_policy: Option<instance_failover_group_read_only_endpoint::FailoverPolicy>,
}
impl InstanceFailoverGroupReadOnlyEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_failover_group_read_only_endpoint {
    use super::*;
    #[doc = "Failover policy of the read-only endpoint for the failover group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverPolicy")]
    pub enum FailoverPolicy {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("FailoverPolicy", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("FailoverPolicy", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Read-write endpoint of the failover group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceFailoverGroupReadWriteEndpoint {
    #[doc = "Failover policy of the read-write endpoint for the failover group. If failoverPolicy is Automatic then failoverWithDataLossGracePeriodMinutes is required."]
    #[serde(rename = "failoverPolicy")]
    pub failover_policy: instance_failover_group_read_write_endpoint::FailoverPolicy,
    #[doc = "Grace period before failover with data loss is attempted for the read-write endpoint. If failoverPolicy is Automatic then failoverWithDataLossGracePeriodMinutes is required."]
    #[serde(
        rename = "failoverWithDataLossGracePeriodMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failover_with_data_loss_grace_period_minutes: Option<i32>,
}
impl InstanceFailoverGroupReadWriteEndpoint {
    pub fn new(failover_policy: instance_failover_group_read_write_endpoint::FailoverPolicy) -> Self {
        Self {
            failover_policy,
            failover_with_data_loss_grace_period_minutes: None,
        }
    }
}
pub mod instance_failover_group_read_write_endpoint {
    use super::*;
    #[doc = "Failover policy of the read-write endpoint for the failover group. If failoverPolicy is Automatic then failoverWithDataLossGracePeriodMinutes is required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverPolicy")]
    pub enum FailoverPolicy {
        Manual,
        Automatic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("FailoverPolicy", 0u32, "Manual"),
                Self::Automatic => serializer.serialize_unit_variant("FailoverPolicy", 1u32, "Automatic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An Azure SQL instance pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancePool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of an instance pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InstancePoolProperties>,
}
impl InstancePool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "The instance pool capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancePoolEditionCapability {
    #[doc = "The instance pool version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The supported families."]
    #[serde(
        rename = "supportedFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_families: Vec<InstancePoolFamilyCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<instance_pool_edition_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl InstancePoolEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_pool_edition_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The instance pool family capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancePoolFamilyCapability {
    #[doc = "Family name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of supported license types."]
    #[serde(
        rename = "supportedLicenseTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "List of supported virtual cores values."]
    #[serde(
        rename = "supportedVcoresValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_vcores_values: Vec<InstancePoolVcoresCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<instance_pool_family_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl InstancePoolFamilyCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_pool_family_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A list of Azure SQL instance pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancePoolListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InstancePool>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InstancePoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InstancePoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an instance pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancePoolProperties {
    #[doc = "Resource ID of the subnet to place this instance pool in."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Count of vCores belonging to this instance pool."]
    #[serde(rename = "vCores")]
    pub v_cores: i32,
    #[doc = "The license type. Possible values are 'LicenseIncluded' (price for SQL license is included) and 'BasePrice' (without SQL license price)."]
    #[serde(rename = "licenseType")]
    pub license_type: instance_pool_properties::LicenseType,
    #[doc = "The Dns Zone that the managed instance pool is in."]
    #[serde(rename = "dnsZone", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone: Option<String>,
    #[doc = "Specifies maintenance configuration id to apply to this managed instance."]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
}
impl InstancePoolProperties {
    pub fn new(subnet_id: String, v_cores: i32, license_type: instance_pool_properties::LicenseType) -> Self {
        Self {
            subnet_id,
            v_cores,
            license_type,
            dns_zone: None,
            maintenance_configuration_id: None,
        }
    }
}
pub mod instance_pool_properties {
    use super::*;
    #[doc = "The license type. Possible values are 'LicenseIncluded' (price for SQL license is included) and 'BasePrice' (without SQL license price)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        LicenseIncluded,
        BasePrice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 0u32, "LicenseIncluded"),
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 1u32, "BasePrice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to an Instance pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancePoolUpdate {
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of an instance pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InstancePoolProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl InstancePoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed instance virtual cores capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancePoolVcoresCapability {
    #[doc = "The virtual cores identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The virtual cores value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "storageLimit", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit: Option<MaxSizeCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<instance_pool_vcores_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl InstancePoolVcoresCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_pool_vcores_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Job {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL job agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobAgent {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<JobAgentIdentity>,
    #[doc = "Properties of a job agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobAgentProperties>,
}
impl JobAgent {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The job agent edition capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentEditionCapability {
    #[doc = "The job agent edition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported service level objectives for the edition."]
    #[serde(
        rename = "supportedServiceLevelObjectives",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_service_level_objectives: Vec<JobAgentServiceLevelObjectiveCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_agent_edition_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl JobAgentEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_agent_edition_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobAgentIdentity {
    #[doc = "The job agent identity tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The job agent identity type"]
    #[serde(rename = "type")]
    pub type_: job_agent_identity::Type,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl JobAgentIdentity {
    pub fn new(type_: job_agent_identity::Type) -> Self {
        Self {
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
pub mod job_agent_identity {
    use super::*;
    #[doc = "The job agent identity type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        SystemAssignedUserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssignedUserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of Azure SQL job agents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobAgent>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobAgentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobAgentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a job agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobAgentProperties {
    #[doc = "Resource ID of the database to store job metadata in."]
    #[serde(rename = "databaseId")]
    pub database_id: String,
    #[doc = "The state of the job agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<job_agent_properties::State>,
}
impl JobAgentProperties {
    pub fn new(database_id: String) -> Self {
        Self { database_id, state: None }
    }
}
pub mod job_agent_properties {
    use super::*;
    #[doc = "The state of the job agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Creating,
        Ready,
        Updating,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("State", 0u32, "Creating"),
                Self::Ready => serializer.serialize_unit_variant("State", 1u32, "Ready"),
                Self::Updating => serializer.serialize_unit_variant("State", 2u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("State", 3u32, "Deleting"),
                Self::Disabled => serializer.serialize_unit_variant("State", 4u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The job agent service level objective capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentServiceLevelObjectiveCapability {
    #[doc = "The service objective name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_agent_service_level_objective_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl JobAgentServiceLevelObjectiveCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_agent_service_level_objective_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "An update to an Azure SQL job agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentUpdate {
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<JobAgentIdentity>,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl JobAgentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentUserAssignedIdentity {
    #[doc = "The Azure Active Directory principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Azure Active Directory client id."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl JobAgentUserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The job agent version capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentVersionCapability {
    #[doc = "The job agent version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported editions."]
    #[serde(
        rename = "supportedEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_editions: Vec<JobAgentEditionCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_agent_version_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl JobAgentVersionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_agent_version_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A stored credential that can be used by a job to connect to target databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCredential {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a job credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobCredentialProperties>,
}
impl JobCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of job credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCredentialListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobCredential>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobCredentialListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobCredentialListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a job credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCredentialProperties {
    #[doc = "The credential user name."]
    pub username: String,
    #[doc = "The credential password."]
    pub password: String,
}
impl JobCredentialProperties {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}
#[doc = "An execution of a job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecution {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for an Azure SQL Database Elastic job execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobExecutionProperties>,
}
impl JobExecution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of job executions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecutionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobExecution>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobExecutionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobExecutionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for an Azure SQL Database Elastic job execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecutionProperties {
    #[doc = "The job version number."]
    #[serde(rename = "jobVersion", default, skip_serializing_if = "Option::is_none")]
    pub job_version: Option<i32>,
    #[doc = "The job step name."]
    #[serde(rename = "stepName", default, skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    #[doc = "The job step id."]
    #[serde(rename = "stepId", default, skip_serializing_if = "Option::is_none")]
    pub step_id: Option<i32>,
    #[doc = "The unique identifier of the job execution."]
    #[serde(rename = "jobExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub job_execution_id: Option<String>,
    #[doc = "The detailed state of the job execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<job_execution_properties::Lifecycle>,
    #[doc = "The ARM provisioning state of the job execution."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<job_execution_properties::ProvisioningState>,
    #[doc = "The time that the job execution was created."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The time that the job execution started."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The time that the job execution completed."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Number of times the job execution has been attempted."]
    #[serde(rename = "currentAttempts", default, skip_serializing_if = "Option::is_none")]
    pub current_attempts: Option<i32>,
    #[doc = "Start time of the current attempt."]
    #[serde(rename = "currentAttemptStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub current_attempt_start_time: Option<time::OffsetDateTime>,
    #[doc = "The last status or error message."]
    #[serde(rename = "lastMessage", default, skip_serializing_if = "Option::is_none")]
    pub last_message: Option<String>,
    #[doc = "The target that a job execution is executed on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<JobExecutionTarget>,
}
impl JobExecutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_execution_properties {
    use super::*;
    #[doc = "The detailed state of the job execution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Lifecycle")]
    pub enum Lifecycle {
        Created,
        InProgress,
        WaitingForChildJobExecutions,
        WaitingForRetry,
        Succeeded,
        SucceededWithSkipped,
        Failed,
        TimedOut,
        Canceled,
        Skipped,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Lifecycle {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Lifecycle {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Lifecycle {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Created => serializer.serialize_unit_variant("Lifecycle", 0u32, "Created"),
                Self::InProgress => serializer.serialize_unit_variant("Lifecycle", 1u32, "InProgress"),
                Self::WaitingForChildJobExecutions => serializer.serialize_unit_variant("Lifecycle", 2u32, "WaitingForChildJobExecutions"),
                Self::WaitingForRetry => serializer.serialize_unit_variant("Lifecycle", 3u32, "WaitingForRetry"),
                Self::Succeeded => serializer.serialize_unit_variant("Lifecycle", 4u32, "Succeeded"),
                Self::SucceededWithSkipped => serializer.serialize_unit_variant("Lifecycle", 5u32, "SucceededWithSkipped"),
                Self::Failed => serializer.serialize_unit_variant("Lifecycle", 6u32, "Failed"),
                Self::TimedOut => serializer.serialize_unit_variant("Lifecycle", 7u32, "TimedOut"),
                Self::Canceled => serializer.serialize_unit_variant("Lifecycle", 8u32, "Canceled"),
                Self::Skipped => serializer.serialize_unit_variant("Lifecycle", 9u32, "Skipped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The ARM provisioning state of the job execution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Created,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Created"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The target that a job execution is executed on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobExecutionTarget {
    #[doc = "The type of the target."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_execution_target::Type>,
    #[doc = "The server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl JobExecutionTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_execution_target {
    use super::*;
    #[doc = "The type of the target."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        TargetGroup,
        SqlDatabase,
        SqlElasticPool,
        SqlShardMap,
        SqlServer,
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
                Self::TargetGroup => serializer.serialize_unit_variant("Type", 0u32, "TargetGroup"),
                Self::SqlDatabase => serializer.serialize_unit_variant("Type", 1u32, "SqlDatabase"),
                Self::SqlElasticPool => serializer.serialize_unit_variant("Type", 2u32, "SqlElasticPool"),
                Self::SqlShardMap => serializer.serialize_unit_variant("Type", 3u32, "SqlShardMap"),
                Self::SqlServer => serializer.serialize_unit_variant("Type", 4u32, "SqlServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Job>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A job agent private endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPrivateEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of job agent private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobPrivateEndpointProperties>,
}
impl JobPrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of job agent private endpoints."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPrivateEndpointListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobPrivateEndpoint>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobPrivateEndpointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobPrivateEndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of job agent private endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobPrivateEndpointProperties {
    #[doc = "ARM resource id of the server the private endpoint will target."]
    #[serde(rename = "targetServerAzureResourceId")]
    pub target_server_azure_resource_id: String,
    #[doc = "Private endpoint id of the private endpoint."]
    #[serde(rename = "privateEndpointId", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_id: Option<String>,
}
impl JobPrivateEndpointProperties {
    pub fn new(target_server_azure_resource_id: String) -> Self {
        Self {
            target_server_azure_resource_id,
            private_endpoint_id: None,
        }
    }
}
#[doc = "Properties of a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "User-defined description of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The job version number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "Scheduling properties of a job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JobSchedule>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scheduling properties of a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobSchedule {
    #[doc = "Schedule start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Schedule end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Schedule interval type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_schedule::Type>,
    #[doc = "Whether or not the schedule is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Value of the schedule's recurring interval, if the ScheduleType is recurring. ISO8601 duration format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}
impl JobSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_schedule {
    use super::*;
    #[doc = "Schedule interval type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Once,
        Recurring,
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Once
        }
    }
}
#[doc = "A job step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStep {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a job step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobStepProperties>,
}
impl JobStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The action to be executed by a job step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStepAction {
    #[doc = "Type of action being executed by the job step."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_step_action::Type>,
    #[doc = "The source of the action to execute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<job_step_action::Source>,
    #[doc = "The action value, for example the text of the T-SQL script to execute."]
    pub value: String,
}
impl JobStepAction {
    pub fn new(value: String) -> Self {
        Self {
            type_: None,
            source: None,
            value,
        }
    }
}
pub mod job_step_action {
    use super::*;
    #[doc = "Type of action being executed by the job step."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        TSql,
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
                Self::TSql => serializer.serialize_unit_variant("Type", 0u32, "TSql"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::TSql
        }
    }
    #[doc = "The source of the action to execute."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        Inline,
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
                Self::Inline => serializer.serialize_unit_variant("Source", 0u32, "Inline"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Source {
        fn default() -> Self {
            Self::Inline
        }
    }
}
#[doc = "The execution options of a job step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStepExecutionOptions {
    #[doc = "Execution timeout for the job step."]
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[doc = "Maximum number of times the job step will be reattempted if the first attempt fails."]
    #[serde(rename = "retryAttempts", default, skip_serializing_if = "Option::is_none")]
    pub retry_attempts: Option<i32>,
    #[doc = "Initial delay between retries for job step execution."]
    #[serde(rename = "initialRetryIntervalSeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_retry_interval_seconds: Option<i32>,
    #[doc = "The maximum amount of time to wait between retries for job step execution."]
    #[serde(rename = "maximumRetryIntervalSeconds", default, skip_serializing_if = "Option::is_none")]
    pub maximum_retry_interval_seconds: Option<i32>,
    #[doc = "The backoff multiplier for the time between retries."]
    #[serde(rename = "retryIntervalBackoffMultiplier", default, skip_serializing_if = "Option::is_none")]
    pub retry_interval_backoff_multiplier: Option<f32>,
}
impl JobStepExecutionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of job steps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStepListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobStep>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobStepListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobStepListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The output configuration of a job step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStepOutput {
    #[doc = "The output destination type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_step_output::Type>,
    #[doc = "The output destination subscription id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The output destination resource group."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "The output destination server name."]
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[doc = "The output destination database."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "The output destination schema."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "The output destination table."]
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[doc = "The resource ID of the credential to use to connect to the output destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
}
impl JobStepOutput {
    pub fn new(server_name: String, database_name: String, table_name: String) -> Self {
        Self {
            type_: None,
            subscription_id: None,
            resource_group_name: None,
            server_name,
            database_name,
            schema_name: None,
            table_name,
            credential: None,
        }
    }
}
pub mod job_step_output {
    use super::*;
    #[doc = "The output destination type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SqlDatabase,
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
                Self::SqlDatabase => serializer.serialize_unit_variant("Type", 0u32, "SqlDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::SqlDatabase
        }
    }
}
#[doc = "Properties of a job step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStepProperties {
    #[doc = "The job step's index within the job. If not specified when creating the job step, it will be created as the last step. If not specified when updating the job step, the step id is not modified."]
    #[serde(rename = "stepId", default, skip_serializing_if = "Option::is_none")]
    pub step_id: Option<i32>,
    #[doc = "The resource ID of the target group that the job step will be executed on."]
    #[serde(rename = "targetGroup")]
    pub target_group: String,
    #[doc = "The resource ID of the job credential that will be used to connect to the targets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[doc = "The action to be executed by a job step."]
    pub action: JobStepAction,
    #[doc = "The output configuration of a job step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<JobStepOutput>,
    #[doc = "The execution options of a job step."]
    #[serde(rename = "executionOptions", default, skip_serializing_if = "Option::is_none")]
    pub execution_options: Option<JobStepExecutionOptions>,
}
impl JobStepProperties {
    pub fn new(target_group: String, action: JobStepAction) -> Self {
        Self {
            step_id: None,
            target_group,
            credential: None,
            action,
            output: None,
            execution_options: None,
        }
    }
}
#[doc = "A job target, for example a specific database or a container of databases that is evaluated during job execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobTarget {
    #[doc = "Whether the target is included or excluded from the group."]
    #[serde(rename = "membershipType", default, skip_serializing_if = "Option::is_none")]
    pub membership_type: Option<job_target::MembershipType>,
    #[doc = "The target type."]
    #[serde(rename = "type")]
    pub type_: job_target::Type,
    #[doc = "The target server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The target database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The target elastic pool name."]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The target shard map."]
    #[serde(rename = "shardMapName", default, skip_serializing_if = "Option::is_none")]
    pub shard_map_name: Option<String>,
    #[doc = "The resource ID of the credential that is used during job execution to connect to the target and determine the list of databases inside the target."]
    #[serde(rename = "refreshCredential", default, skip_serializing_if = "Option::is_none")]
    pub refresh_credential: Option<String>,
}
impl JobTarget {
    pub fn new(type_: job_target::Type) -> Self {
        Self {
            membership_type: None,
            type_,
            server_name: None,
            database_name: None,
            elastic_pool_name: None,
            shard_map_name: None,
            refresh_credential: None,
        }
    }
}
pub mod job_target {
    use super::*;
    #[doc = "Whether the target is included or excluded from the group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MembershipType {
        Include,
        Exclude,
    }
    impl Default for MembershipType {
        fn default() -> Self {
            Self::Include
        }
    }
    #[doc = "The target type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        TargetGroup,
        SqlDatabase,
        SqlElasticPool,
        SqlShardMap,
        SqlServer,
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
                Self::TargetGroup => serializer.serialize_unit_variant("Type", 0u32, "TargetGroup"),
                Self::SqlDatabase => serializer.serialize_unit_variant("Type", 1u32, "SqlDatabase"),
                Self::SqlElasticPool => serializer.serialize_unit_variant("Type", 2u32, "SqlElasticPool"),
                Self::SqlShardMap => serializer.serialize_unit_variant("Type", 3u32, "SqlShardMap"),
                Self::SqlServer => serializer.serialize_unit_variant("Type", 4u32, "SqlServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A group of job targets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobTargetGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of job target group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobTargetGroupProperties>,
}
impl JobTargetGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of target groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobTargetGroupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobTargetGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobTargetGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobTargetGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of job target group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobTargetGroupProperties {
    #[doc = "Members of the target group."]
    pub members: Vec<JobTarget>,
}
impl JobTargetGroupProperties {
    pub fn new(members: Vec<JobTarget>) -> Self {
        Self { members }
    }
}
#[doc = "A job version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl JobVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of job versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobVersionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JobVersion>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JobVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure SQL Database ledger digest upload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LedgerDigestUploads {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database ledger digest upload settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LedgerDigestUploadsProperties>,
}
impl LedgerDigestUploads {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of ledger digest upload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LedgerDigestUploadsListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LedgerDigestUploads>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LedgerDigestUploadsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LedgerDigestUploadsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database ledger digest upload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LedgerDigestUploadsProperties {
    #[doc = "The digest storage endpoint, which must be either an Azure blob storage endpoint or an URI for Azure Confidential Ledger."]
    #[serde(rename = "digestStorageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub digest_storage_endpoint: Option<String>,
    #[doc = "Specifies the state of ledger digest upload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ledger_digest_uploads_properties::State>,
}
impl LedgerDigestUploadsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod ledger_digest_uploads_properties {
    use super::*;
    #[doc = "Specifies the state of ledger digest upload."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "The license type capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseTypeCapability {
    #[doc = "License type identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<license_type_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl LicenseTypeCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod license_type_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The location capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationCapabilities {
    #[doc = "The location name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported server versions."]
    #[serde(
        rename = "supportedServerVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[doc = "The list of supported managed instance versions."]
    #[serde(
        rename = "supportedManagedInstanceVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_managed_instance_versions: Vec<ManagedInstanceVersionCapability>,
    #[doc = "The list of supported job agent versions."]
    #[serde(
        rename = "supportedJobAgentVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_job_agent_versions: Vec<JobAgentVersionCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<location_capabilities::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl LocationCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod location_capabilities {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The log size capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSizeCapability {
    #[doc = "The log size limit (see 'unit' for the units)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The units that the limit is expressed in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<log_size_capability::Unit>,
}
impl LogSizeCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod log_size_capability {
    use super::*;
    #[doc = "The units that the limit is expressed in."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Megabytes,
        Gigabytes,
        Terabytes,
        Petabytes,
        Percent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Megabytes => serializer.serialize_unit_variant("Unit", 0u32, "Megabytes"),
                Self::Gigabytes => serializer.serialize_unit_variant("Unit", 1u32, "Gigabytes"),
                Self::Terabytes => serializer.serialize_unit_variant("Unit", 2u32, "Terabytes"),
                Self::Petabytes => serializer.serialize_unit_variant("Unit", 3u32, "Petabytes"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 4u32, "Percent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A logical database transparent data encryption state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalDatabaseTransparentDataEncryption {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a transparent data encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransparentDataEncryptionProperties>,
}
impl LogicalDatabaseTransparentDataEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of transparent data encryptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalDatabaseTransparentDataEncryptionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LogicalDatabaseTransparentDataEncryption>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogicalDatabaseTransparentDataEncryptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LogicalDatabaseTransparentDataEncryptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the server's Advanced Threat Protection configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalServerAdvancedThreatProtectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerAdvancedThreatProtection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogicalServerAdvancedThreatProtectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LogicalServerAdvancedThreatProtectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the server's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalServerSecurityAlertPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogicalServerSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LogicalServerSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A long term retention backup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionBackup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a long term retention backup"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LongTermRetentionBackupProperties>,
}
impl LongTermRetentionBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of long term retention backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionBackupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LongTermRetentionBackup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LongTermRetentionBackupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LongTermRetentionBackupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A LongTermRetentionBackup operation result resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionBackupOperationResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the operation result properties for long term retention backup operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LongTermRetentionOperationResultProperties>,
}
impl LongTermRetentionBackupOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a long term retention backup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionBackupProperties {
    #[doc = "The server name that the backup database belong to."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The create time of the server."]
    #[serde(rename = "serverCreateTime", default, with = "azure_core::date::rfc3339::option")]
    pub server_create_time: Option<time::OffsetDateTime>,
    #[doc = "The name of the database the backup belong to"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The delete time of the database"]
    #[serde(rename = "databaseDeletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub database_deletion_time: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "backupTime", default, with = "azure_core::date::rfc3339::option")]
    pub backup_time: Option<time::OffsetDateTime>,
    #[doc = "The time the long term retention backup will expire."]
    #[serde(rename = "backupExpirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub backup_expiration_time: Option<time::OffsetDateTime>,
    #[doc = "The storage redundancy type of the backup"]
    #[serde(rename = "backupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_redundancy: Option<long_term_retention_backup_properties::BackupStorageRedundancy>,
    #[doc = "The storage redundancy type of the backup"]
    #[serde(rename = "requestedBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub requested_backup_storage_redundancy: Option<long_term_retention_backup_properties::RequestedBackupStorageRedundancy>,
    #[doc = "The setting whether the LTR backup is immutable"]
    #[serde(rename = "isBackupImmutable", default, skip_serializing_if = "Option::is_none")]
    pub is_backup_immutable: Option<bool>,
    #[doc = "The BackupStorageAccessTier for the LTR backup"]
    #[serde(rename = "backupStorageAccessTier", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_access_tier: Option<long_term_retention_backup_properties::BackupStorageAccessTier>,
}
impl LongTermRetentionBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod long_term_retention_backup_properties {
    use super::*;
    #[doc = "The storage redundancy type of the backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageRedundancy")]
    pub enum BackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("BackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("BackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("BackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("BackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage redundancy type of the backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestedBackupStorageRedundancy")]
    pub enum RequestedBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestedBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestedBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestedBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The BackupStorageAccessTier for the LTR backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageAccessTier")]
    pub enum BackupStorageAccessTier {
        Hot,
        Archive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageAccessTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageAccessTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageAccessTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hot => serializer.serialize_unit_variant("BackupStorageAccessTier", 0u32, "Hot"),
                Self::Archive => serializer.serialize_unit_variant("BackupStorageAccessTier", 1u32, "Archive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains the operation result properties for long term retention backup operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionOperationResultProperties {
    #[doc = "Request Id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Operation type."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[doc = "Source backup resource id"]
    #[serde(rename = "fromBackupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub from_backup_resource_id: Option<String>,
    #[doc = "Target backup resource id"]
    #[serde(rename = "toBackupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub to_backup_resource_id: Option<String>,
    #[doc = "The storage redundancy type of the copied backup"]
    #[serde(rename = "targetBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub target_backup_storage_redundancy: Option<long_term_retention_operation_result_properties::TargetBackupStorageRedundancy>,
    #[doc = "Operation status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Progress message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl LongTermRetentionOperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod long_term_retention_operation_result_properties {
    use super::*;
    #[doc = "The storage redundancy type of the copied backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetBackupStorageRedundancy")]
    pub enum TargetBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("TargetBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A long term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a long term retention policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LongTermRetentionPolicyProperties>,
}
impl LongTermRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of long term retention policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LongTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LongTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LongTermRetentionPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a long term retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionPolicyProperties {
    #[doc = "The setting whether to make LTR backups immutable"]
    #[serde(rename = "makeBackupsImmutable", default, skip_serializing_if = "Option::is_none")]
    pub make_backups_immutable: Option<bool>,
    #[doc = "The BackupStorageAccessTier for the LTR backups"]
    #[serde(rename = "backupStorageAccessTier", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_access_tier: Option<long_term_retention_policy_properties::BackupStorageAccessTier>,
    #[doc = "The weekly retention policy for an LTR backup in an ISO 8601 format."]
    #[serde(rename = "weeklyRetention", default, skip_serializing_if = "Option::is_none")]
    pub weekly_retention: Option<String>,
    #[doc = "The monthly retention policy for an LTR backup in an ISO 8601 format."]
    #[serde(rename = "monthlyRetention", default, skip_serializing_if = "Option::is_none")]
    pub monthly_retention: Option<String>,
    #[doc = "The yearly retention policy for an LTR backup in an ISO 8601 format."]
    #[serde(rename = "yearlyRetention", default, skip_serializing_if = "Option::is_none")]
    pub yearly_retention: Option<String>,
    #[doc = "The week of year to take the yearly backup in an ISO 8601 format."]
    #[serde(rename = "weekOfYear", default, skip_serializing_if = "Option::is_none")]
    pub week_of_year: Option<i32>,
}
impl LongTermRetentionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod long_term_retention_policy_properties {
    use super::*;
    #[doc = "The BackupStorageAccessTier for the LTR backups"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageAccessTier")]
    pub enum BackupStorageAccessTier {
        Hot,
        Archive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageAccessTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageAccessTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageAccessTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hot => serializer.serialize_unit_variant("BackupStorageAccessTier", 0u32, "Hot"),
                Self::Archive => serializer.serialize_unit_variant("BackupStorageAccessTier", 1u32, "Archive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The maintenance configuration capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfigurationCapability {
    #[doc = "Maintenance configuration name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether or not zone redundancy is supported for the maintenance configuration."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<maintenance_configuration_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl MaintenanceConfigurationCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_configuration_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "Maintenance window options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowOptions {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Maintenance window options properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceWindowOptionsProperties>,
}
impl MaintenanceWindowOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window options properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowOptionsProperties {
    #[doc = "Whether maintenance windows are enabled for the database."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Available maintenance cycles e.g. {Saturday, 0, 48*60}, {Wednesday, 0, 24*60}."]
    #[serde(
        rename = "maintenanceWindowCycles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub maintenance_window_cycles: Vec<MaintenanceWindowTimeRange>,
    #[doc = "Minimum duration of maintenance window."]
    #[serde(rename = "minDurationInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub min_duration_in_minutes: Option<i32>,
    #[doc = "Default duration for maintenance window."]
    #[serde(rename = "defaultDurationInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub default_duration_in_minutes: Option<i32>,
    #[doc = "Minimum number of maintenance windows cycles to be set on the database."]
    #[serde(rename = "minCycles", default, skip_serializing_if = "Option::is_none")]
    pub min_cycles: Option<i32>,
    #[doc = "Time granularity in minutes for maintenance windows."]
    #[serde(rename = "timeGranularityInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub time_granularity_in_minutes: Option<i32>,
    #[doc = "Whether we allow multiple maintenance windows per cycle."]
    #[serde(
        rename = "allowMultipleMaintenanceWindowsPerCycle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_multiple_maintenance_windows_per_cycle: Option<bool>,
}
impl MaintenanceWindowOptionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance window time range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowTimeRange {
    #[doc = "Day of maintenance window."]
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<maintenance_window_time_range::DayOfWeek>,
    #[doc = "Start time minutes offset from 12am."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Duration of maintenance window in minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl MaintenanceWindowTimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_window_time_range {
    use super::*;
    #[doc = "Day of maintenance window."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DayOfWeek")]
    pub enum DayOfWeek {
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
    impl FromStr for DayOfWeek {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DayOfWeek {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DayOfWeek {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sunday => serializer.serialize_unit_variant("DayOfWeek", 0u32, "Sunday"),
                Self::Monday => serializer.serialize_unit_variant("DayOfWeek", 1u32, "Monday"),
                Self::Tuesday => serializer.serialize_unit_variant("DayOfWeek", 2u32, "Tuesday"),
                Self::Wednesday => serializer.serialize_unit_variant("DayOfWeek", 3u32, "Wednesday"),
                Self::Thursday => serializer.serialize_unit_variant("DayOfWeek", 4u32, "Thursday"),
                Self::Friday => serializer.serialize_unit_variant("DayOfWeek", 5u32, "Friday"),
                Self::Saturday => serializer.serialize_unit_variant("DayOfWeek", 6u32, "Saturday"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Maintenance windows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindows {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Maintenance windows resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceWindowsProperties>,
}
impl MaintenanceWindows {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance windows resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindowsProperties {
    #[serde(
        rename = "timeRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub time_ranges: Vec<MaintenanceWindowTimeRange>,
}
impl MaintenanceWindowsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A short term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedBackupShortTermRetentionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a short term retention policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedBackupShortTermRetentionPolicyProperties>,
}
impl ManagedBackupShortTermRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of short term retention policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedBackupShortTermRetentionPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedBackupShortTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedBackupShortTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedBackupShortTermRetentionPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a short term retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedBackupShortTermRetentionPolicyProperties {
    #[doc = "The backup retention period in days. This is how many days Point-in-Time Restore will be supported."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}
impl ManagedBackupShortTermRetentionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDatabase {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The managed database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedDatabaseProperties>,
}
impl ManagedDatabase {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A managed database Advanced Threat Protection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseAdvancedThreatProtection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of an Advanced Threat Protection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvancedThreatProtectionProperties>,
}
impl ManagedDatabaseAdvancedThreatProtection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the managed database's Advanced Threat Protection settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseAdvancedThreatProtectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedDatabaseAdvancedThreatProtection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedDatabaseAdvancedThreatProtectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedDatabaseAdvancedThreatProtectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information necessary to perform a managed database move."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDatabaseMoveDefinition {
    #[doc = "The destination managed database ID"]
    #[serde(rename = "destinationManagedDatabaseId")]
    pub destination_managed_database_id: String,
}
impl ManagedDatabaseMoveDefinition {
    pub fn new(destination_managed_database_id: String) -> Self {
        Self {
            destination_managed_database_id,
        }
    }
}
#[doc = "List of managed database move operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseMoveOperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedDatabaseMoveOperationResult>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedDatabaseMoveOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedDatabaseMoveOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed database move operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseMoveOperationResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the operation result properties for managed database move operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedDatabaseMoveOperationResultProperties>,
}
impl ManagedDatabaseMoveOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the operation result properties for managed database move operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseMoveOperationResultProperties {
    #[doc = "The name of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The friendly name of operation."]
    #[serde(rename = "operationFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub operation_friendly_name: Option<String>,
    #[doc = "The operation start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<managed_database_move_operation_result_properties::State>,
    #[doc = "Operation mode."]
    #[serde(rename = "operationMode", default, skip_serializing_if = "Option::is_none")]
    pub operation_mode: Option<managed_database_move_operation_result_properties::OperationMode>,
    #[doc = "Source Managed Instance name."]
    #[serde(rename = "sourceManagedInstanceName", default, skip_serializing_if = "Option::is_none")]
    pub source_managed_instance_name: Option<String>,
    #[doc = "Target Managed Instance name."]
    #[serde(rename = "targetManagedInstanceName", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_instance_name: Option<String>,
    #[doc = "Source Managed Instance resource id."]
    #[serde(rename = "sourceManagedInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub source_managed_instance_id: Option<String>,
    #[doc = "Target Managed instance resource id."]
    #[serde(rename = "targetManagedInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_instance_id: Option<String>,
    #[doc = "Source database name."]
    #[serde(rename = "sourceDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub source_database_name: Option<String>,
    #[doc = "Target database name."]
    #[serde(rename = "targetDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[doc = "Is move operation cancellable."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
    #[doc = "The operation error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The operation error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The operation error severity."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "Whether or not the error is a user error."]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
}
impl ManagedDatabaseMoveOperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_database_move_operation_result_properties {
    use super::*;
    #[doc = "The operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        InProgress,
        Succeeded,
        Failed,
        CancelInProgress,
        Cancelled,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::CancelInProgress => serializer.serialize_unit_variant("State", 4u32, "CancelInProgress"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 5u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Operation mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationMode")]
    pub enum OperationMode {
        Move,
        Copy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Move => serializer.serialize_unit_variant("OperationMode", 0u32, "Move"),
                Self::Copy => serializer.serialize_unit_variant("OperationMode", 1u32, "Copy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The managed database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseProperties {
    #[doc = "Collation of the managed database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "Status of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_database_properties::Status>,
    #[doc = "Creation date of the database."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Earliest restore point in time for point in time restore."]
    #[serde(rename = "earliestRestorePoint", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_point: Option<time::OffsetDateTime>,
    #[doc = "Conditional. If createMode is PointInTimeRestore, this value is required. Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database."]
    #[serde(rename = "restorePointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Geo paired region."]
    #[serde(rename = "defaultSecondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub default_secondary_location: Option<String>,
    #[doc = "Collation of the metadata catalog."]
    #[serde(rename = "catalogCollation", default, skip_serializing_if = "Option::is_none")]
    pub catalog_collation: Option<managed_database_properties::CatalogCollation>,
    #[doc = "Managed database create mode. PointInTimeRestore: Create a database by restoring a point in time backup of an existing database. SourceDatabaseName, SourceManagedInstanceName and PointInTime must be specified. RestoreExternalBackup: Create a database by restoring from external backup files. Collation, StorageContainerUri and StorageContainerSasToken must be specified. Recovery: Creates a database by restoring a geo-replicated backup. RecoverableDatabaseId must be specified as the recoverable database resource ID to restore. RestoreLongTermRetentionBackup: Create a database by restoring from a long term retention backup (longTermRetentionBackupResourceId required)."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<managed_database_properties::CreateMode>,
    #[doc = "Conditional. If createMode is RestoreExternalBackup, this value is required. Specifies the uri of the storage container where backups for this restore are stored."]
    #[serde(rename = "storageContainerUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_uri: Option<String>,
    #[doc = "The resource identifier of the source database associated with create operation of this database."]
    #[serde(rename = "sourceDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub source_database_id: Option<String>,
    #[doc = "The resource identifier of the cross-subscription source database associated with create operation of this database."]
    #[serde(rename = "crossSubscriptionSourceDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub cross_subscription_source_database_id: Option<String>,
    #[doc = "The restorable dropped database resource id to restore when creating this database."]
    #[serde(rename = "restorableDroppedDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub restorable_dropped_database_id: Option<String>,
    #[doc = "The restorable cross-subscription dropped database resource id to restore when creating this database."]
    #[serde(
        rename = "crossSubscriptionRestorableDroppedDatabaseId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_subscription_restorable_dropped_database_id: Option<String>,
    #[doc = "Conditional. If createMode is RestoreExternalBackup, this value is used. Specifies the identity used for storage container authentication. Can be 'SharedAccessSignature' or 'ManagedIdentity'; if not specified 'SharedAccessSignature' is assumed."]
    #[serde(rename = "storageContainerIdentity", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_identity: Option<String>,
    #[doc = "Conditional. If createMode is RestoreExternalBackup and storageContainerIdentity is not ManagedIdentity, this value is required. Specifies the storage container sas token."]
    #[serde(rename = "storageContainerSasToken", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_token: Option<String>,
    #[doc = "Instance Failover Group resource identifier that this managed database belongs to."]
    #[serde(rename = "failoverGroupId", default, skip_serializing_if = "Option::is_none")]
    pub failover_group_id: Option<String>,
    #[doc = "The resource identifier of the recoverable database associated with create operation of this database."]
    #[serde(rename = "recoverableDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_database_id: Option<String>,
    #[doc = "The name of the Long Term Retention backup to be used for restore of this managed database."]
    #[serde(rename = "longTermRetentionBackupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub long_term_retention_backup_resource_id: Option<String>,
    #[doc = "Whether to auto complete restore of this managed database."]
    #[serde(rename = "autoCompleteRestore", default, skip_serializing_if = "Option::is_none")]
    pub auto_complete_restore: Option<bool>,
    #[doc = "Last backup file name for restore of this managed database."]
    #[serde(rename = "lastBackupName", default, skip_serializing_if = "Option::is_none")]
    pub last_backup_name: Option<String>,
    #[doc = "Target managed instance id used in cross-subscription restore."]
    #[serde(
        rename = "crossSubscriptionTargetManagedInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_subscription_target_managed_instance_id: Option<String>,
    #[doc = "Whether or not this database is a ledger database, which means all tables in the database are ledger tables. Note: the value of this property cannot be changed after the database has been created."]
    #[serde(rename = "isLedgerOn", default, skip_serializing_if = "Option::is_none")]
    pub is_ledger_on: Option<bool>,
}
impl ManagedDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_database_properties {
    use super::*;
    #[doc = "Status of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Online,
        Offline,
        Shutdown,
        Creating,
        Inaccessible,
        Restoring,
        Updating,
        Stopping,
        Stopped,
        Starting,
        DbMoving,
        DbCopying,
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
                Self::Online => serializer.serialize_unit_variant("Status", 0u32, "Online"),
                Self::Offline => serializer.serialize_unit_variant("Status", 1u32, "Offline"),
                Self::Shutdown => serializer.serialize_unit_variant("Status", 2u32, "Shutdown"),
                Self::Creating => serializer.serialize_unit_variant("Status", 3u32, "Creating"),
                Self::Inaccessible => serializer.serialize_unit_variant("Status", 4u32, "Inaccessible"),
                Self::Restoring => serializer.serialize_unit_variant("Status", 5u32, "Restoring"),
                Self::Updating => serializer.serialize_unit_variant("Status", 6u32, "Updating"),
                Self::Stopping => serializer.serialize_unit_variant("Status", 7u32, "Stopping"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 8u32, "Stopped"),
                Self::Starting => serializer.serialize_unit_variant("Status", 9u32, "Starting"),
                Self::DbMoving => serializer.serialize_unit_variant("Status", 10u32, "DbMoving"),
                Self::DbCopying => serializer.serialize_unit_variant("Status", 11u32, "DbCopying"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Collation of the metadata catalog."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CatalogCollation")]
    pub enum CatalogCollation {
        #[serde(rename = "DATABASE_DEFAULT")]
        DatabaseDefault,
        #[serde(rename = "SQL_Latin1_General_CP1_CI_AS")]
        SqlLatin1GeneralCp1CiAs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CatalogCollation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CatalogCollation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CatalogCollation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DatabaseDefault => serializer.serialize_unit_variant("CatalogCollation", 0u32, "DATABASE_DEFAULT"),
                Self::SqlLatin1GeneralCp1CiAs => {
                    serializer.serialize_unit_variant("CatalogCollation", 1u32, "SQL_Latin1_General_CP1_CI_AS")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Managed database create mode. PointInTimeRestore: Create a database by restoring a point in time backup of an existing database. SourceDatabaseName, SourceManagedInstanceName and PointInTime must be specified. RestoreExternalBackup: Create a database by restoring from external backup files. Collation, StorageContainerUri and StorageContainerSasToken must be specified. Recovery: Creates a database by restoring a geo-replicated backup. RecoverableDatabaseId must be specified as the recoverable database resource ID to restore. RestoreLongTermRetentionBackup: Create a database by restoring from a long term retention backup (longTermRetentionBackupResourceId required)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        RestoreExternalBackup,
        PointInTimeRestore,
        Recovery,
        RestoreLongTermRetentionBackup,
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
                Self::RestoreExternalBackup => serializer.serialize_unit_variant("CreateMode", 1u32, "RestoreExternalBackup"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 2u32, "PointInTimeRestore"),
                Self::Recovery => serializer.serialize_unit_variant("CreateMode", 3u32, "Recovery"),
                Self::RestoreLongTermRetentionBackup => {
                    serializer.serialize_unit_variant("CreateMode", 4u32, "RestoreLongTermRetentionBackup")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The managed database's restore details backup set properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseRestoreDetailsBackupSetProperties {
    #[doc = "Backup set status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "First stripe name."]
    #[serde(rename = "firstStripeName", default, skip_serializing_if = "Option::is_none")]
    pub first_stripe_name: Option<String>,
    #[doc = "Number of stripes."]
    #[serde(rename = "numberOfStripes", default, skip_serializing_if = "Option::is_none")]
    pub number_of_stripes: Option<i32>,
    #[doc = "Backup size."]
    #[serde(rename = "backupSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub backup_size_mb: Option<i32>,
    #[doc = "Last restored file time."]
    #[serde(rename = "restoreStartedTimestampUtc", default, with = "azure_core::date::rfc3339::option")]
    pub restore_started_timestamp_utc: Option<time::OffsetDateTime>,
    #[doc = "Last restored file time."]
    #[serde(rename = "restoreFinishedTimestampUtc", default, with = "azure_core::date::rfc3339::option")]
    pub restore_finished_timestamp_utc: Option<time::OffsetDateTime>,
}
impl ManagedDatabaseRestoreDetailsBackupSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed database's restore details properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseRestoreDetailsProperties {
    #[doc = "Restore type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Restore status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The reason why restore is in Blocked state."]
    #[serde(rename = "blockReason", default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    #[doc = "Last uploaded file name."]
    #[serde(rename = "lastUploadedFileName", default, skip_serializing_if = "Option::is_none")]
    pub last_uploaded_file_name: Option<String>,
    #[doc = "Last uploaded file time."]
    #[serde(rename = "lastUploadedFileTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_file_time: Option<time::OffsetDateTime>,
    #[doc = "Last restored file name."]
    #[serde(rename = "lastRestoredFileName", default, skip_serializing_if = "Option::is_none")]
    pub last_restored_file_name: Option<String>,
    #[doc = "Last restored file time."]
    #[serde(rename = "lastRestoredFileTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_restored_file_time: Option<time::OffsetDateTime>,
    #[doc = "Percent completed."]
    #[serde(rename = "percentCompleted", default, skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[doc = "Current restored size MB."]
    #[serde(rename = "currentRestoredSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub current_restored_size_mb: Option<i32>,
    #[doc = "Current restore plan size MB."]
    #[serde(rename = "currentRestorePlanSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub current_restore_plan_size_mb: Option<i32>,
    #[doc = "Current backup type."]
    #[serde(rename = "currentBackupType", default, skip_serializing_if = "Option::is_none")]
    pub current_backup_type: Option<String>,
    #[doc = "Current restoring file name."]
    #[serde(rename = "currentRestoringFileName", default, skip_serializing_if = "Option::is_none")]
    pub current_restoring_file_name: Option<String>,
    #[doc = "Number of files detected."]
    #[serde(rename = "numberOfFilesDetected", default, skip_serializing_if = "Option::is_none")]
    pub number_of_files_detected: Option<i32>,
    #[doc = "Number of files queued."]
    #[serde(rename = "numberOfFilesQueued", default, skip_serializing_if = "Option::is_none")]
    pub number_of_files_queued: Option<i32>,
    #[doc = "Number of files skipped."]
    #[serde(rename = "numberOfFilesSkipped", default, skip_serializing_if = "Option::is_none")]
    pub number_of_files_skipped: Option<i32>,
    #[doc = "Number of files restoring."]
    #[serde(rename = "numberOfFilesRestoring", default, skip_serializing_if = "Option::is_none")]
    pub number_of_files_restoring: Option<i32>,
    #[doc = "Number of files restored."]
    #[serde(rename = "numberOfFilesRestored", default, skip_serializing_if = "Option::is_none")]
    pub number_of_files_restored: Option<i32>,
    #[doc = "Number of files unrestorable."]
    #[serde(rename = "numberOfFilesUnrestorable", default, skip_serializing_if = "Option::is_none")]
    pub number_of_files_unrestorable: Option<i32>,
    #[doc = "Full backup sets."]
    #[serde(
        rename = "fullBackupSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub full_backup_sets: Vec<ManagedDatabaseRestoreDetailsBackupSetProperties>,
    #[doc = "Diff backup sets."]
    #[serde(
        rename = "diffBackupSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub diff_backup_sets: Vec<ManagedDatabaseRestoreDetailsBackupSetProperties>,
    #[doc = "Log backup sets."]
    #[serde(
        rename = "logBackupSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub log_backup_sets: Vec<ManagedDatabaseRestoreDetailsBackupSetProperties>,
    #[doc = "Unrestorable files."]
    #[serde(
        rename = "unrestorableFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub unrestorable_files: Vec<ManagedDatabaseRestoreDetailsUnrestorableFileProperties>,
}
impl ManagedDatabaseRestoreDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed database restore details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseRestoreDetailsResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The managed database's restore details properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedDatabaseRestoreDetailsProperties>,
}
impl ManagedDatabaseRestoreDetailsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed database's restore details unrestorable file properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseRestoreDetailsUnrestorableFileProperties {
    #[doc = "File name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ManagedDatabaseRestoreDetailsUnrestorableFileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed database security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
}
impl ManagedDatabaseSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the managed database's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseSecurityAlertPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedDatabaseSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedDatabaseSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedDatabaseSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information necessary to start a managed database move."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDatabaseStartMoveDefinition {
    #[doc = "The destination managed database ID"]
    #[serde(rename = "destinationManagedDatabaseId")]
    pub destination_managed_database_id: String,
    #[doc = "The move operation mode."]
    #[serde(rename = "operationMode", default, skip_serializing_if = "Option::is_none")]
    pub operation_mode: Option<managed_database_start_move_definition::OperationMode>,
}
impl ManagedDatabaseStartMoveDefinition {
    pub fn new(destination_managed_database_id: String) -> Self {
        Self {
            destination_managed_database_id,
            operation_mode: None,
        }
    }
}
pub mod managed_database_start_move_definition {
    use super::*;
    #[doc = "The move operation mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationMode")]
    pub enum OperationMode {
        Move,
        Copy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Move => serializer.serialize_unit_variant("OperationMode", 0u32, "Move"),
                Self::Copy => serializer.serialize_unit_variant("OperationMode", 1u32, "Copy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for OperationMode {
        fn default() -> Self {
            Self::Move
        }
    }
}
#[doc = "An managed database update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseUpdate {
    #[doc = "The managed database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedDatabaseProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedDatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL managed instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The properties of a managed instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceProperties>,
}
impl ManagedInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "An Azure SQL managed instance administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAdministrator {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a managed instance administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceAdministratorProperties>,
}
impl ManagedInstanceAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed instance administrators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAdministratorListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceAdministrator>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceAdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceAdministratorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a managed instance administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstanceAdministratorProperties {
    #[doc = "Type of the managed instance administrator."]
    #[serde(rename = "administratorType")]
    pub administrator_type: managed_instance_administrator_properties::AdministratorType,
    #[doc = "Login name of the managed instance administrator."]
    pub login: String,
    #[doc = "SID (object ID) of the managed instance administrator."]
    pub sid: String,
    #[doc = "Tenant ID of the managed instance administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedInstanceAdministratorProperties {
    pub fn new(administrator_type: managed_instance_administrator_properties::AdministratorType, login: String, sid: String) -> Self {
        Self {
            administrator_type,
            login,
            sid,
            tenant_id: None,
        }
    }
}
pub mod managed_instance_administrator_properties {
    use super::*;
    #[doc = "Type of the managed instance administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AdministratorType")]
    pub enum AdministratorType {
        ActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AdministratorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AdministratorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AdministratorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("AdministratorType", 0u32, "ActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A managed instance Advanced Threat Protection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAdvancedThreatProtection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of an Advanced Threat Protection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvancedThreatProtectionProperties>,
}
impl ManagedInstanceAdvancedThreatProtection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the managed instance's Advanced Threat Protection settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAdvancedThreatProtectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceAdvancedThreatProtection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceAdvancedThreatProtectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceAdvancedThreatProtectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of active directory only authentications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAzureAdOnlyAuthListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceAzureAdOnlyAuthentication>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceAzureAdOnlyAuthListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceAzureAdOnlyAuthListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a active directory only authentication for Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstanceAzureAdOnlyAuthProperties {
    #[doc = "Azure Active Directory only Authentication enabled."]
    #[serde(rename = "azureADOnlyAuthentication")]
    pub azure_ad_only_authentication: bool,
}
impl ManagedInstanceAzureAdOnlyAuthProperties {
    pub fn new(azure_ad_only_authentication: bool) -> Self {
        Self {
            azure_ad_only_authentication,
        }
    }
}
#[doc = "Azure Active Directory only authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAzureAdOnlyAuthentication {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a active directory only authentication for Managed Instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceAzureAdOnlyAuthProperties>,
}
impl ManagedInstanceAzureAdOnlyAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Managed Instance DTC"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceDtc {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of managed instance DTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceDtcProperties>,
}
impl ManagedInstanceDtc {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed instance's DTCs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceDtcListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceDtc>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceDtcListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceDtcListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of managed instance DTC."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceDtcProperties {
    #[doc = "Active status of managed instance DTC."]
    #[serde(rename = "dtcEnabled", default, skip_serializing_if = "Option::is_none")]
    pub dtc_enabled: Option<bool>,
    #[doc = "The Security Settings of managed instance DTC."]
    #[serde(rename = "securitySettings", default, skip_serializing_if = "Option::is_none")]
    pub security_settings: Option<ManagedInstanceDtcSecuritySettings>,
    #[doc = "External dns suffix search list of managed instance DTC."]
    #[serde(
        rename = "externalDnsSuffixSearchList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_dns_suffix_search_list: Vec<String>,
    #[doc = "Host name dns suffix of managed instance DTC."]
    #[serde(rename = "dtcHostNameDnsSuffix", default, skip_serializing_if = "Option::is_none")]
    pub dtc_host_name_dns_suffix: Option<String>,
    #[doc = "Provisioning state of managed instance DTC."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<managed_instance_dtc_properties::ProvisioningState>,
}
impl ManagedInstanceDtcProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_dtc_properties {
    use super::*;
    #[doc = "Provisioning state of managed instance DTC."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Created,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Created"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Security Settings of managed instance DTC."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceDtcSecuritySettings {
    #[doc = "The Transaction Manager Communication Settings of managed instance DTC."]
    #[serde(
        rename = "transactionManagerCommunicationSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_manager_communication_settings: Option<ManagedInstanceDtcTransactionManagerCommunicationSettings>,
    #[doc = "Allow XA Transactions to managed instance DTC."]
    #[serde(rename = "xaTransactionsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub xa_transactions_enabled: Option<bool>,
    #[doc = "Allow SNA LU 6.2 Transactions to managed instance DTC."]
    #[serde(rename = "snaLu6point2TransactionsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub sna_lu6point2_transactions_enabled: Option<bool>,
    #[doc = "Default timeout for XA Transactions (in seconds)."]
    #[serde(rename = "xaTransactionsDefaultTimeout", default, skip_serializing_if = "Option::is_none")]
    pub xa_transactions_default_timeout: Option<i32>,
    #[doc = "Maximum timeout for XA Transactions (in seconds)."]
    #[serde(rename = "xaTransactionsMaximumTimeout", default, skip_serializing_if = "Option::is_none")]
    pub xa_transactions_maximum_timeout: Option<i32>,
}
impl ManagedInstanceDtcSecuritySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Transaction Manager Communication Settings of managed instance DTC."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceDtcTransactionManagerCommunicationSettings {
    #[doc = "Allow Inbound traffic to managed instance DTC."]
    #[serde(rename = "allowInboundEnabled", default, skip_serializing_if = "Option::is_none")]
    pub allow_inbound_enabled: Option<bool>,
    #[doc = "Allow Outbound traffic of managed instance DTC."]
    #[serde(rename = "allowOutboundEnabled", default, skip_serializing_if = "Option::is_none")]
    pub allow_outbound_enabled: Option<bool>,
    #[doc = "Authentication type of managed instance DTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}
impl ManagedInstanceDtcTransactionManagerCommunicationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed server capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceEditionCapability {
    #[doc = "The managed server version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether or not this is a GPv2 variant of General Purpose edition."]
    #[serde(rename = "isGeneralPurposeV2", default, skip_serializing_if = "Option::is_none")]
    pub is_general_purpose_v2: Option<bool>,
    #[doc = "The supported families."]
    #[serde(
        rename = "supportedFamilies",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_families: Vec<ManagedInstanceFamilyCapability>,
    #[doc = "The list of supported storage capabilities for this edition"]
    #[serde(
        rename = "supportedStorageCapabilities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_capabilities: Vec<StorageCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_instance_edition_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ManagedInstanceEditionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_edition_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The managed instance encryption protector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceEncryptionProtector {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Kind of encryption protector. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties for an encryption protector execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceEncryptionProtectorProperties>,
}
impl ManagedInstanceEncryptionProtector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed instance encryption protectors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceEncryptionProtectorListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceEncryptionProtector>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceEncryptionProtectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceEncryptionProtectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for an encryption protector execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstanceEncryptionProtectorProperties {
    #[doc = "The name of the managed instance key."]
    #[serde(rename = "serverKeyName", default, skip_serializing_if = "Option::is_none")]
    pub server_key_name: Option<String>,
    #[doc = "The encryption protector type like 'ServiceManaged', 'AzureKeyVault'."]
    #[serde(rename = "serverKeyType")]
    pub server_key_type: managed_instance_encryption_protector_properties::ServerKeyType,
    #[doc = "The URI of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Thumbprint of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Key auto rotation opt-in flag. Either true or false."]
    #[serde(rename = "autoRotationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_rotation_enabled: Option<bool>,
}
impl ManagedInstanceEncryptionProtectorProperties {
    pub fn new(server_key_type: managed_instance_encryption_protector_properties::ServerKeyType) -> Self {
        Self {
            server_key_name: None,
            server_key_type,
            uri: None,
            thumbprint: None,
            auto_rotation_enabled: None,
        }
    }
}
pub mod managed_instance_encryption_protector_properties {
    use super::*;
    #[doc = "The encryption protector type like 'ServiceManaged', 'AzureKeyVault'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerKeyType")]
    pub enum ServerKeyType {
        ServiceManaged,
        AzureKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceManaged => serializer.serialize_unit_variant("ServerKeyType", 0u32, "ServiceManaged"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("ServerKeyType", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a active directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceExternalAdministrator {
    #[doc = "Type of the sever administrator."]
    #[serde(rename = "administratorType", default, skip_serializing_if = "Option::is_none")]
    pub administrator_type: Option<managed_instance_external_administrator::AdministratorType>,
    #[doc = "Principal Type of the sever administrator."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<managed_instance_external_administrator::PrincipalType>,
    #[doc = "Login name of the server administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[doc = "SID (object ID) of the server administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "Tenant ID of the administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Azure Active Directory only Authentication enabled."]
    #[serde(rename = "azureADOnlyAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_ad_only_authentication: Option<bool>,
}
impl ManagedInstanceExternalAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_external_administrator {
    use super::*;
    #[doc = "Type of the sever administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AdministratorType")]
    pub enum AdministratorType {
        ActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AdministratorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AdministratorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AdministratorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("AdministratorType", 0u32, "ActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Principal Type of the sever administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        Application,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::Application => serializer.serialize_unit_variant("PrincipalType", 2u32, "Application"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The managed server family capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceFamilyCapability {
    #[doc = "Family name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Whether or not zone redundancy is supported for the family."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "List of supported license types."]
    #[serde(
        rename = "supportedLicenseTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "List of supported virtual cores values."]
    #[serde(
        rename = "supportedVcoresValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_vcores_values: Vec<ManagedInstanceVcoresCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_instance_family_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ManagedInstanceFamilyCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_family_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A managed instance key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceKey {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Kind of encryption protector. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties for a key execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceKeyProperties>,
}
impl ManagedInstanceKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed instance keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceKeyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceKey>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceKeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceKeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a key execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstanceKeyProperties {
    #[doc = "The key type like 'ServiceManaged', 'AzureKeyVault'."]
    #[serde(rename = "serverKeyType")]
    pub server_key_type: managed_instance_key_properties::ServerKeyType,
    #[doc = "The URI of the key. If the ServerKeyType is AzureKeyVault, then the URI is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Thumbprint of the key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The key creation date."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Key auto rotation opt-in flag. Either true or false."]
    #[serde(rename = "autoRotationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_rotation_enabled: Option<bool>,
}
impl ManagedInstanceKeyProperties {
    pub fn new(server_key_type: managed_instance_key_properties::ServerKeyType) -> Self {
        Self {
            server_key_type,
            uri: None,
            thumbprint: None,
            creation_date: None,
            auto_rotation_enabled: None,
        }
    }
}
pub mod managed_instance_key_properties {
    use super::*;
    #[doc = "The key type like 'ServiceManaged', 'AzureKeyVault'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerKeyType")]
    pub enum ServerKeyType {
        ServiceManaged,
        AzureKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceManaged => serializer.serialize_unit_variant("ServerKeyType", 0u32, "ServiceManaged"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("ServerKeyType", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of managed instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A long term retention backup for a managed database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionBackup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a long term retention backup"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceLongTermRetentionBackupProperties>,
}
impl ManagedInstanceLongTermRetentionBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of long term retention backups for managed database(s)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionBackupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceLongTermRetentionBackup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceLongTermRetentionBackupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceLongTermRetentionBackupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a long term retention backup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionBackupProperties {
    #[doc = "The managed instance that the backup database belongs to."]
    #[serde(rename = "managedInstanceName", default, skip_serializing_if = "Option::is_none")]
    pub managed_instance_name: Option<String>,
    #[doc = "The create time of the instance."]
    #[serde(rename = "managedInstanceCreateTime", default, with = "azure_core::date::rfc3339::option")]
    pub managed_instance_create_time: Option<time::OffsetDateTime>,
    #[doc = "The name of the database the backup belong to"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The delete time of the database"]
    #[serde(rename = "databaseDeletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub database_deletion_time: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "backupTime", default, with = "azure_core::date::rfc3339::option")]
    pub backup_time: Option<time::OffsetDateTime>,
    #[doc = "The time the long term retention backup will expire."]
    #[serde(rename = "backupExpirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub backup_expiration_time: Option<time::OffsetDateTime>,
    #[doc = "The storage redundancy type of the backup"]
    #[serde(rename = "backupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_redundancy: Option<managed_instance_long_term_retention_backup_properties::BackupStorageRedundancy>,
    #[doc = "The BackupStorageAccessTier for the LTR backup"]
    #[serde(rename = "backupStorageAccessTier", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_access_tier: Option<managed_instance_long_term_retention_backup_properties::BackupStorageAccessTier>,
}
impl ManagedInstanceLongTermRetentionBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_long_term_retention_backup_properties {
    use super::*;
    #[doc = "The storage redundancy type of the backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageRedundancy")]
    pub enum BackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("BackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("BackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("BackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("BackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The BackupStorageAccessTier for the LTR backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageAccessTier")]
    pub enum BackupStorageAccessTier {
        Hot,
        Archive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageAccessTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageAccessTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageAccessTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hot => serializer.serialize_unit_variant("BackupStorageAccessTier", 0u32, "Hot"),
                Self::Archive => serializer.serialize_unit_variant("BackupStorageAccessTier", 1u32, "Archive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A long term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a long term retention policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceLongTermRetentionPolicyProperties>,
}
impl ManagedInstanceLongTermRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of long term retention policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceLongTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceLongTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceLongTermRetentionPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a long term retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionPolicyProperties {
    #[doc = "The BackupStorageAccessTier for the LTR backups"]
    #[serde(rename = "backupStorageAccessTier", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_access_tier: Option<managed_instance_long_term_retention_policy_properties::BackupStorageAccessTier>,
    #[doc = "The weekly retention policy for an LTR backup."]
    #[serde(rename = "weeklyRetention", default, skip_serializing_if = "Option::is_none")]
    pub weekly_retention: Option<String>,
    #[doc = "The monthly retention policy for an LTR backup."]
    #[serde(rename = "monthlyRetention", default, skip_serializing_if = "Option::is_none")]
    pub monthly_retention: Option<String>,
    #[doc = "The yearly retention policy for an LTR backup."]
    #[serde(rename = "yearlyRetention", default, skip_serializing_if = "Option::is_none")]
    pub yearly_retention: Option<String>,
    #[doc = "The week of year to take the yearly backup."]
    #[serde(rename = "weekOfYear", default, skip_serializing_if = "Option::is_none")]
    pub week_of_year: Option<i32>,
}
impl ManagedInstanceLongTermRetentionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_long_term_retention_policy_properties {
    use super::*;
    #[doc = "The BackupStorageAccessTier for the LTR backups"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageAccessTier")]
    pub enum BackupStorageAccessTier {
        Hot,
        Archive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageAccessTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageAccessTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageAccessTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hot => serializer.serialize_unit_variant("BackupStorageAccessTier", 0u32, "Hot"),
                Self::Archive => serializer.serialize_unit_variant("BackupStorageAccessTier", 1u32, "Archive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The maintenance configuration capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceMaintenanceConfigurationCapability {
    #[doc = "Maintenance configuration name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_instance_maintenance_configuration_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ManagedInstanceMaintenanceConfigurationCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_maintenance_configuration_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A managed instance operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a managed instance operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceOperationProperties>,
}
impl ManagedInstanceOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list managed instance operations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceOperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters of a managed instance operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceOperationParametersPair {
    #[serde(rename = "currentParameters", default, skip_serializing_if = "Option::is_none")]
    pub current_parameters: Option<UpsertManagedServerOperationParameters>,
    #[serde(rename = "requestedParameters", default, skip_serializing_if = "Option::is_none")]
    pub requested_parameters: Option<UpsertManagedServerOperationParameters>,
}
impl ManagedInstanceOperationParametersPair {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a managed instance operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceOperationProperties {
    #[doc = "The name of the managed instance the operation is being performed on."]
    #[serde(rename = "managedInstanceName", default, skip_serializing_if = "Option::is_none")]
    pub managed_instance_name: Option<String>,
    #[doc = "The name of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The friendly name of operation."]
    #[serde(rename = "operationFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub operation_friendly_name: Option<String>,
    #[doc = "The percentage of the operation completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The operation start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<managed_instance_operation_properties::State>,
    #[doc = "The operation error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The operation error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The operation error severity."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "Whether or not the error is a user error."]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
    #[doc = "The estimated completion time of the operation."]
    #[serde(rename = "estimatedCompletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub estimated_completion_time: Option<time::OffsetDateTime>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the operation can be cancelled."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
    #[doc = "The parameters of a managed instance operation."]
    #[serde(rename = "operationParameters", default, skip_serializing_if = "Option::is_none")]
    pub operation_parameters: Option<ManagedInstanceOperationParametersPair>,
    #[doc = "The steps of a managed instance operation."]
    #[serde(rename = "operationSteps", default, skip_serializing_if = "Option::is_none")]
    pub operation_steps: Option<ManagedInstanceOperationSteps>,
}
impl ManagedInstanceOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_operation_properties {
    use super::*;
    #[doc = "The operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        InProgress,
        Succeeded,
        Failed,
        CancelInProgress,
        Cancelled,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::CancelInProgress => serializer.serialize_unit_variant("State", 4u32, "CancelInProgress"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 5u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The steps of a managed instance operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceOperationSteps {
    #[doc = "The total number of operation steps."]
    #[serde(rename = "totalSteps", default, skip_serializing_if = "Option::is_none")]
    pub total_steps: Option<String>,
    #[doc = "The number of current operation steps."]
    #[serde(rename = "currentStep", default, skip_serializing_if = "Option::is_none")]
    pub current_step: Option<i32>,
    #[doc = "The operation steps list."]
    #[serde(
        rename = "stepsList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub steps_list: Vec<UpsertManagedServerOperationStepWithEstimatesAndDuration>,
}
impl ManagedInstanceOperationSteps {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pairs of Managed Instances in the failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePairInfo {
    #[doc = "Id of Primary Managed Instance in pair."]
    #[serde(rename = "primaryManagedInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub primary_managed_instance_id: Option<String>,
    #[doc = "Id of Partner Managed Instance in pair."]
    #[serde(rename = "partnerManagedInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub partner_managed_instance_id: Option<String>,
}
impl ManagedInstancePairInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection under a managed instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePecProperty {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstancePrivateEndpointConnectionProperties>,
}
impl ManagedInstancePecProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstancePrivateEndpointConnectionProperties>,
}
impl ManagedInstancePrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateEndpointConnectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstancePrivateEndpointConnection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstancePrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstancePrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<ManagedInstancePrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ManagedInstancePrivateLinkServiceConnectionStateProperty>,
    #[doc = "State of the Private Endpoint Connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ManagedInstancePrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateEndpointProperty {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ManagedInstancePrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstancePrivateLinkProperties>,
}
impl ManagedInstancePrivateLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateLinkListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstancePrivateLink>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstancePrivateLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstancePrivateLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstancePrivateLinkProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource required zone names."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl ManagedInstancePrivateLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstancePrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    pub status: String,
    #[doc = "The private link service connection description."]
    pub description: String,
    #[doc = "The private link service connection description."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl ManagedInstancePrivateLinkServiceConnectionStateProperty {
    pub fn new(status: String, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
    }
}
#[doc = "The properties of a managed instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceProperties {
    #[doc = "Provisioning state of managed instance."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<managed_instance_properties::ProvisioningState>,
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: Regular instance creation.\r\n\r\nRestore: Creates an instance by restoring a set of backups to specific point in time. RestorePointInTime and SourceManagedInstanceId must be specified."]
    #[serde(rename = "managedInstanceCreateMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_instance_create_mode: Option<managed_instance_properties::ManagedInstanceCreateMode>,
    #[doc = "The fully qualified domain name of the managed instance."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[doc = "Whether or not this is a GPv2 variant of General Purpose edition."]
    #[serde(rename = "isGeneralPurposeV2", default, skip_serializing_if = "Option::is_none")]
    pub is_general_purpose_v2: Option<bool>,
    #[doc = "Administrator username for the managed instance. Can only be specified when the managed instance is being created (and is required for creation)."]
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[doc = "The administrator login password (required for managed instance creation)."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "Subnet resource ID for the managed instance."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The state of the managed instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The license type. Possible values are 'LicenseIncluded' (regular price inclusive of a new SQL license) and 'BasePrice' (discounted AHB price for bringing your own SQL licenses)."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<managed_instance_properties::LicenseType>,
    #[doc = "Hybrid secondary usage. Possible values are 'Active' (default value) and 'Passive' (customer uses the secondary as Passive DR)."]
    #[serde(rename = "hybridSecondaryUsage", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_secondary_usage: Option<managed_instance_properties::HybridSecondaryUsage>,
    #[doc = "Hybrid secondary usage detected. Possible values are 'Active' (customer does not meet the requirements to use the secondary as Passive DR) and 'Passive' (customer meets the requirements to use the secondary as Passive DR)."]
    #[serde(rename = "hybridSecondaryUsageDetected", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_secondary_usage_detected: Option<managed_instance_properties::HybridSecondaryUsageDetected>,
    #[doc = "The number of vCores. Allowed values: 8, 16, 24, 32, 40, 64, 80."]
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i32>,
    #[doc = "Storage size in GB. Minimum value: 32. Maximum value: 16384. Increments of 32 GB allowed only. Maximum value depends on the selected hardware family and number of vCores."]
    #[serde(rename = "storageSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_in_gb: Option<i32>,
    #[doc = "Storage IOps. Minimum value: 120. Maximum value: 120000. Increments of 1 IOps allowed only. Maximum value depends on the selected hardware family and number of vCores."]
    #[serde(rename = "storageIOps", default, skip_serializing_if = "Option::is_none")]
    pub storage_i_ops: Option<i32>,
    #[doc = "Storage throughput in MBps. Minimum value: 25. Maximum value: 4000. Increments of 1 MBps allowed only. Maximum value depends on the selected hardware family and number of vCores."]
    #[serde(rename = "storageThroughputMBps", default, skip_serializing_if = "Option::is_none")]
    pub storage_throughput_m_bps: Option<i32>,
    #[doc = "Collation of the managed instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "The Dns Zone that the managed instance is in."]
    #[serde(rename = "dnsZone", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone: Option<String>,
    #[doc = "The resource id of another managed instance whose DNS zone this managed instance will share after creation."]
    #[serde(rename = "dnsZonePartner", default, skip_serializing_if = "Option::is_none")]
    pub dns_zone_partner: Option<String>,
    #[doc = "Whether or not the public data endpoint is enabled."]
    #[serde(rename = "publicDataEndpointEnabled", default, skip_serializing_if = "Option::is_none")]
    pub public_data_endpoint_enabled: Option<bool>,
    #[doc = "The resource identifier of the source managed instance associated with create operation of this instance."]
    #[serde(rename = "sourceManagedInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub source_managed_instance_id: Option<String>,
    #[doc = "Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database."]
    #[serde(rename = "restorePointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Connection type used for connecting to the instance."]
    #[serde(rename = "proxyOverride", default, skip_serializing_if = "Option::is_none")]
    pub proxy_override: Option<managed_instance_properties::ProxyOverride>,
    #[doc = "Id of the timezone. Allowed values are timezones supported by Windows.\r\nWindows keeps details on supported timezones, including the id, in registry under\r\nKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Time Zones.\r\nYou can get those registry values via SQL Server by querying SELECT name AS timezone_id FROM sys.time_zone_info.\r\nList of Ids can also be obtained by executing [System.TimeZoneInfo]::GetSystemTimeZones() in PowerShell.\r\nAn example of valid timezone id is \"Pacific Standard Time\" or \"W. Europe Standard Time\"."]
    #[serde(rename = "timezoneId", default, skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<String>,
    #[doc = "The Id of the instance pool this managed server belongs to."]
    #[serde(rename = "instancePoolId", default, skip_serializing_if = "Option::is_none")]
    pub instance_pool_id: Option<String>,
    #[doc = "Specifies maintenance configuration id to apply to this managed instance."]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "List of private endpoint connections on a managed instance."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<ManagedInstancePecProperty>,
    #[doc = "Minimal TLS version. Allowed values: 'None', '1.0', '1.1', '1.2'"]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<String>,
    #[doc = "The storage account type used to store backups for this instance. The options are Local (LocallyRedundantStorage), Zone (ZoneRedundantStorage), Geo (GeoRedundantStorage) and GeoZone(GeoZoneRedundantStorage)"]
    #[serde(rename = "currentBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub current_backup_storage_redundancy: Option<managed_instance_properties::CurrentBackupStorageRedundancy>,
    #[doc = "The storage account type to be used to store backups for this instance. The options are Local (LocallyRedundantStorage), Zone (ZoneRedundantStorage), Geo (GeoRedundantStorage) and GeoZone(GeoZoneRedundantStorage)"]
    #[serde(rename = "requestedBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub requested_backup_storage_redundancy: Option<managed_instance_properties::RequestedBackupStorageRedundancy>,
    #[doc = "Whether or not the multi-az is enabled."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The resource id of a user assigned identity to be used by default."]
    #[serde(rename = "primaryUserAssignedIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_assigned_identity_id: Option<String>,
    #[doc = "A CMK URI of the key to use for encryption."]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[doc = "Properties of a active directory administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrators: Option<ManagedInstanceExternalAdministrator>,
    #[doc = "The managed instance's service principal configuration for a resource."]
    #[serde(rename = "servicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<ServicePrincipal>,
    #[doc = "Virtual cluster resource id for the Managed Instance."]
    #[serde(rename = "virtualClusterId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
    #[doc = "Status of external governance."]
    #[serde(rename = "externalGovernanceStatus", default, skip_serializing_if = "Option::is_none")]
    pub external_governance_status: Option<managed_instance_properties::ExternalGovernanceStatus>,
    #[doc = "Weather or not Managed Instance is freemium."]
    #[serde(rename = "pricingModel", default, skip_serializing_if = "Option::is_none")]
    pub pricing_model: Option<managed_instance_properties::PricingModel>,
    #[doc = "Specifies the point in time (ISO8601 format) of the Managed Instance creation."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The managed instance's authentication metadata lookup mode."]
    #[serde(rename = "authenticationMetadata", default, skip_serializing_if = "Option::is_none")]
    pub authentication_metadata: Option<managed_instance_properties::AuthenticationMetadata>,
    #[doc = "Specifies the internal format of instance databases specific to the SQL engine version."]
    #[serde(rename = "databaseFormat", default, skip_serializing_if = "Option::is_none")]
    pub database_format: Option<managed_instance_properties::DatabaseFormat>,
}
impl ManagedInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_properties {
    use super::*;
    #[doc = "Provisioning state of managed instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Created,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Created"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: Regular instance creation.\r\n\r\nRestore: Creates an instance by restoring a set of backups to specific point in time. RestorePointInTime and SourceManagedInstanceId must be specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ManagedInstanceCreateMode")]
    pub enum ManagedInstanceCreateMode {
        Default,
        PointInTimeRestore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ManagedInstanceCreateMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ManagedInstanceCreateMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ManagedInstanceCreateMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("ManagedInstanceCreateMode", 0u32, "Default"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("ManagedInstanceCreateMode", 1u32, "PointInTimeRestore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The license type. Possible values are 'LicenseIncluded' (regular price inclusive of a new SQL license) and 'BasePrice' (discounted AHB price for bringing your own SQL licenses)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        LicenseIncluded,
        BasePrice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 0u32, "LicenseIncluded"),
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 1u32, "BasePrice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Hybrid secondary usage. Possible values are 'Active' (default value) and 'Passive' (customer uses the secondary as Passive DR)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridSecondaryUsage")]
    pub enum HybridSecondaryUsage {
        Active,
        Passive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridSecondaryUsage {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridSecondaryUsage {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridSecondaryUsage {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("HybridSecondaryUsage", 0u32, "Active"),
                Self::Passive => serializer.serialize_unit_variant("HybridSecondaryUsage", 1u32, "Passive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Hybrid secondary usage detected. Possible values are 'Active' (customer does not meet the requirements to use the secondary as Passive DR) and 'Passive' (customer meets the requirements to use the secondary as Passive DR)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridSecondaryUsageDetected")]
    pub enum HybridSecondaryUsageDetected {
        Active,
        Passive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridSecondaryUsageDetected {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridSecondaryUsageDetected {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridSecondaryUsageDetected {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("HybridSecondaryUsageDetected", 0u32, "Active"),
                Self::Passive => serializer.serialize_unit_variant("HybridSecondaryUsageDetected", 1u32, "Passive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Connection type used for connecting to the instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProxyOverride")]
    pub enum ProxyOverride {
        Proxy,
        Redirect,
        Default,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProxyOverride {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProxyOverride {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProxyOverride {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Proxy => serializer.serialize_unit_variant("ProxyOverride", 0u32, "Proxy"),
                Self::Redirect => serializer.serialize_unit_variant("ProxyOverride", 1u32, "Redirect"),
                Self::Default => serializer.serialize_unit_variant("ProxyOverride", 2u32, "Default"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type used to store backups for this instance. The options are Local (LocallyRedundantStorage), Zone (ZoneRedundantStorage), Geo (GeoRedundantStorage) and GeoZone(GeoZoneRedundantStorage)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentBackupStorageRedundancy")]
    pub enum CurrentBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("CurrentBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The storage account type to be used to store backups for this instance. The options are Local (LocallyRedundantStorage), Zone (ZoneRedundantStorage), Geo (GeoRedundantStorage) and GeoZone(GeoZoneRedundantStorage)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestedBackupStorageRedundancy")]
    pub enum RequestedBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestedBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestedBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestedBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of external governance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExternalGovernanceStatus")]
    pub enum ExternalGovernanceStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExternalGovernanceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExternalGovernanceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExternalGovernanceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ExternalGovernanceStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ExternalGovernanceStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Weather or not Managed Instance is freemium."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PricingModel")]
    pub enum PricingModel {
        Regular,
        Freemium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PricingModel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PricingModel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PricingModel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Regular => serializer.serialize_unit_variant("PricingModel", 0u32, "Regular"),
                Self::Freemium => serializer.serialize_unit_variant("PricingModel", 1u32, "Freemium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The managed instance's authentication metadata lookup mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationMetadata")]
    pub enum AuthenticationMetadata {
        #[serde(rename = "AzureAD")]
        AzureAd,
        Paired,
        Windows,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationMetadata {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationMetadata {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationMetadata {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureAd => serializer.serialize_unit_variant("AuthenticationMetadata", 0u32, "AzureAD"),
                Self::Paired => serializer.serialize_unit_variant("AuthenticationMetadata", 1u32, "Paired"),
                Self::Windows => serializer.serialize_unit_variant("AuthenticationMetadata", 2u32, "Windows"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the internal format of instance databases specific to the SQL engine version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DatabaseFormat")]
    pub enum DatabaseFormat {
        AlwaysUpToDate,
        #[serde(rename = "SQLServer2022")]
        SqlServer2022,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DatabaseFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DatabaseFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DatabaseFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AlwaysUpToDate => serializer.serialize_unit_variant("DatabaseFormat", 0u32, "AlwaysUpToDate"),
                Self::SqlServer2022 => serializer.serialize_unit_variant("DatabaseFormat", 1u32, "SQLServer2022"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Database query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceQuery {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a database query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
impl ManagedInstanceQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Execution statistics for one particular query"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceQueryStatistics {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<QueryStatistics>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceQueryStatistics {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceQueryStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An update request for an Azure SQL Database managed instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceUpdate {
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The properties of a managed instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed instance virtual cores capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceVcoresCapability {
    #[doc = "The virtual cores identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The virtual cores value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "includedMaxSize", default, skip_serializing_if = "Option::is_none")]
    pub included_max_size: Option<MaxSizeCapability>,
    #[doc = "Storage size ranges."]
    #[serde(
        rename = "supportedStorageSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_storage_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "Included storage IOps."]
    #[serde(rename = "includedStorageIOps", default, skip_serializing_if = "Option::is_none")]
    pub included_storage_i_ops: Option<i64>,
    #[doc = "The maximum limit range capability."]
    #[serde(rename = "supportedStorageIOps", default, skip_serializing_if = "Option::is_none")]
    pub supported_storage_i_ops: Option<MaxLimitRangeCapability>,
    #[doc = "Min IOps override factor per selected storage GB."]
    #[serde(
        rename = "iopsMinValueOverrideFactorPerSelectedStorageGB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_min_value_override_factor_per_selected_storage_gb: Option<f64>,
    #[doc = "Included IOps override factor per selected storage GB."]
    #[serde(
        rename = "iopsIncludedValueOverrideFactorPerSelectedStorageGB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_included_value_override_factor_per_selected_storage_gb: Option<f64>,
    #[doc = "Included storage throughput MBps."]
    #[serde(rename = "includedStorageThroughputMBps", default, skip_serializing_if = "Option::is_none")]
    pub included_storage_throughput_m_bps: Option<i64>,
    #[doc = "The maximum limit range capability."]
    #[serde(rename = "supportedStorageThroughputMBps", default, skip_serializing_if = "Option::is_none")]
    pub supported_storage_throughput_m_bps: Option<MaxLimitRangeCapability>,
    #[doc = "Min throughput MBps override factor per selected storage GB."]
    #[serde(
        rename = "throughputMBpsMinValueOverrideFactorPerSelectedStorageGB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub throughput_m_bps_min_value_override_factor_per_selected_storage_gb: Option<f64>,
    #[doc = "Included throughput MBps override factor per selected storage GB."]
    #[serde(
        rename = "throughputMBpsIncludedValueOverrideFactorPerSelectedStorageGB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub throughput_m_bps_included_value_override_factor_per_selected_storage_gb: Option<f64>,
    #[doc = "True if this service objective is supported for managed instances in an instance pool."]
    #[serde(rename = "instancePoolSupported", default, skip_serializing_if = "Option::is_none")]
    pub instance_pool_supported: Option<bool>,
    #[doc = "True if this service objective is supported for standalone managed instances."]
    #[serde(rename = "standaloneSupported", default, skip_serializing_if = "Option::is_none")]
    pub standalone_supported: Option<bool>,
    #[doc = "List of supported maintenance configurations"]
    #[serde(
        rename = "supportedMaintenanceConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_maintenance_configurations: Vec<ManagedInstanceMaintenanceConfigurationCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_instance_vcores_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ManagedInstanceVcoresCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_vcores_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The managed instance capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceVersionCapability {
    #[doc = "The server version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported managed instance editions."]
    #[serde(
        rename = "supportedEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_editions: Vec<ManagedInstanceEditionCapability>,
    #[doc = "The list of supported instance pool editions."]
    #[serde(
        rename = "supportedInstancePoolEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_instance_pool_editions: Vec<InstancePoolEditionCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<managed_instance_version_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ManagedInstanceVersionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_version_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A managed instance vulnerability assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceVulnerabilityAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a managed instance vulnerability assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedInstanceVulnerabilityAssessmentProperties>,
}
impl ManagedInstanceVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the ManagedInstance's vulnerability assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceVulnerabilityAssessmentListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstanceVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedInstanceVulnerabilityAssessmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a managed instance vulnerability assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstanceVulnerabilityAssessmentProperties {
    #[doc = "A blob storage container path to hold the scan results (e.g. https://myStorage.blob.core.windows.net/VaScans/)."]
    #[serde(rename = "storageContainerPath")]
    pub storage_container_path: String,
    #[doc = "A shared access signature (SAS Key) that has write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required. Applies only if the storage account is not behind a Vnet or a firewall"]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required. Applies only if the storage account is not behind a Vnet or a firewall"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Properties of a Vulnerability Assessment recurring scans."]
    #[serde(rename = "recurringScans", default, skip_serializing_if = "Option::is_none")]
    pub recurring_scans: Option<VulnerabilityAssessmentRecurringScansProperties>,
}
impl ManagedInstanceVulnerabilityAssessmentProperties {
    pub fn new(storage_container_path: String) -> Self {
        Self {
            storage_container_path,
            storage_container_sas_key: None,
            storage_account_access_key: None,
            recurring_scans: None,
        }
    }
}
#[doc = "Azure SQL Database ledger digest upload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedLedgerDigestUploads {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database ledger digest upload settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedLedgerDigestUploadsProperties>,
}
impl ManagedLedgerDigestUploads {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of ledger digest upload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedLedgerDigestUploadsListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedLedgerDigestUploads>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedLedgerDigestUploadsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedLedgerDigestUploadsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a database ledger digest upload settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedLedgerDigestUploadsProperties {
    #[doc = "The digest storage endpoint, which must be either an Azure blob storage endpoint or an URI for Azure Confidential Ledger."]
    #[serde(rename = "digestStorageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub digest_storage_endpoint: Option<String>,
    #[doc = "Specifies the state of ledger digest upload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<managed_ledger_digest_uploads_properties::State>,
}
impl ManagedLedgerDigestUploadsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_ledger_digest_uploads_properties {
    use super::*;
    #[doc = "Specifies the state of ledger digest upload."]
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
}
#[doc = "A managed server DNS alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerDnsAlias {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a managed server DNS alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedServerDnsAliasProperties>,
}
impl ManagedServerDnsAlias {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed server DNS alias acquisition request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServerDnsAliasAcquisition {
    #[doc = "The resource ID of the managed server DNS alias that will be acquired to point to this managed server instead."]
    #[serde(rename = "oldManagedServerDnsAliasResourceId")]
    pub old_managed_server_dns_alias_resource_id: String,
}
impl ManagedServerDnsAliasAcquisition {
    pub fn new(old_managed_server_dns_alias_resource_id: String) -> Self {
        Self {
            old_managed_server_dns_alias_resource_id,
        }
    }
}
#[doc = "A managed server dns alias creation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerDnsAliasCreation {
    #[doc = "Whether or not DNS record should be created for this alias."]
    #[serde(rename = "createDnsRecord", default, skip_serializing_if = "Option::is_none")]
    pub create_dns_record: Option<bool>,
}
impl ManagedServerDnsAliasCreation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed server DNS aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerDnsAliasListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedServerDnsAlias>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedServerDnsAliasListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedServerDnsAliasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a managed server DNS alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerDnsAliasProperties {
    #[doc = "The fully qualified DNS record for managed server alias"]
    #[serde(rename = "azureDnsRecord", default, skip_serializing_if = "Option::is_none")]
    pub azure_dns_record: Option<String>,
    #[doc = "The fully qualified public DNS record for managed server alias"]
    #[serde(rename = "publicAzureDnsRecord", default, skip_serializing_if = "Option::is_none")]
    pub public_azure_dns_record: Option<String>,
}
impl ManagedServerDnsAliasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed server security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertsPolicyProperties>,
}
impl ManagedServerSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the managed Server's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerSecurityAlertPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedServerSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedServerSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedServerSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed database transparent data encryption state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedTransparentDataEncryption {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a transparent data encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedTransparentDataEncryptionProperties>,
}
impl ManagedTransparentDataEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed transparent data encryptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedTransparentDataEncryptionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedTransparentDataEncryption>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedTransparentDataEncryptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedTransparentDataEncryptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a transparent data encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedTransparentDataEncryptionProperties {
    #[doc = "Specifies the state of the transparent data encryption."]
    pub state: managed_transparent_data_encryption_properties::State,
}
impl ManagedTransparentDataEncryptionProperties {
    pub fn new(state: managed_transparent_data_encryption_properties::State) -> Self {
        Self { state }
    }
}
pub mod managed_transparent_data_encryption_properties {
    use super::*;
    #[doc = "Specifies the state of the transparent data encryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "The maximum limit range capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaxLimitRangeCapability {
    #[doc = "Minimum value."]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    #[doc = "Maximum value."]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    #[doc = "Scale/step size for discrete values between the minimum value and the maximum value."]
    #[serde(rename = "scaleSize", default, skip_serializing_if = "Option::is_none")]
    pub scale_size: Option<i64>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<max_limit_range_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl MaxLimitRangeCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod max_limit_range_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The maximum size capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaxSizeCapability {
    #[doc = "The maximum size limit (see 'unit' for the units)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The units that the limit is expressed in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<max_size_capability::Unit>,
}
impl MaxSizeCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod max_size_capability {
    use super::*;
    #[doc = "The units that the limit is expressed in."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Megabytes,
        Gigabytes,
        Terabytes,
        Petabytes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Megabytes => serializer.serialize_unit_variant("Unit", 0u32, "Megabytes"),
                Self::Gigabytes => serializer.serialize_unit_variant("Unit", 1u32, "Gigabytes"),
                Self::Terabytes => serializer.serialize_unit_variant("Unit", 2u32, "Terabytes"),
                Self::Petabytes => serializer.serialize_unit_variant("Unit", 3u32, "Petabytes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The maximum size range capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaxSizeRangeCapability {
    #[doc = "The maximum size capability."]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<MaxSizeCapability>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<MaxSizeCapability>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "scaleSize", default, skip_serializing_if = "Option::is_none")]
    pub scale_size: Option<MaxSizeCapability>,
    #[doc = "The log size capability."]
    #[serde(rename = "logSize", default, skip_serializing_if = "Option::is_none")]
    pub log_size: Option<LogSizeCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<max_size_range_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl MaxSizeRangeCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod max_size_range_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The min capacity capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MinCapacityCapability {
    #[doc = "Min capacity value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<min_capacity_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl MinCapacityCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod min_capacity_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspConfigAccessRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspConfigAccessRuleProperties>,
}
impl NspConfigAccessRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspConfigAccessRuleProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(
        rename = "addressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub address_prefixes: Vec<String>,
    #[serde(
        rename = "fullyQualifiedDomainNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fully_qualified_domain_names: Vec<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscriptions: Vec<String>,
    #[serde(
        rename = "networkSecurityPerimeters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_security_perimeters: Vec<NspConfigNetworkSecurityPerimeterRule>,
}
impl NspConfigAccessRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspConfigAssociation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "accessMode", default, skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
}
impl NspConfigAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspConfigNetworkSecurityPerimeterRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "perimeterGuid", default, skip_serializing_if = "Option::is_none")]
    pub perimeter_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl NspConfigNetworkSecurityPerimeterRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspConfigPerimeter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "perimeterGuid", default, skip_serializing_if = "Option::is_none")]
    pub perimeter_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl NspConfigPerimeter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspConfigProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "accessRulesVersion", default, skip_serializing_if = "Option::is_none")]
    pub access_rules_version: Option<String>,
    #[serde(
        rename = "accessRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub access_rules: Vec<NspConfigAccessRule>,
}
impl NspConfigProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspProvisioningIssue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NspProvisioningIssueProperties>,
}
impl NspProvisioningIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NspProvisioningIssueProperties {
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "suggestedResourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suggested_resource_ids: Vec<String>,
    #[serde(
        rename = "suggestedAccessRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suggested_access_rules: Vec<String>,
}
impl NspProvisioningIssueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM Usage Name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Name {
    #[doc = "Usage name value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Usage name localized value."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl Name {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the ARM resources for which to create private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkIsolationSettings {
    #[doc = "The resource id for the storage account used to store BACPAC file. If set, private endpoint connection will be created for the storage account. Must match storage account used for StorageUri parameter."]
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[doc = "The resource id for the SQL server which is the target of this request. If set, private endpoint connection will be created for the SQL server. Must match server which is target of the operation."]
    #[serde(rename = "sqlServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_resource_id: Option<String>,
}
impl NetworkIsolationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NSP Configuration for a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityPerimeterConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an NSP config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSecurityPerimeterConfigurationProperties>,
}
impl NetworkSecurityPerimeterConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of NSP configurations for a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityPerimeterConfigurationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkSecurityPerimeterConfiguration>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkSecurityPerimeterConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkSecurityPerimeterConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an NSP config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkSecurityPerimeterConfigurationProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "networkSecurityPerimeter", default, skip_serializing_if = "Option::is_none")]
    pub network_security_perimeter: Option<NspConfigPerimeter>,
    #[serde(rename = "resourceAssociation", default, skip_serializing_if = "Option::is_none")]
    pub resource_association: Option<NspConfigAssociation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<NspConfigProfile>,
    #[serde(
        rename = "provisioningIssues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub provisioning_issues: Vec<NspProvisioningIssue>,
}
impl NetworkSecurityPerimeterConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
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
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Display metadata associated with the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The localized friendly form of the resource provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly form of the resource type related to this action/operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list SQL operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An endpoint that the managed instance service requires outbound network access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The type of service accessed by the managed instance service, e.g., Azure Storage, Azure Active Directory, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that the managed instance service communicates with in order to function correctly."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of endpoints that the managed instance service requires outbound network access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpointCollection {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEnvironmentEndpointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OutboundEnvironmentEndpointCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL DB Server Outbound Firewall Rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundFirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an outbound firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OutboundFirewallRuleProperties>,
}
impl OutboundFirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of outbound rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundFirewallRuleListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OutboundFirewallRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundFirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OutboundFirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an outbound firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundFirewallRuleProperties {
    #[doc = "The state of the outbound rule."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl OutboundFirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Partner server information for the failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerInfo {
    #[doc = "Resource identifier of the partner server."]
    pub id: String,
    #[doc = "Geo location of the partner server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Replication role of the partner server."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<partner_info::ReplicationRole>,
}
impl PartnerInfo {
    pub fn new(id: String) -> Self {
        Self {
            id,
            location: None,
            replication_role: None,
        }
    }
}
pub mod partner_info {
    use super::*;
    #[doc = "Replication role of the partner server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationRole")]
    pub enum ReplicationRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ReplicationRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("ReplicationRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Partner region information for the failover group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerRegionInfo {
    #[doc = "Geo location of the partner managed instances."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Replication role of the partner managed instances."]
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<partner_region_info::ReplicationRole>,
}
impl PartnerRegionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod partner_region_info {
    use super::*;
    #[doc = "Replication role of the partner managed instances."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationRole")]
    pub enum ReplicationRole {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("ReplicationRole", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("ReplicationRole", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Supported auto pause delay time range"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerDatabaseAutoPauseDelayTimeRange {
    #[doc = "Minimum value"]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[doc = "Maximum value"]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[doc = "Step value for discrete values between the minimum value and the maximum value."]
    #[serde(rename = "stepSize", default, skip_serializing_if = "Option::is_none")]
    pub step_size: Option<i32>,
    #[doc = "Default value if no value is provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Unit of time that delay is expressed in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<per_database_auto_pause_delay_time_range::Unit>,
    #[doc = "Value that is used to not pause (infinite delay before pause)"]
    #[serde(rename = "doNotPauseValue", default, skip_serializing_if = "Option::is_none")]
    pub do_not_pause_value: Option<i32>,
}
impl PerDatabaseAutoPauseDelayTimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod per_database_auto_pause_delay_time_range {
    use super::*;
    #[doc = "Unit of time that delay is expressed in"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Minutes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Minutes => serializer.serialize_unit_variant("Unit", 0u32, "Minutes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The performance level capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceLevelCapability {
    #[doc = "Performance level value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Unit type used to measure performance level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<performance_level_capability::Unit>,
}
impl PerformanceLevelCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod performance_level_capability {
    use super::*;
    #[doc = "Unit type used to measure performance level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        #[serde(rename = "DTU")]
        Dtu,
        VCores,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dtu => serializer.serialize_unit_variant("Unit", 0u32, "DTU"),
                Self::VCores => serializer.serialize_unit_variant("Unit", 1u32, "VCores"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The phase details properties of a database operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PhaseDetails {
    #[doc = "The operation phase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<phase_details::Phase>,
    #[doc = "The operation phase information."]
    #[serde(rename = "phaseInformation", default, skip_serializing_if = "Option::is_none")]
    pub phase_information: Option<serde_json::Value>,
}
impl PhaseDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod phase_details {
    use super::*;
    #[doc = "The operation phase."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Phase")]
    pub enum Phase {
        Copying,
        Catchup,
        WaitingForCutover,
        CutoverInProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Phase {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Phase {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Phase {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Copying => serializer.serialize_unit_variant("Phase", 0u32, "Copying"),
                Self::Catchup => serializer.serialize_unit_variant("Phase", 1u32, "Catchup"),
                Self::WaitingForCutover => serializer.serialize_unit_variant("Phase", 2u32, "WaitingForCutover"),
                Self::CutoverInProgress => serializer.serialize_unit_variant("Phase", 3u32, "CutoverInProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[doc = "Group IDs."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "State of the private endpoint connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Approving,
        Ready,
        Dropping,
        Failed,
        Rejecting,
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
                Self::Approving => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Approving"),
                Self::Ready => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Ready"),
                Self::Dropping => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Dropping"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Rejecting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Rejecting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains the private endpoint connection requests status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionRequestStatus {
    #[doc = "Resource id for which the private endpoint is created."]
    #[serde(rename = "privateLinkServiceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_id: Option<String>,
    #[doc = "The connection name for the private endpoint."]
    #[serde(rename = "privateEndpointConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_connection_name: Option<String>,
    #[doc = "Status of this private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl PrivateEndpointConnectionRequestStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource required zone names."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    pub status: private_link_service_connection_state_property::Status,
    #[doc = "The private link service connection description."]
    pub description: String,
    #[doc = "The actions required for private link service connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<private_link_service_connection_state_property::ActionsRequired>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new(status: private_link_service_connection_state_property::Status, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
    }
}
pub mod private_link_service_connection_state_property {
    use super::*;
    #[doc = "The private link service connection status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Approved,
        Pending,
        Rejected,
        Disconnected,
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
                Self::Approved => serializer.serialize_unit_variant("Status", 0u32, "Approved"),
                Self::Pending => serializer.serialize_unit_variant("Status", 1u32, "Pending"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The actions required for private link service connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionsRequired")]
    pub enum ActionsRequired {
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionsRequired {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionsRequired {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionsRequired {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ActionsRequired", 0u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ARM proxy resource."]
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
#[doc = "ARM proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResourceWithWritableName {
    #[serde(flatten)]
    pub resource_with_writable_name: ResourceWithWritableName,
}
impl ProxyResourceWithWritableName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Vulnerability Assessment query check object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryCheck {
    #[doc = "SQL Vulnerability Assessment rule query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "SQL Vulnerability Assessment query expected result."]
    #[serde(
        rename = "expectedResult",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expected_result: Vec<Vec<String>>,
    #[doc = "SQL Vulnerability Assessment column names of query expected result."]
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
#[doc = "Properties of a query metrics interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryMetricInterval {
    #[doc = "The start time for the metric interval (ISO-8601 format)."]
    #[serde(rename = "intervalStartTime", default, skip_serializing_if = "Option::is_none")]
    pub interval_start_time: Option<String>,
    #[doc = "Interval type (length)."]
    #[serde(rename = "intervalType", default, skip_serializing_if = "Option::is_none")]
    pub interval_type: Option<query_metric_interval::IntervalType>,
    #[doc = "Execution count of a query in this interval."]
    #[serde(rename = "executionCount", default, skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<i64>,
    #[doc = "List of metric objects for this interval"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<QueryMetricProperties>,
}
impl QueryMetricInterval {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_metric_interval {
    use super::*;
    #[doc = "Interval type (length)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IntervalType")]
    pub enum IntervalType {
        #[serde(rename = "PT1H")]
        Pt1h,
        #[serde(rename = "P1D")]
        P1d,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IntervalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IntervalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IntervalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pt1h => serializer.serialize_unit_variant("IntervalType", 0u32, "PT1H"),
                Self::P1d => serializer.serialize_unit_variant("IntervalType", 1u32, "P1D"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a topquery metric in one interval."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryMetricProperties {
    #[doc = "The name information for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The UI appropriate name for the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<query_metric_properties::Unit>,
    #[doc = "The value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Metric value when min() aggregate function is used over the interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[doc = "Metric value when max() aggregate function is used over the interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[doc = "Metric value when avg() aggregate function is used over the interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    #[doc = "Metric value when sum() aggregate function is used over the interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[doc = "Metric value when stdev aggregate function is used over the interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdev: Option<f64>,
}
impl QueryMetricProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_metric_properties {
    use super::*;
    #[doc = "The unit of the metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        #[serde(rename = "percentage")]
        Percentage,
        #[serde(rename = "KB")]
        Kb,
        #[serde(rename = "microseconds")]
        Microseconds,
        #[serde(rename = "count")]
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Percentage => serializer.serialize_unit_variant("Unit", 0u32, "percentage"),
                Self::Kb => serializer.serialize_unit_variant("Unit", 1u32, "KB"),
                Self::Microseconds => serializer.serialize_unit_variant("Unit", 2u32, "microseconds"),
                Self::Count => serializer.serialize_unit_variant("Unit", 3u32, "count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a database query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryProperties {
    #[doc = "Query text."]
    #[serde(rename = "queryText", default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
}
impl QueryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryStatistics {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a query execution statistics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryStatisticsProperties>,
}
impl QueryStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a query execution statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryStatisticsProperties {
    #[doc = "Database name of the database in which this query was executed."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Unique query id (unique within one database)."]
    #[serde(rename = "queryId", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[doc = "The start time for the metric (ISO-8601 format)."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time for the metric (ISO-8601 format)."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "List of intervals with appropriate metric data"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub intervals: Vec<QueryMetricInterval>,
}
impl QueryStatisticsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The read scale capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadScaleCapability {
    #[doc = "The maximum number of read scale replicas."]
    #[serde(rename = "maxNumberOfReplicas", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_replicas: Option<i32>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<read_scale_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ReadScaleCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod read_scale_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedAction {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for a Database, Server or Elastic Pool Recommended Action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendedActionProperties>,
}
impl RecommendedAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains error information for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedActionErrorInfo {
    #[doc = "Gets the reason why the recommended action was put to error state. e.g., DatabaseHasQdsOff, IndexAlreadyExists"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Gets whether the error could be ignored and recommended action could be retried. Possible values are: Yes/No"]
    #[serde(rename = "isRetryable", default, skip_serializing_if = "Option::is_none")]
    pub is_retryable: Option<recommended_action_error_info::IsRetryable>,
}
impl RecommendedActionErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommended_action_error_info {
    use super::*;
    #[doc = "Gets whether the error could be ignored and recommended action could be retried. Possible values are: Yes/No"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IsRetryable {
        Yes,
        No,
    }
}
#[doc = "Contains information of estimated or observed impact on various metrics for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedActionImpactRecord {
    #[doc = "Gets the name of the impact dimension. e.g., CPUChange, DiskSpaceChange, NumberOfQueriesAffected."]
    #[serde(rename = "dimensionName", default, skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    #[doc = "Gets the name of the impact dimension. e.g., CPUChange, DiskSpaceChange, NumberOfQueriesAffected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Gets the absolute value of this dimension if applicable. e.g., Number of Queries affected"]
    #[serde(rename = "absoluteValue", default, skip_serializing_if = "Option::is_none")]
    pub absolute_value: Option<f64>,
    #[doc = "Gets the absolute change in the value of this dimension. e.g., Absolute Disk space change in Megabytes"]
    #[serde(rename = "changeValueAbsolute", default, skip_serializing_if = "Option::is_none")]
    pub change_value_absolute: Option<f64>,
    #[doc = "Gets the relative change in the value of this dimension. e.g., Relative Disk space change in Percentage"]
    #[serde(rename = "changeValueRelative", default, skip_serializing_if = "Option::is_none")]
    pub change_value_relative: Option<f64>,
}
impl RecommendedActionImpactRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information for manual implementation for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedActionImplementationInfo {
    #[doc = "Gets the method in which this recommended action can be manually implemented. e.g., TSql, AzurePowerShell."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<recommended_action_implementation_info::Method>,
    #[doc = "Gets the manual implementation script. e.g., T-SQL script that could be executed on the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}
impl RecommendedActionImplementationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommended_action_implementation_info {
    use super::*;
    #[doc = "Gets the method in which this recommended action can be manually implemented. e.g., TSql, AzurePowerShell."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Method {
        TSql,
        AzurePowerShell,
    }
}
#[doc = "Contains time series of various impacted metrics for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedActionMetricInfo {
    #[doc = "Gets the name of the metric. e.g., CPU, Number of Queries."]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[doc = "Gets the unit in which metric is measured. e.g., DTU, Frequency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Gets the duration of time interval for the value given by this MetricInfo. e.g., PT1H (1 hour)"]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Gets the start time of time interval given by this MetricInfo."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the value of the metric in the time interval given by this MetricInfo."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl RecommendedActionMetricInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedActionProperties {
    #[doc = "Gets the reason for recommending this action. e.g., DuplicateIndex"]
    #[serde(rename = "recommendationReason", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_reason: Option<String>,
    #[doc = "Gets the time since when this recommended action is valid."]
    #[serde(rename = "validSince", default, with = "azure_core::date::rfc3339::option")]
    pub valid_since: Option<time::OffsetDateTime>,
    #[doc = "Gets time when this recommended action was last refreshed."]
    #[serde(rename = "lastRefresh", default, with = "azure_core::date::rfc3339::option")]
    pub last_refresh: Option<time::OffsetDateTime>,
    #[doc = "Contains information of current state for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
    pub state: RecommendedActionStateInfo,
    #[doc = "Gets if this recommended action is actionable by user"]
    #[serde(rename = "isExecutableAction", default, skip_serializing_if = "Option::is_none")]
    pub is_executable_action: Option<bool>,
    #[doc = "Gets if changes applied by this recommended action can be reverted by user"]
    #[serde(rename = "isRevertableAction", default, skip_serializing_if = "Option::is_none")]
    pub is_revertable_action: Option<bool>,
    #[doc = "Gets if this recommended action was suggested some time ago but user chose to ignore this and system added a new recommended action again."]
    #[serde(rename = "isArchivedAction", default, skip_serializing_if = "Option::is_none")]
    pub is_archived_action: Option<bool>,
    #[doc = "Gets the time when system started applying this recommended action on the user resource. e.g., index creation start time"]
    #[serde(rename = "executeActionStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub execute_action_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the time taken for applying this recommended action on user resource. e.g., time taken for index creation"]
    #[serde(rename = "executeActionDuration", default, skip_serializing_if = "Option::is_none")]
    pub execute_action_duration: Option<String>,
    #[doc = "Gets the time when system started reverting changes of this recommended action on user resource. e.g., time when index drop is executed."]
    #[serde(rename = "revertActionStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub revert_action_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the time taken for reverting changes of this recommended action on user resource. e.g., time taken for dropping the created index."]
    #[serde(rename = "revertActionDuration", default, skip_serializing_if = "Option::is_none")]
    pub revert_action_duration: Option<String>,
    #[doc = "Gets if approval for applying this recommended action was given by user/system."]
    #[serde(rename = "executeActionInitiatedBy", default, skip_serializing_if = "Option::is_none")]
    pub execute_action_initiated_by: Option<recommended_action_properties::ExecuteActionInitiatedBy>,
    #[doc = "Gets the time when this recommended action was approved for execution."]
    #[serde(rename = "executeActionInitiatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub execute_action_initiated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets if approval for reverting this recommended action was given by user/system."]
    #[serde(rename = "revertActionInitiatedBy", default, skip_serializing_if = "Option::is_none")]
    pub revert_action_initiated_by: Option<recommended_action_properties::RevertActionInitiatedBy>,
    #[doc = "Gets the time when this recommended action was approved for revert."]
    #[serde(rename = "revertActionInitiatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub revert_action_initiated_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the impact of this recommended action. Possible values are 1 - Low impact, 2 - Medium Impact and 3 - High Impact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[doc = "Contains information for manual implementation for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
    #[serde(rename = "implementationDetails", default, skip_serializing_if = "Option::is_none")]
    pub implementation_details: Option<RecommendedActionImplementationInfo>,
    #[doc = "Contains error information for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<RecommendedActionErrorInfo>,
    #[doc = "Gets the estimated impact info for this recommended action e.g., Estimated CPU gain, Estimated Disk Space change"]
    #[serde(
        rename = "estimatedImpact",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub estimated_impact: Vec<RecommendedActionImpactRecord>,
    #[doc = "Gets the observed/actual impact info for this recommended action e.g., Actual CPU gain, Actual Disk Space change"]
    #[serde(
        rename = "observedImpact",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub observed_impact: Vec<RecommendedActionImpactRecord>,
    #[doc = "Gets the time series info of metrics for this recommended action e.g., CPU consumption time series"]
    #[serde(
        rename = "timeSeries",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub time_series: Vec<RecommendedActionMetricInfo>,
    #[doc = "Gets the linked objects, if any."]
    #[serde(
        rename = "linkedObjects",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub linked_objects: Vec<String>,
    #[doc = "Gets additional details specific to this recommended action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
impl RecommendedActionProperties {
    pub fn new(state: RecommendedActionStateInfo) -> Self {
        Self {
            recommendation_reason: None,
            valid_since: None,
            last_refresh: None,
            state,
            is_executable_action: None,
            is_revertable_action: None,
            is_archived_action: None,
            execute_action_start_time: None,
            execute_action_duration: None,
            revert_action_start_time: None,
            revert_action_duration: None,
            execute_action_initiated_by: None,
            execute_action_initiated_time: None,
            revert_action_initiated_by: None,
            revert_action_initiated_time: None,
            score: None,
            implementation_details: None,
            error_details: None,
            estimated_impact: Vec::new(),
            observed_impact: Vec::new(),
            time_series: Vec::new(),
            linked_objects: Vec::new(),
            details: None,
        }
    }
}
pub mod recommended_action_properties {
    use super::*;
    #[doc = "Gets if approval for applying this recommended action was given by user/system."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecuteActionInitiatedBy {
        User,
        System,
    }
    #[doc = "Gets if approval for reverting this recommended action was given by user/system."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RevertActionInitiatedBy {
        User,
        System,
    }
}
#[doc = "Contains information of current state for an Azure SQL Database, Server or Elastic Pool Recommended Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedActionStateInfo {
    #[doc = "Current state the recommended action is in. Some commonly used states are: Active      -> recommended action is active and no action has been taken yet. Pending     -> recommended action is approved for and is awaiting execution. Executing   -> recommended action is being applied on the user database. Verifying   -> recommended action was applied and is being verified of its usefulness by the system. Success     -> recommended action was applied and improvement found during verification. Pending Revert  -> verification found little or no improvement so recommended action is queued for revert or user has manually reverted. Reverting   -> changes made while applying recommended action are being reverted on the user database. Reverted    -> successfully reverted the changes made by recommended action on user database. Ignored     -> user explicitly ignored/discarded the recommended action. "]
    #[serde(rename = "currentValue")]
    pub current_value: recommended_action_state_info::CurrentValue,
    #[doc = "Gets who initiated the execution of this recommended action. Possible Value are: User    -> When user explicity notified system to apply the recommended action. System  -> When auto-execute status of this advisor was set to 'Enabled', in which case the system applied it."]
    #[serde(rename = "actionInitiatedBy", default, skip_serializing_if = "Option::is_none")]
    pub action_initiated_by: Option<recommended_action_state_info::ActionInitiatedBy>,
    #[doc = "Gets the time when the state was last modified"]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
}
impl RecommendedActionStateInfo {
    pub fn new(current_value: recommended_action_state_info::CurrentValue) -> Self {
        Self {
            current_value,
            action_initiated_by: None,
            last_modified: None,
        }
    }
}
pub mod recommended_action_state_info {
    use super::*;
    #[doc = "Current state the recommended action is in. Some commonly used states are: Active      -> recommended action is active and no action has been taken yet. Pending     -> recommended action is approved for and is awaiting execution. Executing   -> recommended action is being applied on the user database. Verifying   -> recommended action was applied and is being verified of its usefulness by the system. Success     -> recommended action was applied and improvement found during verification. Pending Revert  -> verification found little or no improvement so recommended action is queued for revert or user has manually reverted. Reverting   -> changes made while applying recommended action are being reverted on the user database. Reverted    -> successfully reverted the changes made by recommended action on user database. Ignored     -> user explicitly ignored/discarded the recommended action. "]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CurrentValue")]
    pub enum CurrentValue {
        Active,
        Pending,
        Executing,
        Verifying,
        PendingRevert,
        RevertCancelled,
        Reverting,
        Reverted,
        Ignored,
        Expired,
        Monitoring,
        Resolved,
        Success,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CurrentValue {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CurrentValue {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CurrentValue {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("CurrentValue", 0u32, "Active"),
                Self::Pending => serializer.serialize_unit_variant("CurrentValue", 1u32, "Pending"),
                Self::Executing => serializer.serialize_unit_variant("CurrentValue", 2u32, "Executing"),
                Self::Verifying => serializer.serialize_unit_variant("CurrentValue", 3u32, "Verifying"),
                Self::PendingRevert => serializer.serialize_unit_variant("CurrentValue", 4u32, "PendingRevert"),
                Self::RevertCancelled => serializer.serialize_unit_variant("CurrentValue", 5u32, "RevertCancelled"),
                Self::Reverting => serializer.serialize_unit_variant("CurrentValue", 6u32, "Reverting"),
                Self::Reverted => serializer.serialize_unit_variant("CurrentValue", 7u32, "Reverted"),
                Self::Ignored => serializer.serialize_unit_variant("CurrentValue", 8u32, "Ignored"),
                Self::Expired => serializer.serialize_unit_variant("CurrentValue", 9u32, "Expired"),
                Self::Monitoring => serializer.serialize_unit_variant("CurrentValue", 10u32, "Monitoring"),
                Self::Resolved => serializer.serialize_unit_variant("CurrentValue", 11u32, "Resolved"),
                Self::Success => serializer.serialize_unit_variant("CurrentValue", 12u32, "Success"),
                Self::Error => serializer.serialize_unit_variant("CurrentValue", 13u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets who initiated the execution of this recommended action. Possible Value are: User    -> When user explicity notified system to apply the recommended action. System  -> When auto-execute status of this advisor was set to 'Enabled', in which case the system applied it."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionInitiatedBy {
        User,
        System,
    }
}
#[doc = "A recommended sensitivity label update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedSensitivityLabelUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an operation executed on a recommended sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendedSensitivityLabelUpdateProperties>,
}
impl RecommendedSensitivityLabelUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of recommended sensitivity label update operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedSensitivityLabelUpdateList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<RecommendedSensitivityLabelUpdate>,
}
impl RecommendedSensitivityLabelUpdateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation executed on a recommended sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedSensitivityLabelUpdateProperties {
    pub op: recommended_sensitivity_label_update_properties::Op,
    #[doc = "Schema name of the column to update."]
    pub schema: String,
    #[doc = "Table name of the column to update."]
    pub table: String,
    #[doc = "Column name to update."]
    pub column: String,
}
impl RecommendedSensitivityLabelUpdateProperties {
    pub fn new(op: recommended_sensitivity_label_update_properties::Op, schema: String, table: String, column: String) -> Self {
        Self { op, schema, table, column }
    }
}
pub mod recommended_sensitivity_label_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "enable")]
        Enable,
        #[serde(rename = "disable")]
        Disable,
    }
}
#[doc = "A recoverable database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableDatabase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The recoverable database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableDatabaseProperties>,
}
impl RecoverableDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of recoverable databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecoverableDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoverableDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RecoverableDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The recoverable database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableDatabaseProperties {
    #[doc = "The edition of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "The service level objective name of the database."]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[doc = "The elastic pool name of the database"]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The last available backup date."]
    #[serde(rename = "lastAvailableBackupDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_available_backup_date: Option<time::OffsetDateTime>,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<serde_json::Value>,
}
impl RecoverableDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A recoverable managed database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableManagedDatabase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The recoverable managed database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableManagedDatabaseProperties>,
}
impl RecoverableManagedDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of recoverable managed databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableManagedDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecoverableManagedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoverableManagedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RecoverableManagedDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The recoverable managed database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableManagedDatabaseProperties {
    #[doc = "The last available backup date."]
    #[serde(rename = "lastAvailableBackupDate", default, skip_serializing_if = "Option::is_none")]
    pub last_available_backup_date: Option<String>,
}
impl RecoverableManagedDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An RefreshExternalGovernanceStatus operation result resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshExternalGovernanceStatusOperationResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the operation result properties for refresh external governance status operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RefreshExternalGovernanceStatusOperationResultProperties>,
}
impl RefreshExternalGovernanceStatusOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An RefreshExternalGovernanceStatus operation result resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshExternalGovernanceStatusOperationResultMi {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Contains the operation result properties for refresh external governance status operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RefreshExternalGovernanceStatusOperationResultPropertiesMi>,
}
impl RefreshExternalGovernanceStatusOperationResultMi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the operation result properties for refresh external governance status operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshExternalGovernanceStatusOperationResultProperties {
    #[doc = "Request Id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Request type."]
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[doc = "Queued time."]
    #[serde(rename = "queuedTime", default, skip_serializing_if = "Option::is_none")]
    pub queued_time: Option<String>,
    #[doc = "Server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl RefreshExternalGovernanceStatusOperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the operation result properties for refresh external governance status operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshExternalGovernanceStatusOperationResultPropertiesMi {
    #[doc = "Request Id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Request type."]
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[doc = "Queued time."]
    #[serde(rename = "queuedTime", default, skip_serializing_if = "Option::is_none")]
    pub queued_time: Option<String>,
    #[doc = "Managed instance name."]
    #[serde(rename = "managedInstanceName", default, skip_serializing_if = "Option::is_none")]
    pub managed_instance_name: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl RefreshExternalGovernanceStatusOperationResultPropertiesMi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Vulnerability Assessment remediation Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Remediation {
    #[doc = "SQL Vulnerability Assessment remediation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "SQL Vulnerability Assessment remediation script."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scripts: Vec<String>,
    #[doc = "SQL Vulnerability Assessment is remediation automated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated: Option<bool>,
    #[doc = "SQL Vulnerability Assessment optional link to remediate in Azure Portal."]
    #[serde(rename = "portalLink", default, skip_serializing_if = "Option::is_none")]
    pub portal_link: Option<String>,
}
impl Remediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A replication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a replication link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationLinkProperties>,
}
impl ReplicationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of replication links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReplicationLink>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReplicationLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a replication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkProperties {
    #[doc = "Resource partner server."]
    #[serde(rename = "partnerServer", default, skip_serializing_if = "Option::is_none")]
    pub partner_server: Option<String>,
    #[doc = "Resource partner database."]
    #[serde(rename = "partnerDatabase", default, skip_serializing_if = "Option::is_none")]
    pub partner_database: Option<String>,
    #[doc = "Resource partner database Id."]
    #[serde(rename = "partnerDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub partner_database_id: Option<String>,
    #[doc = "Resource partner location."]
    #[serde(rename = "partnerLocation", default, skip_serializing_if = "Option::is_none")]
    pub partner_location: Option<String>,
    #[doc = "Local replication role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<replication_link_properties::Role>,
    #[doc = "Partner replication role."]
    #[serde(rename = "partnerRole", default, skip_serializing_if = "Option::is_none")]
    pub partner_role: Option<replication_link_properties::PartnerRole>,
    #[doc = "Replication mode."]
    #[serde(rename = "replicationMode", default, skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<String>,
    #[doc = "Time at which the link was created."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Seeding completion percentage for the link."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "Replication state (PENDING, SEEDING, CATCHUP, SUSPENDED)."]
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<replication_link_properties::ReplicationState>,
    #[doc = "Whether the user is currently allowed to terminate the link."]
    #[serde(rename = "isTerminationAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_termination_allowed: Option<bool>,
    #[doc = "Link type (GEO, NAMED, STANDBY). Update operation does not support NAMED."]
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<replication_link_properties::LinkType>,
}
impl ReplicationLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_link_properties {
    use super::*;
    #[doc = "Local replication role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Primary,
        Secondary,
        NonReadableSecondary,
        Source,
        Copy,
    }
    #[doc = "Partner replication role."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PartnerRole {
        Primary,
        Secondary,
        NonReadableSecondary,
        Source,
        Copy,
    }
    #[doc = "Replication state (PENDING, SEEDING, CATCHUP, SUSPENDED)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationState")]
    pub enum ReplicationState {
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "SEEDING")]
        Seeding,
        #[serde(rename = "CATCH_UP")]
        CatchUp,
        #[serde(rename = "SUSPENDED")]
        Suspended,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("ReplicationState", 0u32, "PENDING"),
                Self::Seeding => serializer.serialize_unit_variant("ReplicationState", 1u32, "SEEDING"),
                Self::CatchUp => serializer.serialize_unit_variant("ReplicationState", 2u32, "CATCH_UP"),
                Self::Suspended => serializer.serialize_unit_variant("ReplicationState", 3u32, "SUSPENDED"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Link type (GEO, NAMED, STANDBY). Update operation does not support NAMED."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkType")]
    pub enum LinkType {
        #[serde(rename = "GEO")]
        Geo,
        #[serde(rename = "NAMED")]
        Named,
        #[serde(rename = "STANDBY")]
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("LinkType", 0u32, "GEO"),
                Self::Named => serializer.serialize_unit_variant("LinkType", 1u32, "NAMED"),
                Self::Standby => serializer.serialize_unit_variant("LinkType", 2u32, "STANDBY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A replication link update request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a replication link update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationLinkUpdateProperties>,
}
impl ReplicationLinkUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a replication link update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkUpdateProperties {
    #[doc = "Link type (GEO, NAMED, STANDBY). Update operation does not support NAMED."]
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<replication_link_update_properties::LinkType>,
}
impl ReplicationLinkUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_link_update_properties {
    use super::*;
    #[doc = "Link type (GEO, NAMED, STANDBY). Update operation does not support NAMED."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkType")]
    pub enum LinkType {
        #[serde(rename = "GEO")]
        Geo,
        #[serde(rename = "NAMED")]
        Named,
        #[serde(rename = "STANDBY")]
        Standby,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("LinkType", 0u32, "GEO"),
                Self::Named => serializer.serialize_unit_variant("LinkType", 1u32, "NAMED"),
                Self::Standby => serializer.serialize_unit_variant("LinkType", 2u32, "STANDBY"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ARM resource."]
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
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
    #[doc = "The Azure Active Directory principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The identity type. Set this to 'SystemAssigned' in order to automatically create and assign an Azure Active Directory principal for the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[doc = "The Azure Active Directory tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_identity {
    use super::*;
    #[doc = "The identity type. Set this to 'SystemAssigned' in order to automatically create and assign an Azure Active Directory principal for the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned,UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains the information necessary to perform a resource move (rename)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMoveDefinition {
    #[doc = "The target ID for the resource"]
    pub id: String,
}
impl ResourceMoveDefinition {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceWithWritableName {
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
impl ResourceWithWritableName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A restorable dropped database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedDatabase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The restorable dropped database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableDroppedDatabaseProperties>,
}
impl RestorableDroppedDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of restorable dropped databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RestorableDroppedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorableDroppedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RestorableDroppedDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restorable dropped database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedDatabaseProperties {
    #[doc = "The name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The max size of the database expressed in bytes."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "The creation date of the database (ISO8601 format)."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The deletion date of the database (ISO8601 format)."]
    #[serde(rename = "deletionDate", default, with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The earliest restore date of the database (ISO8601 format)."]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The storage account type used to store backups for this database."]
    #[serde(rename = "backupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_redundancy: Option<restorable_dropped_database_properties::BackupStorageRedundancy>,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<serde_json::Value>,
}
impl RestorableDroppedDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restorable_dropped_database_properties {
    use super::*;
    #[doc = "The storage account type used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BackupStorageRedundancy")]
    pub enum BackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("BackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("BackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("BackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("BackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A restorable dropped managed database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorableDroppedManagedDatabase {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The restorable dropped managed database's properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableDroppedManagedDatabaseProperties>,
}
impl RestorableDroppedManagedDatabase {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of restorable dropped managed databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedManagedDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RestorableDroppedManagedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorableDroppedManagedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RestorableDroppedManagedDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restorable dropped managed database's properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedManagedDatabaseProperties {
    #[doc = "The name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The creation date of the database (ISO8601 format)."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The deletion date of the database (ISO8601 format)."]
    #[serde(rename = "deletionDate", default, with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The earliest restore date of the database (ISO8601 format)."]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
}
impl RestorableDroppedManagedDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Database restore points."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of a database restore point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorePointProperties>,
}
impl RestorePoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of long term retention backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RestorePoint>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorePointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RestorePointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a database restore point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointProperties {
    #[doc = "The type of restore point"]
    #[serde(rename = "restorePointType", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_type: Option<restore_point_properties::RestorePointType>,
    #[doc = "The earliest time to which this database can be restored"]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "restorePointCreationDate", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_creation_date: Option<time::OffsetDateTime>,
    #[doc = "The label of restore point for backup request by user"]
    #[serde(rename = "restorePointLabel", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_label: Option<String>,
}
impl RestorePointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_point_properties {
    use super::*;
    #[doc = "The type of restore point"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RestorePointType {
        #[serde(rename = "CONTINUOUS")]
        Continuous,
        #[serde(rename = "DISCRETE")]
        Discrete,
    }
}
#[doc = "A list of vulnerability assessment scan results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlVulnerabilityAssessmentScanResults>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlVulnerabilityAssessmentScanListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlVulnerabilityAssessmentScanListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schedule info describing when the server should be started or stopped."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleItem {
    #[doc = "Start day."]
    #[serde(rename = "startDay")]
    pub start_day: schedule_item::StartDay,
    #[doc = "Start time."]
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[doc = "Stop day."]
    #[serde(rename = "stopDay")]
    pub stop_day: schedule_item::StopDay,
    #[doc = "Stop time."]
    #[serde(rename = "stopTime")]
    pub stop_time: String,
}
impl ScheduleItem {
    pub fn new(start_day: schedule_item::StartDay, start_time: String, stop_day: schedule_item::StopDay, stop_time: String) -> Self {
        Self {
            start_day,
            start_time,
            stop_day,
            stop_time,
        }
    }
}
pub mod schedule_item {
    use super::*;
    #[doc = "Start day."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartDay")]
    pub enum StartDay {
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
    impl FromStr for StartDay {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartDay {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartDay {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sunday => serializer.serialize_unit_variant("StartDay", 0u32, "Sunday"),
                Self::Monday => serializer.serialize_unit_variant("StartDay", 1u32, "Monday"),
                Self::Tuesday => serializer.serialize_unit_variant("StartDay", 2u32, "Tuesday"),
                Self::Wednesday => serializer.serialize_unit_variant("StartDay", 3u32, "Wednesday"),
                Self::Thursday => serializer.serialize_unit_variant("StartDay", 4u32, "Thursday"),
                Self::Friday => serializer.serialize_unit_variant("StartDay", 5u32, "Friday"),
                Self::Saturday => serializer.serialize_unit_variant("StartDay", 6u32, "Saturday"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Stop day."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StopDay")]
    pub enum StopDay {
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
    impl FromStr for StopDay {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StopDay {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StopDay {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sunday => serializer.serialize_unit_variant("StopDay", 0u32, "Sunday"),
                Self::Monday => serializer.serialize_unit_variant("StopDay", 1u32, "Monday"),
                Self::Tuesday => serializer.serialize_unit_variant("StopDay", 2u32, "Tuesday"),
                Self::Wednesday => serializer.serialize_unit_variant("StopDay", 3u32, "Wednesday"),
                Self::Thursday => serializer.serialize_unit_variant("StopDay", 4u32, "Thursday"),
                Self::Friday => serializer.serialize_unit_variant("StopDay", 5u32, "Friday"),
                Self::Saturday => serializer.serialize_unit_variant("StopDay", 6u32, "Saturday"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database."]
    pub state: security_alert_policy_properties::State,
    #[doc = "Specifies an array of alerts that are disabled. Allowed values are: Sql_Injection, Sql_Injection_Vulnerability, Access_Anomaly, Data_Exfiltration, Unsafe_Action, Brute_Force"]
    #[serde(
        rename = "disabledAlerts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_alerts: Vec<String>,
    #[doc = "Specifies an array of e-mail addresses to which the alert is sent."]
    #[serde(
        rename = "emailAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_addresses: Vec<String>,
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the Threat Detection audit storage account."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the Threat Detection audit logs."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the UTC creation time of the policy."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
}
impl SecurityAlertPolicyProperties {
    pub fn new(state: security_alert_policy_properties::State) -> Self {
        Self {
            state,
            disabled_alerts: Vec::new(),
            email_addresses: Vec::new(),
            email_account_admins: None,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            creation_time: None,
        }
    }
}
pub mod security_alert_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        New,
        Enabled,
        Disabled,
    }
}
#[doc = "Properties of a security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertsPolicyProperties {
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database."]
    pub state: security_alerts_policy_properties::State,
    #[doc = "Specifies an array of alerts that are disabled. Allowed values are: Sql_Injection, Sql_Injection_Vulnerability, Access_Anomaly, Data_Exfiltration, Unsafe_Action, Brute_Force"]
    #[serde(
        rename = "disabledAlerts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_alerts: Vec<String>,
    #[doc = "Specifies an array of e-mail addresses to which the alert is sent."]
    #[serde(
        rename = "emailAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_addresses: Vec<String>,
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the Threat Detection audit storage account."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the Threat Detection audit logs."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the UTC creation time of the policy."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
}
impl SecurityAlertsPolicyProperties {
    pub fn new(state: security_alerts_policy_properties::State) -> Self {
        Self {
            state,
            disabled_alerts: Vec::new(),
            email_addresses: Vec::new(),
            email_account_admins: None,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            creation_time: None,
        }
    }
}
pub mod security_alerts_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A security event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEvent {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a security event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityEventProperties>,
}
impl SecurityEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of security events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEventCollection {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityEvent>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityEventCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityEventCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a security event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEventProperties {
    #[doc = "The time when the security event occurred."]
    #[serde(rename = "eventTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_time: Option<time::OffsetDateTime>,
    #[doc = "The type of the security event."]
    #[serde(rename = "securityEventType", default, skip_serializing_if = "Option::is_none")]
    pub security_event_type: Option<security_event_properties::SecurityEventType>,
    #[doc = "The subscription name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[doc = "The server name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The database name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The IP address of the client who executed the statement."]
    #[serde(rename = "clientIp", default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[doc = "The application used to execute the statement."]
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[doc = "The principal user who executed the statement"]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "The properties of a security event sql injection additional properties."]
    #[serde(
        rename = "securityEventSqlInjectionAdditionalProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_event_sql_injection_additional_properties: Option<SecurityEventSqlInjectionAdditionalProperties>,
}
impl SecurityEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_event_properties {
    use super::*;
    #[doc = "The type of the security event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SecurityEventType {
        Undefined,
        SqlInjectionVulnerability,
        SqlInjectionExploit,
    }
}
#[doc = "The properties of a security event sql injection additional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEventSqlInjectionAdditionalProperties {
    #[doc = "The threat ID."]
    #[serde(rename = "threatId", default, skip_serializing_if = "Option::is_none")]
    pub threat_id: Option<String>,
    #[doc = "The statement"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
    #[doc = "The statement highlight offset"]
    #[serde(rename = "statementHighlightOffset", default, skip_serializing_if = "Option::is_none")]
    pub statement_highlight_offset: Option<i32>,
    #[doc = "The statement highlight length"]
    #[serde(rename = "statementHighlightLength", default, skip_serializing_if = "Option::is_none")]
    pub statement_highlight_length: Option<i32>,
    #[doc = "The sql error code"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The sql error severity"]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "The sql error message"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl SecurityEventSqlInjectionAdditionalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are supported in the $filter operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEventsFilterParameters {
    #[doc = "Filter on the event time."]
    #[serde(rename = "eventTime", default, with = "azure_core::date::rfc3339::option")]
    pub event_time: Option<time::OffsetDateTime>,
    #[doc = "Whether to show server records or not."]
    #[serde(rename = "showServerRecords", default, skip_serializing_if = "Option::is_none")]
    pub show_server_records: Option<bool>,
}
impl SecurityEventsFilterParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource that manages the sensitivity label."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "Properties of a sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SensitivityLabelProperties>,
}
impl SensitivityLabel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sensitivity labels."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SensitivityLabel>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensitivityLabelListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SensitivityLabelListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelProperties {
    #[doc = "The schema name."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "The table name."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The column name."]
    #[serde(rename = "columnName", default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[doc = "The label name."]
    #[serde(rename = "labelName", default, skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[doc = "The label ID."]
    #[serde(rename = "labelId", default, skip_serializing_if = "Option::is_none")]
    pub label_id: Option<String>,
    #[doc = "The information type."]
    #[serde(rename = "informationType", default, skip_serializing_if = "Option::is_none")]
    pub information_type: Option<String>,
    #[doc = "The information type ID."]
    #[serde(rename = "informationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub information_type_id: Option<String>,
    #[doc = "Is sensitivity recommendation disabled. Applicable for recommended sensitivity label only. Specifies whether the sensitivity recommendation on this column is disabled (dismissed) or not."]
    #[serde(rename = "isDisabled", default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<sensitivity_label_properties::Rank>,
    #[serde(rename = "clientClassificationSource", default, skip_serializing_if = "Option::is_none")]
    pub client_classification_source: Option<sensitivity_label_properties::ClientClassificationSource>,
}
impl SensitivityLabelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sensitivity_label_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rank {
        None,
        Low,
        Medium,
        High,
        Critical,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClientClassificationSource")]
    pub enum ClientClassificationSource {
        None,
        Native,
        Recommended,
        #[serde(rename = "MIP")]
        Mip,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClientClassificationSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClientClassificationSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClientClassificationSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ClientClassificationSource", 0u32, "None"),
                Self::Native => serializer.serialize_unit_variant("ClientClassificationSource", 1u32, "Native"),
                Self::Recommended => serializer.serialize_unit_variant("ClientClassificationSource", 2u32, "Recommended"),
                Self::Mip => serializer.serialize_unit_variant("ClientClassificationSource", 3u32, "MIP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A sensitivity label update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an operation executed on a sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SensitivityLabelUpdateProperties>,
}
impl SensitivityLabelUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sensitivity label update operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabelUpdateList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<SensitivityLabelUpdate>,
}
impl SensitivityLabelUpdateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an operation executed on a sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitivityLabelUpdateProperties {
    pub op: sensitivity_label_update_properties::Op,
    #[doc = "Schema name of the column to update."]
    pub schema: String,
    #[doc = "Table name of the column to update."]
    pub table: String,
    #[doc = "Column name to update."]
    pub column: String,
    #[doc = "A sensitivity label."]
    #[serde(rename = "sensitivityLabel", default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_label: Option<SensitivityLabel>,
}
impl SensitivityLabelUpdateProperties {
    pub fn new(op: sensitivity_label_update_properties::Op, schema: String, table: String, column: String) -> Self {
        Self {
            op,
            schema,
            table,
            column,
            sensitivity_label: None,
        }
    }
}
pub mod sensitivity_label_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "set")]
        Set,
        #[serde(rename = "remove")]
        Remove,
    }
}
#[doc = "An Azure SQL Database server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "Kind of sql server. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}
impl Server {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            kind: None,
            properties: None,
        }
    }
}
#[doc = "A server Advanced Threat Protection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAdvancedThreatProtection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of an Advanced Threat Protection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvancedThreatProtectionProperties>,
}
impl ServerAdvancedThreatProtection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server-level Automatic Tuning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAutomaticTuning {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Server-level Automatic Tuning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutomaticTuningServerProperties>,
}
impl ServerAutomaticTuning {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAzureAdAdministrator {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a active directory administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdministratorProperties>,
}
impl ServerAzureAdAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory only authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAzureAdOnlyAuthentication {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a active directory only authentication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureAdOnlyAuthProperties>,
}
impl ServerAzureAdOnlyAuthentication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBlobAuditingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server blob auditing policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerBlobAuditingPolicyProperties>,
}
impl ServerBlobAuditingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerBlobAuditingPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerBlobAuditingPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server blob auditing policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerBlobAuditingPolicyProperties {
    #[doc = "Specifies the state of devops audit. If state is Enabled, devops logs will be sent to Azure Monitor.\r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled', 'IsAzureMonitorTargetEnabled' as true and 'IsDevopsAuditEnabled' as true\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'DevOpsOperationsAudit' diagnostic logs category on the master database should also be created.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/master/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isDevopsAuditEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_devops_audit_enabled: Option<bool>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\nLEDGER_OPERATION_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(
        rename = "auditActionsAndGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled' and 'IsAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
    #[doc = "Specifies whether Managed Identity is used to access blob storage"]
    #[serde(rename = "isManagedIdentityInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_managed_identity_in_use: Option<bool>,
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: server_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
}
impl ServerBlobAuditingPolicyProperties {
    pub fn new(state: server_blob_auditing_policy_properties::State) -> Self {
        Self {
            is_devops_audit_enabled: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
            is_managed_identity_in_use: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            storage_account_subscription_id: None,
        }
    }
}
pub mod server_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A server configuration option"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfigurationOption {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of server configuration option."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerConfigurationOptionProperties>,
}
impl ServerConfigurationOption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server configuration options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfigurationOptionListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerConfigurationOption>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerConfigurationOptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerConfigurationOptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of server configuration option."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfigurationOptionProperties {
    #[doc = "Value of the server configuration option."]
    #[serde(rename = "serverConfigurationOptionValue")]
    pub server_configuration_option_value: i32,
    #[doc = "Provisioning state of server configuration option."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<server_configuration_option_properties::ProvisioningState>,
}
impl ServerConfigurationOptionProperties {
    pub fn new(server_configuration_option_value: i32) -> Self {
        Self {
            server_configuration_option_value,
            provisioning_state: None,
        }
    }
}
pub mod server_configuration_option_properties {
    use super::*;
    #[doc = "Provisioning state of server configuration option."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Created,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
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
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Created"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A server connection policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConnectionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The properties of a server connection policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerConnectionPolicyProperties>,
}
impl ServerConnectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server connection policy objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConnectionPolicyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerConnectionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerConnectionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerConnectionPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server connection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConnectionPolicyProperties {
    #[doc = "The server connection type."]
    #[serde(rename = "connectionType")]
    pub connection_type: server_connection_policy_properties::ConnectionType,
}
impl ServerConnectionPolicyProperties {
    pub fn new(connection_type: server_connection_policy_properties::ConnectionType) -> Self {
        Self { connection_type }
    }
}
pub mod server_connection_policy_properties {
    use super::*;
    #[doc = "The server connection type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionType")]
    pub enum ConnectionType {
        Default,
        Redirect,
        Proxy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("ConnectionType", 0u32, "Default"),
                Self::Redirect => serializer.serialize_unit_variant("ConnectionType", 1u32, "Redirect"),
                Self::Proxy => serializer.serialize_unit_variant("ConnectionType", 2u32, "Proxy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of server DevOps audit settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDevOpsAuditSettingsListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerDevOpsAuditingSettings>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerDevOpsAuditSettingsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerDevOpsAuditSettingsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server DevOps audit settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerDevOpsAuditSettingsProperties {
    #[doc = "Specifies whether DevOps audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'State' as 'Enabled' and 'IsAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure DevOps audit, Diagnostic Settings with 'DevOpsOperationsAudit' diagnostic logs category on the master database should be also created.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/master/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies whether Managed Identity is used to access blob storage"]
    #[serde(rename = "isManagedIdentityInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_managed_identity_in_use: Option<bool>,
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: server_dev_ops_audit_settings_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
}
impl ServerDevOpsAuditSettingsProperties {
    pub fn new(state: server_dev_ops_audit_settings_properties::State) -> Self {
        Self {
            is_azure_monitor_target_enabled: None,
            is_managed_identity_in_use: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            storage_account_subscription_id: None,
        }
    }
}
pub mod server_dev_ops_audit_settings_properties {
    use super::*;
    #[doc = "Specifies the state of the audit. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A server DevOps auditing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDevOpsAuditingSettings {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a server DevOps audit settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerDevOpsAuditSettingsProperties>,
}
impl ServerDevOpsAuditingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A server DNS alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDnsAlias {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server DNS alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerDnsAliasProperties>,
}
impl ServerDnsAlias {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A server dns alias acquisition request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerDnsAliasAcquisition {
    #[doc = "The id of the server alias that will be acquired to point to this server instead."]
    #[serde(rename = "oldServerDnsAliasId")]
    pub old_server_dns_alias_id: String,
}
impl ServerDnsAliasAcquisition {
    pub fn new(old_server_dns_alias_id: String) -> Self {
        Self { old_server_dns_alias_id }
    }
}
#[doc = "A list of server DNS aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDnsAliasListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerDnsAlias>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerDnsAliasListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerDnsAliasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server DNS alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDnsAliasProperties {
    #[doc = "The fully qualified DNS record for alias"]
    #[serde(rename = "azureDnsRecord", default, skip_serializing_if = "Option::is_none")]
    pub azure_dns_record: Option<String>,
}
impl ServerDnsAliasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a active directory administrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerExternalAdministrator {
    #[doc = "Type of the sever administrator."]
    #[serde(rename = "administratorType", default, skip_serializing_if = "Option::is_none")]
    pub administrator_type: Option<server_external_administrator::AdministratorType>,
    #[doc = "Principal Type of the sever administrator."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<server_external_administrator::PrincipalType>,
    #[doc = "Login name of the server administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[doc = "SID (object ID) of the server administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "Tenant ID of the administrator."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Azure Active Directory only Authentication enabled."]
    #[serde(rename = "azureADOnlyAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub azure_ad_only_authentication: Option<bool>,
}
impl ServerExternalAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_external_administrator {
    use super::*;
    #[doc = "Type of the sever administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AdministratorType")]
    pub enum AdministratorType {
        ActiveDirectory,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AdministratorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AdministratorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AdministratorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActiveDirectory => serializer.serialize_unit_variant("AdministratorType", 0u32, "ActiveDirectory"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Principal Type of the sever administrator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        Application,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::Application => serializer.serialize_unit_variant("PrincipalType", 2u32, "Application"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerFirewallRuleProperties {
    #[doc = "The start IP address of the firewall rule. Must be IPv4 format. Use value '0.0.0.0' for all Azure-internal IP addresses."]
    #[serde(rename = "startIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[doc = "The end IP address of the firewall rule. Must be IPv4 format. Must be greater than or equal to startIpAddress. Use value '0.0.0.0' for all Azure-internal IP addresses."]
    #[serde(rename = "endIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
impl ServerFirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server info for the server trust group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerInfo {
    #[doc = "Server Id."]
    #[serde(rename = "serverId")]
    pub server_id: String,
}
impl ServerInfo {
    pub fn new(server_id: String) -> Self {
        Self { server_id }
    }
}
#[doc = "A server key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerKey {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Kind of encryption protector. This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for a server key execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerKeyProperties>,
}
impl ServerKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerKeyListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerKey>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerKeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerKeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a server key execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerKeyProperties {
    #[doc = "Subregion of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subregion: Option<String>,
    #[doc = "The server key type like 'ServiceManaged', 'AzureKeyVault'."]
    #[serde(rename = "serverKeyType")]
    pub server_key_type: server_key_properties::ServerKeyType,
    #[doc = "The URI of the server key. If the ServerKeyType is AzureKeyVault, then the URI is required. The AKV URI is required to be in this format: 'https://YourVaultName.vault.azure.net/keys/YourKeyName/YourKeyVersion'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Thumbprint of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The server key creation date."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Key auto rotation opt-in flag. Either true or false."]
    #[serde(rename = "autoRotationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_rotation_enabled: Option<bool>,
}
impl ServerKeyProperties {
    pub fn new(server_key_type: server_key_properties::ServerKeyType) -> Self {
        Self {
            subregion: None,
            server_key_type,
            uri: None,
            thumbprint: None,
            creation_date: None,
            auto_rotation_enabled: None,
        }
    }
}
pub mod server_key_properties {
    use super::*;
    #[doc = "The server key type like 'ServiceManaged', 'AzureKeyVault'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerKeyType")]
    pub enum ServerKeyType {
        ServiceManaged,
        AzureKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerKeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerKeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerKeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ServiceManaged => serializer.serialize_unit_variant("ServerKeyType", 0u32, "ServiceManaged"),
                Self::AzureKeyVault => serializer.serialize_unit_variant("ServerKeyType", 1u32, "AzureKeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of servers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Server>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A server operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a server operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerOperationProperties>,
}
impl ServerOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list server operations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerOperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerOperationProperties {
    #[doc = "The name of operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The friendly name of operation."]
    #[serde(rename = "operationFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub operation_friendly_name: Option<String>,
    #[doc = "The percentage of the operation completed."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The name of the server."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The operation start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The operation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<server_operation_properties::State>,
    #[doc = "The operation error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The operation error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The operation error severity."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "Whether or not the error is a user error."]
    #[serde(rename = "isUserError", default, skip_serializing_if = "Option::is_none")]
    pub is_user_error: Option<bool>,
    #[doc = "The estimated completion time of the operation."]
    #[serde(rename = "estimatedCompletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub estimated_completion_time: Option<time::OffsetDateTime>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the operation can be cancelled."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
}
impl ServerOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_operation_properties {
    use super::*;
    #[doc = "The operation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Pending,
        InProgress,
        Succeeded,
        Failed,
        CancelInProgress,
        Cancelled,
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
                Self::Pending => serializer.serialize_unit_variant("State", 0u32, "Pending"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::CancelInProgress => serializer.serialize_unit_variant("State", 4u32, "CancelInProgress"),
                Self::Cancelled => serializer.serialize_unit_variant("State", 5u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A private endpoint connection under a server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerPrivateEndpointConnection {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl ServerPrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[doc = "Administrator username for the server. Once created it cannot be changed."]
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[doc = "The administrator login password (required for server creation)."]
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[doc = "The version of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The state of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The fully qualified domain name of the server."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[doc = "List of private endpoint connections on a server"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<ServerPrivateEndpointConnection>,
    #[doc = "Minimal TLS version. Allowed values: 'None', 1.0', '1.1', '1.2', '1.3'"]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<server_properties::MinimalTlsVersion>,
    #[doc = "Whether or not public endpoint access is allowed for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled' or 'SecuredByPerimeter'"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<server_properties::PublicNetworkAccess>,
    #[doc = "Whether or not existing server has a workspace created and if it allows connection from workspace"]
    #[serde(rename = "workspaceFeature", default, skip_serializing_if = "Option::is_none")]
    pub workspace_feature: Option<server_properties::WorkspaceFeature>,
    #[doc = "The resource id of a user assigned identity to be used by default."]
    #[serde(rename = "primaryUserAssignedIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub primary_user_assigned_identity_id: Option<String>,
    #[doc = "The Client id used for cross tenant CMK scenario"]
    #[serde(rename = "federatedClientId", default, skip_serializing_if = "Option::is_none")]
    pub federated_client_id: Option<String>,
    #[doc = "A CMK URI of the key to use for encryption."]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[doc = "Properties of a active directory administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrators: Option<ServerExternalAdministrator>,
    #[doc = "Whether or not to restrict outbound network access for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[serde(rename = "restrictOutboundNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub restrict_outbound_network_access: Option<server_properties::RestrictOutboundNetworkAccess>,
    #[doc = "Whether or not to enable IPv6 support for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[serde(rename = "isIPv6Enabled", default, skip_serializing_if = "Option::is_none")]
    pub is_i_pv6_enabled: Option<server_properties::IsIPv6Enabled>,
    #[doc = "Status of external governance."]
    #[serde(rename = "externalGovernanceStatus", default, skip_serializing_if = "Option::is_none")]
    pub external_governance_status: Option<server_properties::ExternalGovernanceStatus>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_properties {
    use super::*;
    #[doc = "Minimal TLS version. Allowed values: 'None', 1.0', '1.1', '1.2', '1.3'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimalTlsVersion")]
    pub enum MinimalTlsVersion {
        None,
        #[serde(rename = "1.0")]
        N1_0,
        #[serde(rename = "1.1")]
        N1_1,
        #[serde(rename = "1.2")]
        N1_2,
        #[serde(rename = "1.3")]
        N1_3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinimalTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinimalTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinimalTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("MinimalTlsVersion", 0u32, "None"),
                Self::N1_0 => serializer.serialize_unit_variant("MinimalTlsVersion", 1u32, "1.0"),
                Self::N1_1 => serializer.serialize_unit_variant("MinimalTlsVersion", 2u32, "1.1"),
                Self::N1_2 => serializer.serialize_unit_variant("MinimalTlsVersion", 3u32, "1.2"),
                Self::N1_3 => serializer.serialize_unit_variant("MinimalTlsVersion", 4u32, "1.3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not public endpoint access is allowed for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled' or 'SecuredByPerimeter'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        SecuredByPerimeter,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::SecuredByPerimeter => serializer.serialize_unit_variant("PublicNetworkAccess", 2u32, "SecuredByPerimeter"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not existing server has a workspace created and if it allows connection from workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkspaceFeature")]
    pub enum WorkspaceFeature {
        Connected,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkspaceFeature {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkspaceFeature {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkspaceFeature {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("WorkspaceFeature", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("WorkspaceFeature", 1u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not to restrict outbound network access for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RestrictOutboundNetworkAccess")]
    pub enum RestrictOutboundNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RestrictOutboundNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RestrictOutboundNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RestrictOutboundNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RestrictOutboundNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RestrictOutboundNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not to enable IPv6 support for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsIPv6Enabled")]
    pub enum IsIPv6Enabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsIPv6Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsIPv6Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsIPv6Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("IsIPv6Enabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("IsIPv6Enabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of external governance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExternalGovernanceStatus")]
    pub enum ExternalGovernanceStatus {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExternalGovernanceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExternalGovernanceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExternalGovernanceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("ExternalGovernanceStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ExternalGovernanceStatus", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A server security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertsPolicyProperties>,
}
impl ServerSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server trust certificate imported from box to enable connection between box and Sql Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTrustCertificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a server trust certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerTrustCertificateProperties>,
}
impl ServerTrustCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server trust certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTrustCertificateProperties {
    #[doc = "The certificate public blob"]
    #[serde(rename = "publicBlob", default, skip_serializing_if = "Option::is_none")]
    pub public_blob: Option<String>,
    #[doc = "The certificate thumbprint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The certificate name"]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
}
impl ServerTrustCertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server trust certificates in instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTrustCertificatesListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerTrustCertificate>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerTrustCertificatesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerTrustCertificatesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A server trust group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTrustGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server trust group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerTrustGroupProperties>,
}
impl ServerTrustGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server trust groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerTrustGroupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerTrustGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerTrustGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerTrustGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server trust group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerTrustGroupProperties {
    #[doc = "Group members information for the server trust group."]
    #[serde(rename = "groupMembers")]
    pub group_members: Vec<ServerInfo>,
    #[doc = "Trust scope of the server trust group."]
    #[serde(rename = "trustScopes")]
    pub trust_scopes: Vec<String>,
}
impl ServerTrustGroupProperties {
    pub fn new(group_members: Vec<ServerInfo>, trust_scopes: Vec<String>) -> Self {
        Self {
            group_members,
            trust_scopes,
        }
    }
}
#[doc = "An update request for an Azure SQL Database server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUpdate {
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "The properties of a server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Usage metric of a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUsage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerUsageProperties>,
}
impl ServerUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server usage metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUsageListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerUsage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUsageProperties {
    #[doc = "User-readable name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "Boundary value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl ServerUsageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVersionCapability {
    #[doc = "The server version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported database editions."]
    #[serde(
        rename = "supportedEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_editions: Vec<EditionCapability>,
    #[doc = "The list of supported elastic pool editions."]
    #[serde(
        rename = "supportedElasticPoolEditions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_elastic_pool_editions: Vec<ElasticPoolEditionCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<server_version_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ServerVersionCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_version_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "A server vulnerability assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a server Vulnerability Assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerVulnerabilityAssessmentProperties>,
}
impl ServerVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of the server's vulnerability assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessmentListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerVulnerabilityAssessmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a server Vulnerability Assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVulnerabilityAssessmentProperties {
    #[doc = "A blob storage container path to hold the scan results (e.g. https://myStorage.blob.core.windows.net/VaScans/)."]
    #[serde(rename = "storageContainerPath")]
    pub storage_container_path: String,
    #[doc = "A shared access signature (SAS Key) that has write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required. Applies only if the storage account is not behind a Vnet or a firewall"]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required. Applies only if the storage account is not behind a Vnet or a firewall"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Properties of a Vulnerability Assessment recurring scans."]
    #[serde(rename = "recurringScans", default, skip_serializing_if = "Option::is_none")]
    pub recurring_scans: Option<VulnerabilityAssessmentRecurringScansProperties>,
}
impl ServerVulnerabilityAssessmentProperties {
    pub fn new(storage_container_path: String) -> Self {
        Self {
            storage_container_path,
            storage_container_sas_key: None,
            storage_account_access_key: None,
            recurring_scans: None,
        }
    }
}
#[doc = "The service objectives capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceObjectiveCapability {
    #[doc = "The unique ID of the service objective."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The service objective name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported maximum database sizes."]
    #[serde(
        rename = "supportedMaxSizes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_max_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "The performance level capability."]
    #[serde(rename = "performanceLevel", default, skip_serializing_if = "Option::is_none")]
    pub performance_level: Option<PerformanceLevelCapability>,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "List of supported license types."]
    #[serde(
        rename = "supportedLicenseTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "includedMaxSize", default, skip_serializing_if = "Option::is_none")]
    pub included_max_size: Option<MaxSizeCapability>,
    #[doc = "Whether or not zone redundancy is supported for the service objective."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "Supported auto pause delay time range"]
    #[serde(rename = "supportedAutoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub supported_auto_pause_delay: Option<AutoPauseDelayTimeRange>,
    #[doc = "List of supported min capacities"]
    #[serde(
        rename = "supportedMinCapacities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_min_capacities: Vec<MinCapacityCapability>,
    #[doc = "The compute model"]
    #[serde(rename = "computeModel", default, skip_serializing_if = "Option::is_none")]
    pub compute_model: Option<String>,
    #[doc = "List of supported maintenance configurations"]
    #[serde(
        rename = "supportedMaintenanceConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_maintenance_configurations: Vec<MaintenanceConfigurationCapability>,
    #[doc = "Whether or not zone pinning is supported."]
    #[serde(rename = "zonePinning", default, skip_serializing_if = "Option::is_none")]
    pub zone_pinning: Option<bool>,
    #[doc = "List of supported free limit exhaustion behaviors"]
    #[serde(
        rename = "supportedFreeLimitExhaustionBehaviors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_free_limit_exhaustion_behaviors: Vec<FreeLimitExhaustionBehaviorCapability>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<service_objective_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ServiceObjectiveCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_objective_capability {
    use super::*;
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "The managed instance's service principal configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipal {
    #[doc = "The Azure Active Directory application object id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Azure Active Directory application client id."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The Azure Active Directory tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Service principal type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<service_principal::Type>,
}
impl ServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_principal {
    use super::*;
    #[doc = "Service principal type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An ARM Resource SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU, typically, a letter + Number code, e.g. P3."]
    pub name: String,
    #[doc = "The tier or edition of the particular SKU, e.g. Basic, Premium."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Size of the particular SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Capacity of the particular SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "A recoverable managed database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAgentConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sql agent configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlAgentConfigurationProperties>,
}
impl SqlAgentConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sql agent configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAgentConfigurationProperties {
    #[doc = "The state of Sql Agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sql_agent_configuration_properties::State>,
}
impl SqlAgentConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_agent_configuration_properties {
    use super::*;
    #[doc = "The state of Sql Agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A SQL Vulnerability Assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVulnerabilityAssessmentPolicyProperties>,
}
impl SqlVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SQL Vulnerability Assessments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlVulnerabilityAssessmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentPolicyProperties {
    #[doc = "Specifies the state of the SQL Vulnerability Assessment, whether it is enabled or disabled or a state has not been applied yet on the specific database or server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sql_vulnerability_assessment_policy_properties::State>,
}
impl SqlVulnerabilityAssessmentPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_vulnerability_assessment_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the SQL Vulnerability Assessment, whether it is enabled or disabled or a state has not been applied yet on the specific database or server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "Properties of a vulnerability assessment scan error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl SqlVulnerabilityAssessmentScanError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanRecord {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a vulnerability assessment scan record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVulnerabilityAssessmentScanRecordProperties>,
}
impl SqlVulnerabilityAssessmentScanRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of vulnerability assessment scan records."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanRecordListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlVulnerabilityAssessmentScanRecord>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlVulnerabilityAssessmentScanRecordListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlVulnerabilityAssessmentScanRecordListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanRecordProperties {
    #[doc = "The scan ID."]
    #[serde(rename = "scanId", default, skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[doc = "The scan trigger type."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<sql_vulnerability_assessment_scan_record_properties::TriggerType>,
    #[doc = "The scan status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sql_vulnerability_assessment_scan_record_properties::State>,
    #[doc = "The scan start time (UTC)."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The scan end time (UTC)."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The scan errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<SqlVulnerabilityAssessmentScanError>,
    #[doc = "The server name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The database name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The SQL version."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
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
    #[doc = "The last scan time."]
    #[serde(rename = "lastScanTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_scan_time: Option<time::OffsetDateTime>,
}
impl SqlVulnerabilityAssessmentScanRecordProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_vulnerability_assessment_scan_record_properties {
    use super::*;
    #[doc = "The scan trigger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerType")]
    pub enum TriggerType {
        OnDemand,
        Recurring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnDemand => serializer.serialize_unit_variant("TriggerType", 0u32, "OnDemand"),
                Self::Recurring => serializer.serialize_unit_variant("TriggerType", 1u32, "Recurring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The scan status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Passed,
        Failed,
        FailedToRun,
        InProgress,
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
                Self::Passed => serializer.serialize_unit_variant("State", 0u32, "Passed"),
                Self::Failed => serializer.serialize_unit_variant("State", 1u32, "Failed"),
                Self::FailedToRun => serializer.serialize_unit_variant("State", 2u32, "FailedToRun"),
                Self::InProgress => serializer.serialize_unit_variant("State", 3u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SQL Vulnerability Assessment scan result properties for a single rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanResultProperties {
    #[doc = "SQL Vulnerability Assessment rule Id."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "SQL Vulnerability Assessment rule result status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<sql_vulnerability_assessment_scan_result_properties::Status>,
    #[doc = "SQL Vulnerability Assessment error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "SQL Vulnerability Assessment is the query results trimmed."]
    #[serde(rename = "isTrimmed", default, skip_serializing_if = "Option::is_none")]
    pub is_trimmed: Option<bool>,
    #[doc = "SQL Vulnerability Assessment query results that was run."]
    #[serde(
        rename = "queryResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub query_results: Vec<Vec<String>>,
    #[doc = "SQL Vulnerability Assessment remediation Details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[doc = "SQL Vulnerability Assessment baseline adjusted results"]
    #[serde(rename = "baselineAdjustedResult", default, skip_serializing_if = "Option::is_none")]
    pub baseline_adjusted_result: Option<BaselineAdjustedResult>,
    #[doc = "SQL Vulnerability Assessment rule metadata details."]
    #[serde(rename = "ruleMetadata", default, skip_serializing_if = "Option::is_none")]
    pub rule_metadata: Option<VaRule>,
}
impl SqlVulnerabilityAssessmentScanResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_vulnerability_assessment_scan_result_properties {
    use super::*;
    #[doc = "SQL Vulnerability Assessment rule result status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NonFinding,
        Finding,
        InternalError,
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
                Self::NonFinding => serializer.serialize_unit_variant("Status", 0u32, "NonFinding"),
                Self::Finding => serializer.serialize_unit_variant("Status", 1u32, "Finding"),
                Self::InternalError => serializer.serialize_unit_variant("Status", 2u32, "InternalError"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVulnerabilityAssessmentScanResults {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "SQL Vulnerability Assessment scan result properties for a single rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVulnerabilityAssessmentScanResultProperties>,
}
impl SqlVulnerabilityAssessmentScanResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed instance's Start/Stop schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartStopManagedInstanceSchedule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of managed instance's Start/Stop schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StartStopManagedInstanceScheduleProperties>,
}
impl StartStopManagedInstanceSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed instance's Start/Stop schedule list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartStopManagedInstanceScheduleListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StartStopManagedInstanceSchedule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StartStopManagedInstanceScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StartStopManagedInstanceScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of managed instance's Start/Stop schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartStopManagedInstanceScheduleProperties {
    #[doc = "The description of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The time zone of the schedule."]
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[doc = "Schedule list."]
    #[serde(rename = "scheduleList")]
    pub schedule_list: Vec<ScheduleItem>,
    #[doc = "Next action to be executed (Start or Stop)"]
    #[serde(rename = "nextRunAction", default, skip_serializing_if = "Option::is_none")]
    pub next_run_action: Option<String>,
    #[doc = "Timestamp when the next action will be executed in the corresponding schedule time zone."]
    #[serde(rename = "nextExecutionTime", default, skip_serializing_if = "Option::is_none")]
    pub next_execution_time: Option<String>,
}
impl StartStopManagedInstanceScheduleProperties {
    pub fn new(schedule_list: Vec<ScheduleItem>) -> Self {
        Self {
            description: None,
            time_zone_id: None,
            schedule_list,
            next_run_action: None,
            next_execution_time: None,
        }
    }
}
#[doc = "The storage account type capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageCapability {
    #[doc = "The storage account type for the database's backups."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<storage_capability::StorageAccountType>,
    #[doc = "The status of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<storage_capability::Status>,
    #[doc = "The reason for the capability not being available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl StorageCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_capability {
    use super::*;
    #[doc = "The storage account type for the database's backups."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAccountType {
        #[serde(rename = "GRS")]
        Grs,
        #[serde(rename = "LRS")]
        Lrs,
        #[serde(rename = "ZRS")]
        Zrs,
        #[serde(rename = "GZRS")]
        Gzrs,
    }
    #[doc = "The status of the capability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Visible,
        Available,
        Default,
        Disabled,
    }
}
#[doc = "Usage Metric of a Subscription in a Location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUsage {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a subscription usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionUsageProperties>,
}
impl SubscriptionUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of subscription usage metrics in a location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUsageListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SubscriptionUsage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubscriptionUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SubscriptionUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a subscription usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUsageProperties {
    #[doc = "User-readable name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "Boundary value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl SubscriptionUsageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Synapse link workspace resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynapseLinkWorkspace {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Synapse link workspaces"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SynapseLinkWorkspaceProperties>,
}
impl SynapseLinkWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Synapse link workspaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynapseLinkWorkspaceInfoProperties {
    #[doc = "Synapse link workspace id."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "Link connection name."]
    #[serde(rename = "linkConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub link_connection_name: Option<String>,
}
impl SynapseLinkWorkspaceInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of synapselink workspaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynapseLinkWorkspaceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SynapseLinkWorkspace>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SynapseLinkWorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SynapseLinkWorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Synapse link workspaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SynapseLinkWorkspaceProperties {
    #[doc = "List of all synapselink workspaces"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub workspaces: Vec<SynapseLinkWorkspaceInfoProperties>,
}
impl SynapseLinkWorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL Database sync agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgent {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an Azure SQL Database sync agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncAgentProperties>,
}
impl SyncAgent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure SQL Database sync agent key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgentKeyProperties {
    #[doc = "Key of sync agent."]
    #[serde(rename = "syncAgentKey", default, skip_serializing_if = "Option::is_none")]
    pub sync_agent_key: Option<String>,
}
impl SyncAgentKeyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL Database sync agent linked database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgentLinkedDatabase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an Azure SQL Database sync agent linked database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncAgentLinkedDatabaseProperties>,
}
impl SyncAgentLinkedDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sync agent linked databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgentLinkedDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncAgentLinkedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncAgentLinkedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncAgentLinkedDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure SQL Database sync agent linked database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgentLinkedDatabaseProperties {
    #[doc = "Type of the sync agent linked database."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<sync_agent_linked_database_properties::DatabaseType>,
    #[doc = "Id of the sync agent linked database."]
    #[serde(rename = "databaseId", default, skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[doc = "Description of the sync agent linked database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Server name of the sync agent linked database."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Database name of the sync agent linked database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "User name of the sync agent linked database."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
impl SyncAgentLinkedDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sync_agent_linked_database_properties {
    use super::*;
    #[doc = "Type of the sync agent linked database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DatabaseType")]
    pub enum DatabaseType {
        AzureSqlDatabase,
        SqlServerDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DatabaseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DatabaseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DatabaseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureSqlDatabase => serializer.serialize_unit_variant("DatabaseType", 0u32, "AzureSqlDatabase"),
                Self::SqlServerDatabase => serializer.serialize_unit_variant("DatabaseType", 1u32, "SqlServerDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of sync agents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgentListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncAgent>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncAgentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncAgentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure SQL Database sync agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncAgentProperties {
    #[doc = "Name of the sync agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ARM resource id of the sync database in the sync agent."]
    #[serde(rename = "syncDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub sync_database_id: Option<String>,
    #[doc = "Last alive time of the sync agent."]
    #[serde(rename = "lastAliveTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_alive_time: Option<time::OffsetDateTime>,
    #[doc = "State of the sync agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sync_agent_properties::State>,
    #[doc = "If the sync agent version is up to date."]
    #[serde(rename = "isUpToDate", default, skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<bool>,
    #[doc = "Expiration time of the sync agent version."]
    #[serde(rename = "expiryTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Version of the sync agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SyncAgentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sync_agent_properties {
    use super::*;
    #[doc = "State of the sync agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Online,
        Offline,
        NeverConnected,
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
                Self::Online => serializer.serialize_unit_variant("State", 0u32, "Online"),
                Self::Offline => serializer.serialize_unit_variant("State", 1u32, "Offline"),
                Self::NeverConnected => serializer.serialize_unit_variant("State", 2u32, "NeverConnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of sync database ID properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncDatabaseIdListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncDatabaseIdProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncDatabaseIdListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncDatabaseIdListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the sync database id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncDatabaseIdProperties {
    #[doc = "ARM resource id of sync database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SyncDatabaseIdProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the database full schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncFullSchemaProperties {
    #[doc = "List of tables in the database full schema."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tables: Vec<SyncFullSchemaTable>,
    #[doc = "Last update time of the database schema."]
    #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_time: Option<time::OffsetDateTime>,
}
impl SyncFullSchemaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sync schema properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncFullSchemaPropertiesListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncFullSchemaProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncFullSchemaPropertiesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncFullSchemaPropertiesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the table in the database full schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncFullSchemaTable {
    #[doc = "List of columns in the table of database full schema."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<SyncFullSchemaTableColumn>,
    #[doc = "Error id of the table."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "If there is error in the table."]
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[doc = "Name of the table."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Quoted name of the table."]
    #[serde(rename = "quotedName", default, skip_serializing_if = "Option::is_none")]
    pub quoted_name: Option<String>,
}
impl SyncFullSchemaTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the column in the table of database full schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncFullSchemaTableColumn {
    #[doc = "Data size of the column."]
    #[serde(rename = "dataSize", default, skip_serializing_if = "Option::is_none")]
    pub data_size: Option<String>,
    #[doc = "Data type of the column."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "Error id of the column."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "If there is error in the table."]
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[doc = "If it is the primary key of the table."]
    #[serde(rename = "isPrimaryKey", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_key: Option<bool>,
    #[doc = "Name of the column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Quoted name of the column."]
    #[serde(rename = "quotedName", default, skip_serializing_if = "Option::is_none")]
    pub quoted_name: Option<String>,
}
impl SyncFullSchemaTableColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL Database sync group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of a sync group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupProperties>,
}
impl SyncGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sync groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of sync group log properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupLogListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncGroupLogProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncGroupLogListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncGroupLogListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure SQL Database sync group log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupLogProperties {
    #[doc = "Timestamp of the sync group log."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Type of the sync group log."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sync_group_log_properties::Type>,
    #[doc = "Source of the sync group log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Details of the sync group log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "TracingId of the sync group log."]
    #[serde(rename = "tracingId", default, skip_serializing_if = "Option::is_none")]
    pub tracing_id: Option<String>,
    #[doc = "OperationStatus of the sync group log."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
}
impl SyncGroupLogProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sync_group_log_properties {
    use super::*;
    #[doc = "Type of the sync group log."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        All,
        Error,
        Warning,
        Success,
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
                Self::All => serializer.serialize_unit_variant("Type", 0u32, "All"),
                Self::Error => serializer.serialize_unit_variant("Type", 1u32, "Error"),
                Self::Warning => serializer.serialize_unit_variant("Type", 2u32, "Warning"),
                Self::Success => serializer.serialize_unit_variant("Type", 3u32, "Success"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a sync group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupProperties {
    #[doc = "Sync interval of the sync group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "Last sync time of the sync group."]
    #[serde(rename = "lastSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<time::OffsetDateTime>,
    #[doc = "Conflict resolution policy of the sync group."]
    #[serde(rename = "conflictResolutionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_policy: Option<sync_group_properties::ConflictResolutionPolicy>,
    #[doc = "ARM resource id of the sync database in the sync group."]
    #[serde(rename = "syncDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub sync_database_id: Option<String>,
    #[doc = "User name for the sync group hub database credential."]
    #[serde(rename = "hubDatabaseUserName", default, skip_serializing_if = "Option::is_none")]
    pub hub_database_user_name: Option<String>,
    #[doc = "Password for the sync group hub database credential."]
    #[serde(rename = "hubDatabasePassword", default, skip_serializing_if = "Option::is_none")]
    pub hub_database_password: Option<String>,
    #[doc = "Sync state of the sync group."]
    #[serde(rename = "syncState", default, skip_serializing_if = "Option::is_none")]
    pub sync_state: Option<sync_group_properties::SyncState>,
    #[doc = "Properties of sync group schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<SyncGroupSchema>,
    #[doc = "If conflict logging is enabled."]
    #[serde(rename = "enableConflictLogging", default, skip_serializing_if = "Option::is_none")]
    pub enable_conflict_logging: Option<bool>,
    #[doc = "Conflict logging retention period."]
    #[serde(rename = "conflictLoggingRetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub conflict_logging_retention_in_days: Option<i32>,
    #[doc = "If use private link connection is enabled."]
    #[serde(rename = "usePrivateLinkConnection", default, skip_serializing_if = "Option::is_none")]
    pub use_private_link_connection: Option<bool>,
    #[doc = "Private endpoint name of the sync group if use private link connection is enabled."]
    #[serde(rename = "privateEndpointName", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_name: Option<String>,
}
impl SyncGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sync_group_properties {
    use super::*;
    #[doc = "Conflict resolution policy of the sync group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConflictResolutionPolicy")]
    pub enum ConflictResolutionPolicy {
        HubWin,
        MemberWin,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConflictResolutionPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConflictResolutionPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConflictResolutionPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::HubWin => serializer.serialize_unit_variant("ConflictResolutionPolicy", 0u32, "HubWin"),
                Self::MemberWin => serializer.serialize_unit_variant("ConflictResolutionPolicy", 1u32, "MemberWin"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sync state of the sync group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncState")]
    pub enum SyncState {
        NotReady,
        Error,
        Warning,
        Progressing,
        Good,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotReady => serializer.serialize_unit_variant("SyncState", 0u32, "NotReady"),
                Self::Error => serializer.serialize_unit_variant("SyncState", 1u32, "Error"),
                Self::Warning => serializer.serialize_unit_variant("SyncState", 2u32, "Warning"),
                Self::Progressing => serializer.serialize_unit_variant("SyncState", 3u32, "Progressing"),
                Self::Good => serializer.serialize_unit_variant("SyncState", 4u32, "Good"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of sync group schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupSchema {
    #[doc = "List of tables in sync group schema."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tables: Vec<SyncGroupSchemaTable>,
    #[doc = "Name of master sync member where the schema is from."]
    #[serde(rename = "masterSyncMemberName", default, skip_serializing_if = "Option::is_none")]
    pub master_sync_member_name: Option<String>,
}
impl SyncGroupSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of table in sync group schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupSchemaTable {
    #[doc = "List of columns in sync group schema."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<SyncGroupSchemaTableColumn>,
    #[doc = "Quoted name of sync group schema table."]
    #[serde(rename = "quotedName", default, skip_serializing_if = "Option::is_none")]
    pub quoted_name: Option<String>,
}
impl SyncGroupSchemaTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of column in sync group table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupSchemaTableColumn {
    #[doc = "Quoted name of sync group table column."]
    #[serde(rename = "quotedName", default, skip_serializing_if = "Option::is_none")]
    pub quoted_name: Option<String>,
    #[doc = "Data size of the column."]
    #[serde(rename = "dataSize", default, skip_serializing_if = "Option::is_none")]
    pub data_size: Option<String>,
    #[doc = "Data type of the column."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
impl SyncGroupSchemaTableColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure SQL Database sync member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncMember {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a sync member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncMemberProperties>,
}
impl SyncMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Azure SQL Database sync members."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncMemberListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SyncMember>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncMemberListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SyncMemberListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a sync member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncMemberProperties {
    #[doc = "Database type of the sync member."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<sync_member_properties::DatabaseType>,
    #[doc = "ARM resource id of the sync agent in the sync member."]
    #[serde(rename = "syncAgentId", default, skip_serializing_if = "Option::is_none")]
    pub sync_agent_id: Option<String>,
    #[doc = "SQL Server database id of the sync member."]
    #[serde(rename = "sqlServerDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_database_id: Option<String>,
    #[doc = "ARM resource id of the sync member logical database, for sync members in Azure."]
    #[serde(rename = "syncMemberAzureDatabaseResourceId", default, skip_serializing_if = "Option::is_none")]
    pub sync_member_azure_database_resource_id: Option<String>,
    #[doc = "Whether to use private link connection."]
    #[serde(rename = "usePrivateLinkConnection", default, skip_serializing_if = "Option::is_none")]
    pub use_private_link_connection: Option<bool>,
    #[doc = "Private endpoint name of the sync member if use private link connection is enabled, for sync members in Azure."]
    #[serde(rename = "privateEndpointName", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_name: Option<String>,
    #[doc = "Server name of the member database in the sync member"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Database name of the member database in the sync member."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "User name of the member database in the sync member."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Password of the member database in the sync member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Sync direction of the sync member."]
    #[serde(rename = "syncDirection", default, skip_serializing_if = "Option::is_none")]
    pub sync_direction: Option<sync_member_properties::SyncDirection>,
    #[doc = "Sync state of the sync member."]
    #[serde(rename = "syncState", default, skip_serializing_if = "Option::is_none")]
    pub sync_state: Option<sync_member_properties::SyncState>,
}
impl SyncMemberProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sync_member_properties {
    use super::*;
    #[doc = "Database type of the sync member."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DatabaseType")]
    pub enum DatabaseType {
        AzureSqlDatabase,
        SqlServerDatabase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DatabaseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DatabaseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DatabaseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureSqlDatabase => serializer.serialize_unit_variant("DatabaseType", 0u32, "AzureSqlDatabase"),
                Self::SqlServerDatabase => serializer.serialize_unit_variant("DatabaseType", 1u32, "SqlServerDatabase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sync direction of the sync member."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncDirection")]
    pub enum SyncDirection {
        Bidirectional,
        OneWayMemberToHub,
        OneWayHubToMember,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncDirection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncDirection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncDirection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bidirectional => serializer.serialize_unit_variant("SyncDirection", 0u32, "Bidirectional"),
                Self::OneWayMemberToHub => serializer.serialize_unit_variant("SyncDirection", 1u32, "OneWayMemberToHub"),
                Self::OneWayHubToMember => serializer.serialize_unit_variant("SyncDirection", 2u32, "OneWayHubToMember"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sync state of the sync member."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncState")]
    pub enum SyncState {
        SyncInProgress,
        SyncSucceeded,
        SyncFailed,
        DisabledTombstoneCleanup,
        DisabledBackupRestore,
        SyncSucceededWithWarnings,
        SyncCancelling,
        SyncCancelled,
        UnProvisioned,
        Provisioning,
        Provisioned,
        ProvisionFailed,
        DeProvisioning,
        DeProvisioned,
        DeProvisionFailed,
        Reprovisioning,
        ReprovisionFailed,
        UnReprovisioned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SyncInProgress => serializer.serialize_unit_variant("SyncState", 0u32, "SyncInProgress"),
                Self::SyncSucceeded => serializer.serialize_unit_variant("SyncState", 1u32, "SyncSucceeded"),
                Self::SyncFailed => serializer.serialize_unit_variant("SyncState", 2u32, "SyncFailed"),
                Self::DisabledTombstoneCleanup => serializer.serialize_unit_variant("SyncState", 3u32, "DisabledTombstoneCleanup"),
                Self::DisabledBackupRestore => serializer.serialize_unit_variant("SyncState", 4u32, "DisabledBackupRestore"),
                Self::SyncSucceededWithWarnings => serializer.serialize_unit_variant("SyncState", 5u32, "SyncSucceededWithWarnings"),
                Self::SyncCancelling => serializer.serialize_unit_variant("SyncState", 6u32, "SyncCancelling"),
                Self::SyncCancelled => serializer.serialize_unit_variant("SyncState", 7u32, "SyncCancelled"),
                Self::UnProvisioned => serializer.serialize_unit_variant("SyncState", 8u32, "UnProvisioned"),
                Self::Provisioning => serializer.serialize_unit_variant("SyncState", 9u32, "Provisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("SyncState", 10u32, "Provisioned"),
                Self::ProvisionFailed => serializer.serialize_unit_variant("SyncState", 11u32, "ProvisionFailed"),
                Self::DeProvisioning => serializer.serialize_unit_variant("SyncState", 12u32, "DeProvisioning"),
                Self::DeProvisioned => serializer.serialize_unit_variant("SyncState", 13u32, "DeProvisioned"),
                Self::DeProvisionFailed => serializer.serialize_unit_variant("SyncState", 14u32, "DeProvisionFailed"),
                Self::Reprovisioning => serializer.serialize_unit_variant("SyncState", 15u32, "Reprovisioning"),
                Self::ReprovisionFailed => serializer.serialize_unit_variant("SyncState", 16u32, "ReprovisionFailed"),
                Self::UnReprovisioned => serializer.serialize_unit_variant("SyncState", 17u32, "UnReprovisioned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A TDE certificate that can be uploaded into a server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TdeCertificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a TDE certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TdeCertificateProperties>,
}
impl TdeCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a TDE certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TdeCertificateProperties {
    #[doc = "The base64 encoded certificate private blob."]
    #[serde(rename = "privateBlob")]
    pub private_blob: String,
    #[doc = "The certificate password."]
    #[serde(rename = "certPassword", default, skip_serializing_if = "Option::is_none")]
    pub cert_password: Option<String>,
}
impl TdeCertificateProperties {
    pub fn new(private_blob: String) -> Self {
        Self {
            private_blob,
            cert_password: None,
        }
    }
}
#[doc = "Time Zone property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeZone {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a time zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TimeZoneProperties>,
}
impl TimeZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of time zones."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeZoneListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TimeZone>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TimeZoneListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TimeZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a time zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeZoneProperties {
    #[doc = "The time zone id"]
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[doc = "The time zone display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl TimeZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopQueries {
    #[doc = "Requested number of top queries."]
    #[serde(rename = "numberOfQueries", default, skip_serializing_if = "Option::is_none")]
    pub number_of_queries: Option<i32>,
    #[doc = "Aggregation function used to calculate query metrics."]
    #[serde(rename = "aggregationFunction", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<String>,
    #[doc = "Metric used to rank queries."]
    #[serde(rename = "observationMetric", default, skip_serializing_if = "Option::is_none")]
    pub observation_metric: Option<String>,
    #[doc = "Interval type (length)."]
    #[serde(rename = "intervalType", default, skip_serializing_if = "Option::is_none")]
    pub interval_type: Option<top_queries::IntervalType>,
    #[doc = "The start time for the metric (ISO-8601 format)."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time for the metric (ISO-8601 format)."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "List of top resource consuming queries with appropriate metric data"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queries: Vec<QueryStatisticsProperties>,
}
impl TopQueries {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod top_queries {
    use super::*;
    #[doc = "Interval type (length)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IntervalType")]
    pub enum IntervalType {
        #[serde(rename = "PT1H")]
        Pt1h,
        #[serde(rename = "P1D")]
        P1d,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IntervalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IntervalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IntervalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pt1h => serializer.serialize_unit_variant("IntervalType", 0u32, "PT1H"),
                Self::P1d => serializer.serialize_unit_variant("IntervalType", 1u32, "P1D"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of top resource consuming queries on managed instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopQueriesListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TopQueries>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TopQueriesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TopQueriesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Properties of a transparent data encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransparentDataEncryptionProperties {
    #[doc = "Specifies the state of the transparent data encryption."]
    pub state: transparent_data_encryption_properties::State,
}
impl TransparentDataEncryptionProperties {
    pub fn new(state: transparent_data_encryption_properties::State) -> Self {
        Self { state }
    }
}
pub mod transparent_data_encryption_properties {
    use super::*;
    #[doc = "Specifies the state of the transparent data encryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "Contains the information necessary to perform long term retention backup update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateLongTermRetentionBackupParameters {
    #[doc = "Contains the properties to perform long term retention backup copy operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateLongTermRetentionBackupParametersProperties>,
}
impl UpdateLongTermRetentionBackupParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the properties to perform long term retention backup copy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateLongTermRetentionBackupParametersProperties {
    #[doc = "The storage redundancy type of the copied backup"]
    #[serde(rename = "requestedBackupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub requested_backup_storage_redundancy:
        Option<update_long_term_retention_backup_parameters_properties::RequestedBackupStorageRedundancy>,
}
impl UpdateLongTermRetentionBackupParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_long_term_retention_backup_parameters_properties {
    use super::*;
    #[doc = "The storage redundancy type of the copied backup"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestedBackupStorageRedundancy")]
    pub enum RequestedBackupStorageRedundancy {
        Geo,
        Local,
        Zone,
        GeoZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestedBackupStorageRedundancy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestedBackupStorageRedundancy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestedBackupStorageRedundancy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Geo => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 0u32, "Geo"),
                Self::Local => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 1u32, "Local"),
                Self::Zone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 2u32, "Zone"),
                Self::GeoZone => serializer.serialize_unit_variant("RequestedBackupStorageRedundancy", 3u32, "GeoZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A refresh DNS servers operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateVirtualClusterDnsServersOperation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of dns servers on virtual cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualClusterDnsServersProperties>,
}
impl UpdateVirtualClusterDnsServersOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertManagedServerOperationParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i32>,
    #[serde(rename = "storageSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_in_gb: Option<i32>,
}
impl UpsertManagedServerOperationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertManagedServerOperationStepWithEstimatesAndDuration {
    #[serde(rename = "stepStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub step_start_time: Option<time::OffsetDateTime>,
    #[serde(rename = "stepEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub step_end_time: Option<time::OffsetDateTime>,
    #[serde(rename = "timeElapsed", default, skip_serializing_if = "Option::is_none")]
    pub time_elapsed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upsert_managed_server_operation_step_with_estimates_and_duration::Status>,
}
impl UpsertManagedServerOperationStepWithEstimatesAndDuration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upsert_managed_server_operation_step_with_estimates_and_duration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        InProgress,
        SlowedDown,
        Completed,
        Failed,
        Canceled,
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
                Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::SlowedDown => serializer.serialize_unit_variant("Status", 2u32, "SlowedDown"),
                Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ARM usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "ARM Usage Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Usage unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Usage current value."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = "Usage limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Usage requested limit."]
    #[serde(rename = "requestedLimit", default, skip_serializing_if = "Option::is_none")]
    pub requested_limit: Option<i32>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Usage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "The Azure Active Directory principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Azure Active Directory client id."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL Vulnerability Assessment rule metadata details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaRule {
    #[doc = "SQL Vulnerability Assessment rule Id."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "SQL Vulnerability Assessment rule severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<va_rule::Severity>,
    #[doc = "SQL Vulnerability Assessment rule category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "SQL Vulnerability Assessment rule type."]
    #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<va_rule::RuleType>,
    #[doc = "SQL Vulnerability Assessment rule title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "SQL Vulnerability Assessment rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "SQL Vulnerability Assessment rule rationale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[doc = "SQL Vulnerability Assessment query check object."]
    #[serde(rename = "queryCheck", default, skip_serializing_if = "Option::is_none")]
    pub query_check: Option<QueryCheck>,
    #[doc = "SQL Vulnerability Assessment benchmark references."]
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
pub mod va_rule {
    use super::*;
    #[doc = "SQL Vulnerability Assessment rule severity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        High,
        Medium,
        Low,
        Informational,
        Obsolete,
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
                Self::High => serializer.serialize_unit_variant("Severity", 0u32, "High"),
                Self::Medium => serializer.serialize_unit_variant("Severity", 1u32, "Medium"),
                Self::Low => serializer.serialize_unit_variant("Severity", 2u32, "Low"),
                Self::Informational => serializer.serialize_unit_variant("Severity", 3u32, "Informational"),
                Self::Obsolete => serializer.serialize_unit_variant("Severity", 4u32, "Obsolete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Vulnerability Assessment rule type."]
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
}
#[doc = "An Azure SQL virtual cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualCluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of a virtual cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualClusterProperties>,
}
impl VirtualCluster {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The properties of dns servers on virtual cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualClusterDnsServersProperties {
    #[doc = "The status of the DNS refresh operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_cluster_dns_servers_properties::Status>,
}
impl VirtualClusterDnsServersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_cluster_dns_servers_properties {
    use super::*;
    #[doc = "The status of the DNS refresh operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        InProgress,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 2u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of virtual clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualClusterListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualCluster>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a virtual cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualClusterProperties {
    #[doc = "Subnet resource ID for the virtual cluster."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Virtual cluster version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "List of resources in this virtual cluster."]
    #[serde(
        rename = "childResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub child_resources: Vec<String>,
}
impl VirtualClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An update request for virtual cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualClusterUpdate {
    #[doc = "The properties of a virtual cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualClusterProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VirtualClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
impl VirtualNetworkRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of virtual network rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualNetworkRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualNetworkRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[doc = "The ARM resource id of the virtual network subnet."]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub virtual_network_subnet_id: String,
    #[doc = "Create firewall rule before the virtual network has vnet service endpoint enabled."]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
    #[doc = "Virtual Network Rule State"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<virtual_network_rule_properties::State>,
}
impl VirtualNetworkRuleProperties {
    pub fn new(virtual_network_subnet_id: String) -> Self {
        Self {
            virtual_network_subnet_id,
            ignore_missing_vnet_service_endpoint: None,
            state: None,
        }
    }
}
pub mod virtual_network_rule_properties {
    use super::*;
    #[doc = "Virtual Network Rule State"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Initializing,
        InProgress,
        Ready,
        Failed,
        Deleting,
        Unknown,
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
                Self::Initializing => serializer.serialize_unit_variant("State", 0u32, "Initializing"),
                Self::InProgress => serializer.serialize_unit_variant("State", 1u32, "InProgress"),
                Self::Ready => serializer.serialize_unit_variant("State", 2u32, "Ready"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("State", 4u32, "Deleting"),
                Self::Unknown => serializer.serialize_unit_variant("State", 5u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a Vulnerability Assessment recurring scans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentRecurringScansProperties {
    #[doc = "Recurring scans state."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Specifies that the schedule scan notification will be is sent to the subscription administrators."]
    #[serde(rename = "emailSubscriptionAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_subscription_admins: Option<bool>,
    #[doc = "Specifies an array of e-mail addresses to which the scan notification is sent."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub emails: Vec<String>,
}
impl VulnerabilityAssessmentRecurringScansProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a vulnerability assessment scan error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl VulnerabilityAssessmentScanError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanRecord {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a vulnerability assessment scan record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VulnerabilityAssessmentScanRecordProperties>,
}
impl VulnerabilityAssessmentScanRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of vulnerability assessment scan records."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanRecordListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VulnerabilityAssessmentScanRecord>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VulnerabilityAssessmentScanRecordListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VulnerabilityAssessmentScanRecordListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VulnerabilityAssessmentScanRecordProperties {
    #[doc = "The scan ID."]
    #[serde(rename = "scanId", default, skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[doc = "The scan trigger type."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<vulnerability_assessment_scan_record_properties::TriggerType>,
    #[doc = "The scan status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<vulnerability_assessment_scan_record_properties::State>,
    #[doc = "The scan start time (UTC)."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The scan end time (UTC)."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The scan errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<VulnerabilityAssessmentScanError>,
    #[doc = "The scan results storage container path."]
    #[serde(rename = "storageContainerPath", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_path: Option<String>,
    #[doc = "The number of failed security checks."]
    #[serde(rename = "numberOfFailedSecurityChecks", default, skip_serializing_if = "Option::is_none")]
    pub number_of_failed_security_checks: Option<i32>,
}
impl VulnerabilityAssessmentScanRecordProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vulnerability_assessment_scan_record_properties {
    use super::*;
    #[doc = "The scan trigger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerType")]
    pub enum TriggerType {
        OnDemand,
        Recurring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnDemand => serializer.serialize_unit_variant("TriggerType", 0u32, "OnDemand"),
                Self::Recurring => serializer.serialize_unit_variant("TriggerType", 1u32, "Recurring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The scan status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Passed,
        Failed,
        FailedToRun,
        InProgress,
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
                Self::Passed => serializer.serialize_unit_variant("State", 0u32, "Passed"),
                Self::Failed => serializer.serialize_unit_variant("State", 1u32, "Failed"),
                Self::FailedToRun => serializer.serialize_unit_variant("State", 2u32, "FailedToRun"),
                Self::InProgress => serializer.serialize_unit_variant("State", 3u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Workload classifier operations for a data warehouse"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadClassifier {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workload classifier definition. For more information look at sys.workload_management_workload_classifiers (DMV)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadClassifierProperties>,
}
impl WorkloadClassifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of workload classifiers for a workload group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadClassifierListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkloadClassifier>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadClassifierListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadClassifierListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload classifier definition. For more information look at sys.workload_management_workload_classifiers (DMV)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadClassifierProperties {
    #[doc = "The workload classifier member name."]
    #[serde(rename = "memberName")]
    pub member_name: String,
    #[doc = "The workload classifier label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The workload classifier context."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "The workload classifier start time for classification."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The workload classifier end time for classification."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The workload classifier importance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
}
impl WorkloadClassifierProperties {
    pub fn new(member_name: String) -> Self {
        Self {
            member_name,
            label: None,
            context: None,
            start_time: None,
            end_time: None,
            importance: None,
        }
    }
}
#[doc = "Workload group operations for a data warehouse"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workload group definition. For more information look at sys.workload_management_workload_groups (DMV)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkloadGroupProperties>,
}
impl WorkloadGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of workload groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkloadGroupListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkloadGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkloadGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workload group definition. For more information look at sys.workload_management_workload_groups (DMV)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkloadGroupProperties {
    #[doc = "The workload group minimum percentage resource."]
    #[serde(rename = "minResourcePercent")]
    pub min_resource_percent: i32,
    #[doc = "The workload group cap percentage resource."]
    #[serde(rename = "maxResourcePercent")]
    pub max_resource_percent: i32,
    #[doc = "The workload group request minimum grant percentage."]
    #[serde(rename = "minResourcePercentPerRequest")]
    pub min_resource_percent_per_request: f64,
    #[doc = "The workload group request maximum grant percentage."]
    #[serde(rename = "maxResourcePercentPerRequest", default, skip_serializing_if = "Option::is_none")]
    pub max_resource_percent_per_request: Option<f64>,
    #[doc = "The workload group importance level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    #[doc = "The workload group query execution timeout."]
    #[serde(rename = "queryExecutionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub query_execution_timeout: Option<i32>,
}
impl WorkloadGroupProperties {
    pub fn new(min_resource_percent: i32, max_resource_percent: i32, min_resource_percent_per_request: f64) -> Self {
        Self {
            min_resource_percent,
            max_resource_percent,
            min_resource_percent_per_request,
            max_resource_percent_per_request: None,
            importance: None,
            query_execution_timeout: None,
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
