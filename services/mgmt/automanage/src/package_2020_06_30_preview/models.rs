#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Definition of the Automanage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the Automanage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AccountIdentity>,
}
impl Account {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
        }
    }
}
#[doc = "Identity for the Automanage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountIdentity {
    #[doc = "The principal id of Automanage account identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the Automanage account."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the Automanage account. Currently, the only supported type is 'SystemAssigned', which implicitly creates an identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<account_identity::Type>,
}
impl AccountIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_identity {
    use super::*;
    #[doc = "The type of identity used for the Automanage account. Currently, the only supported type is 'SystemAssigned', which implicitly creates an identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[doc = "The response of the list Account operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountList {
    #[doc = "Result of the list Account operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
}
impl azure_core::Continuable for AccountList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AccountList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the Automanage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Identity for the Automanage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<AccountIdentity>,
}
impl AccountUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration profile assignment is an association between a VM and automanage profile configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfileAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Automanage configuration profile assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProfileAssignmentProperties>,
}
impl ConfigurationProfileAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The compliance status for the configuration profile assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfileAssignmentCompliance {
    #[doc = "The state of compliance, which only appears in the response."]
    #[serde(rename = "updateStatus", default, skip_serializing_if = "Option::is_none")]
    pub update_status: Option<configuration_profile_assignment_compliance::UpdateStatus>,
}
impl ConfigurationProfileAssignmentCompliance {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_profile_assignment_compliance {
    use super::*;
    #[doc = "The state of compliance, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UpdateStatus")]
    pub enum UpdateStatus {
        Succeeded,
        Failed,
        Created,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("UpdateStatus", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("UpdateStatus", 1u32, "Failed"),
                Self::Created => serializer.serialize_unit_variant("UpdateStatus", 2u32, "Created"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the list configuration profile assignment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfileAssignmentList {
    #[doc = "Result of the list configuration profile assignment operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationProfileAssignment>,
}
impl azure_core::Continuable for ConfigurationProfileAssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ConfigurationProfileAssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Automanage configuration profile assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfileAssignmentProperties {
    #[doc = "A value indicating configuration profile."]
    #[serde(rename = "configurationProfile", default, skip_serializing_if = "Option::is_none")]
    pub configuration_profile: Option<configuration_profile_assignment_properties::ConfigurationProfile>,
    #[doc = "The target VM resource URI"]
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[doc = "The Automanage account ARM Resource URI"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The configuration profile custom preferences ARM resource URI"]
    #[serde(rename = "configurationProfilePreferenceId", default, skip_serializing_if = "Option::is_none")]
    pub configuration_profile_preference_id: Option<String>,
    #[doc = "The state of onboarding, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<configuration_profile_assignment_properties::ProvisioningState>,
    #[doc = "The compliance status for the configuration profile assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compliance: Option<ConfigurationProfileAssignmentCompliance>,
}
impl ConfigurationProfileAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_profile_assignment_properties {
    use super::*;
    #[doc = "A value indicating configuration profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigurationProfile")]
    pub enum ConfigurationProfile {
        #[serde(rename = "Azure virtual machine best practices – Dev/Test")]
        AzureVirtualMachineBestPracticesDevTest,
        #[serde(rename = "Azure virtual machine best practices – Production")]
        AzureVirtualMachineBestPracticesProduction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigurationProfile {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigurationProfile {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigurationProfile {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureVirtualMachineBestPracticesDevTest => {
                    serializer.serialize_unit_variant("ConfigurationProfile", 0u32, "Azure virtual machine best practices – Dev/Test")
                }
                Self::AzureVirtualMachineBestPracticesProduction => {
                    serializer.serialize_unit_variant("ConfigurationProfile", 1u32, "Azure virtual machine best practices – Production")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state of onboarding, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Created,
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
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Created"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the configuration profile preference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationProfilePreference {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Automanage configuration profile preference properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProfilePreferenceProperties>,
}
impl ConfigurationProfilePreference {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Automanage configuration profile Antimalware preferences."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfilePreferenceAntiMalware {
    #[doc = "Enables or disables Real Time Protection"]
    #[serde(rename = "enableRealTimeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_real_time_protection: Option<configuration_profile_preference_anti_malware::EnableRealTimeProtection>,
    #[doc = "Extensions, Paths and Processes that must be excluded from scan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<serde_json::Value>,
    #[doc = "Enables or disables a periodic scan for antimalware"]
    #[serde(rename = "runScheduledScan", default, skip_serializing_if = "Option::is_none")]
    pub run_scheduled_scan: Option<configuration_profile_preference_anti_malware::RunScheduledScan>,
    #[doc = "Type of scheduled scan"]
    #[serde(rename = "scanType", default, skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<configuration_profile_preference_anti_malware::ScanType>,
    #[doc = "Schedule scan settings day"]
    #[serde(rename = "scanDay", default, skip_serializing_if = "Option::is_none")]
    pub scan_day: Option<String>,
    #[doc = "Schedule scan settings time"]
    #[serde(rename = "scanTimeInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub scan_time_in_minutes: Option<String>,
}
impl ConfigurationProfilePreferenceAntiMalware {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_profile_preference_anti_malware {
    use super::*;
    #[doc = "Enables or disables Real Time Protection"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnableRealTimeProtection")]
    pub enum EnableRealTimeProtection {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnableRealTimeProtection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnableRealTimeProtection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnableRealTimeProtection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("EnableRealTimeProtection", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("EnableRealTimeProtection", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enables or disables a periodic scan for antimalware"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunScheduledScan")]
    pub enum RunScheduledScan {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunScheduledScan {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunScheduledScan {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunScheduledScan {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("RunScheduledScan", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("RunScheduledScan", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of scheduled scan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScanType")]
    pub enum ScanType {
        Quick,
        Full,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScanType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScanType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScanType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Quick => serializer.serialize_unit_variant("ScanType", 0u32, "Quick"),
                Self::Full => serializer.serialize_unit_variant("ScanType", 1u32, "Full"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the list ConfigurationProfilePreference operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfilePreferenceList {
    #[doc = "Result of the list ConfigurationProfilePreference operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationProfilePreference>,
}
impl azure_core::Continuable for ConfigurationProfilePreferenceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ConfigurationProfilePreferenceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Automanage configuration profile preference properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfilePreferenceProperties {
    #[doc = "Automanage configuration profile VM Backup preferences."]
    #[serde(rename = "vmBackup", default, skip_serializing_if = "Option::is_none")]
    pub vm_backup: Option<ConfigurationProfilePreferenceVmBackup>,
    #[doc = "Automanage configuration profile Antimalware preferences."]
    #[serde(rename = "antiMalware", default, skip_serializing_if = "Option::is_none")]
    pub anti_malware: Option<ConfigurationProfilePreferenceAntiMalware>,
}
impl ConfigurationProfilePreferenceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the configuration profile preference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfilePreferenceUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Automanage configuration profile preference properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProfilePreferenceProperties>,
}
impl ConfigurationProfilePreferenceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Automanage configuration profile VM Backup preferences."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProfilePreferenceVmBackup {
    #[doc = "TimeZone optional input as string. For example: Pacific Standard Time"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Instant RP retention policy range in days"]
    #[serde(rename = "instantRpRetentionRangeInDays", default, skip_serializing_if = "Option::is_none")]
    pub instant_rp_retention_range_in_days: Option<i32>,
    #[doc = "Retention policy with the details on backup copy retention ranges."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<String>,
    #[doc = "Backup schedule specified as part of backup policy."]
    #[serde(rename = "schedulePolicy", default, skip_serializing_if = "Option::is_none")]
    pub schedule_policy: Option<String>,
}
impl ConfigurationProfilePreferenceVmBackup {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Automanage REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: For ex. providers/Microsoft.Automanage/configurationProfileAssignments/write or read"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Provider, Resource, Operation and description values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Provider, Resource, Operation and description values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<operation::Properties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Provider, Resource, Operation and description values."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Automanage"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed:  For ex. "]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description about operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Provider, Resource, Operation and description values."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Service provider: Microsoft.Automanage"]
        #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
        pub status_code: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response model for the list of Automanage operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of Automanage operations supported by the Automanage resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
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
#[doc = "Represents an update resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateResource {
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
