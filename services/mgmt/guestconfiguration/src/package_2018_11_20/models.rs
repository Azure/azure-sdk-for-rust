#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Information about the guest configuration assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentInfo {
    #[doc = "Name of the guest configuration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Information about the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationInfo>,
}
impl AssignmentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the guest configuration assignment report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentReportDetails {
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<assignment_report_details::ComplianceStatus>,
    #[doc = "Start date and time of the guest configuration assignment compliance status check."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End date and time of the guest configuration assignment compliance status check."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "GUID of the report."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Type of report, Consistency or Initial"]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<assignment_report_details::OperationType>,
    #[doc = "The list of resources for which guest configuration assignment compliance is checked."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<AssignmentReportResource>,
}
impl AssignmentReportDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assignment_report_details {
    use super::*;
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceStatus")]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Compliant => serializer.serialize_unit_variant("ComplianceStatus", 0u32, "Compliant"),
                Self::NonCompliant => serializer.serialize_unit_variant("ComplianceStatus", 1u32, "NonCompliant"),
                Self::Pending => serializer.serialize_unit_variant("ComplianceStatus", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of report, Consistency or Initial"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        Consistency,
        Initial,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Consistency => serializer.serialize_unit_variant("OperationType", 0u32, "Consistency"),
                Self::Initial => serializer.serialize_unit_variant("OperationType", 1u32, "Initial"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The guest configuration assignment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentReportResource {
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<assignment_report_resource::ComplianceStatus>,
    #[doc = "Compliance reason and reason code for a resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<AssignmentReportResourceComplianceReason>,
    #[doc = "Properties of a guest configuration assignment resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AssignmentReportResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assignment_report_resource {
    use super::*;
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceStatus")]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Compliant => serializer.serialize_unit_variant("ComplianceStatus", 0u32, "Compliant"),
                Self::NonCompliant => serializer.serialize_unit_variant("ComplianceStatus", 1u32, "NonCompliant"),
                Self::Pending => serializer.serialize_unit_variant("ComplianceStatus", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Reason and code for the compliance of the guest configuration assignment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentReportResourceComplianceReason {
    #[doc = "Reason for the compliance of the guest configuration assignment resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    #[doc = "Code for the compliance of the guest configuration assignment resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl AssignmentReportResourceComplianceReason {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationInfo {
    #[doc = "Name of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a configuration parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationParameter {
    #[doc = "Name of the configuration parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the configuration parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ConfigurationParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration setting of LCM (Local Configuration Manager)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationSetting {
    #[doc = "Specifies how the LCM(Local Configuration Manager) actually applies the configuration to the target nodes. Possible values are ApplyOnly, ApplyAndMonitor, and ApplyAndAutoCorrect."]
    #[serde(rename = "configurationMode", default, skip_serializing_if = "Option::is_none")]
    pub configuration_mode: Option<configuration_setting::ConfigurationMode>,
    #[doc = "If true - new configurations downloaded from the pull service are allowed to overwrite the old ones on the target node. Otherwise, false"]
    #[serde(rename = "allowModuleOverwrite", default, skip_serializing_if = "Option::is_none")]
    pub allow_module_overwrite: Option<bool>,
    #[doc = "Specifies what happens after a reboot during the application of a configuration. The possible values are ContinueConfiguration and StopConfiguration"]
    #[serde(rename = "actionAfterReboot", default, skip_serializing_if = "Option::is_none")]
    pub action_after_reboot: Option<configuration_setting::ActionAfterReboot>,
    #[doc = "The time interval, in minutes, at which the LCM checks a pull service to get updated configurations. This value is ignored if the LCM is not configured in pull mode. The default value is 30."]
    #[serde(rename = "refreshFrequencyMins", default, skip_serializing_if = "Option::is_none")]
    pub refresh_frequency_mins: Option<f64>,
    #[doc = "Set this to true to automatically reboot the node after a configuration that requires reboot is applied. Otherwise, you will have to manually reboot the node for any configuration that requires it. The default value is false. To use this setting when a reboot condition is enacted by something other than DSC (such as Windows Installer), combine this setting with the xPendingReboot module."]
    #[serde(rename = "rebootIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub reboot_if_needed: Option<bool>,
    #[doc = "How often, in minutes, the current configuration is checked and applied. This property is ignored if the ConfigurationMode property is set to ApplyOnly. The default value is 15."]
    #[serde(rename = "configurationModeFrequencyMins", default, skip_serializing_if = "Option::is_none")]
    pub configuration_mode_frequency_mins: Option<f64>,
}
impl ConfigurationSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_setting {
    use super::*;
    #[doc = "Specifies how the LCM(Local Configuration Manager) actually applies the configuration to the target nodes. Possible values are ApplyOnly, ApplyAndMonitor, and ApplyAndAutoCorrect."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigurationMode")]
    pub enum ConfigurationMode {
        ApplyOnly,
        ApplyAndMonitor,
        ApplyAndAutoCorrect,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigurationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigurationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigurationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplyOnly => serializer.serialize_unit_variant("ConfigurationMode", 0u32, "ApplyOnly"),
                Self::ApplyAndMonitor => serializer.serialize_unit_variant("ConfigurationMode", 1u32, "ApplyAndMonitor"),
                Self::ApplyAndAutoCorrect => serializer.serialize_unit_variant("ConfigurationMode", 2u32, "ApplyAndAutoCorrect"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies what happens after a reboot during the application of a configuration. The possible values are ContinueConfiguration and StopConfiguration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionAfterReboot")]
    pub enum ActionAfterReboot {
        ContinueConfiguration,
        StopConfiguration,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionAfterReboot {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionAfterReboot {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionAfterReboot {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ContinueConfiguration => serializer.serialize_unit_variant("ActionAfterReboot", 0u32, "ContinueConfiguration"),
                Self::StopConfiguration => serializer.serialize_unit_variant("ActionAfterReboot", 1u32, "StopConfiguration"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error response of an operation failure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
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
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Detail error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Guest configuration assignment is an association between a machine and guest configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Guest configuration assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestConfigurationAssignmentProperties>,
}
impl GuestConfigurationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the list guest configuration assignment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentList {
    #[doc = "Result of the list guest configuration assignment operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestConfigurationAssignment>,
}
impl azure_core::Continuable for GuestConfigurationAssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GuestConfigurationAssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Guest configuration assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentProperties {
    #[doc = "Guest configuration is an artifact that encapsulates DSC configuration and its dependencies. The artifact is a zip file containing DSC configuration (as MOF) and dependent resources and other dependencies like modules."]
    #[serde(rename = "guestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub guest_configuration: Option<GuestConfigurationNavigation>,
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<guest_configuration_assignment_properties::ComplianceStatus>,
    #[doc = "Date and time when last compliance status was checked."]
    #[serde(rename = "lastComplianceStatusChecked", default, with = "azure_core::date::rfc3339::option")]
    pub last_compliance_status_checked: Option<time::OffsetDateTime>,
    #[doc = "Id of the latest report for the guest configuration assignment. "]
    #[serde(rename = "latestReportId", default, skip_serializing_if = "Option::is_none")]
    pub latest_report_id: Option<String>,
    #[doc = "The list of VM Compliance data for VMSS"]
    #[serde(rename = "vmssVMList", default, skip_serializing_if = "Vec::is_empty")]
    pub vmss_vm_list: Vec<VmssvmInfo>,
    #[doc = "Type of the resource - VMSS / VM"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "parameter hash for the guest configuration assignment. "]
    #[serde(rename = "parameterHash", default, skip_serializing_if = "Option::is_none")]
    pub parameter_hash: Option<String>,
    #[doc = "The source which initiated the guest configuration assignment. Ex: Azure Policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Combined hash of the configuration package and parameters."]
    #[serde(rename = "assignmentHash", default, skip_serializing_if = "Option::is_none")]
    pub assignment_hash: Option<String>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<guest_configuration_assignment_properties::ProvisioningState>,
}
impl GuestConfigurationAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_configuration_assignment_properties {
    use super::*;
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceStatus")]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Compliant => serializer.serialize_unit_variant("ComplianceStatus", 0u32, "Compliant"),
                Self::NonCompliant => serializer.serialize_unit_variant("ComplianceStatus", 1u32, "NonCompliant"),
                Self::Pending => serializer.serialize_unit_variant("ComplianceStatus", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state, which only appears in the response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
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
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Created"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Report for the guest configuration assignment. Report contains information such as compliance status, reason, and more."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentReport {
    #[doc = "ARM resource id of the report for the guest configuration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "GUID that identifies the guest configuration assignment report under a subscription, resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Report for the guest configuration assignment. Report contains information such as compliance status, reason, and more."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestConfigurationAssignmentReportProperties>,
}
impl GuestConfigurationAssignmentReport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of guest configuration assignment reports."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentReportList {
    #[doc = "List of reports for the guest configuration. Report contains information such as compliance status, reason and more."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestConfigurationAssignmentReport>,
}
impl GuestConfigurationAssignmentReportList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Report for the guest configuration assignment. Report contains information such as compliance status, reason, and more."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationAssignmentReportProperties {
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<guest_configuration_assignment_report_properties::ComplianceStatus>,
    #[doc = "GUID that identifies the guest configuration assignment report under a subscription, resource group."]
    #[serde(rename = "reportId", default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[doc = "Information about the guest configuration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment: Option<AssignmentInfo>,
    #[doc = "Information about the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<VmInfo>,
    #[doc = "Start date and time of the guest configuration assignment compliance status check."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End date and time of the guest configuration assignment compliance status check."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the guest configuration assignment report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<AssignmentReportDetails>,
    #[doc = "Azure resource Id of the VMSS."]
    #[serde(rename = "vmssResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vmss_resource_id: Option<String>,
}
impl GuestConfigurationAssignmentReportProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_configuration_assignment_report_properties {
    use super::*;
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceStatus")]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Compliant => serializer.serialize_unit_variant("ComplianceStatus", 0u32, "Compliant"),
                Self::NonCompliant => serializer.serialize_unit_variant("ComplianceStatus", 1u32, "NonCompliant"),
                Self::Pending => serializer.serialize_unit_variant("ComplianceStatus", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Guest configuration is an artifact that encapsulates DSC configuration and its dependencies. The artifact is a zip file containing DSC configuration (as MOF) and dependent resources and other dependencies like modules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestConfigurationNavigation {
    #[doc = "Kind of the guest configuration. For example:DSC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<guest_configuration_navigation::Kind>,
    #[doc = "Name of the guest configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of the guest configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Uri of the storage where guest configuration package is uploaded."]
    #[serde(rename = "contentUri", default, skip_serializing_if = "Option::is_none")]
    pub content_uri: Option<String>,
    #[doc = "Specifies the content type of the configuration. Possible values could be Builtin or Custom."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "Combined hash of the guest configuration package and configuration parameters."]
    #[serde(rename = "contentHash", default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[doc = "Specifies the assignment type and execution of the configuration. Possible values are Audit, DeployAndAutoCorrect, ApplyAndAutoCorrect and ApplyAndMonitor."]
    #[serde(rename = "assignmentType", default, skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<guest_configuration_navigation::AssignmentType>,
    #[doc = "The configuration parameters for the guest configuration."]
    #[serde(rename = "configurationParameter", default, skip_serializing_if = "Vec::is_empty")]
    pub configuration_parameter: Vec<ConfigurationParameter>,
    #[doc = "The protected configuration parameters for the guest configuration."]
    #[serde(rename = "configurationProtectedParameter", default, skip_serializing_if = "Vec::is_empty")]
    pub configuration_protected_parameter: Vec<ConfigurationParameter>,
    #[doc = "Configuration setting of LCM (Local Configuration Manager)."]
    #[serde(rename = "configurationSetting", default, skip_serializing_if = "Option::is_none")]
    pub configuration_setting: Option<ConfigurationSetting>,
}
impl GuestConfigurationNavigation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod guest_configuration_navigation {
    use super::*;
    #[doc = "Kind of the guest configuration. For example:DSC"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "DSC")]
        Dsc,
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
                Self::Dsc => serializer.serialize_unit_variant("Kind", 0u32, "DSC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the assignment type and execution of the configuration. Possible values are Audit, DeployAndAutoCorrect, ApplyAndAutoCorrect and ApplyAndMonitor."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssignmentType")]
    pub enum AssignmentType {
        Audit,
        DeployAndAutoCorrect,
        ApplyAndAutoCorrect,
        ApplyAndMonitor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Audit => serializer.serialize_unit_variant("AssignmentType", 0u32, "Audit"),
                Self::DeployAndAutoCorrect => serializer.serialize_unit_variant("AssignmentType", 1u32, "DeployAndAutoCorrect"),
                Self::ApplyAndAutoCorrect => serializer.serialize_unit_variant("AssignmentType", 2u32, "ApplyAndAutoCorrect"),
                Self::ApplyAndMonitor => serializer.serialize_unit_variant("AssignmentType", 3u32, "ApplyAndMonitor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "GuestConfiguration REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: For ex. providers/Microsoft.GuestConfiguration/guestConfigurationAssignments/write or read"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
        #[doc = "Service provider: Microsoft.GuestConfiguration"]
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
        #[doc = "Service provider: Microsoft.GuestConfiguration"]
        #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
        pub status_code: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response model for the list of Automation operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of Automation operations supported by the Automation resource provider."]
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
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "ARM resource id of the guest configuration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the guest configuration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Region where the VM is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmInfo {
    #[doc = "Azure resource Id of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "UUID(Universally Unique Identifier) of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
impl VmInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about VMSS VM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmssvmInfo {
    #[doc = "UUID of the VM."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Azure resource Id of the VM."]
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<vmssvm_info::ComplianceStatus>,
    #[doc = "Id of the latest report for the guest configuration assignment. "]
    #[serde(rename = "latestReportId", default, skip_serializing_if = "Option::is_none")]
    pub latest_report_id: Option<String>,
    #[doc = "Date and time when last compliance status was checked."]
    #[serde(rename = "lastComplianceChecked", default, with = "azure_core::date::rfc3339::option")]
    pub last_compliance_checked: Option<time::OffsetDateTime>,
}
impl VmssvmInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vmssvm_info {
    use super::*;
    #[doc = "A value indicating compliance status of the machine for the assigned guest configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceStatus")]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Compliant => serializer.serialize_unit_variant("ComplianceStatus", 0u32, "Compliant"),
                Self::NonCompliant => serializer.serialize_unit_variant("ComplianceStatus", 1u32, "NonCompliant"),
                Self::Pending => serializer.serialize_unit_variant("ComplianceStatus", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
