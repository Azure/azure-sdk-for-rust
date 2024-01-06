#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadConnectivityState {
    #[serde(rename = "connectivityState", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_state: Option<aad_connectivity_state::ConnectivityState>,
}
impl AadConnectivityState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod aad_connectivity_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectivityState")]
    pub enum ConnectivityState {
        Discovered,
        NotLicensed,
        Connected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectivityState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectivityState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectivityState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Discovered => serializer.serialize_unit_variant("ConnectivityState", 0u32, "Discovered"),
                Self::NotLicensed => serializer.serialize_unit_variant("ConnectivityState", 1u32, "NotLicensed"),
                Self::Connected => serializer.serialize_unit_variant("ConnectivityState", 2u32, "Connected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents an AAD identity protection solution which sends logs to an OMS workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadExternalSecuritySolution {
    #[serde(flatten)]
    pub external_security_solution: ExternalSecuritySolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadSolutionProperties>,
}
impl AadExternalSecuritySolution {
    pub fn new(external_security_solution: ExternalSecuritySolution) -> Self {
        Self {
            external_security_solution,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadSolutionProperties {
    #[serde(flatten)]
    pub external_security_solution_properties: ExternalSecuritySolutionProperties,
    #[serde(flatten)]
    pub aad_connectivity_state: AadConnectivityState,
}
impl AadSolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration payload for PR Annotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionableRemediation {
    #[doc = "ActionableRemediation Setting.\r\nNone - the setting was never set.\r\nEnabled - ActionableRemediation is enabled.\r\nDisabled - ActionableRemediation is disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ActionableRemediationState>,
    #[doc = "Gets or sets list of categories and severity levels."]
    #[serde(
        rename = "categoryConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_configurations: Vec<CategoryConfiguration>,
    #[doc = "Repository branch configuration for PR Annotations."]
    #[serde(rename = "branchConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub branch_configuration: Option<TargetBranchConfiguration>,
    #[doc = "Update Settings.\r\n\r\nEnabled - Resource should inherit configurations from parent.\r\nDisabled - Resource should not inherit configurations from parent."]
    #[serde(rename = "inheritFromParentState", default, skip_serializing_if = "Option::is_none")]
    pub inherit_from_parent_state: Option<InheritFromParentState>,
}
impl ActionableRemediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ActionableRemediation Setting.\r\nNone - the setting was never set.\r\nEnabled - ActionableRemediation is enabled.\r\nDisabled - ActionableRemediation is disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionableRemediationState")]
pub enum ActionableRemediationState {
    None,
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionableRemediationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionableRemediationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionableRemediationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ActionableRemediationState", 0u32, "None"),
            Self::Disabled => serializer.serialize_unit_variant("ActionableRemediationState", 1u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ActionableRemediationState", 2u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Number of active connections is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveConnectionsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl ActiveConnectionsNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveApplicationControlGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    #[doc = "Represents a machines group and set of rules to be allowed running on a machine"]
    pub properties: AdaptiveApplicationControlGroupData,
}
impl AdaptiveApplicationControlGroup {
    pub fn new(properties: AdaptiveApplicationControlGroupData) -> Self {
        Self {
            resource: Resource::default(),
            location: Location::default(),
            properties,
        }
    }
}
#[doc = "Represents a machines group and set of rules to be allowed running on a machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveApplicationControlGroupData {
    #[doc = "The application control policy enforcement/protection mode of the machine group"]
    #[serde(rename = "enforcementMode", default, skip_serializing_if = "Option::is_none")]
    pub enforcement_mode: Option<EnforcementMode>,
    #[doc = "The protection mode of the collection/file types. Exe/Msi/Script are used for Windows, Executable is used for Linux."]
    #[serde(rename = "protectionMode", default, skip_serializing_if = "Option::is_none")]
    pub protection_mode: Option<ProtectionMode>,
    #[doc = "The configuration status of the machines group or machine or rule"]
    #[serde(rename = "configurationStatus", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status: Option<ConfigurationStatus>,
    #[doc = "The initial recommendation status of the machine group or machine"]
    #[serde(rename = "recommendationStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<RecommendationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<AdaptiveApplicationControlIssuesSummaries>,
    #[doc = "The source type of the machine group"]
    #[serde(rename = "sourceSystem", default, skip_serializing_if = "Option::is_none")]
    pub source_system: Option<SourceSystem>,
    #[serde(rename = "vmRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub vm_recommendations: Option<VmRecommendations>,
    #[serde(rename = "pathRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub path_recommendations: Option<PathRecommendations>,
}
impl AdaptiveApplicationControlGroupData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a list of VM/server groups and set of rules that are Recommended by Microsoft Defender for Cloud to be allowed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveApplicationControlGroups {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AdaptiveApplicationControlGroup>,
}
impl AdaptiveApplicationControlGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert that machines within a group can have"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AdaptiveApplicationControlIssue {
    ViolationsAudited,
    ViolationsBlocked,
    MsiAndScriptViolationsAudited,
    MsiAndScriptViolationsBlocked,
    ExecutableViolationsAudited,
    RulesViolatedManually,
}
#[doc = "Represents a summary of the alerts of the machine group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveApplicationControlIssueSummary {
    #[doc = "An alert that machines within a group can have"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<AdaptiveApplicationControlIssue>,
    #[doc = "The number of machines in the group that have this alert"]
    #[serde(rename = "numberOfVms", default, skip_serializing_if = "Option::is_none")]
    pub number_of_vms: Option<f64>,
}
impl AdaptiveApplicationControlIssueSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AdaptiveApplicationControlIssuesSummaries = Vec<AdaptiveApplicationControlIssueSummary>;
#[doc = "The resource whose properties describes the Adaptive Network Hardening settings for some Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveNetworkHardening {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Adaptive Network Hardening resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdaptiveNetworkHardeningProperties>,
}
impl AdaptiveNetworkHardening {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveNetworkHardeningEnforceRequest {
    #[doc = "The rules to enforce"]
    pub rules: Vec<Rule>,
    #[doc = "The Azure resource IDs of the effective network security groups that will be updated with the created security rules from the Adaptive Network Hardening rules"]
    #[serde(rename = "networkSecurityGroups")]
    pub network_security_groups: Vec<String>,
}
impl AdaptiveNetworkHardeningEnforceRequest {
    pub fn new(rules: Vec<Rule>, network_security_groups: Vec<String>) -> Self {
        Self {
            rules,
            network_security_groups,
        }
    }
}
#[doc = "Adaptive Network Hardening resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveNetworkHardeningProperties {
    #[doc = "The security rules which are recommended to be effective on the VM"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<Rule>,
    #[doc = "The UTC time on which the rules were calculated"]
    #[serde(rename = "rulesCalculationTime", default, with = "azure_core::date::rfc3339::option")]
    pub rules_calculation_time: Option<time::OffsetDateTime>,
    #[doc = "The Network Security Groups effective on the network interfaces of the protected resource"]
    #[serde(
        rename = "effectiveNetworkSecurityGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub effective_network_security_groups: Vec<EffectiveNetworkSecurityGroups>,
}
impl AdaptiveNetworkHardeningProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ListAdaptiveNetworkHardenings API service call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveNetworkHardeningsList {
    #[doc = "A list of Adaptive Network Hardenings resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AdaptiveNetworkHardening>,
    #[doc = "The URL to get the next set of results"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AdaptiveNetworkHardeningsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AdaptiveNetworkHardeningsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sub-assessment resource type"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "assessedResourceType")]
pub enum AdditionalDataUnion {
    ContainerRegistryVulnerability(ContainerRegistryVulnerabilityProperties),
    ServerVulnerabilityAssessment(ServerVulnerabilityProperties),
    SqlServerVulnerability(SqlServerVulnerabilityProperties),
}
#[doc = "Properties of the additional workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalWorkspacesProperties {
    #[doc = "Workspace resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[doc = "Workspace type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<additional_workspaces_properties::Type>,
    #[doc = "List of data types sent to workspace"]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<String>,
}
impl AdditionalWorkspacesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod additional_workspaces_properties {
    use super::*;
    #[doc = "Workspace type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Sentinel,
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
                Self::Sentinel => serializer.serialize_unit_variant("Type", 0u32, "Sentinel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Sentinel
        }
    }
}
#[doc = "The Advanced Threat Protection settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvancedThreatProtectionProperties {
    #[doc = "Indicates whether Advanced Threat Protection is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
