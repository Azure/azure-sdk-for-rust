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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AadExternalSecuritySolution {
    #[serde(flatten)]
    pub external_security_solution: ExternalSecuritySolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadSolutionProperties>,
}
impl AadExternalSecuritySolution {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Factors that increase our confidence that the alert is a true positive"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertConfidenceReason {
    #[doc = "Type of confidence factor"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "description of the confidence reason"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl AlertConfidenceReason {
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
#[doc = "Changing set of properties depending on the alert type."]
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
    #[doc = "State of the alert (Active, Dismissed etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The time the incident was reported to Microsoft.Security in UTC"]
    #[serde(rename = "reportedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub reported_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Name of the vendor that discovered the incident"]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "Name of the alert type"]
    #[serde(rename = "alertName", default, skip_serializing_if = "Option::is_none")]
    pub alert_name: Option<String>,
    #[doc = "Display name of the alert type"]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "The time the incident was detected by the vendor"]
    #[serde(rename = "detectedTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub detected_time_utc: Option<time::OffsetDateTime>,
    #[doc = "Description of the incident and what it means"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Recommended steps to reradiate the incident"]
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[doc = "The action that was taken as a response to the alert (Active, Blocked etc.)"]
    #[serde(rename = "actionTaken", default, skip_serializing_if = "Option::is_none")]
    pub action_taken: Option<String>,
    #[doc = "Estimated severity of this alert"]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<alert_properties::ReportedSeverity>,
    #[doc = "The entity that the incident happened on"]
    #[serde(rename = "compromisedEntity", default, skip_serializing_if = "Option::is_none")]
    pub compromised_entity: Option<String>,
    #[doc = "Azure resource ID of the associated resource"]
    #[serde(rename = "associatedResource", default, skip_serializing_if = "Option::is_none")]
    pub associated_resource: Option<String>,
    #[doc = "Changing set of properties depending on the alert type."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<AlertExtendedProperties>,
    #[doc = "The type of the alerted resource (Azure, Non-Azure)"]
    #[serde(rename = "systemSource", default, skip_serializing_if = "Option::is_none")]
    pub system_source: Option<String>,
    #[doc = "Whether this alert can be investigated with Azure Security Center"]
    #[serde(rename = "canBeInvestigated", default, skip_serializing_if = "Option::is_none")]
    pub can_be_investigated: Option<bool>,
    #[doc = "Whether this alert is for incident type or not (otherwise - single alert)"]
    #[serde(rename = "isIncident", default, skip_serializing_if = "Option::is_none")]
    pub is_incident: Option<bool>,
    #[doc = "objects that are related to this alerts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entities: Vec<AlertEntity>,
    #[doc = "level of confidence we have on the alert"]
    #[serde(rename = "confidenceScore", default, skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f32>,
    #[doc = "reasons the alert got the confidenceScore value"]
    #[serde(
        rename = "confidenceReasons",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub confidence_reasons: Vec<AlertConfidenceReason>,
    #[doc = "Azure subscription ID of the resource that had the security alert or the subscription ID of the workspace that this resource reports to"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Instance ID of the alert."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Azure resource ID of the workspace that the alert was reported to."]
    #[serde(rename = "workspaceArmId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_arm_id: Option<String>,
    #[doc = "Alerts with the same CorrelationKey will be grouped together in Ibiza."]
    #[serde(rename = "correlationKey", default, skip_serializing_if = "Option::is_none")]
    pub correlation_key: Option<String>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_properties {
    use super::*;
    #[doc = "Estimated severity of this alert"]
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
    #[doc = "Expiration date of the rule, if value is not provided or provided as null this field will default to the maximum allowed expiration date."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppWhitelistingGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub location: Location,
    #[doc = "Represents a VM/server group and set of rules that are Recommended by Microsoft Defender for Cloud to be allowed"]
    pub properties: AppWhitelistingGroupData,
}
impl AppWhitelistingGroup {
    pub fn new(properties: AppWhitelistingGroupData) -> Self {
        Self {
            resource: Resource::default(),
            location: Location::default(),
            properties,
        }
    }
}
#[doc = "Represents a VM/server group and set of rules that are Recommended by Microsoft Defender for Cloud to be allowed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppWhitelistingGroupData {
    #[doc = "The application control policy enforcement/protection mode of the VM/server group"]
    #[serde(rename = "enforcementMode", default, skip_serializing_if = "Option::is_none")]
    pub enforcement_mode: Option<EnforcementMode>,
    #[doc = "The protection mode of the collection/file types. Exe/Msi/Script are used for Windows, Executable is used for Linux."]
    #[serde(rename = "protectionMode", default, skip_serializing_if = "Option::is_none")]
    pub protection_mode: Option<ProtectionMode>,
    #[doc = "The configuration status of the VM/server group or machine or rule on the machine"]
    #[serde(rename = "configurationStatus", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status: Option<ConfigurationStatus>,
    #[doc = "The recommendation status of the VM/server group or VM/server"]
    #[serde(rename = "recommendationStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<RecommendationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<AppWhitelistingIssuesSummaries>,
    #[doc = "The source type of the VM/server group"]
    #[serde(rename = "sourceSystem", default, skip_serializing_if = "Option::is_none")]
    pub source_system: Option<SourceSystem>,
    #[serde(rename = "vmRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub vm_recommendations: Option<VmRecommendations>,
    #[serde(rename = "pathRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub path_recommendations: Option<PathRecommendations>,
}
impl AppWhitelistingGroupData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a list of VM/server groups and set of rules that are Recommended by Microsoft Defender for Cloud to be allowed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppWhitelistingGroups {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AppWhitelistingGroup>,
}
impl AppWhitelistingGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert that VMs/servers within a group can have"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppWhitelistingIssue {
    ViolationsAudited,
    ViolationsBlocked,
    MsiAndScriptViolationsAudited,
    MsiAndScriptViolationsBlocked,
    ExecutableViolationsAudited,
    RulesViolatedManually,
}
#[doc = "Represents a summary of the alerts of the VM/server group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppWhitelistingIssueSummary {
    #[doc = "An alert that VMs/servers within a group can have"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<AppWhitelistingIssue>,
    #[doc = "The number of machines in the VM/server group that have this alert"]
    #[serde(rename = "numberOfVms", default, skip_serializing_if = "Option::is_none")]
    pub number_of_vms: Option<f64>,
}
impl AppWhitelistingIssueSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AppWhitelistingIssuesSummaries = Vec<AppWhitelistingIssueSummary>;
#[doc = "The altered data of the recommended VM/server group policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppWhitelistingPutGroupData {
    #[doc = "The application control policy enforcement/protection mode of the VM/server group"]
    #[serde(rename = "enforcementMode", default, skip_serializing_if = "Option::is_none")]
    pub enforcement_mode: Option<EnforcementMode>,
    #[doc = "The protection mode of the collection/file types. Exe/Msi/Script are used for Windows, Executable is used for Linux."]
    #[serde(rename = "protectionMode", default, skip_serializing_if = "Option::is_none")]
    pub protection_mode: Option<ProtectionMode>,
    #[serde(rename = "vmRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub vm_recommendations: Option<VmRecommendations>,
    #[serde(rename = "pathRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub path_recommendations: Option<PathRecommendations>,
}
impl AppWhitelistingPutGroupData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AppWhitelistingResourceType = String;
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
#[doc = "Represents an ATA security solution which sends logs to an OMS workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtaExternalSecuritySolution {
    #[serde(flatten)]
    pub external_security_solution: ExternalSecuritySolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AtaSolutionProperties>,
}
impl AtaExternalSecuritySolution {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The aws connector environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsEnvironmentData {
    #[doc = "The awsOrganization data "]
    #[serde(rename = "organizationalData", default, skip_serializing_if = "Option::is_none")]
    pub organizational_data: Option<AwsOrganizationalDataUnion>,
}
impl AwsEnvironmentData {
    pub fn new() -> Self {
        Self { organizational_data: None }
    }
}
#[doc = "The multi cloud account's membership type in the organization"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "organizationMembershipType")]
pub enum AwsOrganizationalDataUnion {
    Organization(AwsOrganizationalDataMaster),
    Member(AwsOrganizationalDataMember),
}
#[doc = "The awsOrganization data for the master account"]
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
#[doc = "The awsOrganization data for the member account"]
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
#[doc = "The AzureDevOps scope connector's environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsScopeEnvironmentData {}
impl AzureDevOpsScopeEnvironmentData {
    pub fn new() -> Self {
        Self {}
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
#[doc = "Represents a security solution which sends CEF logs to an OMS workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CefExternalSecuritySolution {
    #[serde(flatten)]
    pub external_security_solution: ExternalSecuritySolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CefSolutionProperties>,
}
impl CefExternalSecuritySolution {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The configuration status of the VM/server group or machine or rule on the machine"]
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
#[doc = "Represents a data export setting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSetting {
    #[serde(flatten)]
    pub setting: Setting,
    #[doc = "The data export setting properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportSettingProperties>,
}
impl DataExportSetting {
    pub fn new(setting: Setting) -> Self {
        Self { setting, properties: None }
    }
}
#[doc = "The data export setting properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettingProperties {
    #[doc = "Is the data export setting is enabled"]
    pub enabled: bool,
}
impl DataExportSettingProperties {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
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
#[doc = "The application control policy enforcement/protection mode of the VM/server group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnforcementMode {
    Audit,
    Enforce,
    None,
}
#[doc = "The VM/server supportability of Enforce feature"]
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
#[doc = "Represents a security solution external to Microsoft Defender for Cloud which sends information to an OMS workspace and whose data is displayed by Microsoft Defender for Cloud."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
        Self::default()
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
}
impl GcpOrganizationalDataOrganization {
    pub fn new() -> Self {
        Self {
            excluded_project_numbers: Vec::new(),
            service_account_email_address: None,
            workload_identity_provider_id: None,
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
}
impl GcpProjectEnvironmentData {
    pub fn new() -> Self {
        Self {
            organizational_data: None,
            project_details: None,
        }
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
pub type GroupResourceId = String;
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
#[doc = "Security Solution Aggregated Alert information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedAlert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "Security Solution Aggregated Alert data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecurityAggregatedAlertProperties>,
}
impl IoTSecurityAggregatedAlert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of IoT aggregated security alerts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedAlertList {
    #[doc = "List of aggregated alerts data"]
    pub value: Vec<IoTSecurityAggregatedAlert>,
    #[doc = "The URI to fetch the next page."]
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
#[doc = "Security Solution Aggregated Alert data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedAlertProperties {
    #[doc = "Name of the alert type"]
    #[serde(rename = "alertType", default, skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[doc = "Display name of the alert type"]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "The date the incidents were detected by the vendor"]
    #[serde(rename = "aggregatedDateUtc", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_date_utc: Option<String>,
    #[doc = "Name of the vendor that discovered the incident"]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "Estimated severity of this alert"]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_aggregated_alert_properties::ReportedSeverity>,
    #[doc = "Recommended steps for remediation"]
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[doc = "Description of the incident and what it means"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Occurrence number of the alert within the aggregated date"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Azure resource ID of the resource that got the alerts"]
    #[serde(rename = "effectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub effected_resource_type: Option<String>,
    #[doc = "The type of the alerted resource (Azure, Non-Azure)"]
    #[serde(rename = "systemSource", default, skip_serializing_if = "Option::is_none")]
    pub system_source: Option<String>,
    #[doc = "The action that was taken as a response to the alert (Active, Blocked etc.)"]
    #[serde(rename = "actionTaken", default, skip_serializing_if = "Option::is_none")]
    pub action_taken: Option<String>,
    #[doc = "query in log analytics to get the list of affected devices/alerts"]
    #[serde(rename = "logAnalyticsQuery", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_query: Option<String>,
}
impl IoTSecurityAggregatedAlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod io_t_security_aggregated_alert_properties {
    use super::*;
    #[doc = "Estimated severity of this alert"]
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
#[doc = "Security Solution Recommendation Information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedRecommendation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "Security Solution Recommendation Information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecurityAggregatedRecommendationProperties>,
}
impl IoTSecurityAggregatedRecommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of IoT aggregated security recommendations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAggregatedRecommendationList {
    #[doc = "List of aggregated alerts data"]
    pub value: Vec<IoTSecurityAggregatedRecommendation>,
    #[doc = "The URI to fetch the next page."]
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
#[doc = "Security Solution Recommendation Information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAggregatedRecommendationProperties {
    #[doc = "Name of the recommendation"]
    #[serde(rename = "recommendationName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_name: Option<String>,
    #[doc = "Display name of the recommendation type."]
    #[serde(rename = "recommendationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_display_name: Option<String>,
    #[doc = "Description of the incident and what it means"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The recommendation-type GUID."]
    #[serde(rename = "recommendationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type_id: Option<String>,
    #[doc = "Name of the vendor that discovered the issue"]
    #[serde(rename = "detectedBy", default, skip_serializing_if = "Option::is_none")]
    pub detected_by: Option<String>,
    #[doc = "Recommended steps for remediation"]
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[doc = "Estimated severity of this recommendation"]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_aggregated_recommendation_properties::ReportedSeverity>,
    #[doc = "the number of the healthy devices within the solution"]
    #[serde(rename = "healthyDevices", default, skip_serializing_if = "Option::is_none")]
    pub healthy_devices: Option<i64>,
    #[doc = "the number of the unhealthy devices within the solution"]
    #[serde(rename = "unhealthyDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_device_count: Option<i64>,
    #[doc = "query in log analytics to get the list of affected devices/alerts"]
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
    #[doc = "Estimated severity of this recommendation"]
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
#[doc = "Statistic information about the number of alerts per device during the last period"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityAlertedDevice {
    #[doc = "Name of the alert type"]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "the number of alerts raised for this device"]
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
}
impl IoTSecurityAlertedDevice {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of devices with the count of raised alerts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityAlertedDevicesList {
    #[doc = "List of aggregated alerts data"]
    pub value: Vec<IoTSecurityAlertedDevice>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IoTSecurityAlertedDevicesList {
    pub fn new(value: Vec<IoTSecurityAlertedDevice>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Statistic information about the number of alerts per alert type during the last period"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityDeviceAlert {
    #[doc = "Display name of the alert"]
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[doc = "Estimated severity of this alert"]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_device_alert::ReportedSeverity>,
    #[doc = "the number of alerts raised for this alert type"]
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
    #[doc = "Estimated severity of this alert"]
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
#[doc = "List of alerts with the count of raised alerts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityDeviceAlertsList {
    #[doc = "List of top alerts data"]
    pub value: Vec<IoTSecurityDeviceAlert>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IoTSecurityDeviceAlertsList {
    pub fn new(value: Vec<IoTSecurityDeviceAlert>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Statistic information about the number of recommendations per recommendation type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecurityDeviceRecommendation {
    #[doc = "Display name of the recommendation"]
    #[serde(rename = "recommendationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_display_name: Option<String>,
    #[doc = "Estimated severity of this recommendation"]
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<io_t_security_device_recommendation::ReportedSeverity>,
    #[doc = "the number of device with this recommendation"]
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
    #[doc = "Estimated severity of this recommendation"]
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
#[doc = "List of recommendations with the count of devices"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecurityDeviceRecommendationsList {
    #[doc = "List of aggregated recommendation data"]
    pub value: Vec<IoTSecurityDeviceRecommendation>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IoTSecurityDeviceRecommendationsList {
    pub fn new(value: Vec<IoTSecurityDeviceRecommendation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Security Analytics of a security solution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecuritySolutionAnalyticsModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Security Analytics of a security solution properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTSecuritySolutionAnalyticsModelProperties>,
}
impl IoTSecuritySolutionAnalyticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Security Analytics of a security solution"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionAnalyticsModelList {
    #[doc = "List of Security Analytics of a security solution"]
    pub value: Vec<IoTSecuritySolutionAnalyticsModel>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IoTSecuritySolutionAnalyticsModelList {
    pub fn new(value: Vec<IoTSecuritySolutionAnalyticsModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Security Analytics of a security solution properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSecuritySolutionAnalyticsModelProperties {
    #[doc = "Severity metrics"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<IoTSeverityMetrics>,
    #[doc = "number of unhealthy devices"]
    #[serde(rename = "unhealthyDeviceCount", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_device_count: Option<i64>,
    #[doc = "The list of devices metrics by the aggregated date."]
    #[serde(
        rename = "devicesMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub devices_metrics: Vec<serde_json::Value>,
    #[doc = "List of devices with the count of raised alerts"]
    #[serde(rename = "topAlertedDevices", default, skip_serializing_if = "Option::is_none")]
    pub top_alerted_devices: Option<IoTSecurityAlertedDevicesList>,
    #[doc = "List of alerts with the count of raised alerts"]
    #[serde(rename = "mostPrevalentDeviceAlerts", default, skip_serializing_if = "Option::is_none")]
    pub most_prevalent_device_alerts: Option<IoTSecurityDeviceAlertsList>,
    #[doc = "List of recommendations with the count of devices"]
    #[serde(rename = "mostPrevalentDeviceRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub most_prevalent_device_recommendations: Option<IoTSecurityDeviceRecommendationsList>,
}
impl IoTSecuritySolutionAnalyticsModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Security Solution"]
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
    pub workspace: String,
    #[doc = "Resource display name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Security solution status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<io_t_security_solution_properties::Status>,
    #[doc = "List of additional export to workspace data options"]
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
    #[doc = "Properties of the solution's user defined resources."]
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
    #[doc = "List of recommendation configuration"]
    #[serde(rename = "recommendationsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_configuration: Option<RecommendationConfigurationList>,
}
impl IoTSecuritySolutionProperties {
    pub fn new(workspace: String, display_name: String, iot_hubs: Vec<String>) -> Self {
        Self {
            workspace,
            display_name,
            status: None,
            export: Vec::new(),
            disabled_data_sources: Vec::new(),
            iot_hubs,
            user_defined_resources: None,
            auto_discovered_resources: Vec::new(),
            recommendations_configuration: None,
        }
    }
}
pub mod io_t_security_solution_properties {
    use super::*;
    #[doc = "Security solution status"]
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
}
#[doc = "List of iot solutions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTSecuritySolutionsList {
    #[doc = "List of security solutions"]
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
#[doc = "Severity metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTSeverityMetrics {
    #[doc = "count of high severity items"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    #[doc = "count of medium severity items"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<i64>,
    #[doc = "count of low severity items"]
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
#[doc = "Represents a path that is recommended to be allowed and its properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PathRecommendation {
    #[doc = "The full path to the application to allow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The recommendation action of the VM/server or rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RecommendationAction>,
    #[doc = "The type of the rule to be allowed"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<RecommendationType>,
    #[doc = "Represents the publisher information of a process/rule"]
    #[serde(rename = "publisherInfo", default, skip_serializing_if = "Option::is_none")]
    pub publisher_info: Option<PublisherInfo>,
    #[doc = "Whether the path is commonly run on the machine"]
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
    #[doc = "The configuration status of the VM/server group or machine or rule on the machine"]
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
#[doc = "Microsoft Defender for Cloud is provided in two pricing tiers: free and standard, with the standard tier available with a trial period. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
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
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard, with the standard tier available with a trial period. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[serde(rename = "pricingTier")]
    pub pricing_tier: pricing_properties::PricingTier,
    #[doc = "The duration left for the subscriptions free trial period - in ISO 8601 format (e.g. P3Y6M4DT12H30M5S)."]
    #[serde(rename = "freeTrialRemainingTime", default, skip_serializing_if = "Option::is_none")]
    pub free_trial_remaining_time: Option<String>,
}
impl PricingProperties {
    pub fn new(pricing_tier: pricing_properties::PricingTier) -> Self {
        Self {
            pricing_tier,
            free_trial_remaining_time: None,
        }
    }
}
pub mod pricing_properties {
    use super::*;
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard, with the standard tier available with a trial period. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
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
    #[doc = "The application control policy enforcement/protection mode of the VM/server group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exe: Option<EnforcementMode>,
    #[doc = "The application control policy enforcement/protection mode of the VM/server group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msi: Option<EnforcementMode>,
    #[doc = "The application control policy enforcement/protection mode of the VM/server group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<EnforcementMode>,
    #[doc = "The application control policy enforcement/protection mode of the VM/server group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable: Option<EnforcementMode>,
}
impl ProtectionMode {
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
#[doc = "The recommendation action of the VM/server or rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecommendationAction {
    Recommended,
    Add,
    Remove,
}
pub type RecommendationConfigurationList = Vec<RecommendationConfigurationProperties>;
#[doc = "Recommendation configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationConfigurationProperties {
    #[doc = "The recommendation type."]
    #[serde(rename = "recommendationType")]
    pub recommendation_type: recommendation_configuration_properties::RecommendationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Recommendation status. The recommendation is not generated when the status is disabled"]
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
    #[doc = "The recommendation type."]
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
    #[doc = "Recommendation status. The recommendation is not generated when the status is disabled"]
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
#[doc = "The recommendation status of the VM/server group or VM/server"]
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
#[doc = "the kind of the settings string (DataExportSetting)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SettingUnion {
    DataExportSetting(DataExportSetting),
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
#[doc = "The source type of the VM/server group"]
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
#[doc = "A custom alert rule that checks if a value (depends on the custom alert type) is within the given range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdCustomAlertRule {
    #[serde(flatten)]
    pub custom_alert_rule: CustomAlertRule,
    #[doc = "The minimum threshold."]
    #[serde(rename = "minThreshold")]
    pub min_threshold: i64,
    #[doc = "The maximum threshold."]
    #[serde(rename = "maxThreshold")]
    pub max_threshold: i64,
}
impl ThresholdCustomAlertRule {
    pub fn new(custom_alert_rule: CustomAlertRule, min_threshold: i64, max_threshold: i64) -> Self {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIotSecuritySolutionData {
    #[serde(flatten)]
    pub tags_resource: TagsResource,
    #[doc = "Properties of the solution's user defined resources."]
    #[serde(rename = "userDefinedResources", default, skip_serializing_if = "Option::is_none")]
    pub user_defined_resources: Option<UserDefinedResourcesProperties>,
    #[doc = "List of recommendation configuration"]
    #[serde(rename = "recommendationsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_configuration: Option<RecommendationConfigurationList>,
}
impl UpdateIotSecuritySolutionData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the solution's user defined resources."]
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
    #[doc = "The recommendation action of the VM/server or rule"]
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
#[doc = "Represents a machine that is part of a VM/server group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmRecommendation {
    #[doc = "The configuration status of the VM/server group or machine or rule on the machine"]
    #[serde(rename = "configurationStatus", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status: Option<ConfigurationStatus>,
    #[doc = "The recommendation action of the VM/server or rule"]
    #[serde(rename = "recommendationAction", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_action: Option<RecommendationAction>,
    #[doc = "The full azure resource id of the machine"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<VmResourceId>,
    #[doc = "The VM/server supportability of Enforce feature"]
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
    CspmMonitorGithub(CspmMonitorGithubOffering),
    DefenderCspmAws(DefenderCspmAwsOffering),
    DefenderCspmGcp(DefenderCspmGcpOffering),
    DefenderForDatabasesAws(DefenderFoDatabasesAwsOffering),
    DefenderForContainersAws(DefenderForContainersAwsOffering),
    DefenderForContainersGcp(DefenderForContainersGcpOffering),
    DefenderForDatabasesGcp(DefenderForDatabasesGcpOffering),
    DefenderForDevOpsAzureDevOps(DefenderForDevOpsAzureDevOpsOffering),
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
#[doc = "The CSPM P1 for Aws offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderCspmAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[serde(rename = "vmScanners", default, skip_serializing_if = "Option::is_none")]
    pub vm_scanners: Option<defender_cspm_aws_offering::VmScanners>,
}
impl DefenderCspmAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            vm_scanners: None,
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
            #[doc = "The scanning mode for the vm scan."]
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
            #[doc = "The scanning mode for the vm scan."]
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
#[doc = "The CSPM P1 for GCP offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderCspmGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl DefenderCspmGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
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
}
impl DefenderFoDatabasesAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            arc_auto_provisioning: None,
            rds: None,
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
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
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
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
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
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
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
            #[doc = "The scanning mode for the vm scan."]
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
            #[doc = "The scanning mode for the vm scan."]
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
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
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
