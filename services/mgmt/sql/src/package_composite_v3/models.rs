#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A list of active directory administrators."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerAzureAdAdministrator>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerAzureAdOnlyAuthentication>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureAdOnlyAuthListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "A long term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupLongTermRetentionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a long term retention policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LongTermRetentionPolicyProperties>,
}
impl BackupLongTermRetentionPolicy {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupShortTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BackupShortTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
}
impl BackupShortTermRetentionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a long term retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseLongTermRetentionPolicyProperties {
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
impl BaseLongTermRetentionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Represents a database data masking policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database data masking policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMaskingPolicyProperties>,
    #[doc = "The location of the data masking policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of data masking policy. Metadata, used for Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
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
        Disabled,
        Enabled,
    }
}
#[doc = "Represents a database data masking rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a database data masking rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataMaskingRuleProperties>,
    #[doc = "The location of the data masking rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The kind of Data Masking Rule. Metadata, used for Azure portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl DataMaskingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list data masking rules request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingRuleListResult {
    #[doc = "The list of database data masking rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataMaskingRule>,
}
impl azure_core::Continuable for DataMaskingRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
    #[doc = "The alias name. This is a legacy parameter and is no longer used."]
    #[serde(rename = "aliasName", default, skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
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
            alias_name: None,
            rule_state: None,
            schema_name,
            table_name,
            column_name,
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
        Disabled,
        Enabled,
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
            properties: None,
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: database_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
}
impl DatabaseBlobAuditingPolicyProperties {
    pub fn new(state: database_blob_auditing_policy_properties::State) -> Self {
        Self {
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
        }
    }
}
pub mod database_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "A list of databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "estimatedCompletionTime", with = "azure_core::date::rfc3339::option")]
    pub estimated_completion_time: Option<time::OffsetDateTime>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the operation can be cancelled."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
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
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "restorePointInTime", with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the time that the database was deleted."]
    #[serde(rename = "sourceDatabaseDeletionDate", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region."]
    #[serde(rename = "readScale", default, skip_serializing_if = "Option::is_none")]
    pub read_scale: Option<database_properties::ReadScale>,
    #[doc = "The number of readonly secondary replicas associated with the database."]
    #[serde(rename = "readReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub read_replica_count: Option<i32>,
    #[doc = "An ARM Resource SKU."]
    #[serde(rename = "currentSku", default, skip_serializing_if = "Option::is_none")]
    pub current_sku: Option<Sku>,
    #[doc = "Time in minutes after which database is automatically paused. A value of -1 means that automatic pause is disabled"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
    #[doc = "The storage account type used to store backups for this database."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<database_properties::StorageAccountType>,
    #[doc = "Minimal capacity that database will always have allocated, if not paused"]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "The date when database was paused by user configuration or action(ISO8601 format). Null if the database is ready."]
    #[serde(rename = "pausedDate", with = "azure_core::date::rfc3339::option")]
    pub paused_date: Option<time::OffsetDateTime>,
    #[doc = "The date when database was resumed by user action or database login (ISO8601 format). Null if the database is paused."]
    #[serde(rename = "resumedDate", with = "azure_core::date::rfc3339::option")]
    pub resumed_date: Option<time::OffsetDateTime>,
    #[doc = "The resource identifier of the source associated with the create operation of this database.\r\n\r\nWhen sourceResourceId is specified, sourceDatabaseId, recoverableDatabaseId, restorableDroppedDatabaseId and sourceDatabaseDeletionDate must not be specified and CreateMode must be PointInTimeRestore, Restore or Recover.\r\n\r\nWhen createMode is PointInTimeRestore, sourceResourceId must be the resource ID of an existing database or existing sql pool, and restorePointInTime must be specified.\r\n\r\nWhen createMode is Restore, sourceResourceId must be the resource ID of restorable dropped database or restorable dropped sql pool.\r\n\r\nWhen createMode is Recover, sourceResourceId must be the resource ID of recoverable database or recoverable sql pool.\r\n\r\nThis property allows to restore across subscriptions which is only supported for DataWarehouse edition.\r\n\r\nWhen source subscription belongs to a different tenant than target subscription, “x-ms-authorization-auxiliary” header must contain authentication token for the source tenant. For more details about “x-ms-authorization-auxiliary” header see https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/authenticate-multi-tenant "]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
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
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region."]
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
    #[doc = "The storage account type used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "GRS")]
        Grs,
        #[serde(rename = "LRS")]
        Lrs,
        #[serde(rename = "ZRS")]
        Zrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Grs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "GRS"),
                Self::Lrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "LRS"),
                Self::Zrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "ZRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains information about a database Threat Detection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Properties for a database Threat Detection policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseSecurityAlertPolicyProperties>,
}
impl DatabaseSecurityAlertPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a database Threat Detection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseSecurityAlertPolicyProperties {
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint and storageAccountAccessKey are required."]
    pub state: database_security_alert_policy_properties::State,
    #[doc = "Specifies the semicolon-separated list of alerts that are disabled, or empty string to disable no alerts. Possible values: Sql_Injection; Sql_Injection_Vulnerability; Access_Anomaly; Data_Exfiltration; Unsafe_Action."]
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Option::is_none")]
    pub disabled_alerts: Option<String>,
    #[doc = "Specifies the semicolon-separated list of e-mail addresses to which the alert is sent."]
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<String>,
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<database_security_alert_policy_properties::EmailAccountAdmins>,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs. If state is Enabled, storageEndpoint is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the Threat Detection audit storage account. If state is Enabled, storageAccountAccessKey is required."]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the Threat Detection audit logs."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies whether to use the default server policy."]
    #[serde(rename = "useServerDefault", default, skip_serializing_if = "Option::is_none")]
    pub use_server_default: Option<database_security_alert_policy_properties::UseServerDefault>,
}
impl DatabaseSecurityAlertPolicyProperties {
    pub fn new(state: database_security_alert_policy_properties::State) -> Self {
        Self {
            state,
            disabled_alerts: None,
            email_addresses: None,
            email_account_admins: None,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            use_server_default: None,
        }
    }
}
pub mod database_security_alert_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint and storageAccountAccessKey are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        New,
        Enabled,
        Disabled,
    }
    #[doc = "Specifies that the alert is sent to the account administrators."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EmailAccountAdmins {
        Enabled,
        Disabled,
    }
    #[doc = "Specifies whether to use the default server policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UseServerDefault {
        Enabled,
        Disabled,
    }
}
#[doc = "A database update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUpdate {
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
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
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "restorePointInTime", with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<time::OffsetDateTime>,
    #[doc = "Specifies the time that the database was deleted."]
    #[serde(rename = "sourceDatabaseDeletionDate", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region."]
    #[serde(rename = "readScale", default, skip_serializing_if = "Option::is_none")]
    pub read_scale: Option<database_update_properties::ReadScale>,
    #[doc = "The number of readonly secondary replicas associated with the database."]
    #[serde(rename = "readReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub read_replica_count: Option<i32>,
    #[doc = "An ARM Resource SKU."]
    #[serde(rename = "currentSku", default, skip_serializing_if = "Option::is_none")]
    pub current_sku: Option<Sku>,
    #[doc = "Time in minutes after which database is automatically paused. A value of -1 means that automatic pause is disabled"]
    #[serde(rename = "autoPauseDelay", default, skip_serializing_if = "Option::is_none")]
    pub auto_pause_delay: Option<i32>,
    #[doc = "The storage account type used to store backups for this database."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<database_update_properties::StorageAccountType>,
    #[doc = "Minimal capacity that database will always have allocated, if not paused"]
    #[serde(rename = "minCapacity", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[doc = "The date when database was paused by user configuration or action(ISO8601 format). Null if the database is ready."]
    #[serde(rename = "pausedDate", with = "azure_core::date::rfc3339::option")]
    pub paused_date: Option<time::OffsetDateTime>,
    #[doc = "The date when database was resumed by user action or database login (ISO8601 format). Null if the database is paused."]
    #[serde(rename = "resumedDate", with = "azure_core::date::rfc3339::option")]
    pub resumed_date: Option<time::OffsetDateTime>,
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
    #[doc = "The state of read-only routing. If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica in the same region."]
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
    #[doc = "The storage account type used to store backups for this database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "GRS")]
        Grs,
        #[serde(rename = "LRS")]
        Lrs,
        #[serde(rename = "ZRS")]
        Zrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Grs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "GRS"),
                Self::Lrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "LRS"),
                Self::Zrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "ZRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The database usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUsage {
    #[doc = "The name of the usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The usage metric display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The current value of the usage metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The current limit of the usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The units of the usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The next reset time for the usage metric (ISO8601 format)."]
    #[serde(rename = "nextResetTime", with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
}
impl DatabaseUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list database metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseUsageListResult {
    #[doc = "The list of database usages for the database."]
    pub value: Vec<DatabaseUsage>,
}
impl azure_core::Continuable for DatabaseUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DatabaseUsageListResult {
    pub fn new(value: Vec<DatabaseUsage>) -> Self {
        Self { value }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "A shared access signature (SAS Key) that has read and write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required."]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required."]
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
#[doc = "The edition capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EditionCapability {
    #[doc = "The database edition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported service objectives for the edition."]
    #[serde(rename = "supportedServiceLevelObjectives", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_service_level_objectives: Vec<ServiceObjectiveCapability>,
    #[doc = "Whether or not zone redundancy is supported for the edition."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The read scale capability."]
    #[serde(rename = "readScale", default, skip_serializing_if = "Option::is_none")]
    pub read_scale: Option<ReadScaleCapability>,
    #[doc = "The list of supported storage capabilities for this edition"]
    #[serde(rename = "supportedStorageCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_storage_capabilities: Vec<StorageCapability>,
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
#[doc = "Represents the activity on an elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolActivity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Represents the properties of an elastic pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ElasticPoolActivityProperties>,
}
impl ElasticPoolActivity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list elastic pool activity request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticPoolActivityListResult {
    #[doc = "The list of elastic pool activities."]
    pub value: Vec<ElasticPoolActivity>,
}
impl azure_core::Continuable for ElasticPoolActivityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ElasticPoolActivityListResult {
    pub fn new(value: Vec<ElasticPoolActivity>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the properties of an elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolActivityProperties {
    #[doc = "The time the operation finished (ISO8601 format)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The error code if available."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The error message if available."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The error severity if available."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The unique operation ID."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The percentage complete if available."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The requested max DTU per database if available."]
    #[serde(rename = "requestedDatabaseDtuMax", default, skip_serializing_if = "Option::is_none")]
    pub requested_database_dtu_max: Option<i32>,
    #[doc = "The requested min DTU per database if available."]
    #[serde(rename = "requestedDatabaseDtuMin", default, skip_serializing_if = "Option::is_none")]
    pub requested_database_dtu_min: Option<i32>,
    #[doc = "The requested DTU for the pool if available."]
    #[serde(rename = "requestedDtu", default, skip_serializing_if = "Option::is_none")]
    pub requested_dtu: Option<i32>,
    #[doc = "The requested name for the elastic pool if available."]
    #[serde(rename = "requestedElasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub requested_elastic_pool_name: Option<String>,
    #[doc = "The requested storage limit for the pool in GB if available."]
    #[serde(rename = "requestedStorageLimitInGB", default, skip_serializing_if = "Option::is_none")]
    pub requested_storage_limit_in_gb: Option<i64>,
    #[doc = "The name of the elastic pool."]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The name of the server the elastic pool is in."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The time the operation started (ISO8601 format)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The current state of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The requested storage limit in MB."]
    #[serde(rename = "requestedStorageLimitInMB", default, skip_serializing_if = "Option::is_none")]
    pub requested_storage_limit_in_mb: Option<i32>,
    #[doc = "The requested per database DTU guarantee."]
    #[serde(rename = "requestedDatabaseDtuGuarantee", default, skip_serializing_if = "Option::is_none")]
    pub requested_database_dtu_guarantee: Option<i32>,
    #[doc = "The requested per database DTU cap."]
    #[serde(rename = "requestedDatabaseDtuCap", default, skip_serializing_if = "Option::is_none")]
    pub requested_database_dtu_cap: Option<i32>,
    #[doc = "The requested DTU guarantee."]
    #[serde(rename = "requestedDtuGuarantee", default, skip_serializing_if = "Option::is_none")]
    pub requested_dtu_guarantee: Option<i32>,
}
impl ElasticPoolActivityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the activity on an elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolDatabaseActivity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Represents the properties of an elastic pool database activity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ElasticPoolDatabaseActivityProperties>,
}
impl ElasticPoolDatabaseActivity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list elastic pool database activity request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticPoolDatabaseActivityListResult {
    #[doc = "The list of elastic pool database activities."]
    pub value: Vec<ElasticPoolDatabaseActivity>,
}
impl azure_core::Continuable for ElasticPoolDatabaseActivityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ElasticPoolDatabaseActivityListResult {
    pub fn new(value: Vec<ElasticPoolDatabaseActivity>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the properties of an elastic pool database activity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolDatabaseActivityProperties {
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The time the operation finished (ISO8601 format)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The error code if available."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The error message if available."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The error severity if available."]
    #[serde(rename = "errorSeverity", default, skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<i32>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The unique operation ID."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The percentage complete if available."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The name for the elastic pool the database is moving into if available."]
    #[serde(rename = "requestedElasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub requested_elastic_pool_name: Option<String>,
    #[doc = "The name of the current elastic pool the database is in if available."]
    #[serde(rename = "currentElasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub current_elastic_pool_name: Option<String>,
    #[doc = "The name of the current service objective if available."]
    #[serde(rename = "currentServiceObjective", default, skip_serializing_if = "Option::is_none")]
    pub current_service_objective: Option<String>,
    #[doc = "The name of the requested service objective if available."]
    #[serde(rename = "requestedServiceObjective", default, skip_serializing_if = "Option::is_none")]
    pub requested_service_objective: Option<String>,
    #[doc = "The name of the server the elastic pool is in."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The time the operation started (ISO8601 format)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The current state of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl ElasticPoolDatabaseActivityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The elastic pool edition capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticPoolEditionCapability {
    #[doc = "The elastic pool edition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported elastic pool DTU levels for the edition."]
    #[serde(rename = "supportedElasticPoolPerformanceLevels", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_elastic_pool_performance_levels: Vec<ElasticPoolPerformanceLevelCapability>,
    #[doc = "Whether or not zone redundancy is supported for the edition."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ElasticPool>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ElasticPoolOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticPoolOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "estimatedCompletionTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "supportedLicenseTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "The maximum number of databases supported."]
    #[serde(rename = "maxDatabaseCount", default, skip_serializing_if = "Option::is_none")]
    pub max_database_count: Option<i32>,
    #[doc = "The maximum size capability."]
    #[serde(rename = "includedMaxSize", default, skip_serializing_if = "Option::is_none")]
    pub included_max_size: Option<MaxSizeCapability>,
    #[doc = "The list of supported max sizes."]
    #[serde(rename = "supportedMaxSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_max_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "The list of supported per database max sizes."]
    #[serde(rename = "supportedPerDatabaseMaxSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_per_database_max_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "The list of supported per database max performance levels."]
    #[serde(
        rename = "supportedPerDatabaseMaxPerformanceLevels",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_per_database_max_performance_levels: Vec<ElasticPoolPerDatabaseMaxPerformanceLevelCapability>,
    #[doc = "Whether or not zone redundancy is supported for the performance level."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
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
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The storage limit for the database elastic pool in bytes."]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
    #[doc = "Per database settings of an elastic pool."]
    #[serde(rename = "perDatabaseSettings", default, skip_serializing_if = "Option::is_none")]
    pub per_database_settings: Option<ElasticPoolPerDatabaseSettings>,
    #[doc = "Whether or not this elastic pool is zone redundant, which means the replicas of this elastic pool will be spread across multiple availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The license type to apply for this elastic pool."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<elastic_pool_properties::LicenseType>,
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
    #[doc = "Per database settings of an elastic pool."]
    #[serde(rename = "perDatabaseSettings", default, skip_serializing_if = "Option::is_none")]
    pub per_database_settings: Option<ElasticPoolPerDatabaseSettings>,
    #[doc = "Whether or not this elastic pool is zone redundant, which means the replicas of this elastic pool will be spread across multiple availability zones."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "The license type to apply for this elastic pool."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<elastic_pool_update_properties::LicenseType>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EncryptionProtector>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EncryptionProtectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
}
impl EncryptionProtectorProperties {
    pub fn new(server_key_type: encryption_protector_properties::ServerKeyType) -> Self {
        Self {
            subregion: None,
            server_key_name: None,
            server_key_type,
            uri: None,
            thumbprint: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtendedDatabaseBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtendedDatabaseBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: extended_database_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
}
impl ExtendedDatabaseBlobAuditingPolicyProperties {
    pub fn new(state: extended_database_blob_auditing_policy_properties::State) -> Self {
        Self {
            predicate_expression: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
        }
    }
}
pub mod extended_database_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExtendedServerBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExtendedServerBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Specifies condition of where clause when creating an audit."]
    #[serde(rename = "predicateExpression", default, skip_serializing_if = "Option::is_none")]
    pub predicate_expression: Option<String>,
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: extended_server_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
}
impl ExtendedServerBlobAuditingPolicyProperties {
    pub fn new(state: extended_server_blob_auditing_policy_properties::State) -> Self {
        Self {
            predicate_expression: None,
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
        }
    }
}
pub mod extended_server_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FailoverGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FailoverGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub databases: Vec<String>,
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
}
#[doc = "Read-only endpoint of the failover group instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroupReadOnlyEndpoint {
    #[doc = "Failover policy of the read-only endpoint for the failover group."]
    #[serde(rename = "failoverPolicy", default, skip_serializing_if = "Option::is_none")]
    pub failover_policy: Option<failover_group_read_only_endpoint::FailoverPolicy>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub databases: Vec<String>,
}
impl FailoverGroupUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Kind of server that contains this firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Location of the server that contains this firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Type of resource this is."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Represents the properties of a server firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}
impl FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a List Firewall Rules request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[doc = "The list of server firewall rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
}
impl azure_core::Continuable for FirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a server firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[doc = "The start IP address of the firewall rule. Must be IPv4 format. Use value '0.0.0.0' to represent all Azure-internal IP addresses."]
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[doc = "The end IP address of the firewall rule. Must be IPv4 format. Must be greater than or equal to startIpAddress. Use value '0.0.0.0' to represent all Azure-internal IP addresses."]
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
impl FirewallRuleProperties {
    pub fn new(start_ip_address: String, end_ip_address: String) -> Self {
        Self {
            start_ip_address,
            end_ip_address,
        }
    }
}
#[doc = "A database geo backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoBackupPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the geo backup policy."]
    pub properties: GeoBackupPolicyProperties,
    #[doc = "Kind of geo backup policy.  This is metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Backup policy location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl GeoBackupPolicy {
    pub fn new(properties: GeoBackupPolicyProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            kind: None,
            location: None,
        }
    }
}
#[doc = "The response to a list geo backup policies request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GeoBackupPolicyListResult {
    #[doc = "The list of geo backup policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GeoBackupPolicy>,
}
impl azure_core::Continuable for GeoBackupPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
        Disabled,
        Enabled,
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
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InstanceFailoverGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InstanceFailoverGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "supportedFamilies", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "supportedLicenseTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "List of supported virtual cores values."]
    #[serde(rename = "supportedVcoresValues", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InstancePool>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InstancePoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
}
impl InstancePoolProperties {
    pub fn new(subnet_id: String, v_cores: i32, license_type: instance_pool_properties::LicenseType) -> Self {
        Self {
            subnet_id,
            v_cores,
            license_type,
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
    #[doc = "Properties of a job agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobAgentProperties>,
}
impl JobAgent {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "A list of Azure SQL job agents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobAgent>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobAgentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "An update to an Azure SQL job agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobAgentUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl JobAgentUpdate {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobCredential>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobCredentialListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobExecution>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobExecutionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The time that the job execution started."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The time that the job execution completed."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Number of times the job execution has been attempted."]
    #[serde(rename = "currentAttempts", default, skip_serializing_if = "Option::is_none")]
    pub current_attempts: Option<i32>,
    #[doc = "Start time of the current attempt."]
    #[serde(rename = "currentAttemptStartTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Job>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Schedule end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Schedule interval type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_schedule::Type>,
    #[doc = "Whether or not the schedule is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Value of the schedule's recurring interval, if the schedule type is recurring. ISO8601 duration format."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobStep>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobStepListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub credential: String,
}
impl JobStepOutput {
    pub fn new(server_name: String, database_name: String, table_name: String, credential: String) -> Self {
        Self {
            type_: None,
            subscription_id: None,
            resource_group_name: None,
            server_name,
            database_name,
            schema_name: None,
            table_name,
            credential,
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
    pub credential: String,
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
    pub fn new(target_group: String, credential: String, action: JobStepAction) -> Self {
        Self {
            step_id: None,
            target_group,
            credential,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobTargetGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobTargetGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobVersion>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobVersionListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "supportedServerVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[doc = "The list of supported managed instance versions."]
    #[serde(rename = "supportedManagedInstanceVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_managed_instance_versions: Vec<ManagedInstanceVersionCapability>,
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
#[doc = "A list of the server's security alert policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalServerSecurityAlertPolicyListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogicalServerSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LongTermRetentionBackup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LongTermRetentionBackupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LongTermRetentionBackupListResult {
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
    #[serde(rename = "serverCreateTime", with = "azure_core::date::rfc3339::option")]
    pub server_create_time: Option<time::OffsetDateTime>,
    #[doc = "The name of the database the backup belong to"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The delete time of the database"]
    #[serde(rename = "databaseDeletionTime", with = "azure_core::date::rfc3339::option")]
    pub database_deletion_time: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "backupTime", with = "azure_core::date::rfc3339::option")]
    pub backup_time: Option<time::OffsetDateTime>,
    #[doc = "The time the long term retention backup will expire."]
    #[serde(rename = "backupExpirationTime", with = "azure_core::date::rfc3339::option")]
    pub backup_expiration_time: Option<time::OffsetDateTime>,
}
impl LongTermRetentionBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a long term retention policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LongTermRetentionPolicyProperties {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedBackupShortTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedBackupShortTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "A list of managed databases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDatabaseListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Earliest restore point in time for point in time restore."]
    #[serde(rename = "earliestRestorePoint", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_point: Option<time::OffsetDateTime>,
    #[doc = "Conditional. If createMode is PointInTimeRestore, this value is required. Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database."]
    #[serde(rename = "restorePointInTime", with = "azure_core::date::rfc3339::option")]
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
    #[doc = "The restorable dropped database resource id to restore when creating this database."]
    #[serde(rename = "restorableDroppedDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub restorable_dropped_database_id: Option<String>,
    #[doc = "Conditional. If createMode is RestoreExternalBackup, this value is required. Specifies the storage container sas token."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedDatabaseSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedDatabaseSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedDatabaseSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceAdministrator>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceAdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "A list of active directory only authentications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceAzureAdOnlyAuthListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceAzureAdOnlyAuthentication>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceAzureAdOnlyAuthListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "The managed server capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceEditionCapability {
    #[doc = "The managed server version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The supported families."]
    #[serde(rename = "supportedFamilies", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_families: Vec<ManagedInstanceFamilyCapability>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceEncryptionProtector>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceEncryptionProtectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
}
impl ManagedInstanceEncryptionProtectorProperties {
    pub fn new(server_key_type: managed_instance_encryption_protector_properties::ServerKeyType) -> Self {
        Self {
            server_key_name: None,
            server_key_type,
            uri: None,
            thumbprint: None,
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
#[doc = "The managed server family capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceFamilyCapability {
    #[doc = "Family name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "List of supported license types."]
    #[serde(rename = "supportedLicenseTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_license_types: Vec<LicenseTypeCapability>,
    #[doc = "List of supported virtual cores values."]
    #[serde(rename = "supportedVcoresValues", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceKey>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceKeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
}
impl ManagedInstanceKeyProperties {
    pub fn new(server_key_type: managed_instance_key_properties::ServerKeyType) -> Self {
        Self {
            server_key_type,
            uri: None,
            thumbprint: None,
            creation_date: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceLongTermRetentionBackup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceLongTermRetentionBackupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "managedInstanceCreateTime", with = "azure_core::date::rfc3339::option")]
    pub managed_instance_create_time: Option<time::OffsetDateTime>,
    #[doc = "The name of the database the backup belong to"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The delete time of the database"]
    #[serde(rename = "databaseDeletionTime", with = "azure_core::date::rfc3339::option")]
    pub database_deletion_time: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "backupTime", with = "azure_core::date::rfc3339::option")]
    pub backup_time: Option<time::OffsetDateTime>,
    #[doc = "The time the long term retention backup will expire."]
    #[serde(rename = "backupExpirationTime", with = "azure_core::date::rfc3339::option")]
    pub backup_expiration_time: Option<time::OffsetDateTime>,
}
impl ManagedInstanceLongTermRetentionBackupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A long term retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceLongTermRetentionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a long term retention policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaseLongTermRetentionPolicyProperties>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceLongTermRetentionPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceLongTermRetentionPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedInstanceLongTermRetentionPolicyListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceOperation>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "estimatedCompletionTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "stepsList", default, skip_serializing_if = "Vec::is_empty")]
    pub steps_list: Vec<UpsertManagedServerOperationStep>,
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
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<managed_instance_properties::ProvisioningState>,
    #[doc = "Specifies the mode of database creation.\r\n\r\nDefault: Regular instance creation.\r\n\r\nRestore: Creates an instance by restoring a set of backups to specific point in time. RestorePointInTime and SourceManagedInstanceId must be specified."]
    #[serde(rename = "managedInstanceCreateMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_instance_create_mode: Option<managed_instance_properties::ManagedInstanceCreateMode>,
    #[doc = "The fully qualified domain name of the managed instance."]
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
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
    #[doc = "The number of vCores. Allowed values: 8, 16, 24, 32, 40, 64, 80."]
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i32>,
    #[doc = "Storage size in GB. Minimum value: 32. Maximum value: 8192. Increments of 32 GB allowed only."]
    #[serde(rename = "storageSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_in_gb: Option<i32>,
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
    #[serde(rename = "restorePointInTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<ManagedInstancePecProperty>,
    #[doc = "Minimal TLS version. Allowed values: 'None', '1.0', '1.1', '1.2'"]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<String>,
    #[doc = "The storage account type used to store backups for this instance. The options are LRS (LocallyRedundantStorage), ZRS (ZoneRedundantStorage) and GRS (GeoRedundantStorage)"]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<managed_instance_properties::StorageAccountType>,
    #[doc = "Whether or not the multi-az is enabled."]
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
}
impl ManagedInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_instance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Deleting,
        Updating,
        Unknown,
        Succeeded,
        Failed,
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
    #[doc = "The storage account type used to store backups for this instance. The options are LRS (LocallyRedundantStorage), ZRS (ZoneRedundantStorage) and GRS (GeoRedundantStorage)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "GRS")]
        Grs,
        #[serde(rename = "LRS")]
        Lrs,
        #[serde(rename = "ZRS")]
        Zrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Grs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "GRS"),
                Self::Lrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "LRS"),
                Self::Zrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "ZRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[serde(rename = "supportedStorageSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_storage_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "True if this service objective is supported for managed instances in an instance pool."]
    #[serde(rename = "instancePoolSupported", default, skip_serializing_if = "Option::is_none")]
    pub instance_pool_supported: Option<bool>,
    #[doc = "True if this service objective is supported for standalone managed instances."]
    #[serde(rename = "standaloneSupported", default, skip_serializing_if = "Option::is_none")]
    pub standalone_supported: Option<bool>,
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
    #[serde(rename = "supportedEditions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_editions: Vec<ManagedInstanceEditionCapability>,
    #[doc = "The list of supported instance pool editions."]
    #[serde(rename = "supportedInstancePoolEditions", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedInstanceVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedInstanceVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "A shared access signature (SAS Key) that has read and write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required."]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required."]
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
#[doc = "A managed server security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedServerSecurityAlertPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedServerSecurityAlertPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedServerSecurityAlertPolicyListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Database metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metric {
    #[doc = "The start time for the metric (ISO-8601 format)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time for the metric (ISO-8601 format)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The time step to be used to summarize the metric values."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metric::Unit>,
    #[doc = "A database metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The metric values for the specified time window and timestep."]
    #[serde(rename = "metricValues", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_values: Vec<MetricValue>,
}
impl Metric {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric {
    use super::*;
    #[doc = "The unit of the metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        #[serde(rename = "count")]
        Count,
        #[serde(rename = "bytes")]
        Bytes,
        #[serde(rename = "seconds")]
        Seconds,
        #[serde(rename = "percent")]
        Percent,
        #[serde(rename = "countPerSecond")]
        CountPerSecond,
        #[serde(rename = "bytesPerSecond")]
        BytesPerSecond,
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
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "count"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "bytes"),
                Self::Seconds => serializer.serialize_unit_variant("Unit", 2u32, "seconds"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 3u32, "percent"),
                Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 4u32, "countPerSecond"),
                Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 5u32, "bytesPerSecond"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A metric availability value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[doc = "The length of retention for the database metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    #[doc = "The granularity of the database metric."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A database metric definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[doc = "A database metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The primary aggregation type defining how metric values are displayed."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[doc = "The resource uri of the database."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metric_definition::Unit>,
    #[doc = "The list of database metric availabilities for the metric."]
    #[serde(rename = "metricAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_availabilities: Vec<MetricAvailability>,
}
impl MetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_definition {
    use super::*;
    #[doc = "The primary aggregation type defining how metric values are displayed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrimaryAggregationType")]
    pub enum PrimaryAggregationType {
        None,
        Average,
        Count,
        Minimum,
        Maximum,
        Total,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrimaryAggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrimaryAggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrimaryAggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PrimaryAggregationType", 0u32, "None"),
                Self::Average => serializer.serialize_unit_variant("PrimaryAggregationType", 1u32, "Average"),
                Self::Count => serializer.serialize_unit_variant("PrimaryAggregationType", 2u32, "Count"),
                Self::Minimum => serializer.serialize_unit_variant("PrimaryAggregationType", 3u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("PrimaryAggregationType", 4u32, "Maximum"),
                Self::Total => serializer.serialize_unit_variant("PrimaryAggregationType", 5u32, "Total"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The unit of the metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
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
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "Bytes"),
                Self::Seconds => serializer.serialize_unit_variant("Unit", 2u32, "Seconds"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 3u32, "Percent"),
                Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 4u32, "CountPerSecond"),
                Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 5u32, "BytesPerSecond"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response to a list database metric definitions request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionListResult {
    #[doc = "The list of metric definitions for the database."]
    pub value: Vec<MetricDefinition>,
}
impl azure_core::Continuable for MetricDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricDefinitionListResult {
    pub fn new(value: Vec<MetricDefinition>) -> Self {
        Self { value }
    }
}
#[doc = "The response to a list database metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricListResult {
    #[doc = "The list of metrics for the database."]
    pub value: Vec<Metric>,
}
impl azure_core::Continuable for MetricListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricListResult {
    pub fn new(value: Vec<Metric>) -> Self {
        Self { value }
    }
}
#[doc = "A database metric name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[doc = "The name of the database metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The friendly name of the database metric."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents database metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricValue {
    #[doc = "The number of values for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[doc = "The average value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "The max value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[doc = "The min value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "The metric timestamp (ISO-8601 format)."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The total value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}
impl MetricValue {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The impact of an operation, both in absolute and relative terms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationImpact {
    #[doc = "The name of the impact dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unit in which estimated impact to dimension is measured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The absolute impact to dimension."]
    #[serde(rename = "changeValueAbsolute", default, skip_serializing_if = "Option::is_none")]
    pub change_value_absolute: Option<f64>,
    #[doc = "The relative impact to dimension (null if not applicable)"]
    #[serde(rename = "changeValueRelative", default, skip_serializing_if = "Option::is_none")]
    pub change_value_relative: Option<f64>,
}
impl OperationImpact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list SQL operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Link to retrieve next page of results."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Represents a recommended elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedElasticPool {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Represents the properties of a recommended elastic pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendedElasticPoolProperties>,
}
impl RecommendedElasticPool {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list recommended elastic pool metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedElasticPoolListMetricsResult {
    #[doc = "The list of recommended elastic pools metrics."]
    pub value: Vec<RecommendedElasticPoolMetric>,
}
impl azure_core::Continuable for RecommendedElasticPoolListMetricsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RecommendedElasticPoolListMetricsResult {
    pub fn new(value: Vec<RecommendedElasticPoolMetric>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the response to a list recommended elastic pool request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedElasticPoolListResult {
    #[doc = "The list of recommended elastic pools hosted in the server."]
    pub value: Vec<RecommendedElasticPool>,
}
impl azure_core::Continuable for RecommendedElasticPoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RecommendedElasticPoolListResult {
    pub fn new(value: Vec<RecommendedElasticPool>) -> Self {
        Self { value }
    }
}
#[doc = "Represents recommended elastic pool metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedElasticPoolMetric {
    #[doc = "The time of metric (ISO8601 format)."]
    #[serde(rename = "dateTime", with = "azure_core::date::rfc3339::option")]
    pub date_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the DTUs (Database Transaction Units). See https://azure.microsoft.com/documentation/articles/sql-database-what-is-a-dtu/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dtu: Option<f64>,
    #[doc = "Gets or sets size in gigabytes."]
    #[serde(rename = "sizeGB", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<f64>,
}
impl RecommendedElasticPoolMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a recommended elastic pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedElasticPoolProperties {
    #[doc = "The edition of the recommended elastic pool. The ElasticPoolEdition enumeration contains all the valid editions."]
    #[serde(rename = "databaseEdition", default, skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<recommended_elastic_pool_properties::DatabaseEdition>,
    #[doc = "The DTU for the recommended elastic pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dtu: Option<f64>,
    #[doc = "The minimum DTU for the database."]
    #[serde(rename = "databaseDtuMin", default, skip_serializing_if = "Option::is_none")]
    pub database_dtu_min: Option<f64>,
    #[doc = "The maximum DTU for the database."]
    #[serde(rename = "databaseDtuMax", default, skip_serializing_if = "Option::is_none")]
    pub database_dtu_max: Option<f64>,
    #[doc = "Gets storage size in megabytes."]
    #[serde(rename = "storageMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_mb: Option<f64>,
    #[doc = "The observation period start (ISO8601 format)."]
    #[serde(rename = "observationPeriodStart", with = "azure_core::date::rfc3339::option")]
    pub observation_period_start: Option<time::OffsetDateTime>,
    #[doc = "The observation period start (ISO8601 format)."]
    #[serde(rename = "observationPeriodEnd", with = "azure_core::date::rfc3339::option")]
    pub observation_period_end: Option<time::OffsetDateTime>,
    #[doc = "Gets maximum observed DTU."]
    #[serde(rename = "maxObservedDtu", default, skip_serializing_if = "Option::is_none")]
    pub max_observed_dtu: Option<f64>,
    #[doc = "Gets maximum observed storage in megabytes."]
    #[serde(rename = "maxObservedStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_observed_storage_mb: Option<f64>,
    #[doc = "The list of databases in this pool. Expanded property"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub databases: Vec<TrackedResource>,
    #[doc = "The list of databases housed in the server. Expanded property"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<RecommendedElasticPoolMetric>,
}
impl RecommendedElasticPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommended_elastic_pool_properties {
    use super::*;
    #[doc = "The edition of the recommended elastic pool. The ElasticPoolEdition enumeration contains all the valid editions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DatabaseEdition")]
    pub enum DatabaseEdition {
        Basic,
        Standard,
        Premium,
        GeneralPurpose,
        BusinessCritical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DatabaseEdition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DatabaseEdition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DatabaseEdition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("DatabaseEdition", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("DatabaseEdition", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("DatabaseEdition", 2u32, "Premium"),
                Self::GeneralPurpose => serializer.serialize_unit_variant("DatabaseEdition", 3u32, "GeneralPurpose"),
                Self::BusinessCritical => serializer.serialize_unit_variant("DatabaseEdition", 4u32, "BusinessCritical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a database recommended index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedIndex {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Represents the properties of a database recommended index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendedIndexProperties>,
}
impl RecommendedIndex {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a database recommended index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedIndexProperties {
    #[doc = "The proposed index action. You can create a missing index, drop an unused index, or rebuild an existing index to improve its performance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<recommended_index_properties::Action>,
    #[doc = "The current recommendation state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<recommended_index_properties::State>,
    #[doc = "The UTC datetime showing when this resource was created (ISO8601 format)."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The UTC datetime of when was this resource last changed (ISO8601 format)."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "The type of index (CLUSTERED, NONCLUSTERED, COLUMNSTORE, CLUSTERED COLUMNSTORE)"]
    #[serde(rename = "indexType", default, skip_serializing_if = "Option::is_none")]
    pub index_type: Option<recommended_index_properties::IndexType>,
    #[doc = "The schema where table to build index over resides"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The table on which to build index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "Columns over which to build index"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
    #[doc = "The list of column names to be included in the index"]
    #[serde(rename = "includedColumns", default, skip_serializing_if = "Vec::is_empty")]
    pub included_columns: Vec<String>,
    #[doc = "The full build index script"]
    #[serde(rename = "indexScript", default, skip_serializing_if = "Option::is_none")]
    pub index_script: Option<String>,
    #[doc = "The estimated impact of doing recommended index action."]
    #[serde(rename = "estimatedImpact", default, skip_serializing_if = "Vec::is_empty")]
    pub estimated_impact: Vec<OperationImpact>,
    #[doc = "The values reported after index action is complete."]
    #[serde(rename = "reportedImpact", default, skip_serializing_if = "Vec::is_empty")]
    pub reported_impact: Vec<OperationImpact>,
}
impl RecommendedIndexProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommended_index_properties {
    use super::*;
    #[doc = "The proposed index action. You can create a missing index, drop an unused index, or rebuild an existing index to improve its performance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Create,
        Drop,
        Rebuild,
    }
    #[doc = "The current recommendation state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Active,
        Pending,
        Executing,
        Verifying,
        #[serde(rename = "Pending Revert")]
        PendingRevert,
        Reverting,
        Reverted,
        Ignored,
        Expired,
        Blocked,
        Success,
    }
    #[doc = "The type of index (CLUSTERED, NONCLUSTERED, COLUMNSTORE, CLUSTERED COLUMNSTORE)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IndexType {
        #[serde(rename = "CLUSTERED")]
        Clustered,
        #[serde(rename = "NONCLUSTERED")]
        Nonclustered,
        #[serde(rename = "COLUMNSTORE")]
        Columnstore,
        #[serde(rename = "CLUSTERED COLUMNSTORE")]
        ClusteredColumnstore,
    }
}
#[doc = "A recoverable database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableDatabase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a recoverable database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableDatabaseProperties>,
}
impl RecoverableDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list recoverable databases request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoverableDatabaseListResult {
    #[doc = "A list of recoverable databases"]
    pub value: Vec<RecoverableDatabase>,
}
impl azure_core::Continuable for RecoverableDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RecoverableDatabaseListResult {
    pub fn new(value: Vec<RecoverableDatabase>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of a recoverable database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableDatabaseProperties {
    #[doc = "The edition of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "The service level objective name of the database"]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[doc = "The elastic pool name of the database"]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The last available backup date of the database (ISO8601 format)"]
    #[serde(rename = "lastAvailableBackupDate", with = "azure_core::date::rfc3339::option")]
    pub last_available_backup_date: Option<time::OffsetDateTime>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecoverableManagedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoverableManagedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "Represents a database replication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Location of the server that contains this firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Type of resource this is."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Represents the properties of a database replication link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationLinkProperties>,
}
impl ReplicationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a List database replication link request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkListResult {
    #[doc = "The list of database replication links housed in the database."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationLink>,
}
impl azure_core::Continuable for ReplicationLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ReplicationLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of a database replication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationLinkProperties {
    #[doc = "Legacy value indicating whether termination is allowed.  Currently always returns true."]
    #[serde(rename = "isTerminationAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_termination_allowed: Option<bool>,
    #[doc = "Replication mode of this replication link."]
    #[serde(rename = "replicationMode", default, skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<String>,
    #[doc = "The name of the server hosting the partner database."]
    #[serde(rename = "partnerServer", default, skip_serializing_if = "Option::is_none")]
    pub partner_server: Option<String>,
    #[doc = "The name of the partner database."]
    #[serde(rename = "partnerDatabase", default, skip_serializing_if = "Option::is_none")]
    pub partner_database: Option<String>,
    #[doc = "The Azure Region of the partner database."]
    #[serde(rename = "partnerLocation", default, skip_serializing_if = "Option::is_none")]
    pub partner_location: Option<String>,
    #[doc = "The role of the database in the replication link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<replication_link_properties::Role>,
    #[doc = "The role of the partner database in the replication link."]
    #[serde(rename = "partnerRole", default, skip_serializing_if = "Option::is_none")]
    pub partner_role: Option<replication_link_properties::PartnerRole>,
    #[doc = "The start time for the replication link."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The percentage of seeding complete for the replication link."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The replication state for the replication link."]
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<replication_link_properties::ReplicationState>,
}
impl ReplicationLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_link_properties {
    use super::*;
    #[doc = "The role of the database in the replication link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Primary,
        Secondary,
        NonReadableSecondary,
        Source,
        Copy,
    }
    #[doc = "The role of the partner database in the replication link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PartnerRole {
        Primary,
        Secondary,
        NonReadableSecondary,
        Source,
        Copy,
    }
    #[doc = "The replication state for the replication link."]
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
#[doc = "A restorable dropped database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedDatabase {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The properties of a restorable dropped database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableDroppedDatabaseProperties>,
}
impl RestorableDroppedDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list restorable dropped databases request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorableDroppedDatabaseListResult {
    #[doc = "A list of restorable dropped databases"]
    pub value: Vec<RestorableDroppedDatabase>,
}
impl azure_core::Continuable for RestorableDroppedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableDroppedDatabaseListResult {
    pub fn new(value: Vec<RestorableDroppedDatabase>) -> Self {
        Self { value }
    }
}
#[doc = "The properties of a restorable dropped database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDroppedDatabaseProperties {
    #[doc = "The name of the database"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The edition of the database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "The max size in bytes of the database"]
    #[serde(rename = "maxSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<String>,
    #[doc = "The service level objective name of the database"]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[doc = "The elastic pool name of the database"]
    #[serde(rename = "elasticPoolName", default, skip_serializing_if = "Option::is_none")]
    pub elastic_pool_name: Option<String>,
    #[doc = "The creation date of the database (ISO8601 format)"]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The deletion date of the database (ISO8601 format)"]
    #[serde(rename = "deletionDate", with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The earliest restore date of the database (ISO8601 format)"]
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
}
impl RestorableDroppedDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorableDroppedManagedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorableDroppedManagedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The deletion date of the database (ISO8601 format)."]
    #[serde(rename = "deletionDate", with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
    #[doc = "The earliest restore date of the database (ISO8601 format)."]
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorePoint>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorePointListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
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
    #[serde(rename = "earliestRestoreDate", with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<time::OffsetDateTime>,
    #[doc = "The time the backup was taken"]
    #[serde(rename = "restorePointCreationDate", with = "azure_core::date::rfc3339::option")]
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
#[doc = "Properties of a security alert policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    #[doc = "Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific database."]
    pub state: security_alert_policy_properties::State,
    #[doc = "Specifies an array of alerts that are disabled. Allowed values are: Sql_Injection, Sql_Injection_Vulnerability, Access_Anomaly, Data_Exfiltration, Unsafe_Action"]
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[doc = "Specifies an array of e-mail addresses to which the alert is sent."]
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
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
#[doc = "A sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SensitivityLabel>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SensitivityLabelListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerBlobAuditingPolicy>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerBlobAuditingPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    pub state: server_blob_auditing_policy_properties::State,
    #[doc = "Specifies the blob storage endpoint (e.g. https://MyAccount.blob.core.windows.net). If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled is required."]
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[doc = "Specifies the identifier key of the auditing storage account. \r\nIf state is Enabled and storageEndpoint is specified, not specifying the storageAccountAccessKey will use SQL server system-assigned managed identity to access the storage.\r\nPrerequisites for using managed identity authentication:\r\n1. Assign SQL Server a system-assigned managed identity in Azure Active Directory (AAD).\r\n2. Grant SQL Server identity access to the storage account by adding 'Storage Blob Data Contributor' RBAC role to the server identity.\r\nFor more information, see [Auditing to storage using Managed Identity authentication](https://go.microsoft.com/fwlink/?linkid=2114355)"]
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[doc = "Specifies the number of days to keep in the audit logs in the storage account."]
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[doc = "Specifies the Actions-Groups and Actions to audit.\r\n\r\nThe recommended set of action groups to use is the following combination - this will audit all the queries and stored procedures executed against the database, as well as successful and failed logins:\r\n\r\nBATCH_COMPLETED_GROUP,\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP,\r\nFAILED_DATABASE_AUTHENTICATION_GROUP.\r\n\r\nThis above combination is also the set that is configured by default when enabling auditing from the Azure portal.\r\n\r\nThe supported action groups to audit are (note: choose only specific groups that cover your auditing needs. Using unnecessary groups could lead to very large quantities of audit records):\r\n\r\nAPPLICATION_ROLE_CHANGE_PASSWORD_GROUP\r\nBACKUP_RESTORE_GROUP\r\nDATABASE_LOGOUT_GROUP\r\nDATABASE_OBJECT_CHANGE_GROUP\r\nDATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_OBJECT_PERMISSION_CHANGE_GROUP\r\nDATABASE_OPERATION_GROUP\r\nDATABASE_PERMISSION_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_CHANGE_GROUP\r\nDATABASE_PRINCIPAL_IMPERSONATION_GROUP\r\nDATABASE_ROLE_MEMBER_CHANGE_GROUP\r\nFAILED_DATABASE_AUTHENTICATION_GROUP\r\nSCHEMA_OBJECT_ACCESS_GROUP\r\nSCHEMA_OBJECT_CHANGE_GROUP\r\nSCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\r\nSCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\r\nSUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\r\nUSER_CHANGE_PASSWORD_GROUP\r\nBATCH_STARTED_GROUP\r\nBATCH_COMPLETED_GROUP\r\nDBCC_GROUP\r\nDATABASE_OWNERSHIP_CHANGE_GROUP\r\nDATABASE_CHANGE_GROUP\r\n\r\nThese are groups that cover all sql statements and stored procedures executed against the database, and should not be used in combination with other groups as this will result in duplicate audit logs.\r\n\r\nFor more information, see [Database-Level Audit Action Groups](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-action-groups).\r\n\r\nFor Database auditing policy, specific Actions can also be specified (note that Actions cannot be specified for Server auditing policy). The supported actions to audit are:\r\nSELECT\r\nUPDATE\r\nINSERT\r\nDELETE\r\nEXECUTE\r\nRECEIVE\r\nREFERENCES\r\n\r\nThe general form for defining an action to be audited is:\r\n{action} ON {object} BY {principal}\r\n\r\nNote that <object> in the above format can refer to an object like a table, view, or stored procedure, or an entire database or schema. For the latter cases, the forms DATABASE::{db_name} and SCHEMA::{schema_name} are used, respectively.\r\n\r\nFor example:\r\nSELECT on dbo.myTable by public\r\nSELECT on DATABASE::myDatabase by public\r\nSELECT on SCHEMA::mySchema by public\r\n\r\nFor more information, see [Database-Level Audit Actions](https://docs.microsoft.com/en-us/sql/relational-databases/security/auditing/sql-server-audit-action-groups-and-actions#database-level-audit-actions)"]
    #[serde(rename = "auditActionsAndGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub audit_actions_and_groups: Vec<String>,
    #[doc = "Specifies the blob storage subscription Id."]
    #[serde(rename = "storageAccountSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_subscription_id: Option<String>,
    #[doc = "Specifies whether storageAccountAccessKey value is the storage's secondary key."]
    #[serde(rename = "isStorageSecondaryKeyInUse", default, skip_serializing_if = "Option::is_none")]
    pub is_storage_secondary_key_in_use: Option<bool>,
    #[doc = "Specifies whether audit events are sent to Azure Monitor. \r\nIn order to send the events to Azure Monitor, specify 'state' as 'Enabled' and 'isAzureMonitorTargetEnabled' as true.\r\n\r\nWhen using REST API to configure auditing, Diagnostic Settings with 'SQLSecurityAuditEvents' diagnostic logs category on the database should be also created.\r\nNote that for server level audit you should use the 'master' database as {databaseName}.\r\n\r\nDiagnostic Settings URI format:\r\nPUT https://management.azure.com/subscriptions/{subscriptionId}/resourceGroups/{resourceGroup}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}/providers/microsoft.insights/diagnosticSettings/{settingsName}?api-version=2017-05-01-preview\r\n\r\nFor more information, see [Diagnostic Settings REST API](https://go.microsoft.com/fwlink/?linkid=2033207)\r\nor [Diagnostic Settings PowerShell](https://go.microsoft.com/fwlink/?linkid=2033043)\r\n"]
    #[serde(rename = "isAzureMonitorTargetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
    #[doc = "Specifies the amount of time in milliseconds that can elapse before audit actions are forced to be processed.\r\nThe default minimum value is 1000 (1 second). The maximum is 2,147,483,647."]
    #[serde(rename = "queueDelayMs", default, skip_serializing_if = "Option::is_none")]
    pub queue_delay_ms: Option<i32>,
}
impl ServerBlobAuditingPolicyProperties {
    pub fn new(state: server_blob_auditing_policy_properties::State) -> Self {
        Self {
            state,
            storage_endpoint: None,
            storage_account_access_key: None,
            retention_days: None,
            audit_actions_and_groups: Vec::new(),
            storage_account_subscription_id: None,
            is_storage_secondary_key_in_use: None,
            is_azure_monitor_target_enabled: None,
            queue_delay_ms: None,
        }
    }
}
pub mod server_blob_auditing_policy_properties {
    use super::*;
    #[doc = "Specifies the state of the policy. If state is Enabled, storageEndpoint or isAzureMonitorTargetEnabled are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[doc = "Server communication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerCommunicationLink {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a server communication link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerCommunicationLinkProperties>,
    #[doc = "Communication link location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Communication link kind.  This property is used for Azure Portal metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl ServerCommunicationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server communication links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerCommunicationLinkListResult {
    #[doc = "The list of server communication links."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerCommunicationLink>,
}
impl azure_core::Continuable for ServerCommunicationLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServerCommunicationLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server communication link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerCommunicationLinkProperties {
    #[doc = "The state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The name of the partner server."]
    #[serde(rename = "partnerServer")]
    pub partner_server: String,
}
impl ServerCommunicationLinkProperties {
    pub fn new(partner_server: String) -> Self {
        Self {
            state: None,
            partner_server,
        }
    }
}
#[doc = "A server secure connection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConnectionPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata used for the Azure portal experience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The properties of a server secure connection policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerConnectionPolicyProperties>,
}
impl ServerConnectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server secure connection policy."]
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
    pub enum ConnectionType {
        Default,
        Proxy,
        Redirect,
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
#[doc = "A server DNS alias acquisition request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDnsAliasAcquisition {
    #[doc = "The id of the server alias that will be acquired to point to this server instead."]
    #[serde(rename = "oldServerDnsAliasId", default, skip_serializing_if = "Option::is_none")]
    pub old_server_dns_alias_id: Option<String>,
}
impl ServerDnsAliasAcquisition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of server DNS aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDnsAliasListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerDnsAlias>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerDnsAliasListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerKey>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerKeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "The URI of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Thumbprint of the server key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The server key creation date."]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
}
impl ServerKeyProperties {
    pub fn new(server_key_type: server_key_properties::ServerKeyType) -> Self {
        Self {
            subregion: None,
            server_key_type,
            uri: None,
            thumbprint: None,
            creation_date: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServerListResult {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<ServerPrivateEndpointConnection>,
    #[doc = "Minimal TLS version. Allowed values: '1.0', '1.1', '1.2'"]
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<String>,
    #[doc = "Whether or not public endpoint access is allowed for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<server_properties::PublicNetworkAccess>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_properties {
    use super::*;
    #[doc = "Whether or not public endpoint access is allowed for this server.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
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
    #[doc = "Properties of a security alert policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
}
impl ServerSecurityAlertPolicy {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerTrustGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerTrustGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "Represents server metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUsage {
    #[doc = "Name of the server usage metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The metric display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The current limit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The units of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The next reset time for the metric (ISO8601 format)."]
    #[serde(rename = "nextResetTime", with = "azure_core::date::rfc3339::option")]
    pub next_reset_time: Option<time::OffsetDateTime>,
}
impl ServerUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list server metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerUsageListResult {
    #[doc = "The list of server metrics for the server."]
    pub value: Vec<ServerUsage>,
}
impl azure_core::Continuable for ServerUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServerUsageListResult {
    pub fn new(value: Vec<ServerUsage>) -> Self {
        Self { value }
    }
}
#[doc = "The server capability"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVersionCapability {
    #[doc = "The server version name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of supported database editions."]
    #[serde(rename = "supportedEditions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_editions: Vec<EditionCapability>,
    #[doc = "The list of supported elastic pool editions."]
    #[serde(rename = "supportedElasticPoolEditions", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerVulnerabilityAssessment>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerVulnerabilityAssessmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "A shared access signature (SAS Key) that has read and write access to the blob container specified in 'storageContainerPath' parameter. If 'storageAccountAccessKey' isn't specified, StorageContainerSasKey is required."]
    #[serde(rename = "storageContainerSasKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_container_sas_key: Option<String>,
    #[doc = "Specifies the identifier key of the storage account for vulnerability assessment scan results. If 'StorageContainerSasKey' isn't specified, storageAccountAccessKey is required."]
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
#[doc = "Represents a database service objective."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceObjective {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Represents the properties of a database service objective."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceObjectiveProperties>,
}
impl ServiceObjective {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "supportedMaxSizes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_max_sizes: Vec<MaxSizeRangeCapability>,
    #[doc = "The performance level capability."]
    #[serde(rename = "performanceLevel", default, skip_serializing_if = "Option::is_none")]
    pub performance_level: Option<PerformanceLevelCapability>,
    #[doc = "An ARM Resource SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "List of supported license types."]
    #[serde(rename = "supportedLicenseTypes", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "supportedMinCapacities", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_min_capacities: Vec<MinCapacityCapability>,
    #[doc = "The compute model"]
    #[serde(rename = "computeModel", default, skip_serializing_if = "Option::is_none")]
    pub compute_model: Option<String>,
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
#[doc = "Represents the response to a get database service objectives request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceObjectiveListResult {
    #[doc = "The list of database service objectives."]
    pub value: Vec<ServiceObjective>,
}
impl azure_core::Continuable for ServiceObjectiveListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServiceObjectiveListResult {
    pub fn new(value: Vec<ServiceObjective>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the properties of a database service objective."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceObjectiveProperties {
    #[doc = "The name for the service objective."]
    #[serde(rename = "serviceObjectiveName", default, skip_serializing_if = "Option::is_none")]
    pub service_objective_name: Option<String>,
    #[doc = "Gets whether the service level objective is the default service objective."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Gets whether the service level objective is a system service objective."]
    #[serde(rename = "isSystem", default, skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
    #[doc = "The description for the service level objective."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets whether the service level objective is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ServiceObjectiveProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Service Tier Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTierAdvisor {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Represents the properties of a Service Tier Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceTierAdvisorProperties>,
}
impl ServiceTierAdvisor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list service tier advisor request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceTierAdvisorListResult {
    #[doc = "The list of service tier advisors for specified database."]
    pub value: Vec<ServiceTierAdvisor>,
}
impl azure_core::Continuable for ServiceTierAdvisorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServiceTierAdvisorListResult {
    pub fn new(value: Vec<ServiceTierAdvisor>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the properties of a Service Tier Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTierAdvisorProperties {
    #[doc = "The observation period start (ISO8601 format)."]
    #[serde(rename = "observationPeriodStart", with = "azure_core::date::rfc3339::option")]
    pub observation_period_start: Option<time::OffsetDateTime>,
    #[doc = "The observation period start (ISO8601 format)."]
    #[serde(rename = "observationPeriodEnd", with = "azure_core::date::rfc3339::option")]
    pub observation_period_end: Option<time::OffsetDateTime>,
    #[doc = "The activeTimeRatio for service tier advisor."]
    #[serde(rename = "activeTimeRatio", default, skip_serializing_if = "Option::is_none")]
    pub active_time_ratio: Option<f64>,
    #[doc = "Gets or sets minDtu for service tier advisor."]
    #[serde(rename = "minDtu", default, skip_serializing_if = "Option::is_none")]
    pub min_dtu: Option<f64>,
    #[doc = "Gets or sets avgDtu for service tier advisor."]
    #[serde(rename = "avgDtu", default, skip_serializing_if = "Option::is_none")]
    pub avg_dtu: Option<f64>,
    #[doc = "Gets or sets maxDtu for service tier advisor."]
    #[serde(rename = "maxDtu", default, skip_serializing_if = "Option::is_none")]
    pub max_dtu: Option<f64>,
    #[doc = "Gets or sets maxSizeInGB for service tier advisor."]
    #[serde(rename = "maxSizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_gb: Option<f64>,
    #[doc = "Gets or sets serviceLevelObjectiveUsageMetrics for the service tier advisor."]
    #[serde(rename = "serviceLevelObjectiveUsageMetrics", default, skip_serializing_if = "Vec::is_empty")]
    pub service_level_objective_usage_metrics: Vec<SloUsageMetric>,
    #[doc = "Gets or sets currentServiceLevelObjective for service tier advisor."]
    #[serde(rename = "currentServiceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub current_service_level_objective: Option<String>,
    #[doc = "Gets or sets currentServiceLevelObjectiveId for service tier advisor."]
    #[serde(rename = "currentServiceLevelObjectiveId", default, skip_serializing_if = "Option::is_none")]
    pub current_service_level_objective_id: Option<String>,
    #[doc = "Gets or sets usageBasedRecommendationServiceLevelObjective for service tier advisor."]
    #[serde(
        rename = "usageBasedRecommendationServiceLevelObjective",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub usage_based_recommendation_service_level_objective: Option<String>,
    #[doc = "Gets or sets usageBasedRecommendationServiceLevelObjectiveId for service tier advisor."]
    #[serde(
        rename = "usageBasedRecommendationServiceLevelObjectiveId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub usage_based_recommendation_service_level_objective_id: Option<String>,
    #[doc = "Gets or sets databaseSizeBasedRecommendationServiceLevelObjective for service tier advisor."]
    #[serde(
        rename = "databaseSizeBasedRecommendationServiceLevelObjective",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub database_size_based_recommendation_service_level_objective: Option<String>,
    #[doc = "Gets or sets databaseSizeBasedRecommendationServiceLevelObjectiveId for service tier advisor."]
    #[serde(
        rename = "databaseSizeBasedRecommendationServiceLevelObjectiveId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub database_size_based_recommendation_service_level_objective_id: Option<String>,
    #[doc = "Gets or sets disasterPlanBasedRecommendationServiceLevelObjective for service tier advisor."]
    #[serde(
        rename = "disasterPlanBasedRecommendationServiceLevelObjective",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disaster_plan_based_recommendation_service_level_objective: Option<String>,
    #[doc = "Gets or sets disasterPlanBasedRecommendationServiceLevelObjectiveId for service tier advisor."]
    #[serde(
        rename = "disasterPlanBasedRecommendationServiceLevelObjectiveId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disaster_plan_based_recommendation_service_level_objective_id: Option<String>,
    #[doc = "Gets or sets overallRecommendationServiceLevelObjective for service tier advisor."]
    #[serde(
        rename = "overallRecommendationServiceLevelObjective",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_recommendation_service_level_objective: Option<String>,
    #[doc = "Gets or sets overallRecommendationServiceLevelObjectiveId for service tier advisor."]
    #[serde(
        rename = "overallRecommendationServiceLevelObjectiveId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_recommendation_service_level_objective_id: Option<String>,
    #[doc = "Gets or sets confidence for service tier advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}
impl ServiceTierAdvisorProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A Slo Usage Metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SloUsageMetric {
    #[doc = "The serviceLevelObjective for SLO usage metric."]
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<slo_usage_metric::ServiceLevelObjective>,
    #[doc = "The serviceLevelObjectiveId for SLO usage metric."]
    #[serde(rename = "serviceLevelObjectiveId", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective_id: Option<String>,
    #[doc = "Gets or sets inRangeTimeRatio for SLO usage metric."]
    #[serde(rename = "inRangeTimeRatio", default, skip_serializing_if = "Option::is_none")]
    pub in_range_time_ratio: Option<f64>,
}
impl SloUsageMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod slo_usage_metric {
    use super::*;
    #[doc = "The serviceLevelObjective for SLO usage metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceLevelObjective")]
    pub enum ServiceLevelObjective {
        System,
        System0,
        System1,
        System2,
        System3,
        System4,
        System2L,
        System3L,
        System4L,
        Free,
        Basic,
        S0,
        S1,
        S2,
        S3,
        S4,
        S6,
        S7,
        S9,
        S12,
        P1,
        P2,
        P3,
        P4,
        P6,
        P11,
        P15,
        #[serde(rename = "PRS1")]
        Prs1,
        #[serde(rename = "PRS2")]
        Prs2,
        #[serde(rename = "PRS4")]
        Prs4,
        #[serde(rename = "PRS6")]
        Prs6,
        #[serde(rename = "DW100")]
        Dw100,
        #[serde(rename = "DW200")]
        Dw200,
        #[serde(rename = "DW300")]
        Dw300,
        #[serde(rename = "DW400")]
        Dw400,
        #[serde(rename = "DW500")]
        Dw500,
        #[serde(rename = "DW600")]
        Dw600,
        #[serde(rename = "DW1000")]
        Dw1000,
        #[serde(rename = "DW1200")]
        Dw1200,
        #[serde(rename = "DW1000c")]
        Dw1000c,
        #[serde(rename = "DW1500")]
        Dw1500,
        #[serde(rename = "DW1500c")]
        Dw1500c,
        #[serde(rename = "DW2000")]
        Dw2000,
        #[serde(rename = "DW2000c")]
        Dw2000c,
        #[serde(rename = "DW3000")]
        Dw3000,
        #[serde(rename = "DW2500c")]
        Dw2500c,
        #[serde(rename = "DW3000c")]
        Dw3000c,
        #[serde(rename = "DW6000")]
        Dw6000,
        #[serde(rename = "DW5000c")]
        Dw5000c,
        #[serde(rename = "DW6000c")]
        Dw6000c,
        #[serde(rename = "DW7500c")]
        Dw7500c,
        #[serde(rename = "DW10000c")]
        Dw10000c,
        #[serde(rename = "DW15000c")]
        Dw15000c,
        #[serde(rename = "DW30000c")]
        Dw30000c,
        #[serde(rename = "DS100")]
        Ds100,
        #[serde(rename = "DS200")]
        Ds200,
        #[serde(rename = "DS300")]
        Ds300,
        #[serde(rename = "DS400")]
        Ds400,
        #[serde(rename = "DS500")]
        Ds500,
        #[serde(rename = "DS600")]
        Ds600,
        #[serde(rename = "DS1000")]
        Ds1000,
        #[serde(rename = "DS1200")]
        Ds1200,
        #[serde(rename = "DS1500")]
        Ds1500,
        #[serde(rename = "DS2000")]
        Ds2000,
        ElasticPool,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceLevelObjective {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceLevelObjective {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceLevelObjective {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::System => serializer.serialize_unit_variant("ServiceLevelObjective", 0u32, "System"),
                Self::System0 => serializer.serialize_unit_variant("ServiceLevelObjective", 1u32, "System0"),
                Self::System1 => serializer.serialize_unit_variant("ServiceLevelObjective", 2u32, "System1"),
                Self::System2 => serializer.serialize_unit_variant("ServiceLevelObjective", 3u32, "System2"),
                Self::System3 => serializer.serialize_unit_variant("ServiceLevelObjective", 4u32, "System3"),
                Self::System4 => serializer.serialize_unit_variant("ServiceLevelObjective", 5u32, "System4"),
                Self::System2L => serializer.serialize_unit_variant("ServiceLevelObjective", 6u32, "System2L"),
                Self::System3L => serializer.serialize_unit_variant("ServiceLevelObjective", 7u32, "System3L"),
                Self::System4L => serializer.serialize_unit_variant("ServiceLevelObjective", 8u32, "System4L"),
                Self::Free => serializer.serialize_unit_variant("ServiceLevelObjective", 9u32, "Free"),
                Self::Basic => serializer.serialize_unit_variant("ServiceLevelObjective", 10u32, "Basic"),
                Self::S0 => serializer.serialize_unit_variant("ServiceLevelObjective", 11u32, "S0"),
                Self::S1 => serializer.serialize_unit_variant("ServiceLevelObjective", 12u32, "S1"),
                Self::S2 => serializer.serialize_unit_variant("ServiceLevelObjective", 13u32, "S2"),
                Self::S3 => serializer.serialize_unit_variant("ServiceLevelObjective", 14u32, "S3"),
                Self::S4 => serializer.serialize_unit_variant("ServiceLevelObjective", 15u32, "S4"),
                Self::S6 => serializer.serialize_unit_variant("ServiceLevelObjective", 16u32, "S6"),
                Self::S7 => serializer.serialize_unit_variant("ServiceLevelObjective", 17u32, "S7"),
                Self::S9 => serializer.serialize_unit_variant("ServiceLevelObjective", 18u32, "S9"),
                Self::S12 => serializer.serialize_unit_variant("ServiceLevelObjective", 19u32, "S12"),
                Self::P1 => serializer.serialize_unit_variant("ServiceLevelObjective", 20u32, "P1"),
                Self::P2 => serializer.serialize_unit_variant("ServiceLevelObjective", 21u32, "P2"),
                Self::P3 => serializer.serialize_unit_variant("ServiceLevelObjective", 22u32, "P3"),
                Self::P4 => serializer.serialize_unit_variant("ServiceLevelObjective", 23u32, "P4"),
                Self::P6 => serializer.serialize_unit_variant("ServiceLevelObjective", 24u32, "P6"),
                Self::P11 => serializer.serialize_unit_variant("ServiceLevelObjective", 25u32, "P11"),
                Self::P15 => serializer.serialize_unit_variant("ServiceLevelObjective", 26u32, "P15"),
                Self::Prs1 => serializer.serialize_unit_variant("ServiceLevelObjective", 27u32, "PRS1"),
                Self::Prs2 => serializer.serialize_unit_variant("ServiceLevelObjective", 28u32, "PRS2"),
                Self::Prs4 => serializer.serialize_unit_variant("ServiceLevelObjective", 29u32, "PRS4"),
                Self::Prs6 => serializer.serialize_unit_variant("ServiceLevelObjective", 30u32, "PRS6"),
                Self::Dw100 => serializer.serialize_unit_variant("ServiceLevelObjective", 31u32, "DW100"),
                Self::Dw200 => serializer.serialize_unit_variant("ServiceLevelObjective", 32u32, "DW200"),
                Self::Dw300 => serializer.serialize_unit_variant("ServiceLevelObjective", 33u32, "DW300"),
                Self::Dw400 => serializer.serialize_unit_variant("ServiceLevelObjective", 34u32, "DW400"),
                Self::Dw500 => serializer.serialize_unit_variant("ServiceLevelObjective", 35u32, "DW500"),
                Self::Dw600 => serializer.serialize_unit_variant("ServiceLevelObjective", 36u32, "DW600"),
                Self::Dw1000 => serializer.serialize_unit_variant("ServiceLevelObjective", 37u32, "DW1000"),
                Self::Dw1200 => serializer.serialize_unit_variant("ServiceLevelObjective", 38u32, "DW1200"),
                Self::Dw1000c => serializer.serialize_unit_variant("ServiceLevelObjective", 39u32, "DW1000c"),
                Self::Dw1500 => serializer.serialize_unit_variant("ServiceLevelObjective", 40u32, "DW1500"),
                Self::Dw1500c => serializer.serialize_unit_variant("ServiceLevelObjective", 41u32, "DW1500c"),
                Self::Dw2000 => serializer.serialize_unit_variant("ServiceLevelObjective", 42u32, "DW2000"),
                Self::Dw2000c => serializer.serialize_unit_variant("ServiceLevelObjective", 43u32, "DW2000c"),
                Self::Dw3000 => serializer.serialize_unit_variant("ServiceLevelObjective", 44u32, "DW3000"),
                Self::Dw2500c => serializer.serialize_unit_variant("ServiceLevelObjective", 45u32, "DW2500c"),
                Self::Dw3000c => serializer.serialize_unit_variant("ServiceLevelObjective", 46u32, "DW3000c"),
                Self::Dw6000 => serializer.serialize_unit_variant("ServiceLevelObjective", 47u32, "DW6000"),
                Self::Dw5000c => serializer.serialize_unit_variant("ServiceLevelObjective", 48u32, "DW5000c"),
                Self::Dw6000c => serializer.serialize_unit_variant("ServiceLevelObjective", 49u32, "DW6000c"),
                Self::Dw7500c => serializer.serialize_unit_variant("ServiceLevelObjective", 50u32, "DW7500c"),
                Self::Dw10000c => serializer.serialize_unit_variant("ServiceLevelObjective", 51u32, "DW10000c"),
                Self::Dw15000c => serializer.serialize_unit_variant("ServiceLevelObjective", 52u32, "DW15000c"),
                Self::Dw30000c => serializer.serialize_unit_variant("ServiceLevelObjective", 53u32, "DW30000c"),
                Self::Ds100 => serializer.serialize_unit_variant("ServiceLevelObjective", 54u32, "DS100"),
                Self::Ds200 => serializer.serialize_unit_variant("ServiceLevelObjective", 55u32, "DS200"),
                Self::Ds300 => serializer.serialize_unit_variant("ServiceLevelObjective", 56u32, "DS300"),
                Self::Ds400 => serializer.serialize_unit_variant("ServiceLevelObjective", 57u32, "DS400"),
                Self::Ds500 => serializer.serialize_unit_variant("ServiceLevelObjective", 58u32, "DS500"),
                Self::Ds600 => serializer.serialize_unit_variant("ServiceLevelObjective", 59u32, "DS600"),
                Self::Ds1000 => serializer.serialize_unit_variant("ServiceLevelObjective", 60u32, "DS1000"),
                Self::Ds1200 => serializer.serialize_unit_variant("ServiceLevelObjective", 61u32, "DS1200"),
                Self::Ds1500 => serializer.serialize_unit_variant("ServiceLevelObjective", 62u32, "DS1500"),
                Self::Ds2000 => serializer.serialize_unit_variant("ServiceLevelObjective", 63u32, "DS2000"),
                Self::ElasticPool => serializer.serialize_unit_variant("ServiceLevelObjective", 64u32, "ElasticPool"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionUsage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubscriptionUsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncAgentLinkedDatabase>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncAgentLinkedDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncAgent>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncAgentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "lastAliveTime", with = "azure_core::date::rfc3339::option")]
    pub last_alive_time: Option<time::OffsetDateTime>,
    #[doc = "State of the sync agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sync_agent_properties::State>,
    #[doc = "If the sync agent version is up to date."]
    #[serde(rename = "isUpToDate", default, skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<bool>,
    #[doc = "Expiration time of the sync agent version."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncDatabaseIdProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncDatabaseIdListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<SyncFullSchemaTable>,
    #[doc = "Last update time of the database schema."]
    #[serde(rename = "lastUpdateTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncFullSchemaProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncFullSchemaPropertiesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroupLogProperties>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncGroupLogListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "lastSyncTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncMember>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SyncMemberListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TopQueries>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TopQueriesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "Represents a database transparent data encryption configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryption {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Represents the properties of a database transparent data encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransparentDataEncryptionProperties>,
}
impl TransparentDataEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a database transparent data encryption Scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryptionActivity {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Represents the properties of a database transparent data encryption Scan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransparentDataEncryptionActivityProperties>,
}
impl TransparentDataEncryptionActivity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the response to a list database transparent data encryption activity request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransparentDataEncryptionActivityListResult {
    #[doc = "The list of database transparent data encryption activities."]
    pub value: Vec<TransparentDataEncryptionActivity>,
}
impl azure_core::Continuable for TransparentDataEncryptionActivityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl TransparentDataEncryptionActivityListResult {
    pub fn new(value: Vec<TransparentDataEncryptionActivity>) -> Self {
        Self { value }
    }
}
#[doc = "Represents the properties of a database transparent data encryption Scan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryptionActivityProperties {
    #[doc = "The status of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<transparent_data_encryption_activity_properties::Status>,
    #[doc = "The percent complete of the transparent data encryption scan for a database."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f32>,
}
impl TransparentDataEncryptionActivityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transparent_data_encryption_activity_properties {
    use super::*;
    #[doc = "The status of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Encrypting,
        Decrypting,
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
                Self::Encrypting => serializer.serialize_unit_variant("Status", 0u32, "Encrypting"),
                Self::Decrypting => serializer.serialize_unit_variant("Status", 1u32, "Decrypting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the properties of a database transparent data encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransparentDataEncryptionProperties {
    #[doc = "The status of the database transparent data encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<transparent_data_encryption_properties::Status>,
}
impl TransparentDataEncryptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transparent_data_encryption_properties {
    use super::*;
    #[doc = "The status of the database transparent data encryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[doc = "Represents the parameters for Unlink Replication Link request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnlinkParameters {
    #[doc = "Determines whether link will be terminated in a forced or a friendly way."]
    #[serde(rename = "forcedTermination", default, skip_serializing_if = "Option::is_none")]
    pub forced_termination: Option<bool>,
}
impl UnlinkParameters {
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
pub struct UpsertManagedServerOperationStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upsert_managed_server_operation_step::Status>,
}
impl UpsertManagedServerOperationStep {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upsert_managed_server_operation_step {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        InProgress,
        SlowedDown,
        Completed,
        Failed,
        Canceled,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A list of virtual clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualClusterListResult {
    #[doc = "Array of results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualCluster>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "List of resources in this virtual cluster."]
    #[serde(rename = "childResources", default, skip_serializing_if = "Vec::is_empty")]
    pub child_resources: Vec<String>,
}
impl VirtualClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An update request for an Azure SQL Database virtual cluster."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkRule>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
                Self::Deleting => serializer.serialize_unit_variant("State", 3u32, "Deleting"),
                Self::Unknown => serializer.serialize_unit_variant("State", 4u32, "Unknown"),
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VulnerabilityAssessmentScanRecord>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VulnerabilityAssessmentScanRecordListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The scan end time (UTC)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The scan errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadClassifier>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadClassifierListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkloadGroup>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkloadGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