impl AdvancedThreatProtectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Advanced Threat Protection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvancedThreatProtectionSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Advanced Threat Protection settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdvancedThreatProtectionProperties>,
}
impl AdvancedThreatProtectionSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes security alert properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Changing set of properties depending on the entity type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertEntity {
    #[doc = "Type of entity"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AlertEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Links related to the alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertExtendedLinks {}
impl AlertExtendedLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom properties for the alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertExtendedProperties {}
impl AlertExtendedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of security alerts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertList {
    #[doc = "describes security alert properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Alert>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes security alert properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "Schema version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Unique identifier for the detection logic (all alert instances from the same detection logic will have the same alertType)."]
    #[serde(rename = "alertType", default, skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[doc = "Unique identifier for the alert."]
    #[serde(rename = "systemAlertId", default, skip_serializing_if = "Option::is_none")]
    pub system_alert_id: Option<String>,
    #[doc = "The name of Azure Security Center pricing tier which powering this alert. Learn more: https://docs.microsoft.com/en-us/azure/security-center/security-center-pricing"]
    #[serde(rename = "productComponentName", default, skip_serializing_if = "Option::is_none")]
    pub product_component_name: Option<String>,
    #[doc = "The display name of the alert."]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "Description of the suspicious activity that was detected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The risk level of the threat that was detected. Learn more: https://docs.microsoft.com/en-us/azure/security-center/security-center-alerts-overview#how-are-alerts-classified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<alert_properties::Severity>,
    #[doc = "The kill chain related intent behind the alert. For list of supported values, and explanations of Azure Security Center's supported kill chain intents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<alert_properties::Intent>,
    #[doc = "The UTC time of the first event or activity included in the alert in ISO8601 format."]
    #[serde(rename = "startTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub start_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The UTC time of the last event or activity included in the alert in ISO8601 format."]
    #[serde(rename = "endTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The resource identifiers that can be used to direct the alert to the right product exposure group (tenant, workspace, subscription etc.). There can be multiple identifiers of different type per alert."]
    #[serde(
        rename = "resourceIdentifiers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_identifiers: Vec<ResourceIdentifierUnion>,
    #[doc = "Manual action items to take to remediate the alert."]
    #[serde(
        rename = "remediationSteps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remediation_steps: Vec<String>,
    #[doc = "The name of the vendor that raises the alert."]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "The life cycle status of the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<alert_properties::Status>,
    #[doc = "Links related to the alert"]
    #[serde(
        rename = "extendedLinks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extended_links: Vec<AlertExtendedLinks>,
    #[doc = "A direct link to the alert page in Azure Portal."]
    #[serde(rename = "alertUri", default, skip_serializing_if = "Option::is_none")]
    pub alert_uri: Option<String>,
    #[doc = "The UTC time the alert was generated in ISO8601 format."]
    #[serde(rename = "timeGeneratedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub time_generated_utc: Option<time::OffsetDateTime>,
    #[doc = "The name of the product which published this alert (Microsoft Sentinel, Microsoft Defender for Identity, Microsoft Defender for Endpoint, Microsoft Defender for Office, Microsoft Defender for Cloud Apps, and so on)."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The UTC processing end time of the alert in ISO8601 format."]
    #[serde(rename = "processingEndTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub processing_end_time_utc: Option<time::OffsetDateTime>,
    #[doc = "A list of entities related to the alert."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entities: Vec<AlertEntity>,
    #[doc = "This field determines whether the alert is an incident (a compound grouping of several alerts) or a single alert."]
    #[serde(rename = "isIncident", default, skip_serializing_if = "Option::is_none")]
    pub is_incident: Option<bool>,
    #[doc = "Key for corelating related alerts. Alerts with the same correlation key considered to be related."]
    #[serde(rename = "correlationKey", default, skip_serializing_if = "Option::is_none")]
    pub correlation_key: Option<String>,
    #[doc = "Custom properties for the alert."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<AlertExtendedProperties>,
    #[doc = "The display name of the resource most related to this alert."]
    #[serde(rename = "compromisedEntity", default, skip_serializing_if = "Option::is_none")]
    pub compromised_entity: Option<String>,
    #[doc = "kill chain related techniques behind the alert."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
    #[doc = "Kill chain related sub-techniques behind the alert."]
    #[serde(
        rename = "subTechniques",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_techniques: Vec<String>,
    #[doc = "Changing set of properties depending on the supportingEvidence type."]
    #[serde(rename = "supportingEvidence", default, skip_serializing_if = "Option::is_none")]
    pub supporting_evidence: Option<alert_properties::SupportingEvidence>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_properties {
    use super::*;
    #[doc = "The risk level of the threat that was detected. Learn more: https://docs.microsoft.com/en-us/azure/security-center/security-center-alerts-overview#how-are-alerts-classified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Informational,
        Low,
        Medium,
        High,
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
                Self::Informational => serializer.serialize_unit_variant("Severity", 0u32, "Informational"),
                Self::Low => serializer.serialize_unit_variant("Severity", 1u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("Severity", 2u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("Severity", 3u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The kill chain related intent behind the alert. For list of supported values, and explanations of Azure Security Center's supported kill chain intents."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Intent")]
    pub enum Intent {
        Unknown,
        PreAttack,
        InitialAccess,
        Persistence,
        PrivilegeEscalation,
        DefenseEvasion,
        CredentialAccess,
        Discovery,
        LateralMovement,
        Execution,
        Collection,
        Exfiltration,
        CommandAndControl,
        Impact,
        Probing,
        Exploitation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Intent {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Intent {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Intent {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Intent", 0u32, "Unknown"),
                Self::PreAttack => serializer.serialize_unit_variant("Intent", 1u32, "PreAttack"),
                Self::InitialAccess => serializer.serialize_unit_variant("Intent", 2u32, "InitialAccess"),
                Self::Persistence => serializer.serialize_unit_variant("Intent", 3u32, "Persistence"),
                Self::PrivilegeEscalation => serializer.serialize_unit_variant("Intent", 4u32, "PrivilegeEscalation"),
                Self::DefenseEvasion => serializer.serialize_unit_variant("Intent", 5u32, "DefenseEvasion"),
                Self::CredentialAccess => serializer.serialize_unit_variant("Intent", 6u32, "CredentialAccess"),
                Self::Discovery => serializer.serialize_unit_variant("Intent", 7u32, "Discovery"),
                Self::LateralMovement => serializer.serialize_unit_variant("Intent", 8u32, "LateralMovement"),
                Self::Execution => serializer.serialize_unit_variant("Intent", 9u32, "Execution"),
                Self::Collection => serializer.serialize_unit_variant("Intent", 10u32, "Collection"),
                Self::Exfiltration => serializer.serialize_unit_variant("Intent", 11u32, "Exfiltration"),
                Self::CommandAndControl => serializer.serialize_unit_variant("Intent", 12u32, "CommandAndControl"),
                Self::Impact => serializer.serialize_unit_variant("Intent", 13u32, "Impact"),
                Self::Probing => serializer.serialize_unit_variant("Intent", 14u32, "Probing"),
                Self::Exploitation => serializer.serialize_unit_variant("Intent", 15u32, "Exploitation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The life cycle status of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        InProgress,
        Resolved,
        Dismissed,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Resolved => serializer.serialize_unit_variant("Status", 2u32, "Resolved"),
                Self::Dismissed => serializer.serialize_unit_variant("Status", 3u32, "Dismissed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Changing set of properties depending on the supportingEvidence type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SupportingEvidence {
        #[doc = "Type of the supportingEvidence"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl SupportingEvidence {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Simulate alerts according to this bundles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSimulatorBundlesRequestProperties {
    #[doc = "Bundles list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bundles: Vec<BundleType>,
}
impl AlertSimulatorBundlesRequestProperties {
    pub fn new() -> Self {
        Self { bundles: Vec::new() }
    }
}
#[doc = "Alert Simulator request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertSimulatorRequestBody {
    #[doc = "Describes properties of an alert simulation request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertSimulatorRequestPropertiesUnion>,
}
impl AlertSimulatorRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of alert simulation."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum AlertSimulatorRequestPropertiesUnion {
    Bundles(AlertSimulatorBundlesRequestProperties),
}
#[doc = "The alert sync setting properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSyncSettingProperties {
    #[doc = "Is the alert sync setting enabled"]
    pub enabled: bool,
}
impl AlertSyncSettingProperties {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[doc = "Represents an alert sync setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertSyncSettings {
    #[serde(flatten)]
    pub setting: Setting,
    #[doc = "The alert sync setting properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertSyncSettingProperties>,
}
impl AlertSyncSettings {
    pub fn new(setting: Setting) -> Self {
        Self { setting, properties: None }
    }
}
#[doc = "Describes the suppression rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsSuppressionRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes AlertsSuppressionRule properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsSuppressionRuleProperties>,
}
impl AlertsSuppressionRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes AlertsSuppressionRule properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSuppressionRuleProperties {
    #[doc = "Type of the alert to automatically suppress. For all alert types, use '*'"]
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[doc = "The last time this rule was modified"]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "Expiration date of the rule, if value is not provided or provided as null there will no expiration at all"]
    #[serde(rename = "expirationDateUtc", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date_utc: Option<time::OffsetDateTime>,
    #[doc = "The reason for dismissing the alert"]
    pub reason: String,
    #[doc = "Possible states of the rule"]
    pub state: alerts_suppression_rule_properties::State,
    #[doc = "Any comment regarding the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "suppressionAlertsScope", default, skip_serializing_if = "Option::is_none")]
    pub suppression_alerts_scope: Option<SuppressionAlertsScope>,
}
impl AlertsSuppressionRuleProperties {
    pub fn new(alert_type: String, reason: String, state: alerts_suppression_rule_properties::State) -> Self {
        Self {
            alert_type,
            last_modified_utc: None,
            expiration_date_utc: None,
            reason,
            state,
            comment: None,
            suppression_alerts_scope: None,
        }
    }
}
pub mod alerts_suppression_rule_properties {
    use super::*;
    #[doc = "Possible states of the rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
        Expired,
    }
}
#[doc = "Suppression rules list for subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSuppressionRulesList {
    pub value: Vec<AlertsSuppressionRule>,
    #[doc = "URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertsSuppressionRulesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertsSuppressionRulesList {
    pub fn new(value: Vec<AlertsSuppressionRule>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "List of all possible traffic between Azure resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedConnectionsList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AllowedConnectionsResource>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AllowedConnectionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AllowedConnectionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource whose properties describes the allowed traffic between Azure resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedConnectionsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    #[doc = "Describes the allowed traffic between Azure resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AllowedConnectionsResourceProperties>,
}
impl AllowedConnectionsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the allowed traffic between Azure resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedConnectionsResourceProperties {
    #[doc = "The UTC time on which the allowed connections resource was calculated"]
    #[serde(rename = "calculatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub calculated_date_time: Option<time::OffsetDateTime>,
    #[doc = "List of connectable resources"]
    #[serde(
        rename = "connectableResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub connectable_resources: Vec<ConnectableResource>,
}
impl AllowedConnectionsResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom alert rule that checks if a value (depends on the custom alert type) is allowed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllowlistCustomAlertRule {
    #[serde(flatten)]
    pub list_custom_alert_rule: ListCustomAlertRule,
    #[doc = "The values to allow. The format of the values depends on the rule type."]
    #[serde(rename = "allowlistValues")]
    pub allowlist_values: Vec<String>,
}
impl AllowlistCustomAlertRule {
    pub fn new(list_custom_alert_rule: ListCustomAlertRule, allowlist_values: Vec<String>) -> Self {
        Self {
            list_custom_alert_rule,
            allowlist_values,
        }
    }
}
#[doc = "Number of cloud to device messages (AMQP protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmqpC2dMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl AmqpC2dMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of rejected cloud to device messages (AMQP protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmqpC2dRejectedMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl AmqpC2dRejectedMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of device to cloud messages (AMQP protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmqpD2cMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl AmqpD2cMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Configuration of PR Annotations on default branch.\r\n\r\nEnabled - PR Annotations are enabled on the resource's default branch.\r\nDisabled - PR Annotations are disabled on the resource's default branch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AnnotateDefaultBranchState")]
pub enum AnnotateDefaultBranchState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AnnotateDefaultBranchState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AnnotateDefaultBranchState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AnnotateDefaultBranchState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("AnnotateDefaultBranchState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("AnnotateDefaultBranchState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An API collection as represented by Microsoft Defender for APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of an API collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiCollectionProperties>,
}
impl ApiCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a list of API collections as represented by Microsoft Defender for APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollectionList {
    #[doc = "API collections in this page."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApiCollection>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiCollectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiCollectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an API collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollectionProperties {
    #[doc = "Gets the provisioning state of the API collection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<api_collection_properties::ProvisioningState>,
    #[doc = "The display name of the API collection."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The resource Id of the resource from where this API collection was discovered."]
    #[serde(rename = "discoveredVia", default, skip_serializing_if = "Option::is_none")]
    pub discovered_via: Option<String>,
    #[doc = "The base URI for this API collection. All endpoints of this API collection extend this base URI."]
    #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[doc = "The number of API endpoints discovered in this API collection."]
    #[serde(rename = "numberOfApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection that have not received any API traffic in the last 30 days."]
    #[serde(rename = "numberOfInactiveApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_inactive_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection that are unauthenticated."]
    #[serde(rename = "numberOfUnauthenticatedApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_unauthenticated_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection for which API traffic from the internet was observed."]
    #[serde(rename = "numberOfExternalApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_external_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection which are exposing sensitive data in their requests and/or responses."]
    #[serde(
        rename = "numberOfApiEndpointsWithSensitiveDataExposed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_api_endpoints_with_sensitive_data_exposed: Option<i64>,
    #[doc = "The highest priority sensitivity label from Microsoft Purview in this API collection."]
    #[serde(rename = "sensitivityLabel", default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_label: Option<String>,
}
impl ApiCollectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_collection_properties {
    use super::*;
    #[doc = "Gets the provisioning state of the API collection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        InProgress,
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
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 3u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Security Application over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an application"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application's condition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationCondition {
    #[doc = "The application Condition's Property, e.g. ID, see examples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[doc = "The application Condition's Value like IDs that contain some string, see examples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The application Condition's Operator, for example Contains for id or In for list of possible IDs, see examples"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<application_condition::Operator>,
}
impl ApplicationCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod application_condition {
    use super::*;
    #[doc = "The application Condition's Operator, for example Contains for id or In for list of possible IDs, see examples"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Contains,
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
                Self::Contains => serializer.serialize_unit_variant("Operator", 0u32, "Contains"),
                Self::Equals => serializer.serialize_unit_variant("Operator", 1u32, "Equals"),
                Self::In => serializer.serialize_unit_variant("Operator", 2u32, "In"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of application's condition sets - OR between ConditionSets, AND between conditions in a set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationConditionSets {}
impl ApplicationConditionSets {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ApplicationConditions = Vec<ApplicationCondition>;
#[doc = "Describes properties of an application"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[doc = "display name of the application"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "description of the application"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The application source, what it affects, e.g. Assessments"]
    #[serde(rename = "sourceResourceType")]
    pub source_resource_type: application_properties::SourceResourceType,
    #[doc = "The application conditionSets - see examples"]
    #[serde(rename = "conditionSets")]
    pub condition_sets: Vec<ApplicationConditionSets>,
}
impl ApplicationProperties {
    pub fn new(source_resource_type: application_properties::SourceResourceType, condition_sets: Vec<ApplicationConditionSets>) -> Self {
        Self {
            display_name: None,
            description: None,
            source_resource_type,
            condition_sets,
        }
    }
}
pub mod application_properties {
    use super::*;
    #[doc = "The application source, what it affects, e.g. Assessments"]
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
#[doc = "Page of a security applications list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationsList {
    #[doc = "Collection of applications in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Application>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplicationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ASC location of the subscription is in the \"name\" field"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An empty set of properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscLocationProperties>,
}
impl AscLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of locations where ASC saves your data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscLocationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AscLocation>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AscLocationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AscLocationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An empty set of properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscLocationProperties {}
impl AscLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Links relevant to the assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssessmentLinks {
    #[doc = "Link to assessment in Azure Portal"]
    #[serde(rename = "azurePortalUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_portal_uri: Option<String>,
}
impl AssessmentLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentStatus {
    #[doc = "Programmatic code for the status of the assessment"]
    pub code: assessment_status::Code,
    #[doc = "Programmatic code for the cause of the assessment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[doc = "Human readable description of the assessment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AssessmentStatus {
    pub fn new(code: assessment_status::Code) -> Self {
        Self {
            code,
            cause: None,
            description: None,
        }
    }
}
pub mod assessment_status {
    use super::*;
    #[doc = "Programmatic code for the status of the assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Healthy,
        Unhealthy,
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
                Self::Unhealthy => serializer.serialize_unit_variant("Code", 1u32, "Unhealthy"),
                Self::NotApplicable => serializer.serialize_unit_variant("Code", 2u32, "NotApplicable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of the assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentStatusResponse {
    #[serde(flatten)]
    pub assessment_status: AssessmentStatus,
    #[doc = "The time that the assessment was created and first evaluated. Returned as UTC time in ISO 8601 format"]
    #[serde(rename = "firstEvaluationDate", default, with = "azure_core::date::rfc3339::option")]
    pub first_evaluation_date: Option<time::OffsetDateTime>,
    #[doc = "The time that the status of the assessment last changed. Returned as UTC time in ISO 8601 format"]
    #[serde(rename = "statusChangeDate", default, with = "azure_core::date::rfc3339::option")]
    pub status_change_date: Option<time::OffsetDateTime>,
}
impl AssessmentStatusResponse {
    pub fn new(assessment_status: AssessmentStatus) -> Self {
        Self {
            assessment_status,
            first_evaluation_date: None,
            status_change_date: None,
        }
    }
}
#[doc = "Represents an ATA security solution which sends logs to an OMS workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AtaExternalSecuritySolution {
    #[serde(flatten)]
    pub external_security_solution: ExternalSecuritySolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AtaSolutionProperties>,
}
impl AtaExternalSecuritySolution {
    pub fn new(external_security_solution: ExternalSecuritySolution) -> Self {
        Self {
            external_security_solution,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtaSolutionProperties {
    #[serde(flatten)]
    pub external_security_solution_properties: ExternalSecuritySolutionProperties,
    #[serde(rename = "lastEventReceived", default, skip_serializing_if = "Option::is_none")]
    pub last_event_received: Option<String>,
}
impl AtaSolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings for cloud authentication management"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationDetailsProperties {
    #[doc = "State of the multi-cloud connector"]
    #[serde(rename = "authenticationProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub authentication_provisioning_state: Option<authentication_details_properties::AuthenticationProvisioningState>,
    #[doc = "The permissions detected in the cloud account."]
    #[serde(
        rename = "grantedPermissions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub granted_permissions: Vec<PermissionProperty>,
}
impl AuthenticationDetailsProperties {
    pub fn new() -> Self {
        Self {
            authentication_provisioning_state: None,
            granted_permissions: Vec::new(),
        }
    }
}
pub mod authentication_details_properties {
    use super::*;
    #[doc = "State of the multi-cloud connector"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationProvisioningState")]
    pub enum AuthenticationProvisioningState {
        Valid,
        Invalid,
        Expired,
        IncorrectPolicy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Valid => serializer.serialize_unit_variant("AuthenticationProvisioningState", 0u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("AuthenticationProvisioningState", 1u32, "Invalid"),
                Self::Expired => serializer.serialize_unit_variant("AuthenticationProvisioningState", 2u32, "Expired"),
                Self::IncorrectPolicy => serializer.serialize_unit_variant("AuthenticationProvisioningState", 3u32, "IncorrectPolicy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Connect to your cloud account, for AWS use either account credentials or role-based authentication. For GCP use account organization credentials."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "authenticationType")]
pub enum AuthenticationDetailsPropertiesUnion {
    #[serde(rename = "awsAssumeRole")]
    AwsAssumeRole(AwAssumeRoleAuthenticationDetailsProperties),
    #[serde(rename = "awsCreds")]
    AwsCreds(AwsCredsAuthenticationDetailsProperties),
    #[serde(rename = "gcpCredentials")]
    GcpCredentials(GcpCredentialsDetailsProperties),
}
#[doc = "Authorization payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Authorization {
    #[doc = "Gets or sets one-time OAuth code to exchange for refresh and access tokens.\r\n\r\nOnly used during PUT/PATCH operations. The secret is cleared during GET."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl Authorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AutoDiscovery states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoDiscovery")]
pub enum AutoDiscovery {
    Disabled,
    Enabled,
    NotApplicable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoDiscovery {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoDiscovery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoDiscovery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("AutoDiscovery", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("AutoDiscovery", 1u32, "Enabled"),
            Self::NotApplicable => serializer.serialize_unit_variant("AutoDiscovery", 2u32, "NotApplicable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Auto provisioning setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoProvisioningSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes properties of an auto provisioning setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoProvisioningSettingProperties>,
}
impl AutoProvisioningSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the auto provisioning settings response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoProvisioningSettingList {
    #[doc = "List of all the auto provisioning settings"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AutoProvisioningSetting>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutoProvisioningSettingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutoProvisioningSettingList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes properties of an auto provisioning setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoProvisioningSettingProperties {
    #[doc = "Describes what kind of security agent provisioning action to take"]
    #[serde(rename = "autoProvision")]
    pub auto_provision: auto_provisioning_setting_properties::AutoProvision,
}
impl AutoProvisioningSettingProperties {
    pub fn new(auto_provision: auto_provisioning_setting_properties::AutoProvision) -> Self {
        Self { auto_provision }
    }
}
pub mod auto_provisioning_setting_properties {
    use super::*;
    #[doc = "Describes what kind of security agent provisioning action to take"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoProvision")]
    pub enum AutoProvision {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoProvision {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoProvision {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoProvision {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("AutoProvision", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("AutoProvision", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The security automation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Automation {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A set of properties that defines the behavior of the automation configuration. To learn more about the supported security events data models schemas - please visit https://aka.ms/ASCAutomationSchemas."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutomationProperties>,
}
impl Automation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the action that will be triggered by the Automation"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "actionType")]
pub enum AutomationActionUnion {
    EventHub(AutomationActionEventHub),
    LogicApp(AutomationActionLogicApp),
    Workspace(AutomationActionWorkspace),
}
#[doc = "The target Event Hub to which event data will be exported. To learn more about Microsoft Defender for Cloud continuous export capabilities, visit https://aka.ms/ASCExportLearnMore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationActionEventHub {
    #[doc = "The target Event Hub Azure Resource ID."]
    #[serde(rename = "eventHubResourceId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_resource_id: Option<String>,
    #[doc = "The target Event Hub SAS policy name."]
    #[serde(rename = "sasPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub sas_policy_name: Option<String>,
    #[doc = "The target Event Hub connection string (it will not be included in any response)."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
}
impl AutomationActionEventHub {
    pub fn new() -> Self {
        Self {
            event_hub_resource_id: None,
            sas_policy_name: None,
            connection_string: None,
        }
    }
}
#[doc = "The logic app action that should be triggered. To learn more about Microsoft Defender for Cloud's Workflow Automation capabilities, visit https://aka.ms/ASCWorkflowAutomationLearnMore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationActionLogicApp {
    #[doc = "The triggered Logic App Azure Resource ID. This can also reside on other subscriptions, given that you have permissions to trigger the Logic App"]
    #[serde(rename = "logicAppResourceId", default, skip_serializing_if = "Option::is_none")]
    pub logic_app_resource_id: Option<String>,
    #[doc = "The Logic App trigger URI endpoint (it will not be included in any response)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl AutomationActionLogicApp {
    pub fn new() -> Self {
        Self {
            logic_app_resource_id: None,
            uri: None,
        }
    }
}
#[doc = "The\u{a0}Log\u{a0}Analytics\u{a0}Workspace\u{a0}to\u{a0}which\u{a0}event data will be exported. Security alerts data will reside in the 'SecurityAlert' table and the assessments data will reside in the 'SecurityRecommendation' table (under the 'Security'/'SecurityCenterFree' solutions). Note that in order to view the data in the workspace, the Security Center Log Analytics free/standard solution needs to be enabled on that workspace. To learn more about Microsoft Defender for Cloud continuous export capabilities, visit https://aka.ms/ASCExportLearnMore"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationActionWorkspace {
    #[doc = "The fully qualified Log Analytics Workspace Azure Resource ID."]
    #[serde(rename = "workspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_id: Option<String>,
}
impl AutomationActionWorkspace {
    pub fn new() -> Self {
        Self {
            workspace_resource_id: None,
        }
    }
}
#[doc = "List of security automations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationList {
    #[doc = "The list of security automations under the given scope."]
    pub value: Vec<Automation>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutomationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutomationList {
    pub fn new(value: Vec<Automation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A set of properties that defines the behavior of the automation configuration. To learn more about the supported security events data models schemas - please visit https://aka.ms/ASCAutomationSchemas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationProperties {
    #[doc = "The security automation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Indicates whether the security automation is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "A collection of scopes on which the security automations logic is applied. Supported scopes are the subscription itself or a resource group under that subscription. The automation will only apply on defined scopes."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<AutomationScope>,
    #[doc = "A collection of the source event types which evaluate the security automation set of rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sources: Vec<AutomationSource>,
    #[doc = "A collection of the actions which are triggered if all the configured rules evaluations, within at least one rule set, are true."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<AutomationActionUnion>,
}
impl AutomationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule set which evaluates all its rules upon an event interception. Only when all the included rules in the rule set will be evaluated as 'true', will the event trigger the defined actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationRuleSet {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<AutomationTriggeringRule>,
}
impl AutomationRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single automation scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationScope {
    #[doc = "The resources scope description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resources scope path. Can be the subscription on which the automation is defined on or a resource group under that subscription (fully qualified Azure resource IDs)."]
    #[serde(rename = "scopePath", default, skip_serializing_if = "Option::is_none")]
    pub scope_path: Option<String>,
}
impl AutomationScope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The source event types which evaluate the security automation set of rules. For example - security alerts and security assessments. To learn more about the supported security events data models schemas - please visit https://aka.ms/ASCAutomationSchemas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationSource {
    #[doc = "A valid event source type."]
    #[serde(rename = "eventSource", default, skip_serializing_if = "Option::is_none")]
    pub event_source: Option<automation_source::EventSource>,
    #[doc = "A set of rules which evaluate upon event interception. A logical disjunction is applied between defined rule sets (logical 'or')."]
    #[serde(
        rename = "ruleSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rule_sets: Vec<AutomationRuleSet>,
}
impl AutomationSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automation_source {
    use super::*;
    #[doc = "A valid event source type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventSource")]
    pub enum EventSource {
        Assessments,
        AssessmentsSnapshot,
        SubAssessments,
        SubAssessmentsSnapshot,
        Alerts,
        SecureScores,
        SecureScoresSnapshot,
        SecureScoreControls,
        SecureScoreControlsSnapshot,
        RegulatoryComplianceAssessment,
        RegulatoryComplianceAssessmentSnapshot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Assessments => serializer.serialize_unit_variant("EventSource", 0u32, "Assessments"),
                Self::AssessmentsSnapshot => serializer.serialize_unit_variant("EventSource", 1u32, "AssessmentsSnapshot"),
                Self::SubAssessments => serializer.serialize_unit_variant("EventSource", 2u32, "SubAssessments"),
                Self::SubAssessmentsSnapshot => serializer.serialize_unit_variant("EventSource", 3u32, "SubAssessmentsSnapshot"),
                Self::Alerts => serializer.serialize_unit_variant("EventSource", 4u32, "Alerts"),
                Self::SecureScores => serializer.serialize_unit_variant("EventSource", 5u32, "SecureScores"),
                Self::SecureScoresSnapshot => serializer.serialize_unit_variant("EventSource", 6u32, "SecureScoresSnapshot"),
                Self::SecureScoreControls => serializer.serialize_unit_variant("EventSource", 7u32, "SecureScoreControls"),
                Self::SecureScoreControlsSnapshot => serializer.serialize_unit_variant("EventSource", 8u32, "SecureScoreControlsSnapshot"),
                Self::RegulatoryComplianceAssessment => {
                    serializer.serialize_unit_variant("EventSource", 9u32, "RegulatoryComplianceAssessment")
                }
                Self::RegulatoryComplianceAssessmentSnapshot => {
                    serializer.serialize_unit_variant("EventSource", 10u32, "RegulatoryComplianceAssessmentSnapshot")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A rule which is evaluated upon event interception. The rule is configured by comparing a specific value from the event model to an expected value. This comparison is done by using one of the supported operators set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationTriggeringRule {
    #[doc = "The JPath of the entity model property that should be checked."]
    #[serde(rename = "propertyJPath", default, skip_serializing_if = "Option::is_none")]
    pub property_j_path: Option<String>,
    #[doc = "The data type of the compared operands (string, integer, floating point number or a boolean [true/false]]"]
    #[serde(rename = "propertyType", default, skip_serializing_if = "Option::is_none")]
    pub property_type: Option<automation_triggering_rule::PropertyType>,
    #[doc = "The expected value."]
    #[serde(rename = "expectedValue", default, skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<String>,
    #[doc = "A valid comparer operator to use. A case-insensitive comparison will be applied for String PropertyType."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<automation_triggering_rule::Operator>,
}
impl AutomationTriggeringRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automation_triggering_rule {
    use super::*;
    #[doc = "The data type of the compared operands (string, integer, floating point number or a boolean [true/false]]"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PropertyType")]
    pub enum PropertyType {
        String,
        Integer,
        Number,
        Boolean,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PropertyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PropertyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PropertyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::String => serializer.serialize_unit_variant("PropertyType", 0u32, "String"),
                Self::Integer => serializer.serialize_unit_variant("PropertyType", 1u32, "Integer"),
                Self::Number => serializer.serialize_unit_variant("PropertyType", 2u32, "Number"),
                Self::Boolean => serializer.serialize_unit_variant("PropertyType", 3u32, "Boolean"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A valid comparer operator to use. A case-insensitive comparison will be applied for String PropertyType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        GreaterThan,
        GreaterThanOrEqualTo,
        LesserThan,
        LesserThanOrEqualTo,
        NotEquals,
        Contains,
        StartsWith,
        EndsWith,
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
                Self::GreaterThanOrEqualTo => serializer.serialize_unit_variant("Operator", 2u32, "GreaterThanOrEqualTo"),
                Self::LesserThan => serializer.serialize_unit_variant("Operator", 3u32, "LesserThan"),
                Self::LesserThanOrEqualTo => serializer.serialize_unit_variant("Operator", 4u32, "LesserThanOrEqualTo"),
                Self::NotEquals => serializer.serialize_unit_variant("Operator", 5u32, "NotEquals"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 6u32, "Contains"),
                Self::StartsWith => serializer.serialize_unit_variant("Operator", 7u32, "StartsWith"),
                Self::EndsWith => serializer.serialize_unit_variant("Operator", 8u32, "EndsWith"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The security automation model state property bag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationValidationStatus {
    #[doc = "Indicates whether the model is valid or not."]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "The validation message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AutomationValidationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AWS cloud account connector based assume role, the role enables delegating access to your AWS resources. The role is composed of role Amazon Resource Name (ARN) and external ID. For more details, refer to <a href=\"https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user.html\">Creating a Role to Delegate Permissions to an IAM User (write only)</a>"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwAssumeRoleAuthenticationDetailsProperties {
    #[serde(flatten)]
    pub authentication_details_properties: AuthenticationDetailsProperties,
    #[doc = "The ID of the cloud account"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Assumed role ID is an identifier that you can use to create temporary security credentials."]
    #[serde(rename = "awsAssumeRoleArn")]
    pub aws_assume_role_arn: String,
    #[doc = "A unique identifier that is required when you assume a role in another account."]
    #[serde(rename = "awsExternalId")]
    pub aws_external_id: String,
}
impl AwAssumeRoleAuthenticationDetailsProperties {
    pub fn new(
        authentication_details_properties: AuthenticationDetailsProperties,
        aws_assume_role_arn: String,
        aws_external_id: String,
    ) -> Self {
        Self {
            authentication_details_properties,
            account_id: None,
            aws_assume_role_arn,
            aws_external_id,
        }
    }
}
#[doc = "AWS cloud account connector based credentials, the credentials is composed of access key ID and secret key, for more details, refer to <a href=\"https://docs.aws.amazon.com/IAM/latest/UserGuide/id_users_create.html\">Creating an IAM User in Your AWS Account (write only)</a>"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCredsAuthenticationDetailsProperties {
    #[serde(flatten)]
    pub authentication_details_properties: AuthenticationDetailsProperties,
    #[doc = "The ID of the cloud account"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Public key element of the AWS credential object (write only)"]
    #[serde(rename = "awsAccessKeyId")]
    pub aws_access_key_id: String,
    #[doc = "Secret key element of the AWS credential object (write only)"]
    #[serde(rename = "awsSecretAccessKey")]
    pub aws_secret_access_key: String,
}
impl AwsCredsAuthenticationDetailsProperties {
    pub fn new(
        authentication_details_properties: AuthenticationDetailsProperties,
        aws_access_key_id: String,
        aws_secret_access_key: String,
    ) -> Self {
        Self {
            authentication_details_properties,
            account_id: None,
            aws_access_key_id,
            aws_secret_access_key,
        }
    }
}
#[doc = "The AWS connector environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsEnvironmentData {
    #[doc = "The AWS organization data"]
    #[serde(rename = "organizationalData", default, skip_serializing_if = "Option::is_none")]
    pub organizational_data: Option<AwsOrganizationalDataUnion>,
    #[doc = "list of regions to scan"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub regions: Vec<String>,
    #[doc = "The AWS account name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Scan interval in hours (value should be between 1-hour to 24-hours)"]
    #[serde(rename = "scanInterval", default, skip_serializing_if = "Option::is_none")]
    pub scan_interval: Option<i64>,
}
impl AwsEnvironmentData {
    pub fn new() -> Self {
        Self {
            organizational_data: None,
            regions: Vec::new(),
            account_name: None,
            scan_interval: None,
        }
    }
}
#[doc = "The multi cloud account's membership type in the organization"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "organizationMembershipType")]
pub enum AwsOrganizationalDataUnion {
    Organization(AwsOrganizationalDataMaster),
    Member(AwsOrganizationalDataMember),
}
#[doc = "The AWS organization data for the master account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsOrganizationalDataMaster {
    #[doc = "If the multi cloud account is of membership type organization, this will be the name of the onboarding stackset"]
    #[serde(rename = "stacksetName", default, skip_serializing_if = "Option::is_none")]
    pub stackset_name: Option<String>,
    #[doc = "If the multi cloud account is of membership type organization, list of accounts excluded from offering"]
    #[serde(
        rename = "excludedAccountIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_account_ids: Vec<String>,
}
impl AwsOrganizationalDataMaster {
    pub fn new() -> Self {
        Self {
            stackset_name: None,
            excluded_account_ids: Vec::new(),
        }
    }
}
#[doc = "The AWS organization data for the member account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsOrganizationalDataMember {
    #[doc = "If the multi cloud account is not of membership type organization, this will be the ID of the account's parent"]
    #[serde(rename = "parentHierarchyId", default, skip_serializing_if = "Option::is_none")]
    pub parent_hierarchy_id: Option<String>,
}
impl AwsOrganizationalDataMember {
    pub fn new() -> Self {
        Self { parent_hierarchy_id: None }
    }
}
#[doc = "Azure DevOps Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrg {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Azure DevOps Organization properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDevOpsOrgProperties>,
}
impl AzureDevOpsOrg {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrgListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureDevOpsOrg>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureDevOpsOrgListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureDevOpsOrgListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Organization properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrgProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Configuration payload for PR Annotations."]
    #[serde(rename = "actionableRemediation", default, skip_serializing_if = "Option::is_none")]
    pub actionable_remediation: Option<ActionableRemediation>,
}
impl AzureDevOpsOrgProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureDevOps Org Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrganizationConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "AzureDevOps Project Inventory Configuration.\r\nDictionary of AzureDevOps project name to desired project configuration.\r\nIf AutoDiscovery is Enabled, this field should be empty or null."]
    #[serde(rename = "projectConfigs", default, skip_serializing_if = "Option::is_none")]
    pub project_configs: Option<serde_json::Value>,
}
impl AzureDevOpsOrganizationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProject {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Azure DevOps Project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDevOpsProjectProperties>,
}
impl AzureDevOpsProject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureDevOps Project Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProjectConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "AzureDevOps Repository Inventory Configuration.\r\nDictionary of AzureDevOps repository name to desired repository configuration.\r\nIf AutoDiscovery is Enabled, this field should be null or empty."]
    #[serde(rename = "repositoryConfigs", default, skip_serializing_if = "Option::is_none")]
    pub repository_configs: Option<serde_json::Value>,
}
impl AzureDevOpsProjectConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProjectListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureDevOpsProject>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureDevOpsProjectListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureDevOpsProjectListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProjectProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets parent Azure DevOps Organization name."]
    #[serde(rename = "parentOrgName", default, skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    #[doc = "Gets or sets Azure DevOps Project id."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Configuration payload for PR Annotations."]
    #[serde(rename = "actionableRemediation", default, skip_serializing_if = "Option::is_none")]
    pub actionable_remediation: Option<ActionableRemediation>,
}
impl AzureDevOpsProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Repository resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsRepository {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Azure DevOps Repository properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDevOpsRepositoryProperties>,
}
impl AzureDevOpsRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsRepositoryListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureDevOpsRepository>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureDevOpsRepositoryListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureDevOpsRepositoryListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Repository properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsRepositoryProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets parent Azure DevOps Organization name."]
    #[serde(rename = "parentOrgName", default, skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    #[doc = "Gets or sets parent Azure DevOps Project name."]
    #[serde(rename = "parentProjectName", default, skip_serializing_if = "Option::is_none")]
    pub parent_project_name: Option<String>,
    #[doc = "Gets or sets Azure DevOps Repository id."]
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[doc = "Gets or sets Azure DevOps Repository url."]
    #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "Gets or sets Azure DevOps repository visibility, whether it is public or private etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Configuration payload for PR Annotations."]
    #[serde(rename = "actionableRemediation", default, skip_serializing_if = "Option::is_none")]
    pub actionable_remediation: Option<ActionableRemediation>,
}
impl AzureDevOpsRepositoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The AzureDevOps scope connector's environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsScopeEnvironmentData {}
impl AzureDevOpsScopeEnvironmentData {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Azure resource identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceIdentifier {
    #[doc = "ARM resource identifier for the cloud resource being alerted on"]
    #[serde(rename = "azureResourceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_id: Option<String>,
}
impl AzureResourceIdentifier {
    pub fn new() -> Self {
        Self { azure_resource_id: None }
    }
}
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceLink {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl AzureResourceLink {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AzureResourceLinks = Vec<AzureResourceLink>;
#[doc = "A vulnerability assessments setting on Azure servers in the defined scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureServersSetting {
    #[serde(flatten)]
    pub server_vulnerability_assessments_setting: ServerVulnerabilityAssessmentsSetting,
    #[doc = "Describes the vulnerability assessments setting properties on Azure servers in the defined scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerVulnerabilityAssessmentsAzureSettingProperties>,
}
impl AzureServersSetting {
    pub fn new(server_vulnerability_assessments_setting: ServerVulnerabilityAssessmentsSetting) -> Self {
        Self {
            server_vulnerability_assessments_setting,
            properties: None,
        }
    }
}
#[doc = "Describes an Azure resource with location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrackedResourceLocation {
    #[doc = "Location where the resource is stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AzureTrackedResourceLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base Resource Inventory configuration changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseResourceConfiguration {
    #[doc = "Onboarding states."]
    #[serde(rename = "desiredOnboardingState", default, skip_serializing_if = "Option::is_none")]
    pub desired_onboarding_state: Option<DesiredOnboardingState>,
}
impl BaseResourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Pre-configured sensitive information type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuiltInInfoType {
    #[doc = "Display name of the info type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of the info type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Category of the built-in info type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl BuiltInInfoType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert Simulator supported bundles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BundleType")]
pub enum BundleType {
    AppServices,
    #[serde(rename = "DNS")]
    Dns,
    KeyVaults,
    KubernetesService,
    ResourceManager,
    SqlServers,
    StorageAccounts,
    VirtualMachines,
    CosmosDbs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BundleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BundleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BundleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AppServices => serializer.serialize_unit_variant("BundleType", 0u32, "AppServices"),
            Self::Dns => serializer.serialize_unit_variant("BundleType", 1u32, "DNS"),
            Self::KeyVaults => serializer.serialize_unit_variant("BundleType", 2u32, "KeyVaults"),
            Self::KubernetesService => serializer.serialize_unit_variant("BundleType", 3u32, "KubernetesService"),
            Self::ResourceManager => serializer.serialize_unit_variant("BundleType", 4u32, "ResourceManager"),
            Self::SqlServers => serializer.serialize_unit_variant("BundleType", 5u32, "SqlServers"),
            Self::StorageAccounts => serializer.serialize_unit_variant("BundleType", 6u32, "StorageAccounts"),
            Self::VirtualMachines => serializer.serialize_unit_variant("BundleType", 7u32, "VirtualMachines"),
            Self::CosmosDbs => serializer.serialize_unit_variant("BundleType", 8u32, "CosmosDbs"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "CVE details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cve {
    #[doc = "CVE title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Link url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl Cve {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CVSS details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cvss {
    #[doc = "CVSS base"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<f64>,
}
impl Cvss {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Severity level per category configuration for PR Annotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CategoryConfiguration {
    #[doc = "Gets or sets minimum severity level for a given category."]
    #[serde(rename = "minimumSeverityLevel", default, skip_serializing_if = "Option::is_none")]
    pub minimum_severity_level: Option<String>,
    #[doc = "Rule categories.\r\nCode - code scanning results.\r\nArtifact scanning results.\r\nDependencies scanning results.\r\nIaC results.\r\nSecrets scanning results.\r\nContainer scanning results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<RuleCategory>,
}
impl CategoryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a security solution which sends CEF logs to an OMS workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CefExternalSecuritySolution {
    #[serde(flatten)]
    pub external_security_solution: ExternalSecuritySolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CefSolutionProperties>,
}
impl CefExternalSecuritySolution {
    pub fn new(external_security_solution: ExternalSecuritySolution) -> Self {
        Self {
            external_security_solution,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CefSolutionProperties {
    #[serde(flatten)]
    pub external_security_solution_properties: ExternalSecuritySolutionProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(rename = "lastEventReceived", default, skip_serializing_if = "Option::is_none")]
    pub last_event_received: Option<String>,
}
impl CefSolutionProperties {
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
#[doc = "Compliance of a scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compliance {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Compliance score (percentage) of a Subscription is a sum of all Resources' Compliances under the given Subscription. A Resource Compliance is defined as the compliant ('healthy') Policy Definitions out of all Policy Definitions applicable to a given resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComplianceProperties>,
}
impl Compliance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Compliance objects response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceList {
    #[doc = "List of Compliance objects"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Compliance>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ComplianceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComplianceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Compliance score (percentage) of a Subscription is a sum of all Resources' Compliances under the given Subscription. A Resource Compliance is defined as the compliant ('healthy') Policy Definitions out of all Policy Definitions applicable to a given resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceProperties {
    #[doc = "The timestamp when the Compliance calculation was conducted."]
    #[serde(rename = "assessmentTimestampUtcDate", default, with = "azure_core::date::rfc3339::option")]
    pub assessment_timestamp_utc_date: Option<time::OffsetDateTime>,
    #[doc = "The resource count of the given subscription for which the Compliance calculation was conducted (needed for Management Group Compliance calculation)."]
    #[serde(rename = "resourceCount", default, skip_serializing_if = "Option::is_none")]
    pub resource_count: Option<i64>,
    #[doc = "An array of segment, which is the actually the compliance assessment."]
    #[serde(
        rename = "assessmentResult",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assessment_result: Vec<ComplianceSegment>,
}
impl ComplianceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "a compliance result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Compliance result data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComplianceResultProperties>,
}
impl ComplianceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of compliance results response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceResultList {
    #[doc = "List of compliance results"]
    pub value: Vec<ComplianceResult>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ComplianceResultList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComplianceResultList {
    pub fn new(value: Vec<ComplianceResult>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Compliance result data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceResultProperties {
    #[doc = "The status of the resource regarding a single assessment"]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<compliance_result_properties::ResourceStatus>,
}
impl ComplianceResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compliance_result_properties {
    use super::*;
    #[doc = "The status of the resource regarding a single assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceStatus")]
    pub enum ResourceStatus {
        Healthy,
        NotApplicable,
        OffByPolicy,
        NotHealthy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("ResourceStatus", 0u32, "Healthy"),
                Self::NotApplicable => serializer.serialize_unit_variant("ResourceStatus", 1u32, "NotApplicable"),
                Self::OffByPolicy => serializer.serialize_unit_variant("ResourceStatus", 2u32, "OffByPolicy"),
                Self::NotHealthy => serializer.serialize_unit_variant("ResourceStatus", 3u32, "NotHealthy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A segment of a compliance assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceSegment {
    #[doc = "The segment type, e.g. compliant, non-compliance, insufficient coverage, N/A, etc."]
    #[serde(rename = "segmentType", default, skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    #[doc = "The size (%) of the segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}
impl ComplianceSegment {
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
#[doc = "The configuration status of the machines group or machine or rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConfigurationStatus {
    Configured,
    NotConfigured,
    InProgress,
    Failed,
    NoStatus,
}
#[doc = "Describes the allowed inbound and outbound traffic of an Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectableResource {
    #[doc = "The Azure resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The list of Azure resources that the resource has inbound allowed connection from"]
    #[serde(
        rename = "inboundConnectedResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inbound_connected_resources: Vec<ConnectedResource>,
    #[doc = "The list of Azure resources that the resource has outbound allowed connection to"]
    #[serde(
        rename = "outboundConnectedResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_connected_resources: Vec<ConnectedResource>,
}
impl ConnectableResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a connected resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedResource {
    #[doc = "The Azure resource id of the connected resource"]
    #[serde(rename = "connectedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub connected_resource_id: Option<String>,
    #[doc = "The allowed tcp ports"]
    #[serde(rename = "tcpPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    #[doc = "The allowed udp ports"]
    #[serde(rename = "udpPorts", default, skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
}
impl ConnectedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedWorkspace {
    #[doc = "Azure resource ID of the connected OMS workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ConnectedWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inbound connection from an ip that isn't allowed. Allow list consists of ipv4 or ipv6 range in CIDR notation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionFromIpNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
impl ConnectionFromIpNotAllowed {
    pub fn new(allowlist_custom_alert_rule: AllowlistCustomAlertRule) -> Self {
        Self {
            allowlist_custom_alert_rule,
        }
    }
}
#[doc = "Outbound connection to an ip that isn't allowed. Allow list consists of ipv4 or ipv6 range in CIDR notation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionToIpNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
impl ConnectionToIpNotAllowed {
    pub fn new(allowlist_custom_alert_rule: AllowlistCustomAlertRule) -> Self {
        Self {
            allowlist_custom_alert_rule,
        }
    }
}
#[doc = "The connector setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of a connector setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectorSettingProperties>,
}
impl ConnectorSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "For a subscription, list of all cloud account connectors and their settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorSettingList {
    #[doc = "List of all the cloud account connector settings"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConnectorSetting>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectorSettingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConnectorSettingList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a connector setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorSettingProperties {
    #[doc = "Settings for hybrid compute management"]
    #[serde(rename = "hybridComputeSettings", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_compute_settings: Option<HybridComputeSettingsProperties>,
    #[doc = "Settings for cloud authentication management"]
    #[serde(rename = "authenticationDetails", default, skip_serializing_if = "Option::is_none")]
    pub authentication_details: Option<AuthenticationDetailsPropertiesUnion>,
}
impl ConnectorSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional context fields for container registry Vulnerability assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryVulnerabilityProperties {
    #[doc = "Vulnerability Type. e.g: Vulnerability, Potential Vulnerability, Information Gathered, Vulnerability"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Dictionary from cvss version to cvss details object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss: Option<serde_json::Value>,
    #[doc = "Indicates whether a patch is available or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patchable: Option<bool>,
    #[doc = "List of CVEs"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cve: Vec<Cve>,
    #[doc = "Published time"]
    #[serde(rename = "publishedTime", default, with = "azure_core::date::rfc3339::option")]
    pub published_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "vendorReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vendor_references: Vec<VendorReference>,
    #[doc = "Name of the repository which the vulnerable image belongs to"]
    #[serde(rename = "repositoryName", default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[doc = "Digest of the vulnerable image"]
    #[serde(rename = "imageDigest", default, skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
}
impl ContainerRegistryVulnerabilityProperties {
    pub fn new() -> Self {
        Self {
            type_: None,
            cvss: None,
            patchable: None,
            cve: Vec::new(),
            published_time: None,
            vendor_references: Vec::new(),
            repository_name: None,
            image_digest: None,
        }
    }
}
#[doc = "A custom alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomAlertRule {
    #[doc = "The display name of the custom alert."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the custom alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Status of the custom alert."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl CustomAlertRule {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            display_name: None,
            description: None,
            is_enabled,
        }
    }
}
#[doc = "The type of the custom alert rule."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ruleType")]
pub enum CustomAlertRuleUnion {}
#[doc = "Custom entity store assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomEntityStoreAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "describes the custom entity store assignment properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomEntityStoreAssignmentProperties>,
}
impl CustomEntityStoreAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes the custom entity store assignment properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomEntityStoreAssignmentProperties {
    #[doc = "The principal assigned with entity store. Format of principal is: [AAD type]=[PrincipalObjectId];[TenantId]"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[doc = "The link to entity store database."]
    #[serde(rename = "entityStoreDatabaseLink", default, skip_serializing_if = "Option::is_none")]
    pub entity_store_database_link: Option<String>,
}
impl CustomEntityStoreAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes the custom entity store assignment request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomEntityStoreAssignmentRequest {
    #[doc = "describes properties of custom entity store assignment request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomEntityStoreAssignmentRequestProperties>,
}
impl CustomEntityStoreAssignmentRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes properties of custom entity store assignment request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomEntityStoreAssignmentRequestProperties {
    #[doc = "The principal assigned with entity store. If not provided, will use caller principal. Format of principal is: [AAD type]=[PrincipalObjectId];[TenantId]"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
}
impl CustomEntityStoreAssignmentRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of custom entity store assignments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomEntityStoreAssignmentsListResult {
    #[doc = "Collection of custom entity store assignments"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CustomEntityStoreAssignment>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomEntityStoreAssignmentsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CustomEntityStoreAssignmentsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data export setting properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettingProperties {
    #[doc = "Is the data export setting enabled"]
    pub enabled: bool,
}
impl DataExportSettingProperties {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[doc = "Represents a data export setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettings {
    #[serde(flatten)]
    pub setting: Setting,
    #[doc = "The data export setting properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportSettingProperties>,
}
impl DataExportSettings {
    pub fn new(setting: Setting) -> Self {
        Self { setting, properties: None }
    }
}
#[doc = "The Defender for Storage resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefenderForStorageSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Defender for Storage resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DefenderForStorageSettingProperties>,
}
impl DefenderForStorageSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defender for Storage resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefenderForStorageSettingProperties {
    #[doc = "Indicates whether Defender for Storage is enabled on this storage account."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Properties of Malware Scanning."]
    #[serde(rename = "malwareScanning", default, skip_serializing_if = "Option::is_none")]
    pub malware_scanning: Option<MalwareScanningProperties>,
    #[doc = "Properties of Sensitive Data Discovery."]
    #[serde(rename = "sensitiveDataDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub sensitive_data_discovery: Option<SensitiveDataDiscoveryProperties>,
    #[doc = "Indicates whether the settings defined for this storage account should override the settings defined for the subscription."]
    #[serde(rename = "overrideSubscriptionLevelSettings", default, skip_serializing_if = "Option::is_none")]
    pub override_subscription_level_settings: Option<bool>,
}
impl DefenderForStorageSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom alert rule that checks if a value (depends on the custom alert type) is denied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DenylistCustomAlertRule {
    #[serde(flatten)]
    pub list_custom_alert_rule: ListCustomAlertRule,
    #[doc = "The values to deny. The format of the values depends on the rule type."]
    #[serde(rename = "denylistValues")]
    pub denylist_values: Vec<String>,
}
impl DenylistCustomAlertRule {
    pub fn new(list_custom_alert_rule: ListCustomAlertRule, denylist_values: Vec<String>) -> Self {
        Self {
            list_custom_alert_rule,
            denylist_values,
        }
    }
}
#[doc = "Onboarding states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DesiredOnboardingState")]
pub enum DesiredOnboardingState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DesiredOnboardingState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DesiredOnboardingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DesiredOnboardingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("DesiredOnboardingState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("DesiredOnboardingState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "DevOps Configuration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "DevOps Configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevOpsConfigurationProperties>,
}
impl DevOpsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsConfigurationListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevOpsConfiguration>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevOpsConfigurationListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DevOpsConfigurationListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DevOps Configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsConfigurationProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Authorization payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "List of top-level inventory to select when AutoDiscovery is disabled.\r\nThis field is ignored when AutoDiscovery is enabled."]
    #[serde(
        rename = "topLevelInventoryList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_level_inventory_list: Vec<String>,
}
impl DevOpsConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevOpsProvisioningState")]
pub enum DevOpsProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Pending,
    PendingDeletion,
    DeletionSuccess,
    DeletionFailure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevOpsProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevOpsProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevOpsProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DevOpsProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DevOpsProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DevOpsProvisioningState", 2u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("DevOpsProvisioningState", 3u32, "Pending"),
            Self::PendingDeletion => serializer.serialize_unit_variant("DevOpsProvisioningState", 4u32, "PendingDeletion"),
            Self::DeletionSuccess => serializer.serialize_unit_variant("DevOpsProvisioningState", 5u32, "DeletionSuccess"),
            Self::DeletionFailure => serializer.serialize_unit_variant("DevOpsProvisioningState", 6u32, "DeletionFailure"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The device security group resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceSecurityGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes properties of a security group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeviceSecurityGroupProperties>,
}
impl DeviceSecurityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of device security groups"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceSecurityGroupList {
    #[doc = "List of device security group objects"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeviceSecurityGroup>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceSecurityGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceSecurityGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes properties of a security group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceSecurityGroupProperties {
    #[doc = "The list of custom alert threshold rules."]
    #[serde(
        rename = "thresholdRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threshold_rules: Vec<ThresholdCustomAlertRule>,
    #[doc = "The list of custom alert time-window rules."]
    #[serde(
        rename = "timeWindowRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub time_window_rules: Vec<TimeWindowCustomAlertRule>,
    #[doc = "The allow-list custom alert rules."]
    #[serde(
        rename = "allowlistRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowlist_rules: Vec<AllowlistCustomAlertRule>,
    #[doc = "The deny-list custom alert rules."]
    #[serde(
        rename = "denylistRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub denylist_rules: Vec<DenylistCustomAlertRule>,
}
impl DeviceSecurityGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Number of direct method invokes is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectMethodInvokesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl DirectMethodInvokesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredSecuritySolution {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    pub properties: DiscoveredSecuritySolutionProperties,
}
impl DiscoveredSecuritySolution {
    pub fn new(properties: DiscoveredSecuritySolutionProperties) -> Self {
        Self {
            resource: Resource::default(),
            location: Location::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoveredSecuritySolutionList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DiscoveredSecuritySolution>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiscoveredSecuritySolutionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiscoveredSecuritySolutionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredSecuritySolutionProperties {
    #[doc = "The security family of the discovered solution"]
    #[serde(rename = "securityFamily")]
    pub security_family: discovered_security_solution_properties::SecurityFamily,
    #[doc = "The security solutions' image offer"]
    pub offer: String,
    #[doc = "The security solutions' image publisher"]
    pub publisher: String,
    #[doc = "The security solutions' image sku"]
    pub sku: String,
}
impl DiscoveredSecuritySolutionProperties {
    pub fn new(
        security_family: discovered_security_solution_properties::SecurityFamily,
        offer: String,
        publisher: String,
        sku: String,
    ) -> Self {
        Self {
            security_family,
            offer,
            publisher,
            sku,
        }
    }
}
pub mod discovered_security_solution_properties {
    use super::*;
    #[doc = "The security family of the discovered solution"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecurityFamily")]
    pub enum SecurityFamily {
        Waf,
        Ngfw,
        SaasWaf,
        Va,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecurityFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecurityFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecurityFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Waf => serializer.serialize_unit_variant("SecurityFamily", 0u32, "Waf"),
                Self::Ngfw => serializer.serialize_unit_variant("SecurityFamily", 1u32, "Ngfw"),
                Self::SaasWaf => serializer.serialize_unit_variant("SecurityFamily", 2u32, "SaasWaf"),
                Self::Va => serializer.serialize_unit_variant("SecurityFamily", 3u32, "Va"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ETag {
    #[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ETag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Network Security Groups effective on a network interface"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectiveNetworkSecurityGroups {
    #[doc = "The Azure resource ID of the network interface"]
    #[serde(rename = "networkInterface", default, skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<String>,
    #[doc = "The Network Security Groups effective on the network interface"]
    #[serde(
        rename = "networkSecurityGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_security_groups: Vec<String>,
}
impl EffectiveNetworkSecurityGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The application control policy enforcement/protection mode of the machine group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnforcementMode {
    Audit,
    Enforce,
    None,
}
#[doc = "The machine supportability of Enforce feature"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnforcementSupport {
    Supported,
    NotSupported,
    Unknown,
}
#[doc = "The type of the environment data."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "environmentType")]
pub enum EnvironmentDataUnion {
    AwsAccount(AwsEnvironmentData),
    AzureDevOpsScope(AzureDevOpsScopeEnvironmentData),
    GcpProject(GcpProjectEnvironmentData),
    GithubScope(GithubScopeEnvironmentData),
    GitlabScope(GitlabScopeEnvironmentData),
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
#[doc = "A plan's extension properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[doc = "The extension name. Supported values are: <br><br>**AgentlessDiscoveryForKubernetes** - API-based discovery of information about Kubernetes cluster architecture, workload objects, and setup. Required for Kubernetes inventory, identity and network exposure detection, attack path analysis and risk hunting as part of the cloud security explorer.\r\nAvailable for CloudPosture plan.<br><br>**OnUploadMalwareScanning** - Limits the GB to be scanned per month for each storage account within the subscription. Once this limit reached on a given storage account, Blobs won't be scanned during current calendar month.\r\nAvailable for StorageAccounts plan.<br><br>**SensitiveDataDiscovery** - Sensitive data discovery identifies Blob storage container with sensitive data such as credentials, credit cards, and more, to help prioritize and investigate security events.\r\nAvailable for StorageAccounts and CloudPosture plans.<br><br>**ContainerRegistriesVulnerabilityAssessments** - Provides vulnerability management for images stored in your container registries.\r\nAvailable for CloudPosture and Containers plans."]
    pub name: String,
    #[doc = "Indicates whether the extension is enabled."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: extension::IsEnabled,
    #[doc = "Property values associated with the extension."]
    #[serde(rename = "additionalExtensionProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_extension_properties: Option<serde_json::Value>,
    #[doc = "A status describing the success/failure of the extension's enablement/disablement operation."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<OperationStatus>,
}
impl Extension {
    pub fn new(name: String, is_enabled: extension::IsEnabled) -> Self {
        Self {
            name,
            is_enabled,
            additional_extension_properties: None,
            operation_status: None,
        }
    }
}
pub mod extension {
    use super::*;
    #[doc = "Indicates whether the extension is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsEnabled")]
    pub enum IsEnabled {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsEnabled", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsEnabled", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents a security solution external to Microsoft Defender for Cloud which sends information to an OMS workspace and whose data is displayed by Microsoft Defender for Cloud."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalSecuritySolution {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub external_security_solution_kind: ExternalSecuritySolutionKind,
    #[serde(flatten)]
    pub location: Location,
}
impl ExternalSecuritySolution {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
            external_security_solution_kind: ExternalSecuritySolutionKind::default(),
            location: Location::default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ExternalSecuritySolutionUnion {
    #[serde(rename = "AAD")]
    Aad(AadExternalSecuritySolution),
    #[serde(rename = "ATA")]
    Ata(AtaExternalSecuritySolution),
    #[serde(rename = "CEF")]
    Cef(CefExternalSecuritySolution),
}
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalSecuritySolutionKind {
    #[doc = "The kind of the external solution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<external_security_solution_kind::Kind>,
}
impl ExternalSecuritySolutionKind {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod external_security_solution_kind {
    use super::*;
    #[doc = "The kind of the external solution"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "CEF")]
        Cef,
        #[serde(rename = "ATA")]
        Ata,
        #[serde(rename = "AAD")]
        Aad,
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
                Self::Cef => serializer.serialize_unit_variant("Kind", 0u32, "CEF"),
                Self::Ata => serializer.serialize_unit_variant("Kind", 1u32, "ATA"),
                Self::Aad => serializer.serialize_unit_variant("Kind", 2u32, "AAD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalSecuritySolutionList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExternalSecuritySolutionUnion>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExternalSecuritySolutionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ExternalSecuritySolutionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The solution properties (correspond to the solution kind)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalSecuritySolutionProperties {
    #[serde(rename = "deviceVendor", default, skip_serializing_if = "Option::is_none")]
    pub device_vendor: Option<String>,
    #[serde(rename = "deviceType", default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<ConnectedWorkspace>,
}
impl ExternalSecuritySolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Number of failed local logins is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedLocalLoginsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl FailedLocalLoginsNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "The type of the file (for Linux files - Executable is used)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    Exe,
    Dll,
    Msi,
    Script,
    Executable,
    Unknown,
}
#[doc = "Number of file uploads is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUploadsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl FileUploadsNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "GCP cloud account connector based service to service credentials, the credentials are composed of the organization ID and a JSON API key (write only)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpCredentialsDetailsProperties {
    #[serde(flatten)]
    pub authentication_details_properties: AuthenticationDetailsProperties,
    #[doc = "The organization ID of the GCP cloud account"]
    #[serde(rename = "organizationId")]
    pub organization_id: String,
    #[doc = "Type field of the API key (write only)"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Project ID field of the API key (write only)"]
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[doc = "Private key ID field of the API key (write only)"]
    #[serde(rename = "privateKeyId")]
    pub private_key_id: String,
    #[doc = "Private key field of the API key (write only)"]
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[doc = "Client email field of the API key (write only)"]
    #[serde(rename = "clientEmail")]
    pub client_email: String,
    #[doc = "Client ID field of the API key (write only)"]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Auth URI field of the API key (write only)"]
    #[serde(rename = "authUri")]
    pub auth_uri: String,
    #[doc = "Token URI field of the API key (write only)"]
    #[serde(rename = "tokenUri")]
    pub token_uri: String,
    #[doc = "Auth provider x509 certificate URL field of the API key (write only)"]
    #[serde(rename = "authProviderX509CertUrl")]
    pub auth_provider_x509_cert_url: String,
    #[doc = "Client x509 certificate URL field of the API key (write only)"]
    #[serde(rename = "clientX509CertUrl")]
    pub client_x509_cert_url: String,
}
impl GcpCredentialsDetailsProperties {
    pub fn new(
        authentication_details_properties: AuthenticationDetailsProperties,
        organization_id: String,
        type_: String,
        project_id: String,
        private_key_id: String,
        private_key: String,
        client_email: String,
        client_id: String,
        auth_uri: String,
        token_uri: String,
        auth_provider_x509_cert_url: String,
        client_x509_cert_url: String,
    ) -> Self {
        Self {
            authentication_details_properties,
            organization_id,
            type_,
            project_id,
            private_key_id,
            private_key,
            client_email,
            client_id,
            auth_uri,
            token_uri,
            auth_provider_x509_cert_url,
            client_x509_cert_url,
        }
    }
}
#[doc = "The multi cloud account's membership type in the organization"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "organizationMembershipType")]
pub enum GcpOrganizationalDataUnion {
    Member(GcpOrganizationalDataMember),
    Organization(GcpOrganizationalDataOrganization),
}
#[doc = "The gcpOrganization data for the member account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpOrganizationalDataMember {
    #[doc = "If the multi cloud account is not of membership type organization, this will be the ID of the project's parent"]
    #[serde(rename = "parentHierarchyId", default, skip_serializing_if = "Option::is_none")]
    pub parent_hierarchy_id: Option<String>,
    #[doc = "The GCP management project number from organizational onboarding"]
    #[serde(rename = "managementProjectNumber", default, skip_serializing_if = "Option::is_none")]
    pub management_project_number: Option<String>,
}
impl GcpOrganizationalDataMember {
    pub fn new() -> Self {
        Self {
            parent_hierarchy_id: None,
            management_project_number: None,
        }
    }
}
#[doc = "The gcpOrganization data for the parent account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpOrganizationalDataOrganization {
    #[doc = "If the multi cloud account is of membership type organization, list of accounts excluded from offering"]
    #[serde(
        rename = "excludedProjectNumbers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_project_numbers: Vec<String>,
    #[doc = "The service account email address which represents the organization level permissions container."]
    #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub service_account_email_address: Option<String>,
    #[doc = "The GCP workload identity provider id which represents the permissions required to auto provision security connectors"]
    #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_id: Option<String>,
    #[doc = "GCP organization name"]
    #[serde(rename = "organizationName", default, skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
}
impl GcpOrganizationalDataOrganization {
    pub fn new() -> Self {
        Self {
            excluded_project_numbers: Vec::new(),
            service_account_email_address: None,
            workload_identity_provider_id: None,
            organization_name: None,
        }
    }
}
#[doc = "The details about the project represented by the security connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GcpProjectDetails {
    #[doc = "The unique GCP Project number"]
    #[serde(rename = "projectNumber", default, skip_serializing_if = "Option::is_none")]
    pub project_number: Option<String>,
    #[doc = "The GCP Project id"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The GCP workload identity federation pool id"]
    #[serde(rename = "workloadIdentityPoolId", default, skip_serializing_if = "Option::is_none")]
    pub workload_identity_pool_id: Option<String>,
    #[doc = "GCP project name"]
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
}
impl GcpProjectDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The GCP project connector environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpProjectEnvironmentData {
    #[doc = "The gcpOrganization data"]
    #[serde(rename = "organizationalData", default, skip_serializing_if = "Option::is_none")]
    pub organizational_data: Option<GcpOrganizationalDataUnion>,
    #[doc = "The details about the project represented by the security connector"]
    #[serde(rename = "projectDetails", default, skip_serializing_if = "Option::is_none")]
    pub project_details: Option<GcpProjectDetails>,
    #[doc = "Scan interval in hours (value should be between 1-hour to 24-hours)"]
    #[serde(rename = "scanInterval", default, skip_serializing_if = "Option::is_none")]
    pub scan_interval: Option<i64>,
}
impl GcpProjectEnvironmentData {
    pub fn new() -> Self {
        Self {
            organizational_data: None,
            project_details: None,
            scan_interval: None,
        }
    }
}
#[doc = "A list with a single sensitivity settings resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSensitivitySettingsListResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GetSensitivitySettingsResponse>,
}
impl GetSensitivitySettingsListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data sensitivity settings for sensitive data discovery"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSensitivitySettingsResponse {
    #[doc = "The ID of the sensitivity settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the sensitivity settings"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the sensitivity settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The sensitivity settings properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<get_sensitivity_settings_response::Properties>,
}
impl GetSensitivitySettingsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod get_sensitivity_settings_response {
    use super::*;
    #[doc = "The sensitivity settings properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "List of selected sensitive info types' IDs."]
        #[serde(rename = "sensitiveInfoTypesIds", default, skip_serializing_if = "Option::is_none")]
        pub sensitive_info_types_ids: Option<SensitiveInfoTypesIds>,
        #[doc = "The order of the sensitivity threshold label. Any label at or above this order will be considered sensitive. If set to -1, sensitivity by labels is turned off"]
        #[serde(rename = "sensitivityThresholdLabelOrder", default, skip_serializing_if = "Option::is_none")]
        pub sensitivity_threshold_label_order: Option<f64>,
        #[doc = "The id of the sensitivity threshold label. Any label at or above this rank will be considered sensitive."]
        #[serde(rename = "sensitivityThresholdLabelId", default, skip_serializing_if = "Option::is_none")]
        pub sensitivity_threshold_label_id: Option<String>,
        #[doc = "Microsoft information protection built-in and custom information types, labels, and integration status."]
        #[serde(rename = "mipInformation", default, skip_serializing_if = "Option::is_none")]
        pub mip_information: Option<properties::MipInformation>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Microsoft information protection built-in and custom information types, labels, and integration status."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct MipInformation {
            #[doc = "Microsoft information protection integration status"]
            #[serde(rename = "mipIntegrationStatus", default, skip_serializing_if = "Option::is_none")]
            pub mip_integration_status: Option<MipIntegrationStatus>,
            #[doc = "List of Microsoft information protection sensitivity labels"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub labels: Option<Labels>,
            #[doc = "List of custom user-defined information types"]
            #[serde(
                rename = "customInfoTypes",
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub custom_info_types: Vec<InfoType>,
            #[doc = "List of pre-configured sensitive information types"]
            #[serde(
                rename = "builtInInfoTypes",
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub built_in_info_types: Vec<BuiltInInfoType>,
        }
        impl MipInformation {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "GitHub Owner resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwner {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitHub Owner properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitHubOwnerProperties>,
}
impl GitHubOwner {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Owner Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwnerConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "GitHub Repository Inventory Configuration.\r\nDictionary of GitHub repository name to desired repository configuration.\r\nIf AutoDiscovery is Enabled, this field should be null or empty."]
    #[serde(rename = "repositoryConfigs", default, skip_serializing_if = "Option::is_none")]
    pub repository_configs: Option<serde_json::Value>,
}
impl GitHubOwnerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwnerListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitHubOwner>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitHubOwnerListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitHubOwnerListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Owner properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwnerProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets GitHub Owner url."]
    #[serde(rename = "ownerUrl", default, skip_serializing_if = "Option::is_none")]
    pub owner_url: Option<String>,
    #[doc = "Gets or sets internal GitHub id."]
    #[serde(rename = "gitHubInternalId", default, skip_serializing_if = "Option::is_none")]
    pub git_hub_internal_id: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
}
impl GitHubOwnerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Repository resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubRepository {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitHub Repository properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitHubRepositoryProperties>,
}
impl GitHubRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubRepositoryListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitHubRepository>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitHubRepositoryListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitHubRepositoryListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Repository properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubRepositoryProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets GitHub Repository id.\r\n\r\nThis is a numeric id defined by Github.\r\nEg: \"123456\"."]
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[doc = "Gets or sets GitHub Repository name.\r\nEg: \"new-repo-1\"."]
    #[serde(rename = "repoName", default, skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    #[doc = "Gets or sets GitHub Full Name.\r\nRepository name, prefixed with Owner name.\r\nEg: \"my-org/new-repo-1\"."]
    #[serde(rename = "repoFullName", default, skip_serializing_if = "Option::is_none")]
    pub repo_full_name: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Gets or sets GitHub Repository url."]
    #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "Gets or sets parent GitHub Owner name."]
    #[serde(rename = "parentOwnerName", default, skip_serializing_if = "Option::is_none")]
    pub parent_owner_name: Option<String>,
}
impl GitHubRepositoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitLab Group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitLabGroupProperties>,
}
impl GitLabGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Group Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroupConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "GitLab Project Inventory Configuration.\r\nDictionary of GitLab fully-qualified project name to desired project configuration.\r\nIf AutoDiscovery is Enabled, this field should be null or empty."]
    #[serde(rename = "projectConfigs", default, skip_serializing_if = "Option::is_none")]
    pub project_configs: Option<serde_json::Value>,
}
impl GitLabGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroupListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitLabGroup>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitLabGroupListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitLabGroupListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroupProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets the fully-qualified name of the Group object.\r\n\r\nThis contains the entire namespace hierarchy where namespaces are separated by the '$' character."]
    #[serde(rename = "fullyQualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    #[doc = "Gets or sets the human readable fully-qualified name of the Group object.\r\n\r\nThis contains the entire namespace hierarchy as seen on GitLab UI where namespaces are separated by the '/' character."]
    #[serde(rename = "fullyQualifiedFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_friendly_name: Option<String>,
    #[doc = "Gets or sets the url of the GitLab Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
}
impl GitLabGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabProject {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitLab Project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitLabProjectProperties>,
}
impl GitLabProject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabProjectListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitLabProject>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitLabProjectListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitLabProjectListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabProjectProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets the fully-qualified name of the project object.\r\n\r\nThis contains the entire hierarchy where entities are separated by the '$' character."]
    #[serde(rename = "fullyQualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    #[doc = "Gets or sets the human readable fully-qualified name of the Project object.\r\n\r\nThis contains the entire namespace hierarchy as seen on GitLab UI where entities are separated by the '/' character."]
    #[serde(rename = "fullyQualifiedFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_friendly_name: Option<String>,
    #[doc = "Gets or sets the fully-qualified name of the project's parent group object.\r\n\r\nThis contains the entire hierarchy where namespaces are separated by the '$' character."]
    #[serde(rename = "fullyQualifiedParentGroupName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_parent_group_name: Option<String>,
    #[doc = "Gets or sets the url of the GitLab Project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
}
impl GitLabProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The github scope connector's environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubScopeEnvironmentData {}
impl GithubScopeEnvironmentData {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "The GitLab scope connector's environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitlabScopeEnvironmentData {}
impl GitlabScopeEnvironmentData {
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = "Governance assignment over a given scope"]
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
#[doc = "Describe the additional data of governance assignment - optional"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceAssignmentAdditionalData {
    #[doc = "Ticket number associated with this governance assignment"]
    #[serde(rename = "ticketNumber", default, skip_serializing_if = "Option::is_none")]
    pub ticket_number: Option<i32>,
    #[doc = "Ticket link associated with this governance assignment - for example: https://snow.com"]
    #[serde(rename = "ticketLink", default, skip_serializing_if = "Option::is_none")]
    pub ticket_link: Option<String>,
    #[doc = "The ticket status associated with this governance assignment - for example: Active"]
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
    #[doc = "Describe the additional data of governance assignment - optional"]
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
#[doc = "Page of a governance assignments list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceAssignmentsList {
    #[doc = "Collection of governance assignments in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GovernanceAssignment>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GovernanceAssignmentsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
#[doc = "Governance rule over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an governance rule"]
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
#[doc = "The governance email weekly notification configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleEmailNotification {
    #[doc = "Defines whether manager email notifications are disabled"]
    #[serde(rename = "disableManagerEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub disable_manager_email_notification: Option<bool>,
    #[doc = "Defines whether owner email notifications are disabled"]
    #[serde(rename = "disableOwnerEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub disable_owner_email_notification: Option<bool>,
}
impl GovernanceRuleEmailNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a governance rules list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleList {
    #[doc = "Collection of governance rules in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GovernanceRule>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GovernanceRuleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GovernanceRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The governance rule metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceRuleMetadata {
    #[doc = "Governance rule Created by object id (GUID)"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Governance rule creation date"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Governance rule last updated by object id (GUID)"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "Governance rule last update date"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
}
impl GovernanceRuleMetadata {
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
#[doc = "Describes properties of an governance rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GovernanceRuleProperties {
    #[doc = "The tenantId (GUID)"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Display name of the governance rule"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Description of the governance rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Governance rule remediation timeframe - this is the time that will affect on the grace-period duration e.g. 7.00:00:00 - means 7 days"]
    #[serde(rename = "remediationTimeframe", default, skip_serializing_if = "Option::is_none")]
    pub remediation_timeframe: Option<String>,
    #[doc = "Defines whether there is a grace period on the governance rule"]
    #[serde(rename = "isGracePeriod", default, skip_serializing_if = "Option::is_none")]
    pub is_grace_period: Option<bool>,
    #[doc = "The governance rule priority, priority to the lower number. Rules with the same priority on the same scope will not be allowed"]
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
    #[doc = "Excluded scopes, filter out the descendants of the scope (on management scopes)"]
    #[serde(
        rename = "excludedScopes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub excluded_scopes: Vec<String>,
    #[doc = "The governance rule conditionSets - see examples"]
    #[serde(rename = "conditionSets")]
    pub condition_sets: Vec<GovernanceRuleConditionSets>,
    #[doc = "Defines whether the rule is management scope rule (master connector as a single scope or management scope)"]
    #[serde(rename = "includeMemberScopes", default, skip_serializing_if = "Option::is_none")]
    pub include_member_scopes: Option<bool>,
    #[doc = "Describe the owner source of governance rule"]
    #[serde(rename = "ownerSource")]
    pub owner_source: GovernanceRuleOwnerSource,
    #[doc = "The governance email weekly notification configuration"]
    #[serde(rename = "governanceEmailNotification", default, skip_serializing_if = "Option::is_none")]
    pub governance_email_notification: Option<GovernanceRuleEmailNotification>,
    #[doc = "The governance rule metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GovernanceRuleMetadata>,
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
            tenant_id: None,
            display_name,
            description: None,
            remediation_timeframe: None,
            is_grace_period: None,
            rule_priority,
            is_disabled: None,
            rule_type,
            source_resource_type,
            excluded_scopes: Vec::new(),
            condition_sets,
            include_member_scopes: None,
            owner_source,
            governance_email_notification: None,
            metadata: None,
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
pub type GroupResourceId = String;
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
    pub resource_details: Option<ResourceDetailsUnion>,
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
    #[doc = "The affected defenders sub plans by unhealthy report"]
    #[serde(
        rename = "affectedDefendersSubPlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub affected_defenders_sub_plans: Vec<String>,
    #[doc = "Additional data for the given health report, this field can include more details on the resource and the health scenario."]
    #[serde(rename = "reportAdditionalData", default, skip_serializing_if = "Option::is_none")]
    pub report_additional_data: Option<serde_json::Value>,
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HealthReportsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Number of cloud to device messages (HTTP protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpC2dMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl HttpC2dMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of rejected cloud to device messages (HTTP protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpC2dRejectedMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl HttpC2dRejectedMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of device to cloud messages (HTTP protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpD2cMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl HttpD2cMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Settings for hybrid compute management"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputeSettingsProperties {
    #[doc = "State of the service principal and its secret"]
    #[serde(rename = "hybridComputeProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_compute_provisioning_state: Option<hybrid_compute_settings_properties::HybridComputeProvisioningState>,
    #[doc = "Whether or not to automatically install Azure Arc (hybrid compute) agents on machines"]
    #[serde(rename = "autoProvision")]
    pub auto_provision: hybrid_compute_settings_properties::AutoProvision,
    #[doc = "The name of the resource group where Arc (Hybrid Compute) connectors are connected."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "The location where the metadata of machines will be stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "For a non-Azure machine that is not connected directly to the internet, specify a proxy server that the non-Azure machine can use."]
    #[serde(rename = "proxyServer", default, skip_serializing_if = "Option::is_none")]
    pub proxy_server: Option<ProxyServerProperties>,
    #[doc = "Details of the service principal."]
    #[serde(rename = "servicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<ServicePrincipalProperties>,
}
impl HybridComputeSettingsProperties {
    pub fn new(auto_provision: hybrid_compute_settings_properties::AutoProvision) -> Self {
        Self {
            hybrid_compute_provisioning_state: None,
            auto_provision,
            resource_group_name: None,
            region: None,
            proxy_server: None,
            service_principal: None,
        }
    }
}
pub mod hybrid_compute_settings_properties {
    use super::*;
    #[doc = "State of the service principal and its secret"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridComputeProvisioningState")]
    pub enum HybridComputeProvisioningState {
        Valid,
        Invalid,
        Expired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridComputeProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridComputeProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridComputeProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Valid => serializer.serialize_unit_variant("HybridComputeProvisioningState", 0u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("HybridComputeProvisioningState", 1u32, "Invalid"),
                Self::Expired => serializer.serialize_unit_variant("HybridComputeProvisioningState", 2u32, "Expired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not to automatically install Azure Arc (hybrid compute) agents on machines"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoProvision")]
    pub enum AutoProvision {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoProvision {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoProvision {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoProvision {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("AutoProvision", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("AutoProvision", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Custom user-defined information type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InfoType {
    #[doc = "Display name of the info type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of the info type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Description of the info type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl InfoType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information type keyword."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationProtectionKeyword {
    #[doc = "The keyword pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Indicates whether the keyword is custom or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[doc = "Indicates whether the keyword can be applied on numeric types or not."]
    #[serde(rename = "canBeNumeric", default, skip_serializing_if = "Option::is_none")]
    pub can_be_numeric: Option<bool>,
    #[doc = "Indicates whether the keyword is excluded or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
}
impl InformationProtectionKeyword {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information protection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationProtectionPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes properties of an information protection policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InformationProtectionPolicyProperties>,
}
impl InformationProtectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information protection policies response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationProtectionPolicyList {
    #[doc = "List of information protection policies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InformationProtectionPolicy>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InformationProtectionPolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InformationProtectionPolicyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes properties of an information protection policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationProtectionPolicyProperties {
    #[doc = "Describes the last UTC time the policy was modified."]
    #[serde(rename = "lastModifiedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_utc: Option<time::OffsetDateTime>,
    #[doc = "Describes the version of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Dictionary of sensitivity labels."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[doc = "The sensitivity information types."]
    #[serde(rename = "informationTypes", default, skip_serializing_if = "Option::is_none")]
    pub information_types: Option<serde_json::Value>,
}
impl InformationProtectionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationType {
    #[doc = "The name of the information type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the information type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The order of the information type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "The recommended label id to be associated with this information type."]
    #[serde(rename = "recommendedLabelId", default, skip_serializing_if = "Option::is_none")]
    pub recommended_label_id: Option<String>,
    #[doc = "Indicates whether the information type is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Indicates whether the information type is custom or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    #[doc = "The information type keywords."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub keywords: Vec<InformationProtectionKeyword>,
}
impl InformationType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update Settings.\r\n\r\nEnabled - Resource should inherit configurations from parent.\r\nDisabled - Resource should not inherit configurations from parent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InheritFromParentState")]
pub enum InheritFromParentState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InheritFromParentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InheritFromParentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InheritFromParentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("InheritFromParentState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("InheritFromParentState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Security Solution Aggregated Alert information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedAlert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "IoT Security solution aggregated alert details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecurityAggregatedAlertProperties>,
}
impl IoTSecurityAggregatedAlert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of IoT Security solution aggregated alert data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedAlertList {
    #[doc = "List of aggregated alerts data."]
    pub value: Vec<IoTSecurityAggregatedAlert>,
    #[doc = "When there is too much alert data for one page, use this URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IoTSecurityAggregatedAlertList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IoTSecurityAggregatedAlertList {
    pub fn new(value: Vec<IoTSecurityAggregatedAlert>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "IoT Security solution aggregated alert details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedAlertProperties {
    #[doc = "Name of the alert type."]
    #[serde(rename = "alertType", default, skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[doc = "Display name of the alert type."]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "Date of detection."]
    #[serde(rename = "aggregatedDateUtc", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_date_utc: Option<String>,
    #[doc = "Name of the organization that raised the alert."]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "Assessed alert severity."]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_aggregated_alert_properties::ReportedSeverity>,
    #[doc = "Recommended steps for remediation."]
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[doc = "Description of the suspected vulnerability and meaning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Number of alerts occurrences within the aggregated time window."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Azure resource ID of the resource that received the alerts."]
    #[serde(rename = "effectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub effected_resource_type: Option<String>,
    #[doc = "The type of the alerted resource (Azure, Non-Azure)."]
    #[serde(rename = "systemSource", default, skip_serializing_if = "Option::is_none")]
    pub system_source: Option<String>,
    #[doc = "IoT Security solution alert response."]
    #[serde(rename = "actionTaken", default, skip_serializing_if = "Option::is_none")]
    pub action_taken: Option<String>,
    #[doc = "Log analytics query for getting the list of affected devices/alerts."]
    #[serde(rename = "logAnalyticsQuery", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_query: Option<String>,
    #[doc = "10 devices with the highest number of occurrences of this alert type, on this day."]
    #[serde(
        rename = "topDevicesList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_devices_list: Vec<serde_json::Value>,
}
impl IoTSecurityAggregatedAlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod io_t_security_aggregated_alert_properties {
    use super::*;
    #[doc = "Assessed alert severity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReportedSeverity")]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReportedSeverity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReportedSeverity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReportedSeverity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Informational => serializer.serialize_unit_variant("ReportedSeverity", 0u32, "Informational"),
                Self::Low => serializer.serialize_unit_variant("ReportedSeverity", 1u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("ReportedSeverity", 2u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("ReportedSeverity", 3u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "IoT Security solution recommendation information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedRecommendation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "IoT Security solution aggregated recommendation information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecurityAggregatedRecommendationProperties>,
}
impl IoTSecurityAggregatedRecommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of IoT Security solution aggregated recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedRecommendationList {
    #[doc = "List of aggregated recommendations data."]
    pub value: Vec<IoTSecurityAggregatedRecommendation>,
    #[doc = "When there is too much alert data for one page, use this URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IoTSecurityAggregatedRecommendationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IoTSecurityAggregatedRecommendationList {
    pub fn new(value: Vec<IoTSecurityAggregatedRecommendation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "IoT Security solution aggregated recommendation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedRecommendationProperties {
    #[doc = "Name of the recommendation."]
    #[serde(rename = "recommendationName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_name: Option<String>,
    #[doc = "Display name of the recommendation type."]
    #[serde(rename = "recommendationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_display_name: Option<String>,
    #[doc = "Description of the suspected vulnerability and meaning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Recommendation-type GUID."]
    #[serde(rename = "recommendationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type_id: Option<String>,
    #[doc = "Name of the organization that made the recommendation."]
    #[serde(rename = "detectedBy", default, skip_serializing_if = "Option::is_none")]
    pub detected_by: Option<String>,
    #[doc = "Recommended steps for remediation"]
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[doc = "Assessed recommendation severity."]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_aggregated_recommendation_properties::ReportedSeverity>,
    #[doc = "Number of healthy devices within the IoT Security solution."]
    #[serde(rename = "healthyDevices", default, skip_serializing_if = "Option::is_none")]
    pub healthy_devices: Option<i64>,
    #[doc = "Number of unhealthy devices within the IoT Security solution."]
    #[serde(rename = "unhealthyDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_device_count: Option<i64>,
    #[doc = "Log analytics query for getting the list of affected devices/alerts."]
    #[serde(rename = "logAnalyticsQuery", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_query: Option<String>,
}
impl IoTSecurityAggregatedRecommendationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod io_t_security_aggregated_recommendation_properties {
    use super::*;
    #[doc = "Assessed recommendation severity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReportedSeverity")]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReportedSeverity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReportedSeverity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReportedSeverity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Informational => serializer.serialize_unit_variant("ReportedSeverity", 0u32, "Informational"),
                Self::Low => serializer.serialize_unit_variant("ReportedSeverity", 1u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("ReportedSeverity", 2u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("ReportedSeverity", 3u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Statistical information about the number of alerts per device during last set number of days."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAlertedDevice {
    #[doc = "Device identifier."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Number of alerts raised for this device."]
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
}
impl IoTSecurityAlertedDevice {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type IoTSecurityAlertedDevicesList = Vec<IoTSecurityAlertedDevice>;
#[doc = "Statistical information about the number of alerts per alert type during last set number of days"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityDeviceAlert {
    #[doc = "Display name of the alert"]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "Assessed Alert severity."]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_device_alert::ReportedSeverity>,
    #[doc = "Number of alerts raised for this alert type."]
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
}
impl IoTSecurityDeviceAlert {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod io_t_security_device_alert {
    use super::*;
    #[doc = "Assessed Alert severity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReportedSeverity")]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReportedSeverity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReportedSeverity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReportedSeverity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Informational => serializer.serialize_unit_variant("ReportedSeverity", 0u32, "Informational"),
                Self::Low => serializer.serialize_unit_variant("ReportedSeverity", 1u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("ReportedSeverity", 2u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("ReportedSeverity", 3u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type IoTSecurityDeviceAlertsList = Vec<IoTSecurityDeviceAlert>;
#[doc = "Statistical information about the number of recommendations per device, per recommendation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityDeviceRecommendation {
    #[doc = "Display name of the recommendation."]
    #[serde(rename = "recommendationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_display_name: Option<String>,
    #[doc = "Assessed recommendation severity."]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_device_recommendation::ReportedSeverity>,
    #[doc = "Number of devices with this recommendation."]
    #[serde(rename = "devicesCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_count: Option<i64>,
}
impl IoTSecurityDeviceRecommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod io_t_security_device_recommendation {
    use super::*;
    #[doc = "Assessed recommendation severity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReportedSeverity")]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReportedSeverity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReportedSeverity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReportedSeverity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Informational => serializer.serialize_unit_variant("ReportedSeverity", 0u32, "Informational"),
                Self::Low => serializer.serialize_unit_variant("ReportedSeverity", 1u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("ReportedSeverity", 2u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("ReportedSeverity", 3u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type IoTSecurityDeviceRecommendationsList = Vec<IoTSecurityDeviceRecommendation>;
#[doc = "Security analytics of your IoT Security solution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecuritySolutionAnalyticsModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Security analytics properties of your IoT Security solution"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecuritySolutionAnalyticsModelProperties>,
}
impl IoTSecuritySolutionAnalyticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Security analytics of your IoT Security solution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionAnalyticsModelList {
    #[doc = "List of Security analytics of your IoT Security solution"]
    pub value: Vec<IoTSecuritySolutionAnalyticsModel>,
    #[doc = "When there is too much alert data for one page, use this URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IoTSecuritySolutionAnalyticsModelList {
    pub fn new(value: Vec<IoTSecuritySolutionAnalyticsModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Security analytics properties of your IoT Security solution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecuritySolutionAnalyticsModelProperties {
    #[doc = "IoT Security solution analytics severity metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<IoTSeverityMetrics>,
    #[doc = "Number of unhealthy devices within your IoT Security solution."]
    #[serde(rename = "unhealthyDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_device_count: Option<i64>,
    #[doc = "List of device metrics by the aggregation date."]
    #[serde(
        rename = "devicesMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub devices_metrics: Vec<serde_json::Value>,
    #[doc = "List of devices with open alerts including the count of alerts per device."]
    #[serde(rename = "topAlertedDevices", default, skip_serializing_if = "Option::is_none")]
    pub top_alerted_devices: Option<IoTSecurityAlertedDevicesList>,
    #[doc = "List of alerts with the count of raised alerts"]
    #[serde(rename = "mostPrevalentDeviceAlerts", default, skip_serializing_if = "Option::is_none")]
    pub most_prevalent_device_alerts: Option<IoTSecurityDeviceAlertsList>,
    #[doc = "List of aggregated recommendation data, per recommendation type, per device."]
    #[serde(rename = "mostPrevalentDeviceRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub most_prevalent_device_recommendations: Option<IoTSecurityDeviceRecommendationsList>,
}
impl IoTSecuritySolutionAnalyticsModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Security solution configuration and resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecuritySolutionModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Security Solution setting data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecuritySolutionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl IoTSecuritySolutionModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security Solution setting data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionProperties {
    #[doc = "Workspace resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[doc = "Resource display name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Status of the IoT Security solution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<io_t_security_solution_properties::Status>,
    #[doc = "List of additional options for exporting to workspace data."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub export: Vec<String>,
    #[doc = "Disabled data sources. Disabling these data sources compromises the system."]
    #[serde(
        rename = "disabledDataSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_data_sources: Vec<String>,
    #[doc = "IoT Hub resource IDs"]
    #[serde(rename = "iotHubs")]
    pub iot_hubs: Vec<String>,
    #[doc = "Properties of the IoT Security solution's user defined resources."]
    #[serde(rename = "userDefinedResources", default, skip_serializing_if = "Option::is_none")]
    pub user_defined_resources: Option<UserDefinedResourcesProperties>,
    #[doc = "List of resources that were automatically discovered as relevant to the security solution."]
    #[serde(
        rename = "autoDiscoveredResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub auto_discovered_resources: Vec<String>,
    #[doc = "List of the configuration status for each recommendation type."]
    #[serde(rename = "recommendationsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_configuration: Option<RecommendationConfigurationList>,
    #[doc = "Unmasked IP address logging status"]
    #[serde(rename = "unmaskedIpLoggingStatus", default, skip_serializing_if = "Option::is_none")]
    pub unmasked_ip_logging_status: Option<io_t_security_solution_properties::UnmaskedIpLoggingStatus>,
    #[doc = "List of additional workspaces"]
    #[serde(
        rename = "additionalWorkspaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_workspaces: Vec<AdditionalWorkspacesProperties>,
}
impl IoTSecuritySolutionProperties {
    pub fn new(display_name: String, iot_hubs: Vec<String>) -> Self {
        Self {
            workspace: None,
            display_name,
            status: None,
            export: Vec::new(),
            disabled_data_sources: Vec::new(),
            iot_hubs,
            user_defined_resources: None,
            auto_discovered_resources: Vec::new(),
            recommendations_configuration: None,
            unmasked_ip_logging_status: None,
            additional_workspaces: Vec::new(),
        }
    }
}
pub mod io_t_security_solution_properties {
    use super::*;
    #[doc = "Status of the IoT Security solution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
    #[doc = "Unmasked IP address logging status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UnmaskedIpLoggingStatus")]
    pub enum UnmaskedIpLoggingStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UnmaskedIpLoggingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UnmaskedIpLoggingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UnmaskedIpLoggingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("UnmaskedIpLoggingStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("UnmaskedIpLoggingStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for UnmaskedIpLoggingStatus {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "List of IoT Security solutions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionsList {
    #[doc = "List of IoT Security solutions"]
    pub value: Vec<IoTSecuritySolutionModel>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IoTSecuritySolutionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IoTSecuritySolutionsList {
    pub fn new(value: Vec<IoTSecuritySolutionModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "IoT Security solution analytics severity metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSeverityMetrics {
    #[doc = "Count of high severity alerts/recommendations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    #[doc = "Count of medium severity alerts/recommendations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<i64>,
    #[doc = "Count of low severity alerts/recommendations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
}
impl IoTSeverityMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JitNetworkAccessPoliciesList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<JitNetworkAccessPolicy>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JitNetworkAccessPoliciesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl JitNetworkAccessPoliciesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub kind: Kind,
    #[serde(flatten)]
    pub location: Location,
    pub properties: JitNetworkAccessPolicyProperties,
}
impl JitNetworkAccessPolicy {
    pub fn new(properties: JitNetworkAccessPolicyProperties) -> Self {
        Self {
            resource: Resource::default(),
            kind: Kind::default(),
            location: Location::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPolicyInitiatePort {
    pub number: PortNumber,
    #[doc = "Source of the allowed traffic. If omitted, the request will be for the source IP address of the initiate request."]
    #[serde(rename = "allowedSourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub allowed_source_address_prefix: Option<String>,
    #[doc = "The time to close the request in UTC"]
    #[serde(rename = "endTimeUtc", with = "azure_core::date::rfc3339")]
    pub end_time_utc: time::OffsetDateTime,
}
impl JitNetworkAccessPolicyInitiatePort {
    pub fn new(number: PortNumber, end_time_utc: time::OffsetDateTime) -> Self {
        Self {
            number,
            allowed_source_address_prefix: None,
            end_time_utc,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPolicyInitiateRequest {
    #[doc = "A list of virtual machines & ports to open access for"]
    #[serde(rename = "virtualMachines")]
    pub virtual_machines: Vec<JitNetworkAccessPolicyInitiateVirtualMachine>,
    #[doc = "The justification for making the initiate request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
}
impl JitNetworkAccessPolicyInitiateRequest {
    pub fn new(virtual_machines: Vec<JitNetworkAccessPolicyInitiateVirtualMachine>) -> Self {
        Self {
            virtual_machines,
            justification: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPolicyInitiateVirtualMachine {
    #[doc = "Resource ID of the virtual machine that is linked to this policy"]
    pub id: String,
    #[doc = "The ports to open for the resource with the `id`"]
    pub ports: Vec<JitNetworkAccessPolicyInitiatePort>,
}
impl JitNetworkAccessPolicyInitiateVirtualMachine {
    pub fn new(id: String, ports: Vec<JitNetworkAccessPolicyInitiatePort>) -> Self {
        Self { id, ports }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPolicyProperties {
    #[doc = "Configurations for Microsoft.Compute/virtualMachines resource type."]
    #[serde(rename = "virtualMachines")]
    pub virtual_machines: Vec<JitNetworkAccessPolicyVirtualMachine>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requests: Vec<JitNetworkAccessRequest>,
    #[doc = "Gets the provisioning state of the Just-in-Time policy."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl JitNetworkAccessPolicyProperties {
    pub fn new(virtual_machines: Vec<JitNetworkAccessPolicyVirtualMachine>) -> Self {
        Self {
            virtual_machines,
            requests: Vec::new(),
            provisioning_state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPolicyVirtualMachine {
    #[doc = "Resource ID of the virtual machine that is linked to this policy"]
    pub id: String,
    #[doc = "Port configurations for the virtual machine"]
    pub ports: Vec<JitNetworkAccessPortRule>,
    #[doc = "Public IP address of the Azure Firewall that is linked to this policy, if applicable"]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}
impl JitNetworkAccessPolicyVirtualMachine {
    pub fn new(id: String, ports: Vec<JitNetworkAccessPortRule>) -> Self {
        Self {
            id,
            ports,
            public_ip_address: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessPortRule {
    pub number: PortNumber,
    pub protocol: jit_network_access_port_rule::Protocol,
    #[doc = "Mutually exclusive with the \"allowedSourceAddressPrefixes\" parameter. Should be an IP address or CIDR, for example \"192.168.0.3\" or \"192.168.0.0/16\"."]
    #[serde(rename = "allowedSourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub allowed_source_address_prefix: Option<String>,
    #[doc = "Mutually exclusive with the \"allowedSourceAddressPrefix\" parameter."]
    #[serde(
        rename = "allowedSourceAddressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_source_address_prefixes: Vec<String>,
    #[doc = "Maximum duration requests can be made for. In ISO 8601 duration format. Minimum 5 minutes, maximum 1 day"]
    #[serde(rename = "maxRequestAccessDuration")]
    pub max_request_access_duration: String,
}
impl JitNetworkAccessPortRule {
    pub fn new(number: PortNumber, protocol: jit_network_access_port_rule::Protocol, max_request_access_duration: String) -> Self {
        Self {
            number,
            protocol,
            allowed_source_address_prefix: None,
            allowed_source_address_prefixes: Vec::new(),
            max_request_access_duration,
        }
    }
}
pub mod jit_network_access_port_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
        #[serde(rename = "*")]
        U2a,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "TCP"),
                Self::Udp => serializer.serialize_unit_variant("Protocol", 1u32, "UDP"),
                Self::U2a => serializer.serialize_unit_variant("Protocol", 2u32, "*"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessRequest {
    #[serde(rename = "virtualMachines")]
    pub virtual_machines: Vec<JitNetworkAccessRequestVirtualMachine>,
    #[doc = "The start time of the request in UTC"]
    #[serde(rename = "startTimeUtc", with = "azure_core::date::rfc3339")]
    pub start_time_utc: time::OffsetDateTime,
    #[doc = "The identity of the person who made the request"]
    pub requestor: String,
    #[doc = "The justification for making the initiate request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
}
impl JitNetworkAccessRequest {
    pub fn new(
        virtual_machines: Vec<JitNetworkAccessRequestVirtualMachine>,
        start_time_utc: time::OffsetDateTime,
        requestor: String,
    ) -> Self {
        Self {
            virtual_machines,
            start_time_utc,
            requestor,
            justification: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessRequestPort {
    pub number: PortNumber,
    #[doc = "Mutually exclusive with the \"allowedSourceAddressPrefixes\" parameter. Should be an IP address or CIDR, for example \"192.168.0.3\" or \"192.168.0.0/16\"."]
    #[serde(rename = "allowedSourceAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub allowed_source_address_prefix: Option<String>,
    #[doc = "Mutually exclusive with the \"allowedSourceAddressPrefix\" parameter."]
    #[serde(
        rename = "allowedSourceAddressPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_source_address_prefixes: Vec<String>,
    #[doc = "The date & time at which the request ends in UTC"]
    #[serde(rename = "endTimeUtc", with = "azure_core::date::rfc3339")]
    pub end_time_utc: time::OffsetDateTime,
    #[doc = "The status of the port"]
    pub status: jit_network_access_request_port::Status,
    #[doc = "A description of why the `status` has its value"]
    #[serde(rename = "statusReason")]
    pub status_reason: jit_network_access_request_port::StatusReason,
    #[doc = "The port which is mapped to this port's `number` in the Azure Firewall, if applicable"]
    #[serde(rename = "mappedPort", default, skip_serializing_if = "Option::is_none")]
    pub mapped_port: Option<i64>,
}
impl JitNetworkAccessRequestPort {
    pub fn new(
        number: PortNumber,
        end_time_utc: time::OffsetDateTime,
        status: jit_network_access_request_port::Status,
        status_reason: jit_network_access_request_port::StatusReason,
    ) -> Self {
        Self {
            number,
            allowed_source_address_prefix: None,
            allowed_source_address_prefixes: Vec::new(),
            end_time_utc,
            status,
            status_reason,
            mapped_port: None,
        }
    }
}
pub mod jit_network_access_request_port {
    use super::*;
    #[doc = "The status of the port"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Revoked,
        Initiated,
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
                Self::Revoked => serializer.serialize_unit_variant("Status", 0u32, "Revoked"),
                Self::Initiated => serializer.serialize_unit_variant("Status", 1u32, "Initiated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A description of why the `status` has its value"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StatusReason")]
    pub enum StatusReason {
        Expired,
        UserRequested,
        NewerRequestInitiated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StatusReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StatusReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StatusReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Expired => serializer.serialize_unit_variant("StatusReason", 0u32, "Expired"),
                Self::UserRequested => serializer.serialize_unit_variant("StatusReason", 1u32, "UserRequested"),
                Self::NewerRequestInitiated => serializer.serialize_unit_variant("StatusReason", 2u32, "NewerRequestInitiated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitNetworkAccessRequestVirtualMachine {
    #[doc = "Resource ID of the virtual machine that is linked to this policy"]
    pub id: String,
    #[doc = "The ports that were opened for the virtual machine"]
    pub ports: Vec<JitNetworkAccessRequestPort>,
}
impl JitNetworkAccessRequestVirtualMachine {
    pub fn new(id: String, ports: Vec<JitNetworkAccessRequestPort>) -> Self {
        Self { id, ports }
    }
}
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Kind {
    #[doc = "Kind of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Kind {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft information protection sensitivity label"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Label {
    #[doc = "The display name of the label"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the label"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Labels are ordered by sensitivity level. The higher the order of the label, the more sensitive it is."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
}
impl Label {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Labels = Vec<Label>;
#[doc = "A List custom alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCustomAlertRule {
    #[serde(flatten)]
    pub custom_alert_rule: CustomAlertRule,
    #[doc = "The value type of the items in the list."]
    #[serde(rename = "valueType", default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<list_custom_alert_rule::ValueType>,
}
impl ListCustomAlertRule {
    pub fn new(custom_alert_rule: CustomAlertRule) -> Self {
        Self {
            custom_alert_rule,
            value_type: None,
        }
    }
}
pub mod list_custom_alert_rule {
    use super::*;
    #[doc = "The value type of the items in the list."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValueType")]
    pub enum ValueType {
        IpCidr,
        String,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValueType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValueType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValueType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IpCidr => serializer.serialize_unit_variant("ValueType", 0u32, "IpCidr"),
                Self::String => serializer.serialize_unit_variant("ValueType", 1u32, "String"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Login by a local user that isn't allowed. Allow list consists of login names to allow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalUserNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
impl LocalUserNotAllowed {
    pub fn new(allowlist_custom_alert_rule: AllowlistCustomAlertRule) -> Self {
        Self {
            allowlist_custom_alert_rule,
        }
    }
}
#[doc = "Describes an Azure resource with location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[doc = "Location where the resource is stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Log Analytics workspace scope identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAnalyticsIdentifier {
    #[doc = "The LogAnalytics workspace id that stores this alert."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The azure subscription id for the LogAnalytics workspace storing this alert."]
    #[serde(rename = "workspaceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_subscription_id: Option<String>,
    #[doc = "The azure resource group for the LogAnalytics workspace storing this alert"]
    #[serde(rename = "workspaceResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_group: Option<String>,
    #[doc = "(optional) The LogAnalytics agent id reporting the event that this alert is based on."]
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}
impl LogAnalyticsIdentifier {
    pub fn new() -> Self {
        Self {
            workspace_id: None,
            workspace_subscription_id: None,
            workspace_resource_group: None,
            agent_id: None,
        }
    }
}
#[doc = "Properties of Malware Scanning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MalwareScanningProperties {
    #[doc = "Properties of On Upload malware scanning."]
    #[serde(rename = "onUpload", default, skip_serializing_if = "Option::is_none")]
    pub on_upload: Option<OnUploadProperties>,
    #[doc = "Optional. Resource id of an Event Grid Topic to send scan results to."]
    #[serde(rename = "scanResultsEventGridTopicResourceId", default, skip_serializing_if = "Option::is_none")]
    pub scan_results_event_grid_topic_resource_id: Option<String>,
    #[doc = "A status describing the success/failure of the enablement/disablement operation."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<OperationStatus>,
}
impl MalwareScanningProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource of the configuration or data needed to onboard the machine to MDE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MdeOnboardingData {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the MDE configuration or data parameter needed to onboard the machine to MDE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdeOnboardingDataProperties>,
}
impl MdeOnboardingData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all MDE onboarding data resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MdeOnboardingDataList {
    #[doc = "List of the resources of the configuration or data needed to onboard the machine to MDE"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MdeOnboardingData>,
}
impl MdeOnboardingDataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the MDE configuration or data parameter needed to onboard the machine to MDE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MdeOnboardingDataProperties {
    #[doc = "The onboarding package used to onboard Windows machines to MDE, coded in base64. This can also be used for onboarding using the dedicated VM Extension"]
    #[serde(rename = "onboardingPackageWindows", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_package_windows: Option<String>,
    #[doc = "The onboarding package used to onboard Linux machines to MDE, coded in base64. This can also be used for onboarding using the dedicated VM Extension"]
    #[serde(rename = "onboardingPackageLinux", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_package_linux: Option<String>,
}
impl MdeOnboardingDataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft information protection integration status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MipIntegrationStatus")]
pub enum MipIntegrationStatus {
    Ok,
    #[serde(rename = "noConsent")]
    NoConsent,
    #[serde(rename = "noAutoLabelingRules")]
    NoAutoLabelingRules,
    #[serde(rename = "noMipLabels")]
    NoMipLabels,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MipIntegrationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MipIntegrationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MipIntegrationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ok => serializer.serialize_unit_variant("MipIntegrationStatus", 0u32, "Ok"),
            Self::NoConsent => serializer.serialize_unit_variant("MipIntegrationStatus", 1u32, "noConsent"),
            Self::NoAutoLabelingRules => serializer.serialize_unit_variant("MipIntegrationStatus", 2u32, "noAutoLabelingRules"),
            Self::NoMipLabels => serializer.serialize_unit_variant("MipIntegrationStatus", 3u32, "noMipLabels"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Number of cloud to device messages (MQTT protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttC2dMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl MqttC2dMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of rejected cloud to device messages (MQTT protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttC2dRejectedMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl MqttC2dRejectedMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of device to cloud messages (MQTT protocol) is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MqttD2cMessagesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl MqttD2cMessagesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Properties of On Upload malware scanning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnUploadProperties {
    #[doc = "Indicates whether On Upload malware scanning should be enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Defines the max GB to be scanned per Month. Set to -1 if no capping is needed."]
    #[serde(rename = "capGBPerMonth", default, skip_serializing_if = "Option::is_none")]
    pub cap_gb_per_month: Option<i32>,
}
impl OnUploadProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OnboardingState")]
pub enum OnboardingState {
    NotApplicable,
    OnboardedByOtherConnector,
    Onboarded,
    NotOnboarded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OnboardingState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OnboardingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OnboardingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotApplicable => serializer.serialize_unit_variant("OnboardingState", 0u32, "NotApplicable"),
            Self::OnboardedByOtherConnector => serializer.serialize_unit_variant("OnboardingState", 1u32, "OnboardedByOtherConnector"),
            Self::Onboarded => serializer.serialize_unit_variant("OnboardingState", 2u32, "Onboarded"),
            Self::NotOnboarded => serializer.serialize_unit_variant("OnboardingState", 3u32, "NotOnboarded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Possible operation in the REST API of Microsoft.Security"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Where the operation is originated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Security operation display"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security operation display"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The display name of the security operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of possible operations for Microsoft.Security resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of Security operations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Long run operation status of governance rule over a given scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "The status of the long run operation result of governance rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_result::Status>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_result {
    use super::*;
    #[doc = "The status of the long run operation result of governance rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 2u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A status describing the success/failure of the enablement/disablement operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Additional information regarding the success/failure of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResult {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<OperationStatusResult>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatusResult {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
        }
    }
}
#[doc = "Represents a path that is recommended to be allowed and its properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PathRecommendation {
    #[doc = "The full path of the file, or an identifier of the application"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The recommendation action of the machine or rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RecommendationAction>,
    #[doc = "The type of the rule to be allowed"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<RecommendationType>,
    #[doc = "Represents the publisher information of a process/rule"]
    #[serde(rename = "publisherInfo", default, skip_serializing_if = "Option::is_none")]
    pub publisher_info: Option<PublisherInfo>,
    #[doc = "Whether the application is commonly run on the machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<bool>,
    #[serde(
        rename = "userSids",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_sids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub usernames: Vec<UserRecommendation>,
    #[doc = "The type of the file (for Linux files - Executable is used)"]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    #[doc = "The configuration status of the machines group or machine or rule"]
    #[serde(rename = "configurationStatus", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status: Option<ConfigurationStatus>,
}
impl PathRecommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PathRecommendations = Vec<PathRecommendation>;
#[doc = "A permission detected in the cloud account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PermissionProperty")]
pub enum PermissionProperty {
    #[serde(rename = "AWS::AWSSecurityHubReadOnlyAccess")]
    AwsAwsSecurityHubReadOnlyAccess,
    #[serde(rename = "AWS::SecurityAudit")]
    AwsSecurityAudit,
    #[serde(rename = "AWS::AmazonSSMAutomationRole")]
    AwsAmazonSsmAutomationRole,
    #[serde(rename = "GCP::Security Center Admin Viewer")]
    GcpSecurityCenterAdminViewer,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PermissionProperty {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PermissionProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PermissionProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AwsAwsSecurityHubReadOnlyAccess => {
                serializer.serialize_unit_variant("PermissionProperty", 0u32, "AWS::AWSSecurityHubReadOnlyAccess")
            }
            Self::AwsSecurityAudit => serializer.serialize_unit_variant("PermissionProperty", 1u32, "AWS::SecurityAudit"),
            Self::AwsAmazonSsmAutomationRole => {
                serializer.serialize_unit_variant("PermissionProperty", 2u32, "AWS::AmazonSSMAutomationRole")
            }
            Self::GcpSecurityCenterAdminViewer => {
                serializer.serialize_unit_variant("PermissionProperty", 3u32, "GCP::Security Center Admin Viewer")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type PortNumber = i64;
#[doc = "Microsoft Defender for Cloud is provided in two pricing tiers: free and standard. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pricing {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Pricing properties for the relevant scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PricingProperties>,
}
impl Pricing {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of pricing configurations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingList {
    #[doc = "List of pricing configurations"]
    pub value: Vec<Pricing>,
}
impl PricingList {
    pub fn new(value: Vec<Pricing>) -> Self {
        Self { value }
    }
}
#[doc = "Pricing properties for the relevant scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingProperties {
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[serde(rename = "pricingTier")]
    pub pricing_tier: pricing_properties::PricingTier,
    #[doc = "The sub-plan selected for a Standard pricing configuration, when more than one sub-plan is available. Each sub-plan enables a set of security features. When not specified, full plan is applied."]
    #[serde(rename = "subPlan", default, skip_serializing_if = "Option::is_none")]
    pub sub_plan: Option<String>,
    #[doc = "The duration left for the subscriptions free trial period - in ISO 8601 format (e.g. P3Y6M4DT12H30M5S)."]
    #[serde(rename = "freeTrialRemainingTime", default, skip_serializing_if = "Option::is_none")]
    pub free_trial_remaining_time: Option<String>,
    #[doc = "Optional. If `pricingTier` is `Standard` then this property holds the date of the last time the `pricingTier` was set to `Standard`, when available (e.g 2023-03-01T12:42:42.1921106Z)."]
    #[serde(rename = "enablementTime", default, with = "azure_core::date::rfc3339::option")]
    pub enablement_time: Option<time::OffsetDateTime>,
    #[doc = "Optional. True if the plan is deprecated. If there are replacing plans they will appear in `replacedBy` property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = "Optional. List of plans that replace this plan. This property exists only if this plan is deprecated."]
    #[serde(
        rename = "replacedBy",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replaced_by: Vec<String>,
    #[doc = "Optional. List of extensions offered under a plan."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extensions: Vec<Extension>,
}
impl PricingProperties {
    pub fn new(pricing_tier: pricing_properties::PricingTier) -> Self {
        Self {
            pricing_tier,
            sub_plan: None,
            free_trial_remaining_time: None,
            enablement_time: None,
            deprecated: None,
            replaced_by: Vec::new(),
            extensions: Vec::new(),
        }
    }
}
pub mod pricing_properties {
    use super::*;
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PricingTier")]
    pub enum PricingTier {
        Free,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PricingTier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PricingTier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PricingTier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("PricingTier", 0u32, "Free"),
                Self::Standard => serializer.serialize_unit_variant("PricingTier", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Execution of a process that isn't allowed. Allow list consists of process names to allow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessNotAllowed {
    #[serde(flatten)]
    pub allowlist_custom_alert_rule: AllowlistCustomAlertRule,
}
impl ProcessNotAllowed {
    pub fn new(allowlist_custom_alert_rule: AllowlistCustomAlertRule) -> Self {
        Self {
            allowlist_custom_alert_rule,
        }
    }
}
#[doc = "The protection mode of the collection/file types. Exe/Msi/Script are used for Windows, Executable is used for Linux."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionMode {
    #[doc = "The application control policy enforcement/protection mode of the machine group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exe: Option<EnforcementMode>,
    #[doc = "The application control policy enforcement/protection mode of the machine group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msi: Option<EnforcementMode>,
    #[doc = "The application control policy enforcement/protection mode of the machine group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<EnforcementMode>,
    #[doc = "The application control policy enforcement/protection mode of the machine group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable: Option<EnforcementMode>,
}
impl ProtectionMode {
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
#[doc = "For a non-Azure machine that is not connected directly to the internet, specify a proxy server that the non-Azure machine can use."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyServerProperties {
    #[doc = "Proxy server IP"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "Proxy server port"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}
impl ProxyServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the publisher information of a process/rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublisherInfo {
    #[doc = "The Subject field of the x.509 certificate used to sign the code, using the following fields -  O = Organization, L = Locality, S = State or Province, and C = Country"]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "The product name taken from the file's version resource"]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The \"OriginalName\" field taken from the file's version resource"]
    #[serde(rename = "binaryName", default, skip_serializing_if = "Option::is_none")]
    pub binary_name: Option<String>,
    #[doc = "The binary file version taken from the file's version resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PublisherInfo {
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
#[doc = "Number of device queue purges is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueuePurgesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl QueuePurgesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "The recommendation action of the machine or rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecommendationAction {
    Recommended,
    Add,
    Remove,
}
pub type RecommendationConfigurationList = Vec<RecommendationConfigurationProperties>;
#[doc = "The type of IoT Security recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationConfigurationProperties {
    #[doc = "The type of IoT Security recommendation."]
    #[serde(rename = "recommendationType")]
    pub recommendation_type: recommendation_configuration_properties::RecommendationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Recommendation status. When the recommendation status is disabled recommendations are not generated."]
    pub status: recommendation_configuration_properties::Status,
}
impl RecommendationConfigurationProperties {
    pub fn new(
        recommendation_type: recommendation_configuration_properties::RecommendationType,
        status: recommendation_configuration_properties::Status,
    ) -> Self {
        Self {
            recommendation_type,
            name: None,
            status,
        }
    }
}
pub mod recommendation_configuration_properties {
    use super::*;
    #[doc = "The type of IoT Security recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecommendationType")]
    pub enum RecommendationType {
        #[serde(rename = "IoT_ACRAuthentication")]
        IoTAcrAuthentication,
        #[serde(rename = "IoT_AgentSendsUnutilizedMessages")]
        IoTAgentSendsUnutilizedMessages,
        #[serde(rename = "IoT_Baseline")]
        IoTBaseline,
        #[serde(rename = "IoT_EdgeHubMemOptimize")]
        IoTEdgeHubMemOptimize,
        #[serde(rename = "IoT_EdgeLoggingOptions")]
        IoTEdgeLoggingOptions,
        #[serde(rename = "IoT_InconsistentModuleSettings")]
        IoTInconsistentModuleSettings,
        #[serde(rename = "IoT_InstallAgent")]
        IoTInstallAgent,
        #[serde(rename = "IoT_IPFilter_DenyAll")]
        IoTIpFilterDenyAll,
        #[serde(rename = "IoT_IPFilter_PermissiveRule")]
        IoTIpFilterPermissiveRule,
        #[serde(rename = "IoT_OpenPorts")]
        IoTOpenPorts,
        #[serde(rename = "IoT_PermissiveFirewallPolicy")]
        IoTPermissiveFirewallPolicy,
        #[serde(rename = "IoT_PermissiveInputFirewallRules")]
        IoTPermissiveInputFirewallRules,
        #[serde(rename = "IoT_PermissiveOutputFirewallRules")]
        IoTPermissiveOutputFirewallRules,
        #[serde(rename = "IoT_PrivilegedDockerOptions")]
        IoTPrivilegedDockerOptions,
        #[serde(rename = "IoT_SharedCredentials")]
        IoTSharedCredentials,
        #[serde(rename = "IoT_VulnerableTLSCipherSuite")]
        IoTVulnerableTlsCipherSuite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecommendationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecommendationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecommendationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IoTAcrAuthentication => serializer.serialize_unit_variant("RecommendationType", 0u32, "IoT_ACRAuthentication"),
                Self::IoTAgentSendsUnutilizedMessages => {
                    serializer.serialize_unit_variant("RecommendationType", 1u32, "IoT_AgentSendsUnutilizedMessages")
                }
                Self::IoTBaseline => serializer.serialize_unit_variant("RecommendationType", 2u32, "IoT_Baseline"),
                Self::IoTEdgeHubMemOptimize => serializer.serialize_unit_variant("RecommendationType", 3u32, "IoT_EdgeHubMemOptimize"),
                Self::IoTEdgeLoggingOptions => serializer.serialize_unit_variant("RecommendationType", 4u32, "IoT_EdgeLoggingOptions"),
                Self::IoTInconsistentModuleSettings => {
                    serializer.serialize_unit_variant("RecommendationType", 5u32, "IoT_InconsistentModuleSettings")
                }
                Self::IoTInstallAgent => serializer.serialize_unit_variant("RecommendationType", 6u32, "IoT_InstallAgent"),
                Self::IoTIpFilterDenyAll => serializer.serialize_unit_variant("RecommendationType", 7u32, "IoT_IPFilter_DenyAll"),
                Self::IoTIpFilterPermissiveRule => {
                    serializer.serialize_unit_variant("RecommendationType", 8u32, "IoT_IPFilter_PermissiveRule")
                }
                Self::IoTOpenPorts => serializer.serialize_unit_variant("RecommendationType", 9u32, "IoT_OpenPorts"),
                Self::IoTPermissiveFirewallPolicy => {
                    serializer.serialize_unit_variant("RecommendationType", 10u32, "IoT_PermissiveFirewallPolicy")
                }
                Self::IoTPermissiveInputFirewallRules => {
                    serializer.serialize_unit_variant("RecommendationType", 11u32, "IoT_PermissiveInputFirewallRules")
                }
                Self::IoTPermissiveOutputFirewallRules => {
                    serializer.serialize_unit_variant("RecommendationType", 12u32, "IoT_PermissiveOutputFirewallRules")
                }
                Self::IoTPrivilegedDockerOptions => {
                    serializer.serialize_unit_variant("RecommendationType", 13u32, "IoT_PrivilegedDockerOptions")
                }
                Self::IoTSharedCredentials => serializer.serialize_unit_variant("RecommendationType", 14u32, "IoT_SharedCredentials"),
                Self::IoTVulnerableTlsCipherSuite => {
                    serializer.serialize_unit_variant("RecommendationType", 15u32, "IoT_VulnerableTLSCipherSuite")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Recommendation status. When the recommendation status is disabled recommendations are not generated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Disabled,
        Enabled,
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
                Self::Disabled => serializer.serialize_unit_variant("Status", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("Status", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The initial recommendation status of the machine group or machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecommendationStatus {
    Recommended,
    NotRecommended,
    NotAvailable,
    NoStatus,
}
#[doc = "The type of the rule to be allowed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    File,
    FileHash,
    PublisherSignature,
    ProductSignature,
    BinarySignature,
    VersionAndAboveSignature,
}
#[doc = "Regulatory compliance assessment details and state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegulatoryComplianceAssessment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Regulatory compliance assessment data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegulatoryComplianceAssessmentProperties>,
}
impl RegulatoryComplianceAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of regulatory compliance assessment response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceAssessmentList {
    pub value: Vec<RegulatoryComplianceAssessment>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegulatoryComplianceAssessmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RegulatoryComplianceAssessmentList {
    pub fn new(value: Vec<RegulatoryComplianceAssessment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Regulatory compliance assessment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegulatoryComplianceAssessmentProperties {
    #[doc = "The description of the regulatory compliance assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The expected type of assessment contained in the AssessmentDetailsLink"]
    #[serde(rename = "assessmentType", default, skip_serializing_if = "Option::is_none")]
    pub assessment_type: Option<String>,
    #[doc = "Link to more detailed assessment results data. The response type will be according to the assessmentType field"]
    #[serde(rename = "assessmentDetailsLink", default, skip_serializing_if = "Option::is_none")]
    pub assessment_details_link: Option<String>,
    #[doc = "Aggregative state based on the assessment's scanned resources states"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regulatory_compliance_assessment_properties::State>,
    #[doc = "The given assessment's related resources count with passed state."]
    #[serde(rename = "passedResources", default, skip_serializing_if = "Option::is_none")]
    pub passed_resources: Option<i64>,
    #[doc = "The given assessment's related resources count with failed state."]
    #[serde(rename = "failedResources", default, skip_serializing_if = "Option::is_none")]
    pub failed_resources: Option<i64>,
    #[doc = "The given assessment's related resources count with skipped state."]
    #[serde(rename = "skippedResources", default, skip_serializing_if = "Option::is_none")]
    pub skipped_resources: Option<i64>,
    #[doc = "The given assessment's related resources count with unsupported state."]
    #[serde(rename = "unsupportedResources", default, skip_serializing_if = "Option::is_none")]
    pub unsupported_resources: Option<i64>,
}
impl RegulatoryComplianceAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regulatory_compliance_assessment_properties {
    use super::*;
    #[doc = "Aggregative state based on the assessment's scanned resources states"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Passed,
        Failed,
        Skipped,
        Unsupported,
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
                Self::Skipped => serializer.serialize_unit_variant("State", 2u32, "Skipped"),
                Self::Unsupported => serializer.serialize_unit_variant("State", 3u32, "Unsupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Regulatory compliance control details and state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegulatoryComplianceControl {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Regulatory compliance control data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegulatoryComplianceControlProperties>,
}
impl RegulatoryComplianceControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of regulatory compliance controls response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceControlList {
    #[doc = "List of regulatory compliance controls"]
    pub value: Vec<RegulatoryComplianceControl>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegulatoryComplianceControlList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RegulatoryComplianceControlList {
    pub fn new(value: Vec<RegulatoryComplianceControl>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Regulatory compliance control data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegulatoryComplianceControlProperties {
    #[doc = "The description of the regulatory compliance control"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Aggregative state based on the control's supported assessments states"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regulatory_compliance_control_properties::State>,
    #[doc = "The number of supported regulatory compliance assessments of the given control with a passed state"]
    #[serde(rename = "passedAssessments", default, skip_serializing_if = "Option::is_none")]
    pub passed_assessments: Option<i64>,
    #[doc = "The number of supported regulatory compliance assessments of the given control with a failed state"]
    #[serde(rename = "failedAssessments", default, skip_serializing_if = "Option::is_none")]
    pub failed_assessments: Option<i64>,
    #[doc = "The number of supported regulatory compliance assessments of the given control with a skipped state"]
    #[serde(rename = "skippedAssessments", default, skip_serializing_if = "Option::is_none")]
    pub skipped_assessments: Option<i64>,
}
impl RegulatoryComplianceControlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regulatory_compliance_control_properties {
    use super::*;
    #[doc = "Aggregative state based on the control's supported assessments states"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Passed,
        Failed,
        Skipped,
        Unsupported,
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
                Self::Skipped => serializer.serialize_unit_variant("State", 2u32, "Skipped"),
                Self::Unsupported => serializer.serialize_unit_variant("State", 3u32, "Unsupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Regulatory compliance standard details and state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegulatoryComplianceStandard {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Regulatory compliance standard data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegulatoryComplianceStandardProperties>,
}
impl RegulatoryComplianceStandard {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of regulatory compliance standards response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceStandardList {
    pub value: Vec<RegulatoryComplianceStandard>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegulatoryComplianceStandardList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RegulatoryComplianceStandardList {
    pub fn new(value: Vec<RegulatoryComplianceStandard>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Regulatory compliance standard data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegulatoryComplianceStandardProperties {
    #[doc = "Aggregative state based on the standard's supported controls states"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regulatory_compliance_standard_properties::State>,
    #[doc = "The number of supported regulatory compliance controls of the given standard with a passed state"]
    #[serde(rename = "passedControls", default, skip_serializing_if = "Option::is_none")]
    pub passed_controls: Option<i64>,
    #[doc = "The number of supported regulatory compliance controls of the given standard with a failed state"]
    #[serde(rename = "failedControls", default, skip_serializing_if = "Option::is_none")]
    pub failed_controls: Option<i64>,
    #[doc = "The number of supported regulatory compliance controls of the given standard with a skipped state"]
    #[serde(rename = "skippedControls", default, skip_serializing_if = "Option::is_none")]
    pub skipped_controls: Option<i64>,
    #[doc = "The number of regulatory compliance controls of the given standard which are unsupported by automated assessments"]
    #[serde(rename = "unsupportedControls", default, skip_serializing_if = "Option::is_none")]
    pub unsupported_controls: Option<i64>,
}
impl RegulatoryComplianceStandardProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regulatory_compliance_standard_properties {
    use super::*;
    #[doc = "Aggregative state based on the standard's supported controls states"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Passed,
        Failed,
        Skipped,
        Unsupported,
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
                Self::Skipped => serializer.serialize_unit_variant("State", 2u32, "Skipped"),
                Self::Unsupported => serializer.serialize_unit_variant("State", 3u32, "Unsupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "The platform where the assessed resource resides"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum ResourceDetailsUnion {}
#[doc = "There can be multiple identifiers of different type per alert, this field specify the identifier type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceIdentifierUnion {
    AzureResource(AzureResourceIdentifier),
    LogAnalytics(LogAnalyticsIdentifier),
}
#[doc = "Describes remote addresses that is recommended to communicate with the Azure resource on some (Protocol, Port, Direction). All other remote addresses are recommended to be blocked"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    #[doc = "The name of the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The rule's direction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<rule::Direction>,
    #[serde(rename = "destinationPort", default, skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<PortNumber>,
    #[doc = "The rule's transport protocols"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protocols: Vec<String>,
    #[doc = "The remote IP addresses that should be able to communicate with the Azure resource on the rule's destination port and protocol"]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
}
impl Rule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rule {
    use super::*;
    #[doc = "The rule's direction"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        Inbound,
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("Direction", 0u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Direction", 1u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Rule categories.\r\nCode - code scanning results.\r\nArtifact scanning results.\r\nDependencies scanning results.\r\nIaC results.\r\nSecrets scanning results.\r\nContainer scanning results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleCategory")]
pub enum RuleCategory {
    Code,
    Artifacts,
    Dependencies,
    Secrets,
    IaC,
    Containers,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Code => serializer.serialize_unit_variant("RuleCategory", 0u32, "Code"),
            Self::Artifacts => serializer.serialize_unit_variant("RuleCategory", 1u32, "Artifacts"),
            Self::Dependencies => serializer.serialize_unit_variant("RuleCategory", 2u32, "Dependencies"),
            Self::Secrets => serializer.serialize_unit_variant("RuleCategory", 3u32, "Secrets"),
            Self::IaC => serializer.serialize_unit_variant("RuleCategory", 4u32, "IaC"),
            Self::Containers => serializer.serialize_unit_variant("RuleCategory", 5u32, "Containers"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "A more specific scope used to identify the alerts to suppress."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeElement {
    #[doc = "The alert entity type to suppress by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}
impl ScopeElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Calculation result data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScoreDetails {
    #[doc = "Maximum score available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[doc = "Current score"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    #[doc = "Ratio of the current score divided by the maximum. Rounded to 4 digits after the decimal point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}
impl ScoreDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the security control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlDefinitionItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Security Control Definition Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecureScoreControlDefinitionItemProperties>,
}
impl SecureScoreControlDefinitionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security Control Definition Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlDefinitionItemProperties {
    #[doc = "User friendly display name of the control"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "User friendly description of the control"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Maximum control score (0..10)"]
    #[serde(rename = "maxScore", default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i32>,
    #[doc = "The type of the security control (For example, BuiltIn)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SecureScoreControlDefinitionSource>,
    #[doc = "array of azure resource IDs"]
    #[serde(rename = "assessmentDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub assessment_definitions: Option<AzureResourceLinks>,
}
impl SecureScoreControlDefinitionItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the security control (For example, BuiltIn)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlDefinitionSource {
    #[doc = "The type of security control (for example, BuiltIn)"]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<secure_score_control_definition_source::SourceType>,
}
impl SecureScoreControlDefinitionSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod secure_score_control_definition_source {
    use super::*;
    #[doc = "The type of security control (for example, BuiltIn)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        BuiltIn,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BuiltIn => serializer.serialize_unit_variant("SourceType", 0u32, "BuiltIn"),
                Self::Custom => serializer.serialize_unit_variant("SourceType", 1u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of the security control, its score, and the health status of the relevant resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Calculation result data in control level"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecureScoreControlScoreDetails>,
}
impl SecureScoreControlDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of security controls"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlList {
    #[doc = "Collection of security controls in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecureScoreControlDetails>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecureScoreControlList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecureScoreControlList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Calculation result data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlScore {
    #[doc = "Maximum control score (0..10)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[doc = "Actual score for the control = (achieved points / total points) * max score. if total points is zeroed, the return number is 0.00"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    #[doc = "Ratio of the current score divided by the maximum. Rounded to 4 digits after the decimal point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}
impl SecureScoreControlScore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Calculation result data in control level"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlScoreDetails {
    #[doc = "User friendly display name of the control"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Calculation result data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreDetails>,
    #[doc = "Number of healthy resources in the control"]
    #[serde(rename = "healthyResourceCount", default, skip_serializing_if = "Option::is_none")]
    pub healthy_resource_count: Option<i32>,
    #[doc = "Number of unhealthy resources in the control"]
    #[serde(rename = "unhealthyResourceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_resource_count: Option<i32>,
    #[doc = "Number of not applicable resources in the control"]
    #[serde(rename = "notApplicableResourceCount", default, skip_serializing_if = "Option::is_none")]
    pub not_applicable_resource_count: Option<i32>,
    #[doc = "The relative weight for this specific control in each of your subscriptions. Used when calculating an aggregated score for this control across all of your subscriptions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[doc = "Information about the security control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<SecureScoreControlDefinitionItem>,
}
impl SecureScoreControlScoreDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secure score item data model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of a calculated secure score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecureScoreItemProperties>,
}
impl SecureScoreItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a calculated secure score."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreItemProperties {
    #[doc = "The initiative’s name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Calculation result data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreDetails>,
    #[doc = "The relative weight for each subscription. Used when calculating an aggregated secure score for multiple subscriptions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}
impl SecureScoreItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of secure scores"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoresList {
    #[doc = "Collection of secure scores in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecureScoreItem>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecureScoresList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecureScoresList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security assessment on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAssessment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAssessmentProperties>,
}
impl SecurityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a security assessments list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAssessmentList {
    #[doc = "Collection of security assessments in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityAssessmentResponse>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityAssessmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityAssessmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security assessment metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAssessmentMetadata {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an assessment metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAssessmentMetadataProperties>,
}
impl SecurityAssessmentMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the partner that created the assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentMetadataPartnerData {
    #[doc = "Name of the company of the partner"]
    #[serde(rename = "partnerName")]
    pub partner_name: String,
    #[doc = "Name of the product of the partner that created the assessment"]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Secret to authenticate the partner and verify it created the assessment - write only"]
    pub secret: String,
}
impl SecurityAssessmentMetadataPartnerData {
    pub fn new(partner_name: String, secret: String) -> Self {
        Self {
            partner_name,
            product_name: None,
            secret,
        }
    }
}
#[doc = "Describes properties of an assessment metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentMetadataProperties {
    #[doc = "User friendly display name of the assessment"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Azure resource ID of the policy definition that turns this assessment calculation on"]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Human readable description of the assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Human readable description of what you should do to mitigate this security issue"]
    #[serde(rename = "remediationDescription", default, skip_serializing_if = "Option::is_none")]
    pub remediation_description: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<String>,
    #[doc = "The severity level of the assessment"]
    pub severity: security_assessment_metadata_properties::Severity,
    #[doc = "The user impact of the assessment"]
    #[serde(rename = "userImpact", default, skip_serializing_if = "Option::is_none")]
    pub user_impact: Option<security_assessment_metadata_properties::UserImpact>,
    #[doc = "The implementation effort required to remediate this assessment"]
    #[serde(rename = "implementationEffort", default, skip_serializing_if = "Option::is_none")]
    pub implementation_effort: Option<security_assessment_metadata_properties::ImplementationEffort>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub threats: Vec<String>,
    #[doc = "True if this assessment is in preview release status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[doc = "BuiltIn if the assessment based on built-in Azure Policy definition, Custom if the assessment based on custom Azure Policy definition"]
    #[serde(rename = "assessmentType")]
    pub assessment_type: security_assessment_metadata_properties::AssessmentType,
    #[doc = "Describes the partner that created the assessment"]
    #[serde(rename = "partnerData", default, skip_serializing_if = "Option::is_none")]
    pub partner_data: Option<SecurityAssessmentMetadataPartnerData>,
}
impl SecurityAssessmentMetadataProperties {
    pub fn new(
        display_name: String,
        severity: security_assessment_metadata_properties::Severity,
        assessment_type: security_assessment_metadata_properties::AssessmentType,
    ) -> Self {
        Self {
            display_name,
            policy_definition_id: None,
            description: None,
            remediation_description: None,
            categories: Vec::new(),
            severity,
            user_impact: None,
            implementation_effort: None,
            threats: Vec::new(),
            preview: None,
            assessment_type,
            partner_data: None,
        }
    }
}
pub mod security_assessment_metadata_properties {
    use super::*;
    #[doc = "The severity level of the assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Low,
        Medium,
        High,
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
                Self::Low => serializer.serialize_unit_variant("Severity", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("Severity", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("Severity", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The user impact of the assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserImpact")]
    pub enum UserImpact {
        Low,
        Moderate,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserImpact {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserImpact {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserImpact {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("UserImpact", 0u32, "Low"),
                Self::Moderate => serializer.serialize_unit_variant("UserImpact", 1u32, "Moderate"),
                Self::High => serializer.serialize_unit_variant("UserImpact", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The implementation effort required to remediate this assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImplementationEffort")]
    pub enum ImplementationEffort {
        Low,
        Moderate,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImplementationEffort {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImplementationEffort {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImplementationEffort {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("ImplementationEffort", 0u32, "Low"),
                Self::Moderate => serializer.serialize_unit_variant("ImplementationEffort", 1u32, "Moderate"),
                Self::High => serializer.serialize_unit_variant("ImplementationEffort", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "BuiltIn if the assessment based on built-in Azure Policy definition, Custom if the assessment based on custom Azure Policy definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssessmentType")]
    pub enum AssessmentType {
        BuiltIn,
        CustomPolicy,
        CustomerManaged,
        VerifiedPartner,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssessmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssessmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssessmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BuiltIn => serializer.serialize_unit_variant("AssessmentType", 0u32, "BuiltIn"),
                Self::CustomPolicy => serializer.serialize_unit_variant("AssessmentType", 1u32, "CustomPolicy"),
                Self::CustomerManaged => serializer.serialize_unit_variant("AssessmentType", 2u32, "CustomerManaged"),
                Self::VerifiedPartner => serializer.serialize_unit_variant("AssessmentType", 3u32, "VerifiedPartner"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes properties of an assessment metadata response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentMetadataPropertiesResponse {
    #[serde(flatten)]
    pub security_assessment_metadata_properties: SecurityAssessmentMetadataProperties,
    #[serde(rename = "publishDates", default, skip_serializing_if = "Option::is_none")]
    pub publish_dates: Option<security_assessment_metadata_properties_response::PublishDates>,
    #[serde(rename = "plannedDeprecationDate", default, skip_serializing_if = "Option::is_none")]
    pub planned_deprecation_date: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tactics: Vec<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub techniques: Vec<String>,
}
impl SecurityAssessmentMetadataPropertiesResponse {
    pub fn new(security_assessment_metadata_properties: SecurityAssessmentMetadataProperties) -> Self {
        Self {
            security_assessment_metadata_properties,
            publish_dates: None,
            planned_deprecation_date: None,
            tactics: Vec::new(),
            techniques: Vec::new(),
        }
    }
}
pub mod security_assessment_metadata_properties_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PublishDates {
        #[serde(rename = "GA", default, skip_serializing_if = "Option::is_none")]
        pub ga: Option<String>,
        pub public: String,
    }
    impl PublishDates {
        pub fn new(public: String) -> Self {
            Self { ga: None, public }
        }
    }
}
#[doc = "Security assessment metadata response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAssessmentMetadataResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an assessment metadata response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAssessmentMetadataPropertiesResponse>,
}
impl SecurityAssessmentMetadataResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of security assessment metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAssessmentMetadataResponseList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityAssessmentMetadataResponse>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityAssessmentMetadataResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityAssessmentMetadataResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data regarding 3rd party partner integration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentPartnerData {
    #[doc = "Name of the company of the partner"]
    #[serde(rename = "partnerName")]
    pub partner_name: String,
    #[doc = "secret to authenticate the partner - write only"]
    pub secret: String,
}
impl SecurityAssessmentPartnerData {
    pub fn new(partner_name: String, secret: String) -> Self {
        Self { partner_name, secret }
    }
}
#[doc = "Describes properties of an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentProperties {
    #[serde(flatten)]
    pub security_assessment_properties_base: SecurityAssessmentPropertiesBase,
    #[doc = "The result of the assessment"]
    pub status: AssessmentStatus,
}
impl SecurityAssessmentProperties {
    pub fn new(security_assessment_properties_base: SecurityAssessmentPropertiesBase, status: AssessmentStatus) -> Self {
        Self {
            security_assessment_properties_base,
            status,
        }
    }
}
#[doc = "Describes properties of an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentPropertiesBase {
    #[doc = "Details of the resource that was assessed"]
    #[serde(rename = "resourceDetails")]
    pub resource_details: ResourceDetailsUnion,
    #[doc = "User friendly display name of the assessment"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Additional data regarding the assessment"]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    #[doc = "Links relevant to the assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<AssessmentLinks>,
    #[doc = "Describes properties of an assessment metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SecurityAssessmentMetadataProperties>,
    #[doc = "Data regarding 3rd party partner integration"]
    #[serde(rename = "partnersData", default, skip_serializing_if = "Option::is_none")]
    pub partners_data: Option<SecurityAssessmentPartnerData>,
}
impl SecurityAssessmentPropertiesBase {
    pub fn new(resource_details: ResourceDetailsUnion) -> Self {
        Self {
            resource_details,
            display_name: None,
            additional_data: None,
            links: None,
            metadata: None,
            partners_data: None,
        }
    }
}
#[doc = "Describes properties of an assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentPropertiesResponse {
    #[serde(flatten)]
    pub security_assessment_properties_base: SecurityAssessmentPropertiesBase,
    #[doc = "The result of the assessment"]
    pub status: AssessmentStatusResponse,
}
impl SecurityAssessmentPropertiesResponse {
    pub fn new(security_assessment_properties_base: SecurityAssessmentPropertiesBase, status: AssessmentStatusResponse) -> Self {
        Self {
            security_assessment_properties_base,
            status,
        }
    }
}
#[doc = "Security assessment on a resource - response format"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityAssessmentResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAssessmentPropertiesResponse>,
}
impl SecurityAssessmentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security connector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityConnector {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "A set of properties that defines the security connector configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityConnectorProperties>,
}
impl SecurityConnector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of properties that defines the security connector configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityConnectorProperties {
    #[doc = "The multi cloud resource identifier (account id in case of AWS connector, project number in case of GCP connector)."]
    #[serde(rename = "hierarchyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_identifier: Option<String>,
    #[doc = "The date on which the trial period will end, if applicable. Trial period exists for 30 days after upgrading to payed offerings."]
    #[serde(rename = "hierarchyIdentifierTrialEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub hierarchy_identifier_trial_end_date: Option<time::OffsetDateTime>,
    #[doc = "The multi cloud resource's cloud name."]
    #[serde(rename = "environmentName", default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<security_connector_properties::EnvironmentName>,
    #[doc = "A collection of offerings for the security connector."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub offerings: Vec<CloudOfferingUnion>,
    #[doc = "The security connector environment data."]
    #[serde(rename = "environmentData", default, skip_serializing_if = "Option::is_none")]
    pub environment_data: Option<EnvironmentDataUnion>,
}
impl SecurityConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_connector_properties {
    use super::*;
    #[doc = "The multi cloud resource's cloud name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnvironmentName")]
    pub enum EnvironmentName {
        Azure,
        #[serde(rename = "AWS")]
        Aws,
        #[serde(rename = "GCP")]
        Gcp,
        Github,
        AzureDevOps,
        GitLab,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnvironmentName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnvironmentName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnvironmentName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Azure => serializer.serialize_unit_variant("EnvironmentName", 0u32, "Azure"),
                Self::Aws => serializer.serialize_unit_variant("EnvironmentName", 1u32, "AWS"),
                Self::Gcp => serializer.serialize_unit_variant("EnvironmentName", 2u32, "GCP"),
                Self::Github => serializer.serialize_unit_variant("EnvironmentName", 3u32, "Github"),
                Self::AzureDevOps => serializer.serialize_unit_variant("EnvironmentName", 4u32, "AzureDevOps"),
                Self::GitLab => serializer.serialize_unit_variant("EnvironmentName", 5u32, "GitLab"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of security connectors response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityConnectorsList {
    #[doc = "The list of security connectors under the given scope."]
    pub value: Vec<SecurityConnector>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityConnectorsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityConnectorsList {
    pub fn new(value: Vec<SecurityConnector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Contact details and configurations for notifications coming from Microsoft Defender for Cloud."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityContact {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes security contact properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityContactProperties>,
}
impl SecurityContact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of security contacts response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityContactList {
    #[doc = "List of security contacts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityContact>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityContactList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityContactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes security contact properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityContactProperties {
    #[doc = "List of email addresses which will get notifications from Microsoft Defender for Cloud by the configurations defined in this security contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<String>,
    #[doc = "The security contact's phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Defines whether to send email notifications about new security alerts"]
    #[serde(rename = "alertNotifications", default, skip_serializing_if = "Option::is_none")]
    pub alert_notifications: Option<security_contact_properties::AlertNotifications>,
    #[doc = "Defines whether to send email notifications from Microsoft Defender for Cloud to persons with specific RBAC roles on the subscription."]
    #[serde(rename = "notificationsByRole", default, skip_serializing_if = "Option::is_none")]
    pub notifications_by_role: Option<security_contact_properties::NotificationsByRole>,
}
impl SecurityContactProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_contact_properties {
    use super::*;
    #[doc = "Defines whether to send email notifications about new security alerts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AlertNotifications {
        #[doc = "Defines if email notifications will be sent about new security alerts"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<alert_notifications::State>,
        #[doc = "Defines the minimal alert severity which will be sent as email notifications"]
        #[serde(rename = "minimalSeverity", default, skip_serializing_if = "Option::is_none")]
        pub minimal_severity: Option<alert_notifications::MinimalSeverity>,
    }
    impl AlertNotifications {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod alert_notifications {
        use super::*;
        #[doc = "Defines if email notifications will be sent about new security alerts"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "State")]
        pub enum State {
            On,
            Off,
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
                    Self::On => serializer.serialize_unit_variant("State", 0u32, "On"),
                    Self::Off => serializer.serialize_unit_variant("State", 1u32, "Off"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Defines the minimal alert severity which will be sent as email notifications"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "MinimalSeverity")]
        pub enum MinimalSeverity {
            High,
            Medium,
            Low,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for MinimalSeverity {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for MinimalSeverity {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for MinimalSeverity {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::High => serializer.serialize_unit_variant("MinimalSeverity", 0u32, "High"),
                    Self::Medium => serializer.serialize_unit_variant("MinimalSeverity", 1u32, "Medium"),
                    Self::Low => serializer.serialize_unit_variant("MinimalSeverity", 2u32, "Low"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "Defines whether to send email notifications from Microsoft Defender for Cloud to persons with specific RBAC roles on the subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NotificationsByRole {
        #[doc = "Defines whether to send email notifications from AMicrosoft Defender for Cloud to persons with specific RBAC roles on the subscription."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<notifications_by_role::State>,
        #[doc = "Defines which RBAC roles will get email notifications from Microsoft Defender for Cloud. List of allowed RBAC roles: "]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub roles: Vec<SecurityContactRole>,
    }
    impl NotificationsByRole {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod notifications_by_role {
        use super::*;
        #[doc = "Defines whether to send email notifications from AMicrosoft Defender for Cloud to persons with specific RBAC roles on the subscription."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "State")]
        pub enum State {
            On,
            Off,
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
                    Self::On => serializer.serialize_unit_variant("State", 0u32, "On"),
                    Self::Off => serializer.serialize_unit_variant("State", 1u32, "Off"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "A possible role to configure sending security notification alerts to"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecurityContactRole")]
pub enum SecurityContactRole {
    AccountAdmin,
    ServiceAdmin,
    Owner,
    Contributor,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecurityContactRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecurityContactRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecurityContactRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountAdmin => serializer.serialize_unit_variant("SecurityContactRole", 0u32, "AccountAdmin"),
            Self::ServiceAdmin => serializer.serialize_unit_variant("SecurityContactRole", 1u32, "ServiceAdmin"),
            Self::Owner => serializer.serialize_unit_variant("SecurityContactRole", 2u32, "Owner"),
            Self::Contributor => serializer.serialize_unit_variant("SecurityContactRole", 3u32, "Contributor"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Security operator under a given subscription and pricing"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityOperator {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl SecurityOperator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of SecurityOperator response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityOperatorList {
    #[doc = "List of SecurityOperator configurations"]
    pub value: Vec<SecurityOperator>,
}
impl SecurityOperatorList {
    pub fn new(value: Vec<SecurityOperator>) -> Self {
        Self { value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySolution {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecuritySolutionProperties>,
}
impl SecuritySolution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySolutionList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecuritySolution>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecuritySolutionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecuritySolutionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySolutionProperties {
    #[doc = "The security family of the security solution"]
    #[serde(rename = "securityFamily")]
    pub security_family: security_solution_properties::SecurityFamily,
    #[doc = "The security family provisioning State"]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: security_solution_properties::ProvisioningState,
    #[doc = "The security solutions' template"]
    pub template: String,
    #[doc = "The security solutions' status"]
    #[serde(rename = "protectionStatus")]
    pub protection_status: String,
}
impl SecuritySolutionProperties {
    pub fn new(
        security_family: security_solution_properties::SecurityFamily,
        provisioning_state: security_solution_properties::ProvisioningState,
        template: String,
        protection_status: String,
    ) -> Self {
        Self {
            security_family,
            provisioning_state,
            template,
            protection_status,
        }
    }
}
pub mod security_solution_properties {
    use super::*;
    #[doc = "The security family of the security solution"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecurityFamily")]
    pub enum SecurityFamily {
        Waf,
        Ngfw,
        SaasWaf,
        Va,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecurityFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecurityFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecurityFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Waf => serializer.serialize_unit_variant("SecurityFamily", 0u32, "Waf"),
                Self::Ngfw => serializer.serialize_unit_variant("SecurityFamily", 1u32, "Ngfw"),
                Self::SaasWaf => serializer.serialize_unit_variant("SecurityFamily", 2u32, "SaasWaf"),
                Self::Va => serializer.serialize_unit_variant("SecurityFamily", 3u32, "Va"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The security family provisioning State"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
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
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Security sub-assessment on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySubAssessment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of an sub-assessment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecuritySubAssessmentProperties>,
}
impl SecuritySubAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of security sub-assessments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySubAssessmentList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecuritySubAssessment>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecuritySubAssessmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecuritySubAssessmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of an sub-assessment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySubAssessmentProperties {
    #[doc = "Vulnerability ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "User friendly display name of the sub-assessment"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Status of the sub-assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubAssessmentStatus>,
    #[doc = "Information on how to remediate this sub-assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[doc = "Description of the impact of this sub-assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[doc = "Category of the sub-assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Human readable description of the assessment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The date and time the sub-assessment was generated"]
    #[serde(rename = "timeGenerated", default, with = "azure_core::date::rfc3339::option")]
    pub time_generated: Option<time::OffsetDateTime>,
    #[doc = "Details of the resource that was assessed"]
    #[serde(rename = "resourceDetails", default, skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetailsUnion>,
    #[doc = "Details of the sub-assessment"]
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataUnion>,
}
impl SecuritySubAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security task that we recommend to do in order to strengthen security"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityTask {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes properties of a task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityTaskProperties>,
}
impl SecurityTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of security task recommendations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityTaskList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecurityTask>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityTaskList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecurityTaskList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Changing set of properties, depending on the task type that is derived from the name field"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityTaskParameters {
    #[doc = "Name of the task type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SecurityTaskParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes properties of a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityTaskProperties {
    #[doc = "State of the task (Active, Resolved etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The time this task was discovered in UTC"]
    #[serde(rename = "creationTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Changing set of properties, depending on the task type that is derived from the name field"]
    #[serde(rename = "securityTaskParameters", default, skip_serializing_if = "Option::is_none")]
    pub security_task_parameters: Option<SecurityTaskParameters>,
    #[doc = "The time this task's details were last changed in UTC"]
    #[serde(rename = "lastStateChangeTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_state_change_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Additional data on the state of the task"]
    #[serde(rename = "subState", default, skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<String>,
}
impl SecurityTaskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Sensitive Data Discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitiveDataDiscoveryProperties {
    #[doc = "Indicates whether Sensitive Data Discovery should be enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "A status describing the success/failure of the enablement/disablement operation."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<OperationStatus>,
}
impl SensitiveDataDiscoveryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SensitiveInfoTypesIds = Vec<String>;
#[doc = "The sensitivity label."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SensitivityLabel {
    #[doc = "The name of the sensitivity label."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The rank of the sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<sensitivity_label::Rank>,
    #[doc = "The order of the sensitivity label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Indicates whether the label is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl SensitivityLabel {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sensitivity_label {
    use super::*;
    #[doc = "The rank of the sensitivity label."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rank {
        None,
        Low,
        Medium,
        High,
        Critical,
    }
}
#[doc = "Describes the server vulnerability assessment details on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes ServerVulnerabilityAssessment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerVulnerabilityAssessmentProperties>,
}
impl ServerVulnerabilityAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes ServerVulnerabilityAssessment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessmentProperties {
    #[doc = "The provisioningState of the vulnerability assessment capability on the VM"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<server_vulnerability_assessment_properties::ProvisioningState>,
}
impl ServerVulnerabilityAssessmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod server_vulnerability_assessment_properties {
    use super::*;
    #[doc = "The provisioningState of the vulnerability assessment capability on the VM"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Deprovisioning,
    }
}
#[doc = "Describes the vulnerability assessments setting properties on Azure servers in the defined scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVulnerabilityAssessmentsAzureSettingProperties {
    #[doc = "The selected vulnerability assessments provider on Azure servers in the defined scope."]
    #[serde(rename = "selectedProvider")]
    pub selected_provider: server_vulnerability_assessments_azure_setting_properties::SelectedProvider,
}
impl ServerVulnerabilityAssessmentsAzureSettingProperties {
    pub fn new(selected_provider: server_vulnerability_assessments_azure_setting_properties::SelectedProvider) -> Self {
        Self { selected_provider }
    }
}
pub mod server_vulnerability_assessments_azure_setting_properties {
    use super::*;
    #[doc = "The selected vulnerability assessments provider on Azure servers in the defined scope."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SelectedProvider")]
    pub enum SelectedProvider {
        MdeTvm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SelectedProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SelectedProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SelectedProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MdeTvm => serializer.serialize_unit_variant("SelectedProvider", 0u32, "MdeTvm"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of server vulnerability assessments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessmentsList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerVulnerabilityAssessment>,
}
impl ServerVulnerabilityAssessmentsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A base vulnerability assessments setting on servers in the defined scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVulnerabilityAssessmentsSetting {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ServerVulnerabilityAssessmentsSetting {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
        }
    }
}
#[doc = "The kind of the server vulnerability assessments setting"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerVulnerabilityAssessmentsSettingUnion {
    AzureServersSetting(AzureServersSetting),
}
#[doc = "The kind of the server vulnerability assessments setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerVulnerabilityAssessmentsSettingKind")]
pub enum ServerVulnerabilityAssessmentsSettingKind {
    AzureServersSetting,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerVulnerabilityAssessmentsSettingKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerVulnerabilityAssessmentsSettingKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerVulnerabilityAssessmentsSettingKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureServersSetting => {
                serializer.serialize_unit_variant("ServerVulnerabilityAssessmentsSettingKind", 0u32, "AzureServersSetting")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A page of a server vulnerability assessments settings list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVulnerabilityAssessmentsSettingsList {
    #[doc = "A collection of server vulnerability assessments settings in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServerVulnerabilityAssessmentsSettingUnion>,
    #[doc = "The URI to fetch the next page"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerVulnerabilityAssessmentsSettingsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerVulnerabilityAssessmentsSettingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional context fields for server vulnerability assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVulnerabilityProperties {
    #[doc = "Vulnerability Type. e.g: Vulnerability, Potential Vulnerability, Information Gathered"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Dictionary from cvss version to cvss details object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss: Option<serde_json::Value>,
    #[doc = "Indicates whether a patch is available or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patchable: Option<bool>,
    #[doc = "List of CVEs"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cve: Vec<Cve>,
    #[doc = "Threat name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threat: Option<String>,
    #[doc = "Published time"]
    #[serde(rename = "publishedTime", default, with = "azure_core::date::rfc3339::option")]
    pub published_time: Option<time::OffsetDateTime>,
    #[serde(
        rename = "vendorReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vendor_references: Vec<VendorReference>,
}
impl ServerVulnerabilityProperties {
    pub fn new() -> Self {
        Self {
            type_: None,
            cvss: None,
            patchable: None,
            cve: Vec::new(),
            threat: None,
            published_time: None,
            vendor_references: Vec::new(),
        }
    }
}
#[doc = "Details of the service principal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalProperties {
    #[doc = "Application ID of service principal."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "A secret string that the application uses to prove its identity, also can be referred to as application password (write only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl ServicePrincipalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the security setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    #[serde(flatten)]
    pub resource: Resource,
}
impl Setting {
    pub fn new() -> Self {
        Self {
            resource: Resource::default(),
        }
    }
}
#[doc = "the kind of the settings string"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SettingUnion {
    AlertSyncSettings(AlertSyncSettings),
    DataExportSettings(DataExportSettings),
}
#[doc = "Subscription settings list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingsList {
    #[doc = "The settings list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SettingUnion>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SettingsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SettingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a software data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Software {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Software Inventory resource properties"]
    pub properties: SoftwareProperties,
}
impl Software {
    pub fn new(properties: SoftwareProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Software Inventory resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareProperties {
    #[doc = "Unique identifier for the virtual machine in the service."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Platform of the operating system running on the device."]
    #[serde(rename = "osPlatform", default, skip_serializing_if = "Option::is_none")]
    pub os_platform: Option<String>,
    #[doc = "Name of the software vendor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[doc = "Name of the software product."]
    #[serde(rename = "softwareName", default, skip_serializing_if = "Option::is_none")]
    pub software_name: Option<String>,
    #[doc = "Version number of the software product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "End of support status."]
    #[serde(rename = "endOfSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub end_of_support_status: Option<software_properties::EndOfSupportStatus>,
    #[doc = "The end of support date in case the product is upcoming end of support."]
    #[serde(rename = "endOfSupportDate", default, skip_serializing_if = "Option::is_none")]
    pub end_of_support_date: Option<String>,
    #[doc = "Number of weaknesses."]
    #[serde(rename = "numberOfKnownVulnerabilities", default, skip_serializing_if = "Option::is_none")]
    pub number_of_known_vulnerabilities: Option<i32>,
    #[doc = "First time that the software was seen in the device."]
    #[serde(rename = "firstSeenAt", default, skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<String>,
}
impl SoftwareProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod software_properties {
    use super::*;
    #[doc = "End of support status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndOfSupportStatus")]
    pub enum EndOfSupportStatus {
        None,
        #[serde(rename = "noLongerSupported")]
        NoLongerSupported,
        #[serde(rename = "versionNoLongerSupported")]
        VersionNoLongerSupported,
        #[serde(rename = "upcomingNoLongerSupported")]
        UpcomingNoLongerSupported,
        #[serde(rename = "upcomingVersionNoLongerSupported")]
        UpcomingVersionNoLongerSupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndOfSupportStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndOfSupportStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndOfSupportStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("EndOfSupportStatus", 0u32, "None"),
                Self::NoLongerSupported => serializer.serialize_unit_variant("EndOfSupportStatus", 1u32, "noLongerSupported"),
                Self::VersionNoLongerSupported => serializer.serialize_unit_variant("EndOfSupportStatus", 2u32, "versionNoLongerSupported"),
                Self::UpcomingNoLongerSupported => {
                    serializer.serialize_unit_variant("EndOfSupportStatus", 3u32, "upcomingNoLongerSupported")
                }
                Self::UpcomingVersionNoLongerSupported => {
                    serializer.serialize_unit_variant("EndOfSupportStatus", 4u32, "upcomingVersionNoLongerSupported")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the software inventory of the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwaresList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Software>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SoftwaresList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SoftwaresList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The source type of the machine group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceSystem {
    #[serde(rename = "Azure_AppLocker")]
    AzureAppLocker,
    #[serde(rename = "Azure_AuditD")]
    AzureAuditD,
    #[serde(rename = "NonAzure_AppLocker")]
    NonAzureAppLocker,
    #[serde(rename = "NonAzure_AuditD")]
    NonAzureAuditD,
    None,
}
#[doc = "Details of the resource that was assessed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerVulnerabilityProperties {
    #[doc = "The resource type the sub assessment refers to in its resource details"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The T-SQL query that runs on your SQL database to perform the particular check"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl SqlServerVulnerabilityProperties {
    pub fn new() -> Self {
        Self { type_: None, query: None }
    }
}
#[doc = "Status of the sub-assessment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubAssessmentStatus {
    #[doc = "Programmatic code for the status of the assessment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<sub_assessment_status::Code>,
    #[doc = "Programmatic code for the cause of the assessment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[doc = "Human readable description of the assessment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The sub-assessment severity level"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<sub_assessment_status::Severity>,
}
impl SubAssessmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sub_assessment_status {
    use super::*;
    #[doc = "Programmatic code for the status of the assessment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Healthy,
        Unhealthy,
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
                Self::Unhealthy => serializer.serialize_unit_variant("Code", 1u32, "Unhealthy"),
                Self::NotApplicable => serializer.serialize_unit_variant("Code", 2u32, "NotApplicable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The sub-assessment severity level"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Low,
        Medium,
        High,
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
                Self::Low => serializer.serialize_unit_variant("Severity", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("Severity", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("Severity", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionAlertsScope {
    #[doc = "All the conditions inside need to be true in order to suppress the alert"]
    #[serde(rename = "allOf")]
    pub all_of: Vec<ScopeElement>,
}
impl SuppressionAlertsScope {
    pub fn new(all_of: Vec<ScopeElement>) -> Self {
        Self { all_of }
    }
}
#[doc = "A list of key value pairs that describe the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "A list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags."]
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
#[doc = "Repository branch configuration for PR Annotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetBranchConfiguration {
    #[doc = "Gets or sets branches that should have annotations."]
    #[serde(
        rename = "branchNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_names: Vec<String>,
    #[doc = "Configuration of PR Annotations on default branch.\r\n\r\nEnabled - PR Annotations are enabled on the resource's default branch.\r\nDisabled - PR Annotations are disabled on the resource's default branch."]
    #[serde(rename = "annotateDefaultBranch", default, skip_serializing_if = "Option::is_none")]
    pub annotate_default_branch: Option<AnnotateDefaultBranchState>,
}
impl TargetBranchConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom alert rule that checks if a value (depends on the custom alert type) is within the given range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdCustomAlertRule {
    #[serde(flatten)]
    pub custom_alert_rule: CustomAlertRule,
    #[doc = "The minimum threshold."]
    #[serde(rename = "minThreshold")]
    pub min_threshold: i32,
    #[doc = "The maximum threshold."]
    #[serde(rename = "maxThreshold")]
    pub max_threshold: i32,
}
impl ThresholdCustomAlertRule {
    pub fn new(custom_alert_rule: CustomAlertRule, min_threshold: i32, max_threshold: i32) -> Self {
        Self {
            custom_alert_rule,
            min_threshold,
            max_threshold,
        }
    }
}
#[doc = "A custom alert rule that checks if the number of activities (depends on the custom alert type) in a time window is within the given range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindowCustomAlertRule {
    #[serde(flatten)]
    pub threshold_custom_alert_rule: ThresholdCustomAlertRule,
    #[doc = "The time window size in iso8601 format."]
    #[serde(rename = "timeWindowSize")]
    pub time_window_size: String,
}
impl TimeWindowCustomAlertRule {
    pub fn new(threshold_custom_alert_rule: ThresholdCustomAlertRule, time_window_size: String) -> Self {
        Self {
            threshold_custom_alert_rule,
            time_window_size,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologyList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TopologyResource>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TopologyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TopologyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologyResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopologyResourceProperties>,
}
impl TopologyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologyResourceProperties {
    #[doc = "The UTC time on which the topology was calculated"]
    #[serde(rename = "calculatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub calculated_date_time: Option<time::OffsetDateTime>,
    #[doc = "Azure resources which are part of this topology resource"]
    #[serde(
        rename = "topologyResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub topology_resources: Vec<TopologySingleResource>,
}
impl TopologyResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologySingleResource {
    #[doc = "Azure resource id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The security severity of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Indicates if the resource has security recommendations"]
    #[serde(rename = "recommendationsExist", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_exist: Option<bool>,
    #[doc = "Indicates the resource connectivity level to the Internet (InternetFacing, Internal ,etc.)"]
    #[serde(rename = "networkZones", default, skip_serializing_if = "Option::is_none")]
    pub network_zones: Option<String>,
    #[doc = "Score of the resource based on its security severity"]
    #[serde(rename = "topologyScore", default, skip_serializing_if = "Option::is_none")]
    pub topology_score: Option<i64>,
    #[doc = "The location of this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Azure resources connected to this resource which are in higher level in the topology view"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parents: Vec<TopologySingleResourceParent>,
    #[doc = "Azure resources connected to this resource which are in lower level in the topology view"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children: Vec<TopologySingleResourceChild>,
}
impl TopologySingleResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologySingleResourceChild {
    #[doc = "Azure resource id which serves as child resource in topology view"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl TopologySingleResourceChild {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TopologySingleResourceParent {
    #[doc = "Azure resource id which serves as parent resource in topology view"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl TopologySingleResourceParent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure tracked resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub azure_tracked_resource_location: AzureTrackedResourceLocation,
    #[serde(flatten)]
    pub kind: Kind,
    #[serde(flatten)]
    pub e_tag: ETag,
    #[serde(flatten)]
    pub tags: Tags,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Number of twin updates is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwinUpdatesNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl TwinUpdatesNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Number of unauthorized operations is not in allowed range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnauthorizedOperationsNotInAllowedRange {
    #[serde(flatten)]
    pub time_window_custom_alert_rule: TimeWindowCustomAlertRule,
}
impl UnauthorizedOperationsNotInAllowedRange {
    pub fn new(time_window_custom_alert_rule: TimeWindowCustomAlertRule) -> Self {
        Self {
            time_window_custom_alert_rule,
        }
    }
}
#[doc = "Update Security Solution setting data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIoTSecuritySolutionProperties {
    #[doc = "Properties of the IoT Security solution's user defined resources."]
    #[serde(rename = "userDefinedResources", default, skip_serializing_if = "Option::is_none")]
    pub user_defined_resources: Option<UserDefinedResourcesProperties>,
    #[doc = "List of the configuration status for each recommendation type."]
    #[serde(rename = "recommendationsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_configuration: Option<RecommendationConfigurationList>,
}
impl UpdateIoTSecuritySolutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIotSecuritySolutionData {
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "Update Security Solution setting data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateIoTSecuritySolutionProperties>,
}
impl UpdateIotSecuritySolutionData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to update data sensitivity settings for sensitive data discovery"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSensitivitySettingsRequest {
    #[doc = "List of selected sensitive info types' IDs."]
    #[serde(rename = "sensitiveInfoTypesIds")]
    pub sensitive_info_types_ids: SensitiveInfoTypesIds,
    #[doc = "The order of the sensitivity threshold label. Any label at or above this order will be considered sensitive. If set to -1, sensitivity by labels is turned off"]
    #[serde(rename = "sensitivityThresholdLabelOrder", default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_threshold_label_order: Option<f64>,
    #[doc = "The id of the sensitivity threshold label. Any label at or above this rank will be considered sensitive."]
    #[serde(rename = "sensitivityThresholdLabelId", default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_threshold_label_id: Option<String>,
}
impl UpdateSensitivitySettingsRequest {
    pub fn new(sensitive_info_types_ids: SensitiveInfoTypesIds) -> Self {
        Self {
            sensitive_info_types_ids,
            sensitivity_threshold_label_order: None,
            sensitivity_threshold_label_id: None,
        }
    }
}
#[doc = "Properties of the IoT Security solution's user defined resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDefinedResourcesProperties {
    #[doc = "Azure Resource Graph query which represents the security solution's user defined resources. Required to start with \"where type != \"Microsoft.Devices/IotHubs\"\""]
    pub query: String,
    #[doc = "List of Azure subscription ids on which the user defined resources query should be executed."]
    #[serde(rename = "querySubscriptions")]
    pub query_subscriptions: Vec<String>,
}
impl UserDefinedResourcesProperties {
    pub fn new(query: String, query_subscriptions: Vec<String>) -> Self {
        Self {
            query,
            query_subscriptions,
        }
    }
}
#[doc = "Represents a user that is recommended to be allowed for a certain rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRecommendation {
    #[doc = "Represents a user that is recommended to be allowed for a certain rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The recommendation action of the machine or rule"]
    #[serde(rename = "recommendationAction", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_action: Option<RecommendationAction>,
}
impl UserRecommendation {
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
#[doc = "Vendor reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorReference {
    #[doc = "Link title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Link url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl VendorReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a machine that is part of a machine group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmRecommendation {
    #[doc = "The configuration status of the machines group or machine or rule"]
    #[serde(rename = "configurationStatus", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status: Option<ConfigurationStatus>,
    #[doc = "The recommendation action of the machine or rule"]
    #[serde(rename = "recommendationAction", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_action: Option<RecommendationAction>,
    #[doc = "The full resource id of the machine"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<VmResourceId>,
    #[doc = "The machine supportability of Enforce feature"]
    #[serde(rename = "enforcementSupport", default, skip_serializing_if = "Option::is_none")]
    pub enforcement_support: Option<EnforcementSupport>,
}
impl VmRecommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type VmRecommendations = Vec<VmRecommendation>;
pub type VmResourceId = String;
#[doc = "Configures where to store the OMS agent data for workspaces under a scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Workspace setting data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceSettingProperties>,
}
impl WorkspaceSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of workspace settings response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSettingList {
    #[doc = "List of workspace settings"]
    pub value: Vec<WorkspaceSetting>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceSettingList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceSettingList {
    pub fn new(value: Vec<WorkspaceSetting>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Workspace setting data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSettingProperties {
    #[doc = "The full Azure ID of the workspace to save the data in"]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "All the VMs in this scope will send their security data to the mentioned workspace unless overridden by a setting with more specific scope"]
    pub scope: String,
}
impl WorkspaceSettingProperties {
    pub fn new(workspace_id: String, scope: String) -> Self {
        Self { workspace_id, scope }
    }
}
#[doc = "The security offering details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudOffering {
    #[doc = "The offering description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CloudOffering {
    pub fn new() -> Self {
        Self { description: None }
    }
}
#[doc = "The type of the security offering."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "offeringType")]
pub enum CloudOfferingUnion {
    CspmMonitorAws(CspmMonitorAwsOffering),
    CspmMonitorAzureDevOps(CspmMonitorAzureDevOpsOffering),
    CspmMonitorGcp(CspmMonitorGcpOffering),
    CspmMonitorGitLab(CspmMonitorGitLabOffering),
    CspmMonitorGithub(CspmMonitorGithubOffering),
    DefenderCspmAws(DefenderCspmAwsOffering),
    DefenderCspmGcp(DefenderCspmGcpOffering),
    DefenderForDatabasesAws(DefenderFoDatabasesAwsOffering),
    DefenderForContainersAws(DefenderForContainersAwsOffering),
    DefenderForContainersGcp(DefenderForContainersGcpOffering),
    DefenderForDatabasesGcp(DefenderForDatabasesGcpOffering),
    DefenderForDevOpsAzureDevOps(DefenderForDevOpsAzureDevOpsOffering),
    DefenderForDevOpsGitLab(DefenderForDevOpsGitLabOffering),
    DefenderForDevOpsGithub(DefenderForDevOpsGithubOffering),
    DefenderForServersAws(DefenderForServersAwsOffering),
    DefenderForServersGcp(DefenderForServersGcpOffering),
    InformationProtectionAws(InformationProtectionAwsOffering),
}
#[doc = "The CSPM monitoring for AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<cspm_monitor_aws_offering::NativeCloudConnection>,
}
impl CspmMonitorAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
        }
    }
}
pub mod cspm_monitor_aws_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The CSPM monitoring for AzureDevOps offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorAzureDevOpsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl CspmMonitorAzureDevOpsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The CSPM monitoring for GCP offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<cspm_monitor_gcp_offering::NativeCloudConnection>,
}
impl CspmMonitorGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
        }
    }
}
pub mod cspm_monitor_gcp_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The GCP workload identity provider id for the offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The CSPM (Cloud security posture management) monitoring for gitlab offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorGitLabOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl CspmMonitorGitLabOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The CSPM monitoring for github offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorGithubOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl CspmMonitorGithubOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "Custom Assessment Automation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomAssessmentAutomation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "describes the Custom Assessment Automation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomAssessmentAutomationProperties>,
}
impl CustomAssessmentAutomation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes the Custom Assessment Automation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomAssessmentAutomationProperties {
    #[doc = "GZip encoded KQL query representing the assessment automation results required."]
    #[serde(rename = "compressedQuery", default, skip_serializing_if = "Option::is_none")]
    pub compressed_query: Option<String>,
    #[doc = "Relevant cloud for the custom assessment automation."]
    #[serde(rename = "supportedCloud", default, skip_serializing_if = "Option::is_none")]
    pub supported_cloud: Option<custom_assessment_automation_properties::SupportedCloud>,
    #[doc = "The severity to relate to the assessments generated by this assessment automation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<custom_assessment_automation_properties::Severity>,
    #[doc = "The display name of the assessments generated by this assessment automation."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description to relate to the assessments generated by this assessment automation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The remediation description to relate to the assessments generated by this assessment automation."]
    #[serde(rename = "remediationDescription", default, skip_serializing_if = "Option::is_none")]
    pub remediation_description: Option<String>,
    #[doc = "The assessment metadata key used when an assessment is generated for this assessment automation."]
    #[serde(rename = "assessmentKey", default, skip_serializing_if = "Option::is_none")]
    pub assessment_key: Option<String>,
}
impl CustomAssessmentAutomationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_assessment_automation_properties {
    use super::*;
    #[doc = "Relevant cloud for the custom assessment automation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportedCloud")]
    pub enum SupportedCloud {
        #[serde(rename = "AWS")]
        Aws,
        #[serde(rename = "GCP")]
        Gcp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SupportedCloud {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SupportedCloud {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SupportedCloud {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aws => serializer.serialize_unit_variant("SupportedCloud", 0u32, "AWS"),
                Self::Gcp => serializer.serialize_unit_variant("SupportedCloud", 1u32, "GCP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The severity to relate to the assessments generated by this assessment automation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        High,
        Medium,
        Low,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Custom Assessment Automation request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomAssessmentAutomationRequest {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "describes the Custom Assessment Automation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomAssessmentAutomationRequestProperties>,
}
impl CustomAssessmentAutomationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "describes the Custom Assessment Automation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomAssessmentAutomationRequestProperties {
    #[doc = "Base 64 encoded KQL query representing the assessment automation results required."]
    #[serde(rename = "compressedQuery", default, skip_serializing_if = "Option::is_none")]
    pub compressed_query: Option<String>,
    #[doc = "Relevant cloud for the custom assessment automation."]
    #[serde(rename = "supportedCloud", default, skip_serializing_if = "Option::is_none")]
    pub supported_cloud: Option<custom_assessment_automation_request_properties::SupportedCloud>,
    #[doc = "The severity to relate to the assessments generated by this assessment automation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<custom_assessment_automation_request_properties::Severity>,
    #[doc = "The display name of the assessments generated by this assessment automation."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description to relate to the assessments generated by this assessment automation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The remediation description to relate to the assessments generated by this assessment automation."]
    #[serde(rename = "remediationDescription", default, skip_serializing_if = "Option::is_none")]
    pub remediation_description: Option<String>,
}
impl CustomAssessmentAutomationRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod custom_assessment_automation_request_properties {
    use super::*;
    #[doc = "Relevant cloud for the custom assessment automation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportedCloud")]
    pub enum SupportedCloud {
        #[serde(rename = "AWS")]
        Aws,
        #[serde(rename = "GCP")]
        Gcp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SupportedCloud {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SupportedCloud {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SupportedCloud {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aws => serializer.serialize_unit_variant("SupportedCloud", 0u32, "AWS"),
                Self::Gcp => serializer.serialize_unit_variant("SupportedCloud", 1u32, "GCP"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The severity to relate to the assessments generated by this assessment automation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        High,
        Medium,
        Low,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of Custom Assessment Automations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomAssessmentAutomationsListResult {
    #[doc = "Collection of Custom Assessment Automations"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CustomAssessmentAutomation>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomAssessmentAutomationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CustomAssessmentAutomationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The CSPM P1 for AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderCspmAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[serde(rename = "vmScanners", default, skip_serializing_if = "Option::is_none")]
    pub vm_scanners: Option<defender_cspm_aws_offering::VmScanners>,
    #[doc = "The Microsoft Defender Data Sensitivity discovery configuration"]
    #[serde(rename = "dataSensitivityDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub data_sensitivity_discovery: Option<defender_cspm_aws_offering::DataSensitivityDiscovery>,
    #[doc = "The databases DSPM configuration"]
    #[serde(rename = "databasesDspm", default, skip_serializing_if = "Option::is_none")]
    pub databases_dspm: Option<defender_cspm_aws_offering::DatabasesDspm>,
    #[doc = "Defenders CSPM Cloud infrastructure entitlement management (CIEM) offering configurations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ciem: Option<defender_cspm_aws_offering::Ciem>,
    #[doc = "The Microsoft Defender container image assessment configuration"]
    #[serde(rename = "mdcContainersImageAssessment", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_image_assessment: Option<defender_cspm_aws_offering::MdcContainersImageAssessment>,
    #[doc = "The Microsoft Defender container agentless discovery K8s configuration"]
    #[serde(rename = "mdcContainersAgentlessDiscoveryK8s", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_agentless_discovery_k8s: Option<defender_cspm_aws_offering::MdcContainersAgentlessDiscoveryK8s>,
}
impl DefenderCspmAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            vm_scanners: None,
            data_sensitivity_discovery: None,
            databases_dspm: None,
            ciem: None,
            mdc_containers_image_assessment: None,
            mdc_containers_agentless_discovery_k8s: None,
        }
    }
}
pub mod defender_cspm_aws_offering {
    use super::*;
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VmScanners {
        #[doc = "Is Microsoft Defender for Server VM scanning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<vm_scanners::Configuration>,
    }
    impl VmScanners {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod vm_scanners {
        use super::*;
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The cloud role ARN in AWS for this feature"]
            #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
            pub cloud_role_arn: Option<String>,
            #[doc = "The scanning mode for the VM scan."]
            #[serde(rename = "scanningMode", default, skip_serializing_if = "Option::is_none")]
            pub scanning_mode: Option<configuration::ScanningMode>,
            #[doc = "VM tags that indicates that VM should not be scanned"]
            #[serde(rename = "exclusionTags", default, skip_serializing_if = "Option::is_none")]
            pub exclusion_tags: Option<serde_json::Value>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The scanning mode for the VM scan."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "ScanningMode")]
            pub enum ScanningMode {
                Default,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for ScanningMode {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for ScanningMode {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for ScanningMode {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::Default => serializer.serialize_unit_variant("ScanningMode", 0u32, "Default"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "The Microsoft Defender Data Sensitivity discovery configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataSensitivityDiscovery {
        #[doc = "Is Microsoft Defender Data Sensitivity discovery enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl DataSensitivityDiscovery {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The databases DSPM configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DatabasesDspm {
        #[doc = "Is databases DSPM protection enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl DatabasesDspm {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Defenders CSPM Cloud infrastructure entitlement management (CIEM) offering configurations"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ciem {
        #[doc = "Defender CSPM CIEM discovery configuration"]
        #[serde(rename = "ciemDiscovery", default, skip_serializing_if = "Option::is_none")]
        pub ciem_discovery: Option<ciem::CiemDiscovery>,
        #[doc = "Defender CSPM CIEM AWS OIDC (open id connect) configuration"]
        #[serde(rename = "ciemOidc", default, skip_serializing_if = "Option::is_none")]
        pub ciem_oidc: Option<ciem::CiemOidc>,
    }
    impl Ciem {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod ciem {
        use super::*;
        #[doc = "Defender CSPM CIEM discovery configuration"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct CiemDiscovery {
            #[doc = "The cloud role ARN in AWS for CIEM discovery"]
            #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
            pub cloud_role_arn: Option<String>,
        }
        impl CiemDiscovery {
            pub fn new() -> Self {
                Self::default()
            }
        }
        #[doc = "Defender CSPM CIEM AWS OIDC (open id connect) configuration"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct CiemOidc {
            #[doc = "The cloud role ARN in AWS for CIEM oidc connection"]
            #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
            pub cloud_role_arn: Option<String>,
            #[doc = "the azure active directory app name used of authenticating against AWS"]
            #[serde(rename = "azureActiveDirectoryAppName", default, skip_serializing_if = "Option::is_none")]
            pub azure_active_directory_app_name: Option<String>,
        }
        impl CiemOidc {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The Microsoft Defender container image assessment configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersImageAssessment {
        #[doc = "Is Microsoft Defender container image assessment enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl MdcContainersImageAssessment {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender container agentless discovery K8s configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersAgentlessDiscoveryK8s {
        #[doc = "Is Microsoft Defender container agentless discovery K8s enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl MdcContainersAgentlessDiscoveryK8s {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The CSPM P1 for GCP offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderCspmGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "GCP Defenders CSPM Cloud infrastructure entitlement management (CIEM) discovery offering configurations"]
    #[serde(rename = "ciemDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub ciem_discovery: Option<defender_cspm_gcp_offering::CiemDiscovery>,
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[serde(rename = "vmScanners", default, skip_serializing_if = "Option::is_none")]
    pub vm_scanners: Option<defender_cspm_gcp_offering::VmScanners>,
    #[doc = "The Microsoft Defender Data Sensitivity discovery configuration"]
    #[serde(rename = "dataSensitivityDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub data_sensitivity_discovery: Option<defender_cspm_gcp_offering::DataSensitivityDiscovery>,
    #[doc = "The Microsoft Defender Container image assessment configuration"]
    #[serde(rename = "mdcContainersImageAssessment", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_image_assessment: Option<defender_cspm_gcp_offering::MdcContainersImageAssessment>,
    #[doc = "The Microsoft Defender Container agentless discovery configuration"]
    #[serde(rename = "mdcContainersAgentlessDiscoveryK8s", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_agentless_discovery_k8s: Option<defender_cspm_gcp_offering::MdcContainersAgentlessDiscoveryK8s>,
}
impl DefenderCspmGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            ciem_discovery: None,
            vm_scanners: None,
            data_sensitivity_discovery: None,
            mdc_containers_image_assessment: None,
            mdc_containers_agentless_discovery_k8s: None,
        }
    }
}
pub mod defender_cspm_gcp_offering {
    use super::*;
    #[doc = "GCP Defenders CSPM Cloud infrastructure entitlement management (CIEM) discovery offering configurations"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CiemDiscovery {
        #[doc = "The GCP workload identity provider id for CIEM discovery offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for CIEM discovery offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "the azure active directory app name used of authenticating against GCP workload identity federation"]
        #[serde(rename = "azureActiveDirectoryAppName", default, skip_serializing_if = "Option::is_none")]
        pub azure_active_directory_app_name: Option<String>,
    }
    impl CiemDiscovery {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VmScanners {
        #[doc = "Is Microsoft Defender for Server VM scanning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<vm_scanners::Configuration>,
    }
    impl VmScanners {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod vm_scanners {
        use super::*;
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The scanning mode for the VM scan."]
            #[serde(rename = "scanningMode", default, skip_serializing_if = "Option::is_none")]
            pub scanning_mode: Option<configuration::ScanningMode>,
            #[doc = "VM tags that indicates that VM should not be scanned"]
            #[serde(rename = "exclusionTags", default, skip_serializing_if = "Option::is_none")]
            pub exclusion_tags: Option<serde_json::Value>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The scanning mode for the VM scan."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "ScanningMode")]
            pub enum ScanningMode {
                Default,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for ScanningMode {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for ScanningMode {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for ScanningMode {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::Default => serializer.serialize_unit_variant("ScanningMode", 0u32, "Default"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "The Microsoft Defender Data Sensitivity discovery configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataSensitivityDiscovery {
        #[doc = "Is Microsoft Defender Data Sensitivity discovery enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl DataSensitivityDiscovery {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender Container image assessment configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersImageAssessment {
        #[doc = "Is Microsoft Defender container image assessment enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl MdcContainersImageAssessment {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender Container agentless discovery configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersAgentlessDiscoveryK8s {
        #[doc = "Is Microsoft Defender container agentless discovery enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl MdcContainersAgentlessDiscoveryK8s {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Databases AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderFoDatabasesAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_fo_databases_aws_offering::ArcAutoProvisioning>,
    #[doc = "The RDS configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rds: Option<defender_fo_databases_aws_offering::Rds>,
    #[doc = "The databases data security posture management (DSPM) configuration"]
    #[serde(rename = "databasesDspm", default, skip_serializing_if = "Option::is_none")]
    pub databases_dspm: Option<defender_fo_databases_aws_offering::DatabasesDspm>,
}
impl DefenderFoDatabasesAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            arc_auto_provisioning: None,
            rds: None,
            databases_dspm: None,
        }
    }
}
pub mod defender_fo_databases_aws_offering {
    use super::*;
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<arc_auto_provisioning::Configuration>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "Optional http proxy endpoint to use for the Arc agent"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub proxy: Option<String>,
            #[doc = "Optional Arc private link scope resource id to link the Arc agent"]
            #[serde(rename = "privateLinkScope", default, skip_serializing_if = "Option::is_none")]
            pub private_link_scope: Option<String>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The RDS configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Rds {
        #[doc = "Is RDS protection enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl Rds {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The databases data security posture management (DSPM) configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DatabasesDspm {
        #[doc = "Is databases data security posture management (DSPM) protection enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl DatabasesDspm {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Containers AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForContainersAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The kubernetes service connection configuration"]
    #[serde(rename = "kubernetesService", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_service: Option<defender_for_containers_aws_offering::KubernetesService>,
    #[doc = "The kubernetes to scuba connection configuration"]
    #[serde(rename = "kubernetesScubaReader", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_scuba_reader: Option<defender_for_containers_aws_offering::KubernetesScubaReader>,
    #[doc = "The cloudwatch to kinesis connection configuration"]
    #[serde(rename = "cloudWatchToKinesis", default, skip_serializing_if = "Option::is_none")]
    pub cloud_watch_to_kinesis: Option<defender_for_containers_aws_offering::CloudWatchToKinesis>,
    #[doc = "The kinesis to s3 connection configuration"]
    #[serde(rename = "kinesisToS3", default, skip_serializing_if = "Option::is_none")]
    pub kinesis_to_s3: Option<defender_for_containers_aws_offering::KinesisToS3>,
    #[doc = "The container vulnerability assessment configuration"]
    #[serde(rename = "containerVulnerabilityAssessment", default, skip_serializing_if = "Option::is_none")]
    pub container_vulnerability_assessment: Option<defender_for_containers_aws_offering::ContainerVulnerabilityAssessment>,
    #[doc = "The container vulnerability assessment task configuration"]
    #[serde(rename = "containerVulnerabilityAssessmentTask", default, skip_serializing_if = "Option::is_none")]
    pub container_vulnerability_assessment_task: Option<defender_for_containers_aws_offering::ContainerVulnerabilityAssessmentTask>,
    #[doc = "Enable container vulnerability assessment feature"]
    #[serde(
        rename = "enableContainerVulnerabilityAssessment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_container_vulnerability_assessment: Option<bool>,
    #[doc = "Is audit logs pipeline auto provisioning enabled"]
    #[serde(rename = "autoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub auto_provisioning: Option<bool>,
    #[doc = "The retention time in days of kube audit logs set on the CloudWatch log group"]
    #[serde(rename = "kubeAuditRetentionTime", default, skip_serializing_if = "Option::is_none")]
    pub kube_audit_retention_time: Option<i64>,
    #[doc = "The externalId used by the data reader to prevent the confused deputy attack"]
    #[serde(rename = "scubaExternalId", default, skip_serializing_if = "Option::is_none")]
    pub scuba_external_id: Option<String>,
    #[doc = "The Microsoft Defender container image assessment configuration"]
    #[serde(rename = "mdcContainersImageAssessment", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_image_assessment: Option<defender_for_containers_aws_offering::MdcContainersImageAssessment>,
    #[doc = "The Microsoft Defender container agentless discovery K8s configuration"]
    #[serde(rename = "mdcContainersAgentlessDiscoveryK8s", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_agentless_discovery_k8s: Option<defender_for_containers_aws_offering::MdcContainersAgentlessDiscoveryK8s>,
}
impl DefenderForContainersAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            kubernetes_service: None,
            kubernetes_scuba_reader: None,
            cloud_watch_to_kinesis: None,
            kinesis_to_s3: None,
            container_vulnerability_assessment: None,
            container_vulnerability_assessment_task: None,
            enable_container_vulnerability_assessment: None,
            auto_provisioning: None,
            kube_audit_retention_time: None,
            scuba_external_id: None,
            mdc_containers_image_assessment: None,
            mdc_containers_agentless_discovery_k8s: None,
        }
    }
}
pub mod defender_for_containers_aws_offering {
    use super::*;
    #[doc = "The kubernetes service connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KubernetesService {
        #[doc = "The cloud role ARN in AWS for this feature used for provisioning resources"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KubernetesService {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The kubernetes to scuba connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KubernetesScubaReader {
        #[doc = "The cloud role ARN in AWS for this feature used for reading data"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KubernetesScubaReader {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The cloudwatch to kinesis connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CloudWatchToKinesis {
        #[doc = "The cloud role ARN in AWS used by CloudWatch to transfer data into Kinesis"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl CloudWatchToKinesis {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The kinesis to s3 connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KinesisToS3 {
        #[doc = "The cloud role ARN in AWS used by Kinesis to transfer data into S3"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KinesisToS3 {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The container vulnerability assessment configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ContainerVulnerabilityAssessment {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl ContainerVulnerabilityAssessment {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The container vulnerability assessment task configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ContainerVulnerabilityAssessmentTask {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl ContainerVulnerabilityAssessmentTask {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender container image assessment configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersImageAssessment {
        #[doc = "Is Microsoft Defender container image assessment enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl MdcContainersImageAssessment {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender container agentless discovery K8s configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersAgentlessDiscoveryK8s {
        #[doc = "Is Microsoft Defender container agentless discovery K8s enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl MdcContainersAgentlessDiscoveryK8s {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The containers GCP offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForContainersGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<defender_for_containers_gcp_offering::NativeCloudConnection>,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "dataPipelineNativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub data_pipeline_native_cloud_connection: Option<defender_for_containers_gcp_offering::DataPipelineNativeCloudConnection>,
    #[doc = "Is audit logs data collection enabled"]
    #[serde(rename = "auditLogsAutoProvisioningFlag", default, skip_serializing_if = "Option::is_none")]
    pub audit_logs_auto_provisioning_flag: Option<bool>,
    #[doc = "Is Microsoft Defender for Cloud Kubernetes agent auto provisioning enabled"]
    #[serde(rename = "defenderAgentAutoProvisioningFlag", default, skip_serializing_if = "Option::is_none")]
    pub defender_agent_auto_provisioning_flag: Option<bool>,
    #[doc = "Is Policy Kubernetes agent auto provisioning enabled"]
    #[serde(rename = "policyAgentAutoProvisioningFlag", default, skip_serializing_if = "Option::is_none")]
    pub policy_agent_auto_provisioning_flag: Option<bool>,
    #[doc = "The Microsoft Defender Container image assessment configuration"]
    #[serde(rename = "mdcContainersImageAssessment", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_image_assessment: Option<defender_for_containers_gcp_offering::MdcContainersImageAssessment>,
    #[doc = "The Microsoft Defender Container agentless discovery configuration"]
    #[serde(rename = "mdcContainersAgentlessDiscoveryK8s", default, skip_serializing_if = "Option::is_none")]
    pub mdc_containers_agentless_discovery_k8s: Option<defender_for_containers_gcp_offering::MdcContainersAgentlessDiscoveryK8s>,
}
impl DefenderForContainersGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
            data_pipeline_native_cloud_connection: None,
            audit_logs_auto_provisioning_flag: None,
            defender_agent_auto_provisioning_flag: None,
            policy_agent_auto_provisioning_flag: None,
            mdc_containers_image_assessment: None,
            mdc_containers_agentless_discovery_k8s: None,
        }
    }
}
pub mod defender_for_containers_gcp_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "The GCP workload identity provider id for this offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataPipelineNativeCloudConnection {
        #[doc = "The data collection service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "The data collection GCP workload identity provider id for this offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
    }
    impl DataPipelineNativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender Container image assessment configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersImageAssessment {
        #[doc = "Is Microsoft Defender container image assessment enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl MdcContainersImageAssessment {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The Microsoft Defender Container agentless discovery configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdcContainersAgentlessDiscoveryK8s {
        #[doc = "Is Microsoft Defender container agentless discovery enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl MdcContainersAgentlessDiscoveryK8s {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Databases GCP offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForDatabasesGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_databases_gcp_offering::ArcAutoProvisioning>,
    #[doc = "The native cloud connection configuration"]
    #[serde(
        rename = "defenderForDatabasesArcAutoProvisioning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub defender_for_databases_arc_auto_provisioning: Option<defender_for_databases_gcp_offering::DefenderForDatabasesArcAutoProvisioning>,
}
impl DefenderForDatabasesGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            arc_auto_provisioning: None,
            defender_for_databases_arc_auto_provisioning: None,
        }
    }
}
pub mod defender_for_databases_gcp_offering {
    use super::*;
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<arc_auto_provisioning::Configuration>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "Optional http proxy endpoint to use for the Arc agent"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub proxy: Option<String>,
            #[doc = "Optional Arc private link scope resource id to link the Arc agent"]
            #[serde(rename = "privateLinkScope", default, skip_serializing_if = "Option::is_none")]
            pub private_link_scope: Option<String>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForDatabasesArcAutoProvisioning {
        #[doc = "The service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "The GCP workload identity provider id for this offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
    }
    impl DefenderForDatabasesArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for DevOps for Azure DevOps offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForDevOpsAzureDevOpsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl DefenderForDevOpsAzureDevOpsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The Defender for DevOps for Gitlab offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForDevOpsGitLabOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl DefenderForDevOpsGitLabOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The Defender for DevOps for Github offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForDevOpsGithubOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl DefenderForDevOpsGithubOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The Defender for Servers AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForServersAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Defender for servers connection configuration"]
    #[serde(rename = "defenderForServers", default, skip_serializing_if = "Option::is_none")]
    pub defender_for_servers: Option<defender_for_servers_aws_offering::DefenderForServers>,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_servers_aws_offering::ArcAutoProvisioning>,
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[serde(rename = "vaAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub va_auto_provisioning: Option<defender_for_servers_aws_offering::VaAutoProvisioning>,
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[serde(rename = "mdeAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub mde_auto_provisioning: Option<defender_for_servers_aws_offering::MdeAutoProvisioning>,
    #[doc = "configuration for the servers offering subPlan"]
    #[serde(rename = "subPlan", default, skip_serializing_if = "Option::is_none")]
    pub sub_plan: Option<defender_for_servers_aws_offering::SubPlan>,
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[serde(rename = "vmScanners", default, skip_serializing_if = "Option::is_none")]
    pub vm_scanners: Option<defender_for_servers_aws_offering::VmScanners>,
}
impl DefenderForServersAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            defender_for_servers: None,
            arc_auto_provisioning: None,
            va_auto_provisioning: None,
            mde_auto_provisioning: None,
            sub_plan: None,
            vm_scanners: None,
        }
    }
}
pub mod defender_for_servers_aws_offering {
    use super::*;
    #[doc = "The Defender for servers connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForServers {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl DefenderForServers {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<arc_auto_provisioning::Configuration>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "Optional HTTP proxy endpoint to use for the Arc agent"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub proxy: Option<String>,
            #[doc = "Optional Arc private link scope resource id to link the Arc agent"]
            #[serde(rename = "privateLinkScope", default, skip_serializing_if = "Option::is_none")]
            pub private_link_scope: Option<String>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VaAutoProvisioning {
        #[doc = "Is Vulnerability Assessment auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<va_auto_provisioning::Configuration>,
    }
    impl VaAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod va_auto_provisioning {
        use super::*;
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<configuration::Type>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                Qualys,
                #[serde(rename = "TVM")]
                Tvm,
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
                        Self::Qualys => serializer.serialize_unit_variant("Type", 0u32, "Qualys"),
                        Self::Tvm => serializer.serialize_unit_variant("Type", 1u32, "TVM"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdeAutoProvisioning {
        #[doc = "Is Microsoft Defender for Endpoint auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Endpoint autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<serde_json::Value>,
    }
    impl MdeAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "configuration for the servers offering subPlan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SubPlan {
        #[doc = "The available sub plans"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<sub_plan::Type>,
    }
    impl SubPlan {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod sub_plan {
        use super::*;
        #[doc = "The available sub plans"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            P1,
            P2,
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
                    Self::P1 => serializer.serialize_unit_variant("Type", 0u32, "P1"),
                    Self::P2 => serializer.serialize_unit_variant("Type", 1u32, "P2"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VmScanners {
        #[doc = "Is Microsoft Defender for Server VM scanning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<vm_scanners::Configuration>,
    }
    impl VmScanners {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod vm_scanners {
        use super::*;
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The cloud role ARN in AWS for this feature"]
            #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
            pub cloud_role_arn: Option<String>,
            #[doc = "The scanning mode for the VM scan."]
            #[serde(rename = "scanningMode", default, skip_serializing_if = "Option::is_none")]
            pub scanning_mode: Option<configuration::ScanningMode>,
            #[doc = "VM tags that indicates that VM should not be scanned"]
            #[serde(rename = "exclusionTags", default, skip_serializing_if = "Option::is_none")]
            pub exclusion_tags: Option<serde_json::Value>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The scanning mode for the VM scan."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "ScanningMode")]
            pub enum ScanningMode {
                Default,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for ScanningMode {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for ScanningMode {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for ScanningMode {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::Default => serializer.serialize_unit_variant("ScanningMode", 0u32, "Default"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
}
#[doc = "The Defender for Servers GCP offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForServersGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Defender for servers connection configuration"]
    #[serde(rename = "defenderForServers", default, skip_serializing_if = "Option::is_none")]
    pub defender_for_servers: Option<defender_for_servers_gcp_offering::DefenderForServers>,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_servers_gcp_offering::ArcAutoProvisioning>,
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[serde(rename = "vaAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub va_auto_provisioning: Option<defender_for_servers_gcp_offering::VaAutoProvisioning>,
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[serde(rename = "mdeAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub mde_auto_provisioning: Option<defender_for_servers_gcp_offering::MdeAutoProvisioning>,
    #[doc = "configuration for the servers offering subPlan"]
    #[serde(rename = "subPlan", default, skip_serializing_if = "Option::is_none")]
    pub sub_plan: Option<defender_for_servers_gcp_offering::SubPlan>,
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[serde(rename = "vmScanners", default, skip_serializing_if = "Option::is_none")]
    pub vm_scanners: Option<defender_for_servers_gcp_offering::VmScanners>,
}
impl DefenderForServersGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            defender_for_servers: None,
            arc_auto_provisioning: None,
            va_auto_provisioning: None,
            mde_auto_provisioning: None,
            sub_plan: None,
            vm_scanners: None,
        }
    }
}
pub mod defender_for_servers_gcp_offering {
    use super::*;
    #[doc = "The Defender for servers connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForServers {
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl DefenderForServers {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<arc_auto_provisioning::Configuration>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Configuration for servers Arc auto provisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "Optional HTTP proxy endpoint to use for the Arc agent"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub proxy: Option<String>,
            #[doc = "Optional Arc private link scope resource id to link the Arc agent"]
            #[serde(rename = "privateLinkScope", default, skip_serializing_if = "Option::is_none")]
            pub private_link_scope: Option<String>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VaAutoProvisioning {
        #[doc = "Is Vulnerability Assessment auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<va_auto_provisioning::Configuration>,
    }
    impl VaAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod va_auto_provisioning {
        use super::*;
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<configuration::Type>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                Qualys,
                #[serde(rename = "TVM")]
                Tvm,
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
                        Self::Qualys => serializer.serialize_unit_variant("Type", 0u32, "Qualys"),
                        Self::Tvm => serializer.serialize_unit_variant("Type", 1u32, "TVM"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdeAutoProvisioning {
        #[doc = "Is Microsoft Defender for Endpoint auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Endpoint autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<serde_json::Value>,
    }
    impl MdeAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "configuration for the servers offering subPlan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SubPlan {
        #[doc = "The available sub plans"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<sub_plan::Type>,
    }
    impl SubPlan {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod sub_plan {
        use super::*;
        #[doc = "The available sub plans"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            P1,
            P2,
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
                    Self::P1 => serializer.serialize_unit_variant("Type", 0u32, "P1"),
                    Self::P2 => serializer.serialize_unit_variant("Type", 1u32, "P2"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VmScanners {
        #[doc = "Is Microsoft Defender for Server VM scanning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<vm_scanners::Configuration>,
    }
    impl VmScanners {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod vm_scanners {
        use super::*;
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The scanning mode for the VM scan."]
            #[serde(rename = "scanningMode", default, skip_serializing_if = "Option::is_none")]
            pub scanning_mode: Option<configuration::ScanningMode>,
            #[doc = "VM tags that indicate that VM should not be scanned"]
            #[serde(rename = "exclusionTags", default, skip_serializing_if = "Option::is_none")]
            pub exclusion_tags: Option<serde_json::Value>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The scanning mode for the VM scan."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "ScanningMode")]
            pub enum ScanningMode {
                Default,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for ScanningMode {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for ScanningMode {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for ScanningMode {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::Default => serializer.serialize_unit_variant("ScanningMode", 0u32, "Default"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
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
    pub scope: Option<String>,
}
impl HealthDataClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information protection for AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationProtectionAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "informationProtection", default, skip_serializing_if = "Option::is_none")]
    pub information_protection: Option<information_protection_aws_offering::InformationProtection>,
}
impl InformationProtectionAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            information_protection: None,
        }
    }
}
pub mod information_protection_aws_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InformationProtection {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl InformationProtection {
        pub fn new() -> Self {
            Self::default()
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
#[doc = "List of security controls definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureScoreControlDefinitionList {
    #[doc = "Collection of security controls definition in this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecureScoreControlDefinitionItem>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecureScoreControlDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SecureScoreControlDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySolutionsReferenceData {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    pub properties: SecuritySolutionsReferenceDataProperties,
}
impl SecuritySolutionsReferenceData {
    pub fn new(properties: SecuritySolutionsReferenceDataProperties) -> Self {
        Self {
            resource: Resource::default(),
            location: Location::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecuritySolutionsReferenceDataList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecuritySolutionsReferenceData>,
}
impl SecuritySolutionsReferenceDataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySolutionsReferenceDataProperties {
    #[doc = "The security family of the security solution"]
    #[serde(rename = "securityFamily")]
    pub security_family: security_solutions_reference_data_properties::SecurityFamily,
    #[doc = "The security solutions' vendor name"]
    #[serde(rename = "alertVendorName")]
    pub alert_vendor_name: String,
    #[doc = "The security solutions' package info url"]
    #[serde(rename = "packageInfoUrl")]
    pub package_info_url: String,
    #[doc = "The security solutions' product name"]
    #[serde(rename = "productName")]
    pub product_name: String,
    #[doc = "The security solutions' publisher"]
    pub publisher: String,
    #[doc = "The security solutions' publisher display name"]
    #[serde(rename = "publisherDisplayName")]
    pub publisher_display_name: String,
    #[doc = "The security solutions' template"]
    pub template: String,
}
impl SecuritySolutionsReferenceDataProperties {
    pub fn new(
        security_family: security_solutions_reference_data_properties::SecurityFamily,
        alert_vendor_name: String,
        package_info_url: String,
        product_name: String,
        publisher: String,
        publisher_display_name: String,
        template: String,
    ) -> Self {
        Self {
            security_family,
            alert_vendor_name,
            package_info_url,
            product_name,
            publisher,
            publisher_display_name,
            template,
        }
    }
}
pub mod security_solutions_reference_data_properties {
    use super::*;
    #[doc = "The security family of the security solution"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecurityFamily")]
    pub enum SecurityFamily {
        Waf,
        Ngfw,
        SaasWaf,
        Va,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecurityFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecurityFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecurityFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Waf => serializer.serialize_unit_variant("SecurityFamily", 0u32, "Waf"),
                Self::Ngfw => serializer.serialize_unit_variant("SecurityFamily", 1u32, "Ngfw"),
                Self::SaasWaf => serializer.serialize_unit_variant("SecurityFamily", 2u32, "SaasWaf"),
                Self::Va => serializer.serialize_unit_variant("SecurityFamily", 3u32, "Va"),
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
    #[doc = "The reason of the given status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The date of when the resource was scanned in the last time"]
    #[serde(rename = "lastScannedDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_scanned_date: Option<time::OffsetDateTime>,
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
