#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Configurable properties that the user can set locally via the azcmagent config command, or remotely via ARM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentConfiguration {
    #[doc = "Specifies the URL of the proxy to be used."]
    #[serde(rename = "proxyUrl", default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[doc = "Specifies the list of ports that the agent will be able to listen on."]
    #[serde(
        rename = "incomingConnectionsPorts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub incoming_connections_ports: Vec<String>,
    #[doc = "Array of extensions that are allowed to be installed or updated."]
    #[serde(
        rename = "extensionsAllowList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions_allow_list: Vec<ConfigurationExtension>,
    #[doc = "Array of extensions that are blocked (cannot be installed or updated)"]
    #[serde(
        rename = "extensionsBlockList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions_block_list: Vec<ConfigurationExtension>,
    #[doc = "List of service names which should not use the specified proxy server."]
    #[serde(
        rename = "proxyBypass",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub proxy_bypass: Vec<String>,
    #[doc = "Specifies whether the extension service is enabled or disabled."]
    #[serde(rename = "extensionsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub extensions_enabled: Option<String>,
    #[doc = "Specified whether the guest configuration service is enabled or disabled."]
    #[serde(rename = "guestConfigurationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub guest_configuration_enabled: Option<String>,
    #[doc = "Name of configuration mode to use. Modes are pre-defined configurations of security controls, extension allowlists and guest configuration, maintained by Microsoft."]
    #[serde(rename = "configMode", default, skip_serializing_if = "Option::is_none")]
    pub config_mode: Option<agent_configuration::ConfigMode>,
}
impl AgentConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_configuration {
    use super::*;
    #[doc = "Name of configuration mode to use. Modes are pre-defined configurations of security controls, extension allowlists and guest configuration, maintained by Microsoft."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfigMode")]
    pub enum ConfigMode {
        #[serde(rename = "full")]
        Full,
        #[serde(rename = "monitor")]
        Monitor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfigMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfigMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfigMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Full => serializer.serialize_unit_variant("ConfigMode", 0u32, "full"),
                Self::Monitor => serializer.serialize_unit_variant("ConfigMode", 1u32, "monitor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The info w.r.t Agent Upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentUpgrade {
    #[doc = "Specifies the version info w.r.t AgentUpgrade for the machine."]
    #[serde(rename = "desiredVersion", default, skip_serializing_if = "Option::is_none")]
    pub desired_version: Option<String>,
    #[doc = "The correlation ID passed in from RSM per upgrade."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Specifies if RSM should try to upgrade this machine"]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Specifies the version of the last attempt"]
    #[serde(rename = "lastAttemptDesiredVersion", default, skip_serializing_if = "Option::is_none")]
    pub last_attempt_desired_version: Option<String>,
    #[doc = "Timestamp of last upgrade attempt"]
    #[serde(rename = "lastAttemptTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_attempt_timestamp: Option<String>,
    #[doc = "Specifies the status of Agent Upgrade."]
    #[serde(rename = "lastAttemptStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_attempt_status: Option<agent_upgrade::LastAttemptStatus>,
    #[doc = "Failure message of last upgrade attempt if any."]
    #[serde(rename = "lastAttemptMessage", default, skip_serializing_if = "Option::is_none")]
    pub last_attempt_message: Option<String>,
}
impl AgentUpgrade {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agent_upgrade {
    use super::*;
    #[doc = "Specifies the status of Agent Upgrade."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastAttemptStatus")]
    pub enum LastAttemptStatus {
        Success,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastAttemptStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastAttemptStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastAttemptStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Success => serializer.serialize_unit_variant("LastAttemptStatus", 0u32, "Success"),
                Self::Failed => serializer.serialize_unit_variant("LastAttemptStatus", 1u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes properties of Agent Version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentVersion {
    #[doc = "Represents the agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Represents the download link of specific agent version."]
    #[serde(rename = "downloadLink", default, skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,
    #[doc = "Defines the os type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}
impl AgentVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes AgentVersions List."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentVersionsList {
    #[doc = "The list of available Agent Versions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AgentVersion>,
    #[doc = "The URI to fetch the next 10 available Agent Versions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AgentVersionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates which kind of Arc machine placement on-premises, such as HCI, SCVMM or VMware etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArcKindEnum")]
pub enum ArcKindEnum {
    #[serde(rename = "AVS")]
    Avs,
    #[serde(rename = "HCI")]
    Hci,
    #[serde(rename = "SCVMM")]
    Scvmm,
    VMware,
    #[serde(rename = "EPS")]
    Eps,
    #[serde(rename = "GCP")]
    Gcp,
    #[serde(rename = "AWS")]
    Aws,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArcKindEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArcKindEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArcKindEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Avs => serializer.serialize_unit_variant("ArcKindEnum", 0u32, "AVS"),
            Self::Hci => serializer.serialize_unit_variant("ArcKindEnum", 1u32, "HCI"),
            Self::Scvmm => serializer.serialize_unit_variant("ArcKindEnum", 2u32, "SCVMM"),
            Self::VMware => serializer.serialize_unit_variant("ArcKindEnum", 3u32, "VMware"),
            Self::Eps => serializer.serialize_unit_variant("ArcKindEnum", 4u32, "EPS"),
            Self::Gcp => serializer.serialize_unit_variant("ArcKindEnum", 5u32, "GCP"),
            Self::Aws => serializer.serialize_unit_variant("ArcKindEnum", 6u32, "AWS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Summarization of patches available for installation on the machine by classification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailablePatchCountByClassification {
    #[doc = "Number of security patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<i32>,
    #[doc = "Number of critical patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<i32>,
    #[doc = "Number of definition patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<i32>,
    #[doc = "Number of update Rollup patches available for installation."]
    #[serde(rename = "updateRollup", default, skip_serializing_if = "Option::is_none")]
    pub update_rollup: Option<i32>,
    #[doc = "Number of feature pack patches available for installation."]
    #[serde(rename = "featurePack", default, skip_serializing_if = "Option::is_none")]
    pub feature_pack: Option<i32>,
    #[doc = "Number of service pack patches available for installation."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<i32>,
    #[doc = "Number of tools patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<i32>,
    #[doc = "Number of updates category patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<i32>,
    #[doc = "Number of other patches available for installation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<i32>,
}
impl AvailablePatchCountByClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metadata of the cloud environment (Azure/GCP/AWS/OCI...)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudMetadata {
    #[doc = "Specifies the cloud provider (Azure/AWS/GCP...)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl CloudMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties that can identify extensions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationExtension {
    #[doc = "Publisher of the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Type of the extension."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ConfigurationExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionDetail {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The private endpoint connection private ip address"]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The private endpoint connection link identifier"]
    #[serde(rename = "linkIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub link_identifier: Option<String>,
    #[doc = "The private endpoint connection group id"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private endpoint connection member name"]
    #[serde(rename = "memberName", default, skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
}
impl ConnectionDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detected properties from the machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetectedProperties {}
impl DetectedProperties {
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
#[doc = "The ESU eligibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EsuEligibility")]
pub enum EsuEligibility {
    Eligible,
    Ineligible,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EsuEligibility {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EsuEligibility {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EsuEligibility {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Eligible => serializer.serialize_unit_variant("EsuEligibility", 0u32, "Eligible"),
            Self::Ineligible => serializer.serialize_unit_variant("EsuEligibility", 1u32, "Ineligible"),
            Self::Unknown => serializer.serialize_unit_variant("EsuEligibility", 2u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ESU key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EsuKey {
    #[doc = "SKU number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The current status of the license profile key."]
    #[serde(rename = "licenseStatus", default, skip_serializing_if = "Option::is_none")]
    pub license_status: Option<String>,
}
impl EsuKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ESU key state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EsuKeyState")]
pub enum EsuKeyState {
    Inactive,
    Active,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EsuKeyState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EsuKeyState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EsuKeyState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inactive => serializer.serialize_unit_variant("EsuKeyState", 0u32, "Inactive"),
            Self::Active => serializer.serialize_unit_variant("EsuKeyState", 1u32, "Active"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the Update properties of a License Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EsuProfileUpdateProperties {
    #[doc = "The resource id of the license."]
    #[serde(rename = "assignedLicense", default, skip_serializing_if = "Option::is_none")]
    pub assigned_license: Option<String>,
}
impl EsuProfileUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server types for Esu."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EsuServerType")]
pub enum EsuServerType {
    Standard,
    Datacenter,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EsuServerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EsuServerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EsuServerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("EsuServerType", 0u32, "Standard"),
            Self::Datacenter => serializer.serialize_unit_variant("EsuServerType", 1u32, "Datacenter"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the Machine Extension Target Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTarget {}
impl ExtensionTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extension Target Version Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionTargetProperties {
    #[doc = "Extension Upgrade Target Version."]
    #[serde(rename = "targetVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_version: Option<TargetVersion>,
}
impl ExtensionTargetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Extension Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionValue {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes Extension Metadata properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtensionValueProperties>,
}
impl ExtensionValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Extension Metadata response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionValueListResult {
    #[doc = "The list of extension metadata"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExtensionValue>,
}
impl azure_core::Continuable for ExtensionValueListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ExtensionValueListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Extension Metadata properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionValueProperties {
    #[doc = "The version of the Extension being received."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The type of the Extension being received."]
    #[serde(rename = "extensionType", default, skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,
    #[doc = "The publisher of the Extension being received."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
}
impl ExtensionValueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Arc PrivateLinkScope definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScope {
    #[serde(flatten)]
    pub private_link_scopes_resource: PrivateLinkScopesResource,
    #[doc = "Properties that define a Azure Arc PrivateLinkScope resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridComputePrivateLinkScopeProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl HybridComputePrivateLinkScope {
    pub fn new(private_link_scopes_resource: PrivateLinkScopesResource) -> Self {
        Self {
            private_link_scopes_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Describes the list of Azure Arc PrivateLinkScope resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScopeListResult {
    #[doc = "List of Azure Arc PrivateLinkScope definitions."]
    pub value: Vec<HybridComputePrivateLinkScope>,
    #[doc = "The URI to get the next set of Azure Arc PrivateLinkScope definitions if too many PrivateLinkScopes where returned in the result set."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HybridComputePrivateLinkScopeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HybridComputePrivateLinkScopeListResult {
    pub fn new(value: Vec<HybridComputePrivateLinkScope>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties that define a Azure Arc PrivateLinkScope resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridComputePrivateLinkScopeProperties {
    #[doc = "The network access policy to determine if Azure Arc agents can use public Azure Arc service endpoints. Defaults to disabled (access to Azure Arc services only via private link)."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "Current state of this PrivateLinkScope: whether or not is has been provisioned within the resource group it is defined. Users cannot change this value but are able to read from it. Values will include Provisioning ,Succeeded, Canceled and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The Guid id of the private link scope."]
    #[serde(rename = "privateLinkScopeId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_id: Option<String>,
    #[doc = "The collection of associated Private Endpoint Connections."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionDataModel>,
}
impl HybridComputePrivateLinkScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the HybridIdentityMetadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridIdentityMetadata {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the resource properties."]
    pub properties: HybridIdentityMetadataProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl HybridIdentityMetadata {
    pub fn new(properties: HybridIdentityMetadataProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of HybridIdentityMetadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridIdentityMetadataList {
    #[doc = "Url to follow for getting next page of HybridIdentityMetadata."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of HybridIdentityMetadata"]
    pub value: Vec<HybridIdentityMetadata>,
}
impl azure_core::Continuable for HybridIdentityMetadataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HybridIdentityMetadataList {
    pub fn new(value: Vec<HybridIdentityMetadata>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridIdentityMetadataProperties {
    #[doc = "The unique identifier for the resource."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The Public Key."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl HybridIdentityMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Describes properties of the IP address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddress {
    #[doc = "Represents the IP Address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "Represents the Ip Address Version."]
    #[serde(rename = "ipAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_version: Option<String>,
    #[doc = "Describes the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
}
impl IpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a license in a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct License {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a License Profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LicenseProperties>,
}
impl License {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Describes the license core type (pCore or vCore)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseCoreType")]
pub enum LicenseCoreType {
    #[serde(rename = "pCore")]
    PCore,
    #[serde(rename = "vCore")]
    VCore,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseCoreType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseCoreType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseCoreType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PCore => serializer.serialize_unit_variant("LicenseCoreType", 0u32, "pCore"),
            Self::VCore => serializer.serialize_unit_variant("LicenseCoreType", 1u32, "vCore"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the properties of a License."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseDetails {
    #[doc = "Describes the state of the license."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<LicenseState>,
    #[doc = "Describes the license target server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<LicenseTarget>,
    #[doc = "Describes the edition of the license. The values are either Standard or Datacenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<LicenseEdition>,
    #[doc = "Describes the license core type (pCore or vCore)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<LicenseCoreType>,
    #[doc = "Describes the number of processors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<i32>,
    #[doc = "Describes the number of assigned licenses."]
    #[serde(rename = "assignedLicenses", default, skip_serializing_if = "Option::is_none")]
    pub assigned_licenses: Option<i32>,
    #[doc = "Describes the immutable id."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
}
impl LicenseDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the edition of the license. The values are either Standard or Datacenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseEdition")]
pub enum LicenseEdition {
    Standard,
    Datacenter,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseEdition {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseEdition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseEdition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("LicenseEdition", 0u32, "Standard"),
            Self::Datacenter => serializer.serialize_unit_variant("LicenseEdition", 1u32, "Datacenter"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a license profile in a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseProfile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describe the properties of a license profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<license_profile::Properties>,
}
impl LicenseProfile {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub mod license_profile {
    use super::*;
    #[doc = "Describe the properties of a license profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Describes the properties of a License Profile ARM model."]
        #[serde(rename = "esuProfile", default, skip_serializing_if = "Option::is_none")]
        pub esu_profile: Option<LicenseProfileArmEsuProperties>,
        #[doc = "The provisioning state, which only appears in the response."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ProvisioningState>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Describes the properties of a License Profile ARM model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProfileArmEsuProperties {
    #[serde(flatten)]
    pub license_profile_arm_esu_properties_without_assigned_license: LicenseProfileArmEsuPropertiesWithoutAssignedLicense,
    #[doc = "The resource id of the license."]
    #[serde(rename = "assignedLicense", default, skip_serializing_if = "Option::is_none")]
    pub assigned_license: Option<String>,
}
impl LicenseProfileArmEsuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a License Profile ARM model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProfileArmEsuPropertiesWithoutAssignedLicense {
    #[serde(flatten)]
    pub license_profile_storage_model_esu_properties: LicenseProfileStorageModelEsuProperties,
    #[doc = "The server types for Esu."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<EsuServerType>,
    #[doc = "The ESU eligibility."]
    #[serde(rename = "esuEligibility", default, skip_serializing_if = "Option::is_none")]
    pub esu_eligibility: Option<EsuEligibility>,
    #[doc = "The ESU key state."]
    #[serde(rename = "esuKeyState", default, skip_serializing_if = "Option::is_none")]
    pub esu_key_state: Option<EsuKeyState>,
}
impl LicenseProfileArmEsuPropertiesWithoutAssignedLicense {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "License Profile Instance View in Machine Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProfileMachineInstanceView {
    #[doc = "Properties for the Machine ESU profile."]
    #[serde(rename = "esuProfile", default, skip_serializing_if = "Option::is_none")]
    pub esu_profile: Option<LicenseProfileMachineInstanceViewEsuProperties>,
}
impl LicenseProfileMachineInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the Machine ESU profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProfileMachineInstanceViewEsuProperties {
    #[serde(flatten)]
    pub license_profile_arm_esu_properties_without_assigned_license: LicenseProfileArmEsuPropertiesWithoutAssignedLicense,
    #[doc = "Describes a license in a hybrid machine."]
    #[serde(rename = "assignedLicense", default, skip_serializing_if = "Option::is_none")]
    pub assigned_license: Option<License>,
    #[doc = "Describes the license assignment state (Assigned or NotAssigned)."]
    #[serde(rename = "licenseAssignmentState", default, skip_serializing_if = "Option::is_none")]
    pub license_assignment_state: Option<license_profile_machine_instance_view_esu_properties::LicenseAssignmentState>,
}
impl LicenseProfileMachineInstanceViewEsuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod license_profile_machine_instance_view_esu_properties {
    use super::*;
    #[doc = "Describes the license assignment state (Assigned or NotAssigned)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseAssignmentState")]
    pub enum LicenseAssignmentState {
        Assigned,
        NotAssigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseAssignmentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseAssignmentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseAssignmentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Assigned => serializer.serialize_unit_variant("LicenseAssignmentState", 0u32, "Assigned"),
                Self::NotAssigned => serializer.serialize_unit_variant("LicenseAssignmentState", 1u32, "NotAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "License profile storage model for ESU properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProfileStorageModelEsuProperties {
    #[doc = "The guid id of the license."]
    #[serde(rename = "assignedLicenseImmutableId", default, skip_serializing_if = "Option::is_none")]
    pub assigned_license_immutable_id: Option<String>,
    #[doc = "The list of ESU keys."]
    #[serde(
        rename = "esuKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub esu_keys: Vec<EsuKey>,
}
impl LicenseProfileStorageModelEsuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a License Profile Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProfileUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[doc = "Describe the Update properties of a license profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<license_profile_update::Properties>,
}
impl LicenseProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod license_profile_update {
    use super::*;
    #[doc = "Describe the Update properties of a license profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Describes the Update properties of a License Profile."]
        #[serde(rename = "esuProfile", default, skip_serializing_if = "Option::is_none")]
        pub esu_profile: Option<EsuProfileUpdateProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List hybrid machine license profile operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseProfilesListResult {
    #[doc = "The list of license profiles."]
    pub value: Vec<LicenseProfile>,
    #[doc = "The URI to fetch the next page of Machines. Call ListNext() with this URI to fetch the next page of license profile."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LicenseProfilesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LicenseProfilesListResult {
    pub fn new(value: Vec<LicenseProfile>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a License Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseProperties {
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Describes the tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of the license resource."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<LicenseType>,
    #[doc = "Describes the properties of a License."]
    #[serde(rename = "licenseDetails", default, skip_serializing_if = "Option::is_none")]
    pub license_details: Option<LicenseDetails>,
}
impl LicenseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the state of the license."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseState")]
pub enum LicenseState {
    Activated,
    Deactivated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Activated => serializer.serialize_unit_variant("LicenseState", 0u32, "Activated"),
            Self::Deactivated => serializer.serialize_unit_variant("LicenseState", 1u32, "Deactivated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the license target server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseTarget")]
pub enum LicenseTarget {
    #[serde(rename = "Windows Server 2012")]
    WindowsServer2012,
    #[serde(rename = "Windows Server 2012 R2")]
    WindowsServer2012R2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseTarget {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseTarget {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseTarget {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WindowsServer2012 => serializer.serialize_unit_variant("LicenseTarget", 0u32, "Windows Server 2012"),
            Self::WindowsServer2012R2 => serializer.serialize_unit_variant("LicenseTarget", 1u32, "Windows Server 2012 R2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the license resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseType")]
pub enum LicenseType {
    #[serde(rename = "ESU")]
    Esu,
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
            Self::Esu => serializer.serialize_unit_variant("LicenseType", 0u32, "ESU"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a License Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[doc = "Describes the Update properties of a License Profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LicenseUpdateProperties>,
}
impl LicenseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Update properties of a License Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LicenseUpdateProperties {
    #[doc = "The type of the license resource."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<LicenseType>,
    #[serde(rename = "licenseDetails", default, skip_serializing_if = "Option::is_none")]
    pub license_details: Option<license_update_properties::LicenseDetails>,
}
impl LicenseUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod license_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LicenseDetails {
        #[doc = "Describes the state of the license."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<LicenseState>,
        #[doc = "Describes the license target server."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<LicenseTarget>,
        #[doc = "Describes the edition of the license. The values are either Standard or Datacenter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub edition: Option<LicenseEdition>,
        #[doc = "Describes the license core type (pCore or vCore)."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<LicenseCoreType>,
        #[doc = "Describes the number of processors."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub processors: Option<i32>,
    }
    impl LicenseDetails {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List license operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicensesListResult {
    #[doc = "The list of licenses."]
    pub value: Vec<License>,
    #[doc = "The URI to fetch the next page of Machines. Call ListNext() with this URI to fetch the next page of license profile."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LicensesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LicensesListResult {
    pub fn new(value: Vec<License>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Input for InstallPatches on a Linux VM, as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxParameters {
    #[doc = "The update classifications to select when installing patches for Linux."]
    #[serde(
        rename = "classificationsToInclude",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications_to_include: Vec<String>,
    #[doc = "packages to include in the patch operation. Format: packageName_packageVersion"]
    #[serde(
        rename = "packageNameMasksToInclude",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_name_masks_to_include: Vec<String>,
    #[doc = "packages to exclude in the patch operation. Format: packageName_packageVersion"]
    #[serde(
        rename = "packageNameMasksToExclude",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub package_name_masks_to_exclude: Vec<String>,
}
impl LinuxParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a hybrid machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineProperties>,
    #[doc = "The list of extensions affiliated to the machine"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<MachineExtension>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Indicates which kind of Arc machine placement on-premises, such as HCI, SCVMM or VMware etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ArcKindEnum>,
}
impl Machine {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            resources: Vec::new(),
            identity: None,
            kind: None,
        }
    }
}
#[doc = "Describes the properties of an AssessPatches result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineAssessPatchesResult {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_assess_patches_result::Status>,
    #[doc = "The activity ID of the operation that produced this result."]
    #[serde(rename = "assessmentActivityId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_activity_id: Option<String>,
    #[doc = "The overall reboot status of the VM. It will be true when partially installed patches require a reboot to complete installation but the reboot has not yet occurred."]
    #[serde(rename = "rebootPending", default, skip_serializing_if = "Option::is_none")]
    pub reboot_pending: Option<bool>,
    #[doc = "Summarization of patches available for installation on the machine by classification."]
    #[serde(rename = "availablePatchCountByClassification", default, skip_serializing_if = "Option::is_none")]
    pub available_patch_count_by_classification: Option<AvailablePatchCountByClassification>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp when the operation finished."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<machine_assess_patches_result::StartedBy>,
    #[doc = "Specifies the patch service used for the operation."]
    #[serde(rename = "patchServiceUsed", default, skip_serializing_if = "Option::is_none")]
    pub patch_service_used: Option<machine_assess_patches_result::PatchServiceUsed>,
    #[doc = "The operating system type of the machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<machine_assess_patches_result::OsType>,
    #[doc = "The error detail."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetail>,
}
impl MachineAssessPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_assess_patches_result {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartedBy")]
    pub enum StartedBy {
        User,
        Platform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StartedBy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartedBy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartedBy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("StartedBy", 0u32, "User"),
                Self::Platform => serializer.serialize_unit_variant("StartedBy", 1u32, "Platform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the patch service used for the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchServiceUsed")]
    pub enum PatchServiceUsed {
        Unknown,
        #[serde(rename = "WU")]
        Wu,
        #[serde(rename = "WU_WSUS")]
        WuWsus,
        #[serde(rename = "YUM")]
        Yum,
        #[serde(rename = "APT")]
        Apt,
        Zypper,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchServiceUsed {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchServiceUsed {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchServiceUsed {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PatchServiceUsed", 0u32, "Unknown"),
                Self::Wu => serializer.serialize_unit_variant("PatchServiceUsed", 1u32, "WU"),
                Self::WuWsus => serializer.serialize_unit_variant("PatchServiceUsed", 2u32, "WU_WSUS"),
                Self::Yum => serializer.serialize_unit_variant("PatchServiceUsed", 3u32, "YUM"),
                Self::Apt => serializer.serialize_unit_variant("PatchServiceUsed", 4u32, "APT"),
                Self::Zypper => serializer.serialize_unit_variant("PatchServiceUsed", 5u32, "Zypper"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operating system type of the machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtension {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of a Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionProperties>,
}
impl MachineExtension {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Describes the Machine Extension Instance View."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionInstanceView {
    #[doc = "The machine extension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_extension_instance_view::Status>,
}
impl MachineExtensionInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_extension_instance_view {
    use super::*;
    #[doc = "Instance view status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "The status code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The level code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<status::Level>,
        #[doc = "The short localizable label for the status."]
        #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
        pub display_status: Option<String>,
        #[doc = "The detailed status message, including for alerts and error messages."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The time of the status."]
        #[serde(default, with = "azure_core::date::rfc3339::option")]
        pub time: Option<time::OffsetDateTime>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "The level code."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Level")]
        pub enum Level {
            Info,
            Warning,
            Error,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Level {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Level {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Level {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Info => serializer.serialize_unit_variant("Level", 0u32, "Info"),
                    Self::Warning => serializer.serialize_unit_variant("Level", 1u32, "Warning"),
                    Self::Error => serializer.serialize_unit_variant("Level", 2u32, "Error"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Describes the Machine Extension Instance View."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<MachineExtensionInstanceView>,
}
impl MachineExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Machine Extension Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[doc = "Describes the properties of a Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionUpdateProperties>,
}
impl MachineExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdateProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
}
impl MachineExtensionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extension Upgrade Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpgrade {
    #[doc = "Describes the Machine Extension Target Properties"]
    #[serde(rename = "extensionTargets", default, skip_serializing_if = "Option::is_none")]
    pub extension_targets: Option<ExtensionTarget>,
}
impl MachineExtensionUpgrade {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extensions List Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionsListResult {
    #[doc = "The list of extensions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MachineExtension>,
    #[doc = "The uri to fetch the next page of machine extensions. Call ListNext() with this to fetch the next page of extensions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineExtensionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MachineExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for InstallPatches as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineInstallPatchesParameters {
    #[doc = "Specifies the maximum amount of time that the operation will run. It must be an ISO 8601-compliant duration string such as PT4H (4 hours)"]
    #[serde(rename = "maximumDuration")]
    pub maximum_duration: String,
    #[doc = "Defines when it is acceptable to reboot a VM during a software update operation."]
    #[serde(rename = "rebootSetting")]
    pub reboot_setting: machine_install_patches_parameters::RebootSetting,
    #[doc = "Input for InstallPatches on a Windows VM, as directly received by the API"]
    #[serde(rename = "windowsParameters", default, skip_serializing_if = "Option::is_none")]
    pub windows_parameters: Option<WindowsParameters>,
    #[doc = "Input for InstallPatches on a Linux VM, as directly received by the API"]
    #[serde(rename = "linuxParameters", default, skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
}
impl MachineInstallPatchesParameters {
    pub fn new(maximum_duration: String, reboot_setting: machine_install_patches_parameters::RebootSetting) -> Self {
        Self {
            maximum_duration,
            reboot_setting,
            windows_parameters: None,
            linux_parameters: None,
        }
    }
}
pub mod machine_install_patches_parameters {
    use super::*;
    #[doc = "Defines when it is acceptable to reboot a VM during a software update operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootSetting")]
    pub enum RebootSetting {
        IfRequired,
        Never,
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IfRequired => serializer.serialize_unit_variant("RebootSetting", 0u32, "IfRequired"),
                Self::Never => serializer.serialize_unit_variant("RebootSetting", 1u32, "Never"),
                Self::Always => serializer.serialize_unit_variant("RebootSetting", 2u32, "Always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result summary of an installation operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineInstallPatchesResult {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Failed\", \"Succeeded\", \"Unknown\" or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_install_patches_result::Status>,
    #[doc = "The activity ID of the operation that produced this result."]
    #[serde(rename = "installationActivityId", default, skip_serializing_if = "Option::is_none")]
    pub installation_activity_id: Option<String>,
    #[doc = "The reboot state of the VM following completion of the operation."]
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<machine_install_patches_result::RebootStatus>,
    #[doc = "Whether the operation ran out of time before it completed all its intended actions."]
    #[serde(rename = "maintenanceWindowExceeded", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_exceeded: Option<bool>,
    #[doc = "The number of patches that were not installed due to the user blocking their installation."]
    #[serde(rename = "excludedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub excluded_patch_count: Option<i32>,
    #[doc = "The number of patches that were detected as available for install, but did not meet the operation's criteria."]
    #[serde(rename = "notSelectedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub not_selected_patch_count: Option<i32>,
    #[doc = "The number of patches that were identified as meeting the installation criteria, but were not able to be installed. Typically this happens when maintenanceWindowExceeded == true."]
    #[serde(rename = "pendingPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_patch_count: Option<i32>,
    #[doc = "The number of patches successfully installed."]
    #[serde(rename = "installedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub installed_patch_count: Option<i32>,
    #[doc = "The number of patches that could not be installed due to some issue. See errors for details."]
    #[serde(rename = "failedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_patch_count: Option<i32>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp when the operation finished."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<machine_install_patches_result::StartedBy>,
    #[doc = "Specifies the patch service used for the operation."]
    #[serde(rename = "patchServiceUsed", default, skip_serializing_if = "Option::is_none")]
    pub patch_service_used: Option<machine_install_patches_result::PatchServiceUsed>,
    #[doc = "The operating system type of the machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<machine_install_patches_result::OsType>,
    #[doc = "The error detail."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetail>,
}
impl MachineInstallPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_install_patches_result {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Failed\", \"Succeeded\", \"Unknown\" or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reboot state of the VM following completion of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootStatus")]
    pub enum RebootStatus {
        Unknown,
        NotNeeded,
        Required,
        Started,
        Failed,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RebootStatus", 0u32, "Unknown"),
                Self::NotNeeded => serializer.serialize_unit_variant("RebootStatus", 1u32, "NotNeeded"),
                Self::Required => serializer.serialize_unit_variant("RebootStatus", 2u32, "Required"),
                Self::Started => serializer.serialize_unit_variant("RebootStatus", 3u32, "Started"),
                Self::Failed => serializer.serialize_unit_variant("RebootStatus", 4u32, "Failed"),
                Self::Completed => serializer.serialize_unit_variant("RebootStatus", 5u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if operation was triggered by user or by platform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StartedBy")]
    pub enum StartedBy {
        User,
        Platform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StartedBy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StartedBy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StartedBy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("StartedBy", 0u32, "User"),
                Self::Platform => serializer.serialize_unit_variant("StartedBy", 1u32, "Platform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the patch service used for the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchServiceUsed")]
    pub enum PatchServiceUsed {
        Unknown,
        #[serde(rename = "WU")]
        Wu,
        #[serde(rename = "WU_WSUS")]
        WuWsus,
        #[serde(rename = "YUM")]
        Yum,
        #[serde(rename = "APT")]
        Apt,
        Zypper,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchServiceUsed {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchServiceUsed {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchServiceUsed {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PatchServiceUsed", 0u32, "Unknown"),
                Self::Wu => serializer.serialize_unit_variant("PatchServiceUsed", 1u32, "WU"),
                Self::WuWsus => serializer.serialize_unit_variant("PatchServiceUsed", 2u32, "WU_WSUS"),
                Self::Yum => serializer.serialize_unit_variant("PatchServiceUsed", 3u32, "YUM"),
                Self::Apt => serializer.serialize_unit_variant("PatchServiceUsed", 4u32, "APT"),
                Self::Zypper => serializer.serialize_unit_variant("PatchServiceUsed", 5u32, "Zypper"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operating system type of the machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The List hybrid machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListResult {
    #[doc = "The list of hybrid machines."]
    pub value: Vec<Machine>,
    #[doc = "The URI to fetch the next page of Machines. Call ListNext() with this URI to fetch the next page of hybrid machines."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MachineListResult {
    pub fn new(value: Vec<Machine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[doc = "Metadata pertaining to the geographic location of the resource."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[doc = "Configurable properties that the user can set locally via the azcmagent config command, or remotely via ARM."]
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentConfiguration>,
    #[doc = "Reports the state and behavior of dependent services."]
    #[serde(rename = "serviceStatuses", default, skip_serializing_if = "Option::is_none")]
    pub service_statuses: Option<ServiceStatuses>,
    #[doc = "The metadata of the cloud environment (Azure/GCP/AWS/OCI...)."]
    #[serde(rename = "cloudMetadata", default, skip_serializing_if = "Option::is_none")]
    pub cloud_metadata: Option<CloudMetadata>,
    #[doc = "The info w.r.t Agent Upgrade."]
    #[serde(rename = "agentUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub agent_upgrade: Option<AgentUpgrade>,
    #[doc = "Specifies the operating system settings for the hybrid machine."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "License Profile Instance View in Machine Properties."]
    #[serde(rename = "licenseProfile", default, skip_serializing_if = "Option::is_none")]
    pub license_profile: Option<LicenseProfileMachineInstanceView>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The status of the hybrid machine agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_properties::Status>,
    #[doc = "The time of the last status change."]
    #[serde(rename = "lastStatusChange", default, with = "azure_core::date::rfc3339::option")]
    pub last_status_change: Option<time::OffsetDateTime>,
    #[doc = "Details about the error state."]
    #[serde(
        rename = "errorDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_details: Vec<ErrorDetail>,
    #[doc = "The hybrid machine agent full version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Specifies the hybrid machine unique ID."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Specifies the hybrid machine display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Specifies the hybrid machine FQDN."]
    #[serde(rename = "machineFqdn", default, skip_serializing_if = "Option::is_none")]
    pub machine_fqdn: Option<String>,
    #[doc = "Public Key that the client provides to be used during initial resource onboarding"]
    #[serde(rename = "clientPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[doc = "The Operating System running on the hybrid machine."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of Operating System running on the hybrid machine."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "The type of Operating System (windows/linux)."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Specifies the Arc Machine's unique SMBIOS ID"]
    #[serde(rename = "vmUuid", default, skip_serializing_if = "Option::is_none")]
    pub vm_uuid: Option<String>,
    #[doc = "Machine Extensions information (deprecated field)"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions: Vec<MachineExtensionInstanceView>,
    #[doc = "Specifies the Operating System product SKU."]
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<String>,
    #[doc = "Specifies the Windows domain name."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Specifies the AD fully qualified display name."]
    #[serde(rename = "adFqdn", default, skip_serializing_if = "Option::is_none")]
    pub ad_fqdn: Option<String>,
    #[doc = "Specifies the DNS fully qualified display name."]
    #[serde(rename = "dnsFqdn", default, skip_serializing_if = "Option::is_none")]
    pub dns_fqdn: Option<String>,
    #[doc = "The resource id of the private link scope this machine is assigned to, if any."]
    #[serde(rename = "privateLinkScopeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_resource_id: Option<String>,
    #[doc = "The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any."]
    #[serde(rename = "parentClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_cluster_resource_id: Option<String>,
    #[doc = "Specifies whether any MS SQL instance is discovered on the machine."]
    #[serde(rename = "mssqlDiscovered", default, skip_serializing_if = "Option::is_none")]
    pub mssql_discovered: Option<String>,
    #[doc = "Detected properties from the machine."]
    #[serde(rename = "detectedProperties", default, skip_serializing_if = "Option::is_none")]
    pub detected_properties: Option<DetectedProperties>,
    #[doc = "Describes the network information on this machine."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_properties {
    use super::*;
    #[doc = "The status of the hybrid machine agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Connected,
        Disconnected,
        Error,
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
                Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
                Self::Error => serializer.serialize_unit_variant("Status", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a hybrid machine Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Indicates which kind of Arc machine placement on-premises, such as HCI, SCVMM or VMware etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ArcKindEnum>,
    #[doc = "Describes the ARM updatable properties of a hybrid machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineUpdateProperties>,
}
impl MachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the ARM updatable properties of a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdateProperties {
    #[doc = "Metadata pertaining to the geographic location of the resource."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[doc = "Specifies the operating system settings for the hybrid machine."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The metadata of the cloud environment (Azure/GCP/AWS/OCI...)."]
    #[serde(rename = "cloudMetadata", default, skip_serializing_if = "Option::is_none")]
    pub cloud_metadata: Option<CloudMetadata>,
    #[doc = "The info w.r.t Agent Upgrade."]
    #[serde(rename = "agentUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub agent_upgrade: Option<AgentUpgrade>,
    #[doc = "The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any."]
    #[serde(rename = "parentClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_cluster_resource_id: Option<String>,
    #[doc = "The resource id of the private link scope this machine is assigned to, if any."]
    #[serde(rename = "privateLinkScopeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_resource_id: Option<String>,
}
impl MachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "The list of IP addresses in this interface."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<IpAddress>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the network information on this machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "The list of network interfaces."]
    #[serde(
        rename = "networkInterfaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interfaces: Vec<NetworkInterface>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the operating system settings for the hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the hybrid machine."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "Specifies the windows configuration for update management."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<os_profile::WindowsConfiguration>,
    #[doc = "Specifies the linux configuration for update management."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<os_profile::LinuxConfiguration>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_profile {
    use super::*;
    #[doc = "Specifies the windows configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WindowsConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl WindowsConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Specifies the linux configuration for update management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct LinuxConfiguration {
        #[doc = "Specifies the patch settings."]
        #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
        pub patch_settings: Option<PatchSettings>,
    }
    impl LinuxConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List Compute Operation operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of compute operations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationValue>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValue {
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a Hybrid Compute Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationValueDisplay>,
    #[doc = "This property indicates if the operation is an action or a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Hybrid Compute Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValueDisplay {
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the patch settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchSettings {
    #[doc = "Specifies the assessment mode."]
    #[serde(rename = "assessmentMode", default, skip_serializing_if = "Option::is_none")]
    pub assessment_mode: Option<patch_settings::AssessmentMode>,
    #[doc = "Specifies the patch mode."]
    #[serde(rename = "patchMode", default, skip_serializing_if = "Option::is_none")]
    pub patch_mode: Option<patch_settings::PatchMode>,
}
impl PatchSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_settings {
    use super::*;
    #[doc = "Specifies the assessment mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssessmentMode")]
    pub enum AssessmentMode {
        ImageDefault,
        AutomaticByPlatform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssessmentMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssessmentMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssessmentMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ImageDefault => serializer.serialize_unit_variant("AssessmentMode", 0u32, "ImageDefault"),
                Self::AutomaticByPlatform => serializer.serialize_unit_variant("AssessmentMode", 1u32, "AutomaticByPlatform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the patch mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchMode")]
    pub enum PatchMode {
        ImageDefault,
        AutomaticByPlatform,
        #[serde(rename = "AutomaticByOS")]
        AutomaticByOs,
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ImageDefault => serializer.serialize_unit_variant("PatchMode", 0u32, "ImageDefault"),
                Self::AutomaticByPlatform => serializer.serialize_unit_variant("PatchMode", 1u32, "AutomaticByPlatform"),
                Self::AutomaticByOs => serializer.serialize_unit_variant("PatchMode", 2u32, "AutomaticByOS"),
                Self::Manual => serializer.serialize_unit_variant("PatchMode", 3u32, "Manual"),
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Model for a Private Endpoint Connection associated with a Private Link Scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionDataModel {
    #[doc = "The ARM Resource Id of the Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Name of the Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnectionDataModel {
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
    #[doc = "Private endpoint which the connection belongs to."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "List of group IDs."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint which the connection belongs to."]
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
    #[doc = "Required DNS zone names of the the private link resource."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkScopeValidationDetails {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The network access policy to determine if Azure Arc agents can use public Azure Arc service endpoints. Defaults to disabled (access to Azure Arc services only via private link)."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[doc = "List of Private Endpoint Connection details."]
    #[serde(
        rename = "connectionDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub connection_details: Vec<ConnectionDetail>,
}
impl PrivateLinkScopeValidationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopesResource {
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
impl PrivateLinkScopesResource {
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
#[doc = "State of the private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    pub status: String,
    #[doc = "The private link service connection description."]
    pub description: String,
    #[doc = "The actions required for private link service connection."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new(status: String, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
    }
}
#[doc = "The provisioning state, which only appears in the response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Accepted,
    Canceled,
    Deleted,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Accepted"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "The network access policy to determine if Azure Arc agents can use public Azure Arc service endpoints. Defaults to disabled (access to Azure Arc services only via private link)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccessType")]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccessType", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for PublicNetworkAccessType {
    fn default() -> Self {
        Self::Disabled
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Update Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the status and behavior of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceStatus {
    #[doc = "The current status of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The behavior of the service when the Arc-enabled machine starts up."]
    #[serde(rename = "startupType", default, skip_serializing_if = "Option::is_none")]
    pub startup_type: Option<String>,
}
impl ServiceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reports the state and behavior of dependent services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceStatuses {
    #[doc = "Describes the status and behavior of a service."]
    #[serde(rename = "extensionService", default, skip_serializing_if = "Option::is_none")]
    pub extension_service: Option<ServiceStatus>,
    #[doc = "Describes the status and behavior of a service."]
    #[serde(rename = "guestConfigurationService", default, skip_serializing_if = "Option::is_none")]
    pub guest_configuration_service: Option<ServiceStatus>,
}
impl ServiceStatuses {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[doc = "Represents address prefix."]
    #[serde(rename = "addressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags on a PrivateLinkScope instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TargetVersion = String;
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
#[doc = "Input for InstallPatches on a Windows VM, as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsParameters {
    #[doc = "The update classifications to select when installing patches for Windows."]
    #[serde(
        rename = "classificationsToInclude",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications_to_include: Vec<String>,
    #[doc = "Kbs to include in the patch operation"]
    #[serde(
        rename = "kbNumbersToInclude",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kb_numbers_to_include: Vec<String>,
    #[doc = "Kbs to exclude in the patch operation"]
    #[serde(
        rename = "kbNumbersToExclude",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kb_numbers_to_exclude: Vec<String>,
    #[doc = "Filters out Kbs that don't have an InstallationRebootBehavior of 'NeverReboots' when this is set to true."]
    #[serde(rename = "excludeKbsRequiringReboot", default, skip_serializing_if = "Option::is_none")]
    pub exclude_kbs_requiring_reboot: Option<bool>,
    #[doc = "This is used to install patches that were published on or before this given max published date."]
    #[serde(rename = "maxPatchPublishDate", default, with = "azure_core::date::rfc3339::option")]
    pub max_patch_publish_date: Option<time::OffsetDateTime>,
}
impl WindowsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to the geographic location of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationData {
    #[doc = "A canonical name for the geographic or physical location."]
    pub name: String,
    #[doc = "The city or locality where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The district, state, or province where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[doc = "The country or region where the resource is located"]
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
impl LocationData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            city: None,
            district: None,
            country_or_region: None,
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
