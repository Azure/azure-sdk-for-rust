#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The additional information for a property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalInformation {
    #[doc = "The title name for the property."]
    #[serde(rename = "titleName", default, skip_serializing_if = "Option::is_none")]
    pub title_name: Option<String>,
    #[doc = "The title value for the property."]
    #[serde(rename = "titleValue", default, skip_serializing_if = "Option::is_none")]
    pub title_value: Option<String>,
    #[doc = "The list of properties which are included in the additional information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Indicates if properties are present or not."]
    #[serde(rename = "hasProperties", default, skip_serializing_if = "Option::is_none")]
    pub has_properties: Option<bool>,
}
impl AdditionalInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of key value properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddsConfiguration {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Item>,
    #[doc = "The total count of configuration."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for AddsConfiguration {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AddsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server details for ADDS service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddsServiceMember {
    #[doc = "The domain name."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The site name."]
    #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[doc = "The list of ADDS roles."]
    #[serde(rename = "addsRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub adds_roles: Vec<String>,
    #[doc = "Indicates if the global catalog for this domain is reachable or not."]
    #[serde(rename = "gcReachable", default, skip_serializing_if = "Option::is_none")]
    pub gc_reachable: Option<bool>,
    #[doc = "Indicates if the Dc is advertising or not."]
    #[serde(rename = "isAdvertising", default, skip_serializing_if = "Option::is_none")]
    pub is_advertising: Option<bool>,
    #[doc = "Indicates if the primary domain controller is reachable or not."]
    #[serde(rename = "pdcReachable", default, skip_serializing_if = "Option::is_none")]
    pub pdc_reachable: Option<bool>,
    #[doc = "Indicates if the SYSVOL state is healthy or not."]
    #[serde(rename = "sysvolState", default, skip_serializing_if = "Option::is_none")]
    pub sysvol_state: Option<bool>,
    #[doc = "The list of domain controller types."]
    #[serde(rename = "dcTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub dc_types: Vec<String>,
    #[doc = "The id of the server."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
    #[doc = "The service id to whom this server belongs."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The tenant id to whom this server belongs."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The total number of alerts that are currently active for the server."]
    #[serde(rename = "activeAlerts", default, skip_serializing_if = "Option::is_none")]
    pub active_alerts: Option<i64>,
    #[doc = "The additional information, if any, for the server."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    #[doc = "The date time , in UTC, when the server was onboarded to Azure Active Directory Connect Health."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The server specific configuration related dimensions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Item>,
    #[doc = "Indicates if the server is disabled or not. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[doc = "The reason for disabling the server."]
    #[serde(rename = "disabledReason", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<i64>,
    #[doc = "The list of installed QFEs for the server."]
    #[serde(rename = "installedQfes", default, skip_serializing_if = "Vec::is_empty")]
    pub installed_qfes: Vec<Hotfix>,
    #[doc = "The date and time , in UTC, when the server was last disabled."]
    #[serde(rename = "lastDisabled", default, with = "azure_core::date::rfc3339::option")]
    pub last_disabled: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the server was last rebooted."]
    #[serde(rename = "lastReboot", default, with = "azure_core::date::rfc3339::option")]
    pub last_reboot: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the server's data monitoring configuration was last changed."]
    #[serde(
        rename = "lastServerReportedMonitoringLevelChange",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub last_server_reported_monitoring_level_change: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the server properties were last updated."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The id of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "The name of the server."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The monitoring configuration of the server which determines what activities are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoringConfigurationsComputed", default, skip_serializing_if = "Vec::is_empty")]
    pub monitoring_configurations_computed: Vec<Item>,
    #[doc = "The customized monitoring configuration of the server which determines what activities are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoringConfigurationsCustomized", default, skip_serializing_if = "Vec::is_empty")]
    pub monitoring_configurations_customized: Vec<Item>,
    #[doc = "The name of the operating system installed in the machine."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of the operating system installed in the machine."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Server specific properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Item>,
    #[doc = "The list of recommended hotfixes for the server."]
    #[serde(rename = "recommendedQfes", default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_qfes: Vec<Hotfix>,
    #[doc = "The total count of alerts that are resolved for this server."]
    #[serde(rename = "resolvedAlerts", default, skip_serializing_if = "Option::is_none")]
    pub resolved_alerts: Option<i64>,
    #[doc = "The service role that is being monitored in the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The monitoring level reported by the server."]
    #[serde(rename = "serverReportedMonitoringLevel", default, skip_serializing_if = "Option::is_none")]
    pub server_reported_monitoring_level: Option<adds_service_member::ServerReportedMonitoringLevel>,
    #[doc = "The health status of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl AddsServiceMember {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod adds_service_member {
    use super::*;
    #[doc = "The monitoring level reported by the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerReportedMonitoringLevel {
        Partial,
        Full,
        Off,
    }
}
#[doc = "The list of  ADDS service members."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddsServiceMembers {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AddsServiceMember>,
    #[doc = "The total count of service members."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for AddsServiceMembers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AddsServiceMembers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Agent {
    #[doc = "The tenant Id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The machine Id."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "The agent credential details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<serde_json::Value>,
    #[doc = "The machine name."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The date and time, in UTC, when the agent was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = " The connector hash key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl Agent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = " The alert details indicating an issue with service or server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[doc = "The alert Id."]
    #[serde(rename = "alertId", default, skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<String>,
    #[doc = "The alert level which indicates the severity of the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<alert::Level>,
    #[doc = "The alert state which can be either active or resolved with multiple resolution types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<alert::State>,
    #[doc = "The alert short name."]
    #[serde(rename = "shortName", default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[doc = "The display name for the alert."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The alert description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The alert remediation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[doc = "The help links to get more information related to the alert."]
    #[serde(rename = "relatedLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub related_links: Vec<HelpLink>,
    #[doc = "The scope of the alert. Indicates if it is a service or a server related alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Additional information related to the alert."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_information: Vec<AdditionalInformation>,
    #[doc = "The date and time,in UTC,when the alert was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the alert was resolved."]
    #[serde(rename = "resolvedDate", default, with = "azure_core::date::rfc3339::option")]
    pub resolved_date: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the alert was last updated."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The monitoring role type for which the alert was raised."]
    #[serde(rename = "monitorRoleType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_role_type: Option<String>,
    #[doc = "The active alert properties."]
    #[serde(rename = "activeAlertProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub active_alert_properties: Vec<Item>,
    #[doc = "The resolved alert properties."]
    #[serde(rename = "resolvedAlertProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub resolved_alert_properties: Vec<Item>,
    #[doc = "The tenant Id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The service Id."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The server Id."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert {
    use super::*;
    #[doc = "The alert level which indicates the severity of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Level")]
    pub enum Level {
        Warning,
        Error,
        PreWarning,
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
                Self::Warning => serializer.serialize_unit_variant("Level", 0u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("Level", 1u32, "Error"),
                Self::PreWarning => serializer.serialize_unit_variant("Level", 2u32, "PreWarning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The alert state which can be either active or resolved with multiple resolution types."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Active,
        ResolvedByPositiveResult,
        ResolvedManually,
        ResolvedByTimer,
        ResolvedByStateChange,
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
                Self::Active => serializer.serialize_unit_variant("State", 0u32, "Active"),
                Self::ResolvedByPositiveResult => serializer.serialize_unit_variant("State", 1u32, "ResolvedByPositiveResult"),
                Self::ResolvedManually => serializer.serialize_unit_variant("State", 2u32, "ResolvedManually"),
                Self::ResolvedByTimer => serializer.serialize_unit_variant("State", 3u32, "ResolvedByTimer"),
                Self::ResolvedByStateChange => serializer.serialize_unit_variant("State", 4u32, "ResolvedByStateChange"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The alert feedback details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertFeedback {
    #[doc = "The alert level which indicates the severity of the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The alert state which can be either active or resolved with multiple resolution types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The alert short name."]
    #[serde(rename = "shortName", default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[doc = "The feedback for the alert which indicates if the customer likes or dislikes the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[doc = "Additional comments related to the alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Indicates if the alert feedback can be shared from product team."]
    #[serde(rename = "consentedToShare", default, skip_serializing_if = "Option::is_none")]
    pub consented_to_share: Option<bool>,
    #[doc = "The server Id of the alert."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
    #[doc = "The date and time,in UTC,when the alert was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
}
impl AlertFeedback {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of alert feedback."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertFeedbacks {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertFeedback>,
}
impl azure_core::Continuable for AlertFeedbacks {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AlertFeedbacks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of alerts for a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alerts {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The total count of alert elements."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for Alerts {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Alerts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that hold sync object details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedObject {
    #[doc = "The display name of the object."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The distinguished name of the object."]
    #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,
    #[doc = "The last dirSync time."]
    #[serde(rename = "lastDirSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_dir_sync_time: Option<time::OffsetDateTime>,
    #[doc = "The email of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[doc = "The object guid."]
    #[serde(rename = "objectGuid", default, skip_serializing_if = "Option::is_none")]
    pub object_guid: Option<String>,
    #[doc = "The object type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The On-premises UPN."]
    #[serde(rename = "onpremisesUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub onpremises_user_principal_name: Option<String>,
    #[doc = "The proxy addresses."]
    #[serde(rename = "proxyAddresses", default, skip_serializing_if = "Option::is_none")]
    pub proxy_addresses: Option<String>,
    #[doc = "The source anchor."]
    #[serde(rename = "sourceAnchor", default, skip_serializing_if = "Option::is_none")]
    pub source_anchor: Option<String>,
    #[doc = "The source of authority."]
    #[serde(rename = "sourceOfAuthority", default, skip_serializing_if = "Option::is_none")]
    pub source_of_authority: Option<String>,
    #[doc = " The time of the error."]
    #[serde(rename = "timeOccurred", default, with = "azure_core::date::rfc3339::option")]
    pub time_occurred: Option<time::OffsetDateTime>,
    #[doc = " The UPN."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
impl AssociatedObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The delta attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeDelta {
    #[doc = "The delta values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<ValueDelta>,
    #[doc = "The name of the attribute delta."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The attribute delta operation type."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<attribute_delta::OperationType>,
    #[doc = "The value type."]
    #[serde(rename = "valueType", default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<attribute_delta::ValueType>,
    #[doc = "Indicates if the attribute delta is multivalued or not."]
    #[serde(rename = "multiValued", default, skip_serializing_if = "Option::is_none")]
    pub multi_valued: Option<bool>,
}
impl AttributeDelta {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod attribute_delta {
    use super::*;
    #[doc = "The attribute delta operation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        Undefined,
        Add,
        Replace,
        Update,
        Delete,
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
                Self::Undefined => serializer.serialize_unit_variant("OperationType", 0u32, "Undefined"),
                Self::Add => serializer.serialize_unit_variant("OperationType", 1u32, "Add"),
                Self::Replace => serializer.serialize_unit_variant("OperationType", 2u32, "Replace"),
                Self::Update => serializer.serialize_unit_variant("OperationType", 3u32, "Update"),
                Self::Delete => serializer.serialize_unit_variant("OperationType", 4u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The value type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValueType")]
    pub enum ValueType {
        Undefined,
        Dn,
        Binary,
        String,
        Integer,
        Boolean,
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
                Self::Undefined => serializer.serialize_unit_variant("ValueType", 0u32, "Undefined"),
                Self::Dn => serializer.serialize_unit_variant("ValueType", 1u32, "Dn"),
                Self::Binary => serializer.serialize_unit_variant("ValueType", 2u32, "Binary"),
                Self::String => serializer.serialize_unit_variant("ValueType", 3u32, "String"),
                Self::Integer => serializer.serialize_unit_variant("ValueType", 4u32, "Integer"),
                Self::Boolean => serializer.serialize_unit_variant("ValueType", 5u32, "Boolean"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The attribute mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeMapping {
    #[doc = "The attribute mapping source."]
    #[serde(rename = "mappingSource", default, skip_serializing_if = "Option::is_none")]
    pub mapping_source: Option<AttributeMppingSource>,
    #[doc = "The attribute mapping type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<attribute_mapping::Type>,
    #[doc = "The destination attribute."]
    #[serde(rename = "destinationAttribute", default, skip_serializing_if = "Option::is_none")]
    pub destination_attribute: Option<String>,
    #[doc = "The context Id."]
    #[serde(rename = "contextId", default, skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}
impl AttributeMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod attribute_mapping {
    use super::*;
    #[doc = "The attribute mapping type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Constant,
        Direct,
        DnPart,
        Script,
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
                Self::Constant => serializer.serialize_unit_variant("Type", 0u32, "Constant"),
                Self::Direct => serializer.serialize_unit_variant("Type", 1u32, "Direct"),
                Self::DnPart => serializer.serialize_unit_variant("Type", 2u32, "DnPart"),
                Self::Script => serializer.serialize_unit_variant("Type", 3u32, "Script"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The attribute mapping source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeMppingSource {
    #[doc = "The source attribute."]
    #[serde(rename = "sourceAttribute", default, skip_serializing_if = "Vec::is_empty")]
    pub source_attribute: Vec<String>,
    #[doc = "The value for dn part."]
    #[serde(rename = "dnPart", default, skip_serializing_if = "Option::is_none")]
    pub dn_part: Option<i64>,
    #[doc = "The script context."]
    #[serde(rename = "scriptContext", default, skip_serializing_if = "Option::is_none")]
    pub script_context: Option<String>,
    #[doc = "The constant value."]
    #[serde(rename = "constantValue", default, skip_serializing_if = "Option::is_none")]
    pub constant_value: Option<String>,
}
impl AttributeMppingSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The changes which are not re-imported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeNotReimported {
    #[doc = "The delta in a change that is not re-imported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<ChangeNotReimportedDelta>,
    #[doc = "The object entry in a change that is not re-imported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<ChangeNotReimportedEntry>,
}
impl ChangeNotReimported {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The delta in a change that is not re-imported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeNotReimportedDelta {
    #[doc = "The anchor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[doc = "The delta attributes for distinguished names."]
    #[serde(rename = "dnAttributes", default, skip_serializing_if = "Vec::is_empty")]
    pub dn_attributes: Vec<AttributeDelta>,
    #[doc = "The attributes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeDelta>,
    #[doc = "The operation type."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<change_not_reimported_delta::OperationType>,
}
impl ChangeNotReimportedDelta {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod change_not_reimported_delta {
    use super::*;
    #[doc = "The operation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        Undefined,
        None,
        Add,
        Replace,
        Update,
        Delete,
        Obsolete,
        DeleteAdd,
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
                Self::Undefined => serializer.serialize_unit_variant("OperationType", 0u32, "Undefined"),
                Self::None => serializer.serialize_unit_variant("OperationType", 1u32, "None"),
                Self::Add => serializer.serialize_unit_variant("OperationType", 2u32, "Add"),
                Self::Replace => serializer.serialize_unit_variant("OperationType", 3u32, "Replace"),
                Self::Update => serializer.serialize_unit_variant("OperationType", 4u32, "Update"),
                Self::Delete => serializer.serialize_unit_variant("OperationType", 5u32, "Delete"),
                Self::Obsolete => serializer.serialize_unit_variant("OperationType", 6u32, "Obsolete"),
                Self::DeleteAdd => serializer.serialize_unit_variant("OperationType", 7u32, "DeleteAdd"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The object entry in a change that is not re-imported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeNotReimportedEntry {
    #[doc = "The anchor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[doc = "The parent anchor."]
    #[serde(rename = "parentAnchor", default, skip_serializing_if = "Option::is_none")]
    pub parent_anchor: Option<String>,
    #[doc = "The primary object class."]
    #[serde(rename = "primaryObjectClass", default, skip_serializing_if = "Option::is_none")]
    pub primary_object_class: Option<String>,
    #[doc = "The list of object classes."]
    #[serde(rename = "objectClasses", default, skip_serializing_if = "Vec::is_empty")]
    pub object_classes: Vec<String>,
    #[doc = "The delta attributes for distinguished names."]
    #[serde(rename = "dnAttributes", default, skip_serializing_if = "Vec::is_empty")]
    pub dn_attributes: Vec<AttributeDelta>,
    #[doc = "The attributes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeDelta>,
    #[doc = "The distinguished name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
}
impl ChangeNotReimportedEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connect details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Connector {
    #[doc = "The connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The connector Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The connector name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The connector version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The connector type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The connector description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The schema xml for the connector."]
    #[serde(rename = "schemaXml", default, skip_serializing_if = "Option::is_none")]
    pub schema_xml: Option<String>,
    #[doc = "The password management settings of the connector."]
    #[serde(rename = "passwordManagementSettings", default, skip_serializing_if = "Option::is_none")]
    pub password_management_settings: Option<serde_json::Value>,
    #[doc = "The password hash synchronization configuration of the connector."]
    #[serde(rename = "passwordHashSyncConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub password_hash_sync_configuration: Option<serde_json::Value>,
    #[doc = "The date and time when this connector was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "The date and time when this connector was last modified."]
    #[serde(rename = "timeLastModified", default, with = "azure_core::date::rfc3339::option")]
    pub time_last_modified: Option<time::OffsetDateTime>,
    #[doc = "The partitions of the connector."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub partitions: Vec<Partition>,
    #[doc = "The run profiles of the connector."]
    #[serde(rename = "runProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub run_profiles: Vec<RunProfile>,
    #[doc = "The class inclusion list of the connector."]
    #[serde(rename = "classesIncluded", default, skip_serializing_if = "Vec::is_empty")]
    pub classes_included: Vec<String>,
    #[doc = "The attribute inclusion list of the connector."]
    #[serde(rename = "attributesIncluded", default, skip_serializing_if = "Vec::is_empty")]
    pub attributes_included: Vec<String>,
}
impl Connector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connector connection error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorConnectionError {
    #[doc = "The error Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The run step result Id."]
    #[serde(rename = "runStepResultId", default, skip_serializing_if = "Option::is_none")]
    pub run_step_result_id: Option<String>,
    #[doc = "The connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The type of error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The message for the connection error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time when the connection error occurred."]
    #[serde(rename = "timeOccured", default, with = "azure_core::date::rfc3339::option")]
    pub time_occured: Option<time::OffsetDateTime>,
    #[doc = "The server where the connection error happened."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}
impl ConnectorConnectionError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of connector connection errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorConnectionErrors {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectorConnectionError>,
}
impl ConnectorConnectionErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the list of connectors and run profile names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorMetadata {
    #[doc = "The list of connectors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connectors: Vec<ConnectorMetadataDetails>,
    #[doc = "The list of run profile names."]
    #[serde(rename = "runProfileNames", default, skip_serializing_if = "Vec::is_empty")]
    pub run_profile_names: Vec<String>,
}
impl ConnectorMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorMetadataDetails {
    #[doc = "The Connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The Connector Display Name"]
    #[serde(rename = "connectorDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub connector_display_name: Option<String>,
}
impl ConnectorMetadataDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connector object error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorObjectError {
    #[doc = "The error Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The run step result Id."]
    #[serde(rename = "runStepResultId", default, skip_serializing_if = "Option::is_none")]
    pub run_step_result_id: Option<String>,
    #[doc = "The connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The type of error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The message for the object error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The entry number for object error occurred."]
    #[serde(rename = "entryNumber", default, skip_serializing_if = "Option::is_none")]
    pub entry_number: Option<i64>,
    #[doc = "The line number for the object error."]
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i64>,
    #[doc = "The column number for the object error."]
    #[serde(rename = "columnNumber", default, skip_serializing_if = "Option::is_none")]
    pub column_number: Option<i64>,
    #[doc = "The distinguished name of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[doc = "The name for the anchor of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[doc = "The attribute name of the object."]
    #[serde(rename = "attributeName", default, skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc = "The server side error details."]
    #[serde(rename = "serverErrorDetail", default, skip_serializing_if = "Option::is_none")]
    pub server_error_detail: Option<String>,
    #[doc = "The value corresponding to attribute name."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
impl ConnectorObjectError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of connector object errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorObjectErrors {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectorObjectError>,
}
impl ConnectorObjectErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of connects for a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Connectors {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Connector>,
}
impl azure_core::Continuable for Connectors {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Connectors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The credential for a given server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Credential {
    #[doc = "The credential identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The type of credential."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The credential data."]
    #[serde(rename = "credentialData", default, skip_serializing_if = "Vec::is_empty")]
    pub credential_data: Vec<String>,
}
impl Credential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of agent credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Credentials {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Credential>,
}
impl azure_core::Continuable for Credentials {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Credentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data freshness details for the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataFreshnessDetails {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Item>,
}
impl azure_core::Continuable for DataFreshnessDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataFreshnessDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connector object error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[doc = "The health status for the domain controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<dimension::Health>,
    #[doc = "List of service specific configuration properties."]
    #[serde(rename = "simpleProperties", default, skip_serializing_if = "Option::is_none")]
    pub simple_properties: Option<serde_json::Value>,
    #[doc = "The count of alerts that are currently active for the service."]
    #[serde(rename = "activeAlerts", default, skip_serializing_if = "Option::is_none")]
    pub active_alerts: Option<i64>,
    #[doc = "The additional information related to the service."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    #[doc = "The date or time , in UTC, when the service properties were last updated."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The display name of the service."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The total count of alerts that has been resolved for the service."]
    #[serde(rename = "resolvedAlerts", default, skip_serializing_if = "Option::is_none")]
    pub resolved_alerts: Option<i64>,
    #[doc = "The signature of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "The service type for the services onboarded to Azure Active Directory Connect Health. Depending on whether the service is monitoring, ADFS, Sync or ADDS roles, the service type can either be AdFederationService or AadSyncService or AdDomainService."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dimension {
    use super::*;
    #[doc = "The health status for the domain controller."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        Healthy,
        Warning,
        Error,
        NotMonitored,
        Missing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("Health", 0u32, "Healthy"),
                Self::Warning => serializer.serialize_unit_variant("Health", 1u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("Health", 2u32, "Error"),
                Self::NotMonitored => serializer.serialize_unit_variant("Health", 3u32, "NotMonitored"),
                Self::Missing => serializer.serialize_unit_variant("Health", 4u32, "Missing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of dimensions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimensions {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dimension>,
    #[doc = "The total count of dimensions."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for Dimensions {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Dimensions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Displays the details related to operations supported by Azure Active Directory Connect Health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = " The description for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The details of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error count details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorCount {
    #[doc = "The error bucket."]
    #[serde(rename = "errorBucket", default, skip_serializing_if = "Option::is_none")]
    pub error_bucket: Option<String>,
    #[doc = "The error count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Indicates if the error count is truncated or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}
impl ErrorCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of error counts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorCounts {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ErrorCount>,
}
impl azure_core::Continuable for ErrorCounts {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorCounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The knowledge base article url which contains more information about the error."]
    #[serde(rename = "kbUrl", default, skip_serializing_if = "Option::is_none")]
    pub kb_url: Option<String>,
    #[doc = "Additional details related to the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = "The objects with sync errors."]
    #[serde(rename = "objectsWithSyncError", default, skip_serializing_if = "Option::is_none")]
    pub objects_with_sync_error: Option<ObjectWithSyncError>,
    #[doc = "The merged export error."]
    #[serde(rename = "objectWithSyncError", default, skip_serializing_if = "Option::is_none")]
    pub object_with_sync_error: Option<MergedExportError>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of bad password log in attempt entries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorReportUsersEntries {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ErrorReportUsersEntry>,
}
impl azure_core::Continuable for ErrorReportUsersEntries {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorReportUsersEntries {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The bad password login attempt details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorReportUsersEntry {
    #[doc = "The user ID value."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The IP address corresponding to the last error event."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The date and time when the last error event was logged."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The list of unique IP addresses."]
    #[serde(rename = "uniqueIpAddresses", default, skip_serializing_if = "Option::is_none")]
    pub unique_ip_addresses: Option<String>,
    #[doc = "The total count of specific error events."]
    #[serde(rename = "totalErrorAttempts", default, skip_serializing_if = "Option::is_none")]
    pub total_error_attempts: Option<i64>,
}
impl ErrorReportUsersEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The export error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportError {
    #[doc = "The error Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The run step result Id."]
    #[serde(rename = "runStepResultId", default, skip_serializing_if = "Option::is_none")]
    pub run_step_result_id: Option<String>,
    #[doc = "The connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The type of error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The export error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The server error detail."]
    #[serde(rename = "serverErrorDetail", default, skip_serializing_if = "Option::is_none")]
    pub server_error_detail: Option<String>,
    #[doc = "The date and time when the export error first occurred."]
    #[serde(rename = "timeFirstOccured", default, with = "azure_core::date::rfc3339::option")]
    pub time_first_occured: Option<time::OffsetDateTime>,
    #[doc = "The retry count."]
    #[serde(rename = "retryCount", default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
    #[doc = "The cloud object Id."]
    #[serde(rename = "csObjectId", default, skip_serializing_if = "Option::is_none")]
    pub cs_object_id: Option<String>,
    #[doc = "The distinguished name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[doc = "The minimum limit."]
    #[serde(rename = "minLimit", default, skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<String>,
    #[doc = "The maximum limit."]
    #[serde(rename = "maxLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<String>,
    #[doc = "The name of the cloud anchor."]
    #[serde(rename = "cloudAnchor", default, skip_serializing_if = "Option::is_none")]
    pub cloud_anchor: Option<String>,
    #[doc = "The attribute name."]
    #[serde(rename = "attributeName", default, skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc = "The attribute value."]
    #[serde(rename = "attributeValue", default, skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[doc = "Indicates if the attribute is multi valued or not."]
    #[serde(rename = "attributeMultiValue", default, skip_serializing_if = "Option::is_none")]
    pub attribute_multi_value: Option<bool>,
    #[doc = "The object Id with which there was an attribute conflict."]
    #[serde(rename = "objectIdConflict", default, skip_serializing_if = "Option::is_none")]
    pub object_id_conflict: Option<String>,
    #[doc = "The SAM account name."]
    #[serde(rename = "samAccountName", default, skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    #[doc = "The AD object type"]
    #[serde(rename = "adObjectType", default, skip_serializing_if = "Option::is_none")]
    pub ad_object_type: Option<String>,
    #[doc = "The AD object guid."]
    #[serde(rename = "adObjectGuid", default, skip_serializing_if = "Option::is_none")]
    pub ad_object_guid: Option<String>,
    #[doc = "The display name for the AD object."]
    #[serde(rename = "adDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub ad_display_name: Option<String>,
    #[doc = "The source of authority for the AD object."]
    #[serde(rename = "adSourceOfAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ad_source_of_authority: Option<String>,
    #[doc = "The AD source anchor."]
    #[serde(rename = "adSourceAnchor", default, skip_serializing_if = "Option::is_none")]
    pub ad_source_anchor: Option<String>,
    #[doc = "The user principal name for the AD object."]
    #[serde(rename = "adUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub ad_user_principal_name: Option<String>,
    #[doc = "The distinguished name for the AD object."]
    #[serde(rename = "adDistinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub ad_distinguished_name: Option<String>,
    #[doc = "The email for the AD object."]
    #[serde(rename = "adMail", default, skip_serializing_if = "Option::is_none")]
    pub ad_mail: Option<String>,
    #[doc = "The date and time of occurrence."]
    #[serde(rename = "timeOccured", default, with = "azure_core::date::rfc3339::option")]
    pub time_occured: Option<time::OffsetDateTime>,
    #[doc = "The AAD side object type."]
    #[serde(rename = "aadObjectType", default, skip_serializing_if = "Option::is_none")]
    pub aad_object_type: Option<String>,
    #[doc = "The AAD side object guid."]
    #[serde(rename = "aadObjectGuid", default, skip_serializing_if = "Option::is_none")]
    pub aad_object_guid: Option<String>,
    #[doc = "The AAD side display name"]
    #[serde(rename = "aadDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub aad_display_name: Option<String>,
    #[doc = "The AAD side source of authority for the object."]
    #[serde(rename = "aadSourceOfAuthority", default, skip_serializing_if = "Option::is_none")]
    pub aad_source_of_authority: Option<String>,
    #[doc = "The AAD side user principal name."]
    #[serde(rename = "aadUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub aad_user_principal_name: Option<String>,
    #[doc = "The AAD side distinguished name for the object."]
    #[serde(rename = "aadDistinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub aad_distinguished_name: Option<String>,
    #[doc = "The AAD side email for the object."]
    #[serde(rename = "aadMail", default, skip_serializing_if = "Option::is_none")]
    pub aad_mail: Option<String>,
    #[doc = "The date and time of last sync run."]
    #[serde(rename = "lastDirSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_dir_sync_time: Option<time::OffsetDateTime>,
    #[doc = "The modified attribute value."]
    #[serde(rename = "modifiedAttributeValue", default, skip_serializing_if = "Option::is_none")]
    pub modified_attribute_value: Option<String>,
}
impl ExportError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of export errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportErrors {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExportError>,
}
impl ExportErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the export status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportStatus {
    #[doc = "The id of the service for whom the export status is being reported."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The server Id for whom the export status is being reported."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
    #[doc = "The date and time when the export ended."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The run step result Id."]
    #[serde(rename = "runStepResultId", default, skip_serializing_if = "Option::is_none")]
    pub run_step_result_id: Option<String>,
}
impl ExportStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of export statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportStatuses {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExportStatus>,
    #[doc = "The total count of service elements."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for ExportStatuses {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExportStatuses {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The extension error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionErrorInfo {
    #[doc = "The extension name."]
    #[serde(rename = "extensionName", default, skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
    #[doc = "The extension context."]
    #[serde(rename = "extensionContext", default, skip_serializing_if = "Option::is_none")]
    pub extension_context: Option<String>,
    #[doc = "The call stack for the error."]
    #[serde(rename = "callStack", default, skip_serializing_if = "Option::is_none")]
    pub call_stack: Option<String>,
}
impl ExtensionErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The forest summary for an ADDS domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForestSummary {
    #[doc = "The forest name."]
    #[serde(rename = "forestName", default, skip_serializing_if = "Option::is_none")]
    pub forest_name: Option<String>,
    #[doc = "The domain count."]
    #[serde(rename = "domainCount", default, skip_serializing_if = "Option::is_none")]
    pub domain_count: Option<i64>,
    #[doc = "The site count."]
    #[serde(rename = "siteCount", default, skip_serializing_if = "Option::is_none")]
    pub site_count: Option<i64>,
    #[doc = "The number of domain controllers that are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoredDcCount", default, skip_serializing_if = "Option::is_none")]
    pub monitored_dc_count: Option<i64>,
    #[doc = "The total domain controllers."]
    #[serde(rename = "totalDcCount", default, skip_serializing_if = "Option::is_none")]
    pub total_dc_count: Option<i64>,
    #[doc = "The list of domain controller names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<String>,
    #[doc = "The list of site names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sites: Vec<String>,
}
impl ForestSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The global configuration settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalConfiguration {
    #[doc = "The version for the global configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The schema for the configuration."]
    #[serde(rename = "schemaXml", default, skip_serializing_if = "Option::is_none")]
    pub schema_xml: Option<String>,
    #[doc = "Indicates if password sync is enabled or not."]
    #[serde(rename = "passwordSyncEnabled", default, skip_serializing_if = "Option::is_none")]
    pub password_sync_enabled: Option<bool>,
    #[doc = "The number of saved password events."]
    #[serde(rename = "numSavedPwdEvent", default, skip_serializing_if = "Option::is_none")]
    pub num_saved_pwd_event: Option<i64>,
    #[doc = "The list of additional feature sets."]
    #[serde(rename = "featureSet", default, skip_serializing_if = "Vec::is_empty")]
    pub feature_set: Vec<Item>,
}
impl GlobalConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of global configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalConfigurations {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GlobalConfiguration>,
}
impl azure_core::Continuable for GlobalConfigurations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GlobalConfigurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The help link which contains more information related to an alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelpLink {
    #[doc = "The title for the link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The url for the help document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl HelpLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the hotfix installed in the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Hotfix {
    #[doc = "The name of the hotfix KB."]
    #[serde(rename = "kbName", default, skip_serializing_if = "Option::is_none")]
    pub kb_name: Option<String>,
    #[doc = "The link to the KB Article."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[doc = "The date and time, in UTC, when the KB was installed in the server."]
    #[serde(rename = "installedDate", default, with = "azure_core::date::rfc3339::option")]
    pub installed_date: Option<time::OffsetDateTime>,
}
impl Hotfix {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of hotfixes installed in the server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Hotfixes {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Hotfix>,
}
impl Hotfixes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key value pair for properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressAggregate {
    #[doc = "Unique ID for the entree"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tenant ID"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The service ID"]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The IP address from where the attempted login originated from."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "When the event occurred"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "The first audit timestamp"]
    #[serde(rename = "firstAuditTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_audit_timestamp: Option<String>,
    #[doc = "The last audit timestamp"]
    #[serde(rename = "lastAuditTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_audit_timestamp: Option<String>,
    #[doc = "The extranet lockout error count"]
    #[serde(rename = "extranetLockoutErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub extranet_lockout_error_count: Option<i64>,
    #[doc = "The bad password error count"]
    #[serde(rename = "badPasswordErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub bad_password_error_count: Option<i64>,
    #[doc = "The unique usernames attempted"]
    #[serde(rename = "uniqueUsernamesAttemptedCount", default, skip_serializing_if = "Option::is_none")]
    pub unique_usernames_attempted_count: Option<i64>,
    #[doc = "A value indicating whether the attempt count threshold been exceeded"]
    #[serde(rename = "attemptCountThresholdIsExceeded", default, skip_serializing_if = "Option::is_none")]
    pub attempt_count_threshold_is_exceeded: Option<bool>,
    #[doc = "The duration of the event"]
    #[serde(rename = "timeSpan", default, skip_serializing_if = "Option::is_none")]
    pub time_span: Option<String>,
    #[doc = "A value indicating whether the IP address has been whitelisted."]
    #[serde(rename = "isWhitelistedIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub is_whitelisted_ip_address: Option<bool>,
    #[doc = "The network location"]
    #[serde(rename = "networkLocation", default, skip_serializing_if = "Option::is_none")]
    pub network_location: Option<String>,
    #[doc = "The attempted count threshold on trigger."]
    #[serde(rename = "attemptCountThresholdOnTrigger", default, skip_serializing_if = "Option::is_none")]
    pub attempt_count_threshold_on_trigger: Option<i64>,
    #[doc = "The attempted threshold type on trigger."]
    #[serde(rename = "attemptThresholdTypeOnTrigger", default, skip_serializing_if = "Option::is_none")]
    pub attempt_threshold_type_on_trigger: Option<String>,
    #[doc = "The geographic location."]
    #[serde(rename = "geographicLocation", default, skip_serializing_if = "Option::is_none")]
    pub geographic_location: Option<String>,
}
impl IpAddressAggregate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key value pair for IP aggregate thresholds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressAggregateSetting {
    #[doc = "Unique ID for the entree"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "This threshold setting defines the per day trigger for a new event to be generated in the report."]
    #[serde(
        rename = "badPasswordAndExtranetLockoutCombinedDailyThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bad_password_and_extranet_lockout_combined_daily_threshold: Option<i64>,
    #[doc = "This threshold setting defines the per hour trigger for a new event to be generated in the report."]
    #[serde(
        rename = "badPasswordAndExtranetLockoutCombinedHourlyThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bad_password_and_extranet_lockout_combined_hourly_threshold: Option<i64>,
    #[doc = "This threshold setting defines the per hour trigger for a new event to be generated in the report."]
    #[serde(rename = "extranetLockoutDailyThreshold", default, skip_serializing_if = "Option::is_none")]
    pub extranet_lockout_daily_threshold: Option<i64>,
    #[doc = "This threshold setting defines the per hour trigger for a new event to be generated in the report."]
    #[serde(rename = "extranetLockoutHourlyThreshold", default, skip_serializing_if = "Option::is_none")]
    pub extranet_lockout_hourly_threshold: Option<i64>,
    #[doc = "A value indicating whether email notification has been enabled."]
    #[serde(rename = "emailNotificationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub email_notification_enabled: Option<bool>,
}
impl IpAddressAggregateSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IP address aggregates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressAggregates {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IpAddressAggregate>,
    #[doc = "URL to get the next set of IP Aggregate list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The number of results."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for IpAddressAggregates {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IpAddressAggregates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The import error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportError {
    #[doc = "The error Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The run step result Id."]
    #[serde(rename = "runStepResultId", default, skip_serializing_if = "Option::is_none")]
    pub run_step_result_id: Option<String>,
    #[doc = "The connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The type of error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The time when the import error occurred."]
    #[serde(rename = "timeOccurred", default, with = "azure_core::date::rfc3339::option")]
    pub time_occurred: Option<time::OffsetDateTime>,
    #[doc = "The time when the import error first occurred."]
    #[serde(rename = "timeFirstOccurred", default, with = "azure_core::date::rfc3339::option")]
    pub time_first_occurred: Option<time::OffsetDateTime>,
    #[doc = "The retry count."]
    #[serde(rename = "retryCount", default, skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
    #[doc = "The operation type specific  to error reporting."]
    #[serde(rename = "algorithmStepType", default, skip_serializing_if = "Option::is_none")]
    pub algorithm_step_type: Option<import_error::AlgorithmStepType>,
    #[doc = "The changes which are not re-imported."]
    #[serde(rename = "changeNotReimported", default, skip_serializing_if = "Option::is_none")]
    pub change_not_reimported: Option<ChangeNotReimported>,
    #[doc = "The extension error details."]
    #[serde(rename = "extensionErrorInfo", default, skip_serializing_if = "Option::is_none")]
    pub extension_error_info: Option<ExtensionErrorInfo>,
    #[doc = "The error details in legacy rule processing."]
    #[serde(rename = "ruleErrorInfo", default, skip_serializing_if = "Option::is_none")]
    pub rule_error_info: Option<RuleErrorInfo>,
    #[doc = "The object Id."]
    #[serde(rename = "csObjectId", default, skip_serializing_if = "Option::is_none")]
    pub cs_object_id: Option<String>,
    #[doc = "The distinguished name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
}
impl ImportError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod import_error {
    use super::*;
    #[doc = "The operation type specific  to error reporting."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlgorithmStepType")]
    pub enum AlgorithmStepType {
        Undefined,
        Staging,
        ConnectorFilter,
        Join,
        Projection,
        ImportFlow,
        Provisioning,
        ValidateConnectorFilter,
        Deprovisioning,
        ExportFlow,
        MvDeletion,
        Recall,
        MvObjectTypeChange,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlgorithmStepType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlgorithmStepType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlgorithmStepType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undefined => serializer.serialize_unit_variant("AlgorithmStepType", 0u32, "Undefined"),
                Self::Staging => serializer.serialize_unit_variant("AlgorithmStepType", 1u32, "Staging"),
                Self::ConnectorFilter => serializer.serialize_unit_variant("AlgorithmStepType", 2u32, "ConnectorFilter"),
                Self::Join => serializer.serialize_unit_variant("AlgorithmStepType", 3u32, "Join"),
                Self::Projection => serializer.serialize_unit_variant("AlgorithmStepType", 4u32, "Projection"),
                Self::ImportFlow => serializer.serialize_unit_variant("AlgorithmStepType", 5u32, "ImportFlow"),
                Self::Provisioning => serializer.serialize_unit_variant("AlgorithmStepType", 6u32, "Provisioning"),
                Self::ValidateConnectorFilter => serializer.serialize_unit_variant("AlgorithmStepType", 7u32, "ValidateConnectorFilter"),
                Self::Deprovisioning => serializer.serialize_unit_variant("AlgorithmStepType", 8u32, "Deprovisioning"),
                Self::ExportFlow => serializer.serialize_unit_variant("AlgorithmStepType", 9u32, "ExportFlow"),
                Self::MvDeletion => serializer.serialize_unit_variant("AlgorithmStepType", 10u32, "MvDeletion"),
                Self::Recall => serializer.serialize_unit_variant("AlgorithmStepType", 11u32, "Recall"),
                Self::MvObjectTypeChange => serializer.serialize_unit_variant("AlgorithmStepType", 12u32, "MvObjectTypeChange"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of import errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportErrors {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ImportError>,
}
impl ImportErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The replication summary for the domain controller inbound neighbor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundReplicationNeighbor {
    #[doc = "The name of the source domain controller."]
    #[serde(rename = "sourceDomainController", default, skip_serializing_if = "Option::is_none")]
    pub source_domain_controller: Option<String>,
    #[doc = "The number of consecutive failure counts."]
    #[serde(rename = "consecutiveFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub consecutive_failure_count: Option<i64>,
    #[doc = "The naming context."]
    #[serde(rename = "namingContext", default, skip_serializing_if = "Option::is_none")]
    pub naming_context: Option<String>,
    #[doc = "The health status for the domain controller"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[doc = "The last time a sync was attempted on the domain controller."]
    #[serde(rename = "lastAttemptedSync", default, with = "azure_core::date::rfc3339::option")]
    pub last_attempted_sync: Option<time::OffsetDateTime>,
    #[doc = "The last time when a successful sync happened."]
    #[serde(rename = "lastSuccessfulSync", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_sync: Option<time::OffsetDateTime>,
    #[doc = "The last error code."]
    #[serde(rename = "lastErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<i64>,
    #[doc = "The error message of the last error."]
    #[serde(rename = "lastErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[doc = "The error title."]
    #[serde(rename = "errorTitle", default, skip_serializing_if = "Option::is_none")]
    pub error_title: Option<String>,
    #[doc = "The error description."]
    #[serde(rename = "errorDescription", default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[doc = "The link for the fix of the error."]
    #[serde(rename = "fixLink", default, skip_serializing_if = "Option::is_none")]
    pub fix_link: Option<String>,
    #[doc = "The details of the fix."]
    #[serde(rename = "fixDetails", default, skip_serializing_if = "Option::is_none")]
    pub fix_details: Option<String>,
    #[doc = "The additional details."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}
impl InboundReplicationNeighbor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of replication summary for the domain controller inbound neighbor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundReplicationNeighbors {
    #[doc = "The details of inbound replication neighbors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InboundReplicationNeighbor>,
}
impl InboundReplicationNeighbors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key value pair for properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Item {
    #[doc = "The key for the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The value for the key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Item {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of key value properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Items {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Item>,
}
impl azure_core::Continuable for Items {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Items {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The merged export error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergedExportError {
    #[doc = "The error Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The incoming object display name."]
    #[serde(rename = "incomingObjectDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub incoming_object_display_name: Option<String>,
    #[doc = "The incoming object type."]
    #[serde(rename = "incomingObjectType", default, skip_serializing_if = "Option::is_none")]
    pub incoming_object_type: Option<String>,
    #[doc = "The user principal name"]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "The type of the error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The attribute name."]
    #[serde(rename = "attributeName", default, skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc = "The attribute value."]
    #[serde(rename = "attributeValue", default, skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[doc = "The date and time when the error occurred."]
    #[serde(rename = "timeOccurred", default, with = "azure_core::date::rfc3339::option")]
    pub time_occurred: Option<time::OffsetDateTime>,
    #[doc = "The time when the error first occurred."]
    #[serde(rename = "timeFirstOccurred", default, with = "azure_core::date::rfc3339::option")]
    pub time_first_occurred: Option<time::OffsetDateTime>,
    #[doc = " the cs object Id."]
    #[serde(rename = "csObjectId", default, skip_serializing_if = "Option::is_none")]
    pub cs_object_id: Option<String>,
    #[doc = "the DN of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[doc = "Object that hold sync object details."]
    #[serde(rename = "incomingObject", default, skip_serializing_if = "Option::is_none")]
    pub incoming_object: Option<AssociatedObject>,
    #[doc = "Object that hold sync object details."]
    #[serde(rename = "existingObject", default, skip_serializing_if = "Option::is_none")]
    pub existing_object: Option<AssociatedObject>,
    #[doc = "The modified or removed attribute value."]
    #[serde(rename = "modifiedOrRemovedAttributeValue", default, skip_serializing_if = "Option::is_none")]
    pub modified_or_removed_attribute_value: Option<String>,
    #[doc = "The run step result Id."]
    #[serde(rename = "runStepResultId", default, skip_serializing_if = "Option::is_none")]
    pub run_step_result_id: Option<String>,
    #[doc = "The sam account name."]
    #[serde(rename = "samAccountName", default, skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    #[doc = "The server error details."]
    #[serde(rename = "serverErrorDetail", default, skip_serializing_if = "Option::is_none")]
    pub server_error_detail: Option<String>,
    #[doc = "The service Id."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The server Id."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
    #[doc = "The merged entity Id."]
    #[serde(rename = "mergedEntityId", default, skip_serializing_if = "Option::is_none")]
    pub merged_entity_id: Option<String>,
    #[doc = "The date and time, in UTC, when the error was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The export error status."]
    #[serde(rename = "exportErrorStatus", default, skip_serializing_if = "Option::is_none")]
    pub export_error_status: Option<i64>,
}
impl MergedExportError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of export errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergedExportErrors {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MergedExportError>,
}
impl azure_core::Continuable for MergedExportErrors {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MergedExportErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = " The metric group details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricGroup {
    #[doc = "The key for the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The display name for the group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "indicates if the metric group is displayed in Azure Active Directory Connect Health UI."]
    #[serde(rename = "invisibleForUi", default, skip_serializing_if = "Option::is_none")]
    pub invisible_for_ui: Option<bool>,
}
impl MetricGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric meta data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricMetadata {
    #[doc = "The name of the class which retrieve and process the metric."]
    #[serde(rename = "metricsProcessorClassName", default, skip_serializing_if = "Option::is_none")]
    pub metrics_processor_class_name: Option<String>,
    #[doc = "The metric name"]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[doc = "The groupings for the metrics."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groupings: Vec<MetricGroup>,
    #[doc = "The display name for the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicates if the metrics is a rate,value, percent or duration type."]
    #[serde(rename = "valueKind", default, skip_serializing_if = "Option::is_none")]
    pub value_kind: Option<String>,
    #[doc = "The minimum value."]
    #[serde(rename = "minValue", default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    #[doc = "The maximum value."]
    #[serde(rename = "maxValue", default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    #[doc = "Indicates whether the dashboard to represent the metric is a line, bar,pie, area or donut chart."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Indicates if the metric is a default metric or not."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Indicates if the metric is a performance counter metric or not."]
    #[serde(rename = "isPerfCounter", default, skip_serializing_if = "Option::is_none")]
    pub is_perf_counter: Option<bool>,
    #[doc = "Indicates if the metric is visible to DevOps or not."]
    #[serde(rename = "isDevOps", default, skip_serializing_if = "Option::is_none")]
    pub is_dev_ops: Option<bool>,
}
impl MetricMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of metric metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricMetadataList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricMetadata>,
    #[doc = "The total count of service elements."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for MetricMetadataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MetricMetadataList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = " The set of metric values. Example of a MetricSet are Values of token requests for a Server1 or RelyingParty1."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSet {
    #[doc = "The name of the set."]
    #[serde(rename = "setName", default, skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    #[doc = "The list of the metric values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<i64>,
}
impl MetricSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metrics data represented set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSets {
    #[doc = "The list of metric set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sets: Vec<MetricSet>,
    #[doc = "The list of timestamps for each metric in the metric set."]
    #[serde(rename = "timeStamps", default, skip_serializing_if = "Vec::is_empty")]
    pub time_stamps: Vec<time::OffsetDateTime>,
}
impl MetricSets {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of metric items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metrics {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Item>,
    #[doc = "The total count of metrics."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for Metrics {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The module configuration as required by the Agent service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleConfiguration {
    #[doc = "The name of agent service."]
    #[serde(rename = "agentService", default, skip_serializing_if = "Option::is_none")]
    pub agent_service: Option<String>,
    #[doc = "The name of the module for which the configuration is applicable."]
    #[serde(rename = "moduleName", default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[doc = "The key value pairs of properties required for configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ModuleConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of module configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleConfigurations {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ModuleConfiguration>,
}
impl ModuleConfigurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The objects with sync errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectWithSyncError {
    #[doc = "The source of authority."]
    #[serde(rename = "sourceOfAuthority", default, skip_serializing_if = "Option::is_none")]
    pub source_of_authority: Option<String>,
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The object type."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The attribute name."]
    #[serde(rename = "attributeName", default, skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc = "The attribute value."]
    #[serde(rename = "attributeValue", default, skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[doc = "The modified value."]
    #[serde(rename = "modififedValue", default, skip_serializing_if = "Option::is_none")]
    pub modififed_value: Option<String>,
    #[doc = "The user principal name."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "The object guid."]
    #[serde(rename = "objectGuid", default, skip_serializing_if = "Option::is_none")]
    pub object_guid: Option<String>,
    #[doc = "Indicates if the attribute is multi-valued or not."]
    #[serde(rename = "attributeMultiValues", default, skip_serializing_if = "Option::is_none")]
    pub attribute_multi_values: Option<bool>,
    #[doc = "The minimum limit."]
    #[serde(rename = "minLimit", default, skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<String>,
    #[doc = "The maximum limit."]
    #[serde(rename = "maxLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<String>,
    #[doc = "The distinguished name."]
    #[serde(rename = "distinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,
    #[doc = "The email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[doc = "The date and time of occurrence."]
    #[serde(rename = "timeOccured", default, with = "azure_core::date::rfc3339::option")]
    pub time_occured: Option<time::OffsetDateTime>,
    #[doc = "The error type."]
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[doc = "The source anchor."]
    #[serde(rename = "sourceAnchor", default, skip_serializing_if = "Option::is_none")]
    pub source_anchor: Option<String>,
}
impl ObjectWithSyncError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display details for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists all of the available REST API operations for Azure Active Directory Connect Health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResponse {
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "List of operations supported by the Microsoft.ADHybridHealthService resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The total count of operations."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token to get next set of operations."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for OperationListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the partition in Synchronization service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Partition {
    #[doc = "The partition Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The distinguished name for the partition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[doc = "Indicates if the partition object is selected or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The date and time when the partition is created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "The time and date when the partition was last modified."]
    #[serde(rename = "timeLastModified", default, with = "azure_core::date::rfc3339::option")]
    pub time_last_modified: Option<time::OffsetDateTime>,
    #[doc = "The connector partition scope."]
    #[serde(rename = "partitionScope", default, skip_serializing_if = "Option::is_none")]
    pub partition_scope: Option<PartitionScope>,
    #[doc = "The name of the partition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates if the partition is a domain or not."]
    #[serde(rename = "isDomain", default, skip_serializing_if = "Option::is_none")]
    pub is_domain: Option<bool>,
    #[doc = "The partition type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Partition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connector partition scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionScope {
    #[doc = "Indicates if the partition scope is default or not."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "The in-scope object classes."]
    #[serde(rename = "objectClasses", default, skip_serializing_if = "Vec::is_empty")]
    pub object_classes: Vec<String>,
    #[doc = "The list of containers included."]
    #[serde(rename = "containersIncluded", default, skip_serializing_if = "Vec::is_empty")]
    pub containers_included: Vec<String>,
    #[doc = "The list of containers excluded."]
    #[serde(rename = "containersExcluded", default, skip_serializing_if = "Vec::is_empty")]
    pub containers_excluded: Vec<String>,
}
impl PartitionScope {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The password has synchronization configuration settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordHashSyncConfiguration {
    #[doc = "Indicates if the password hash synchronization configuration settings is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl PasswordHashSyncConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The password management settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordManagementSettings {
    #[doc = "Indicates if the password extension is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The file path of the password management extension."]
    #[serde(rename = "extensionFilePath", default, skip_serializing_if = "Option::is_none")]
    pub extension_file_path: Option<String>,
    #[doc = "Connection point of password management."]
    #[serde(rename = "connectTo", default, skip_serializing_if = "Option::is_none")]
    pub connect_to: Option<String>,
    #[doc = "Connection timeout for password extension."]
    #[serde(rename = "connectionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,
    #[doc = "User to execute password extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The supported password operations."]
    #[serde(rename = "supportedPasswordOperations", default, skip_serializing_if = "Option::is_none")]
    pub supported_password_operations: Option<password_management_settings::SupportedPasswordOperations>,
    #[doc = "The maximum number of retries."]
    #[serde(rename = "maximumRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i64>,
    #[doc = "The time between retries."]
    #[serde(rename = "retryIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub retry_interval_in_seconds: Option<i64>,
    #[doc = "Indicates if a secure connection is required for password management."]
    #[serde(rename = "requiresSecureConnection", default, skip_serializing_if = "Option::is_none")]
    pub requires_secure_connection: Option<bool>,
    #[doc = "Indicates if accounts should be unlocked when resetting password."]
    #[serde(rename = "unlockAccount", default, skip_serializing_if = "Option::is_none")]
    pub unlock_account: Option<bool>,
}
impl PasswordManagementSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod password_management_settings {
    use super::*;
    #[doc = "The supported password operations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportedPasswordOperations")]
    pub enum SupportedPasswordOperations {
        Undefined,
        Set,
        Change,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SupportedPasswordOperations {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SupportedPasswordOperations {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SupportedPasswordOperations {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undefined => serializer.serialize_unit_variant("SupportedPasswordOperations", 0u32, "Undefined"),
                Self::Set => serializer.serialize_unit_variant("SupportedPasswordOperations", 1u32, "Set"),
                Self::Change => serializer.serialize_unit_variant("SupportedPasswordOperations", 2u32, "Change"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of replication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationDetailsList {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationSummary>,
    #[doc = "The total count of replication detail elements."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ReplicationDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = " Replication summary for a domain controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationStatus {
    #[doc = "The forest name."]
    #[serde(rename = "forestName", default, skip_serializing_if = "Option::is_none")]
    pub forest_name: Option<String>,
    #[doc = "The total number of domain controllers for a given forest."]
    #[serde(rename = "totalDcCount", default, skip_serializing_if = "Option::is_none")]
    pub total_dc_count: Option<i64>,
    #[doc = "The total number of domain controllers with error in a given forest."]
    #[serde(rename = "errorDcCount", default, skip_serializing_if = "Option::is_none")]
    pub error_dc_count: Option<i64>,
}
impl ReplicationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The replication summary for a domain controller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationSummary {
    #[doc = "The domain controller name."]
    #[serde(rename = "targetServer", default, skip_serializing_if = "Option::is_none")]
    pub target_server: Option<String>,
    #[doc = "The site name for a given domain controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[doc = "The domain name for a given domain controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The health status for a domain controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[doc = "The last time when a sync was attempted for a given domain controller."]
    #[serde(rename = "lastAttemptedSync", default, with = "azure_core::date::rfc3339::option")]
    pub last_attempted_sync: Option<time::OffsetDateTime>,
    #[doc = "The time when the last successful sync happened for a given domain controller."]
    #[serde(rename = "lastSuccessfulSync", default, with = "azure_core::date::rfc3339::option")]
    pub last_successful_sync: Option<time::OffsetDateTime>,
    #[doc = "List of individual domain controller neighbor's inbound replication status."]
    #[serde(rename = "inboundNeighborCollection", default, skip_serializing_if = "Vec::is_empty")]
    pub inbound_neighbor_collection: Vec<InboundReplicationNeighbor>,
}
impl ReplicationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of replication summary details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationSummaryList {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationSummary>,
}
impl azure_core::Continuable for ReplicationSummaryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ReplicationSummaryList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result for an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Result {
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
impl Result {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The blob uri pointing to Risky IP Report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RiskyIpBlobUri {
    #[doc = "The tenant id for whom the report belongs to."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The service id for whom the report belongs to."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The blob uri for the report."]
    #[serde(rename = "resultSasUri", default, skip_serializing_if = "Option::is_none")]
    pub result_sas_uri: Option<String>,
    #[doc = "Time at which the new Risky IP report was requested."]
    #[serde(rename = "blobCreateDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub blob_create_date_time: Option<time::OffsetDateTime>,
    #[doc = "Time at which the blob creation job for the new Risky IP report was completed."]
    #[serde(rename = "jobCompletionTime", default, with = "azure_core::date::rfc3339::option")]
    pub job_completion_time: Option<time::OffsetDateTime>,
    #[doc = "Status of the Risky IP report generation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RiskyIpBlobUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list containing blob uris."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RiskyIpBlobUris {
    #[doc = "The list of blob uris."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RiskyIpBlobUri>,
}
impl azure_core::Continuable for RiskyIpBlobUris {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RiskyIpBlobUris {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error details in legacy rule processing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleErrorInfo {
    #[doc = "The attribute mapping details."]
    #[serde(rename = "attributeMapping", default, skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<AttributeMapping>,
    #[doc = "The connector Id."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[doc = "The connector name."]
    #[serde(rename = "connectorName", default, skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[doc = "The object Id."]
    #[serde(rename = "csObjectId", default, skip_serializing_if = "Option::is_none")]
    pub cs_object_id: Option<String>,
    #[doc = "The distinguished name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
}
impl RuleErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the run profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunProfile {
    #[doc = "The run profile Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The run profile name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The run steps of the run profile."]
    #[serde(rename = "runSteps", default, skip_serializing_if = "Vec::is_empty")]
    pub run_steps: Vec<RunStep>,
}
impl RunProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of run profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunProfiles {
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RunProfile>,
}
impl RunProfiles {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The run step for a run profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunStep {
    #[doc = "The batch size used by the run step."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    #[doc = "The object processing limit."]
    #[serde(rename = "objectProcessLimit", default, skip_serializing_if = "Option::is_none")]
    pub object_process_limit: Option<i64>,
    #[doc = "The object deletion limit."]
    #[serde(rename = "objectDeleteLimit", default, skip_serializing_if = "Option::is_none")]
    pub object_delete_limit: Option<i64>,
    #[doc = "The page size of the run step."]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[doc = "The Id of the partition that a current run step operation is executing."]
    #[serde(rename = "partitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<String>,
    #[doc = "The run step operation types."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<i64>,
    #[doc = "The operation timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
impl RunStep {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceConfiguration {
    #[doc = "The version of the sync service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The service type of the server."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<i64>,
    #[doc = "The service account."]
    #[serde(rename = "serviceAccount", default, skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    #[doc = "The SQL server information."]
    #[serde(rename = "sqlServer", default, skip_serializing_if = "Option::is_none")]
    pub sql_server: Option<String>,
    #[doc = "The SQL version."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
    #[doc = "The SQL edition"]
    #[serde(rename = "sqlEdition", default, skip_serializing_if = "Option::is_none")]
    pub sql_edition: Option<String>,
    #[doc = "The SQL instance details."]
    #[serde(rename = "sqlInstance", default, skip_serializing_if = "Option::is_none")]
    pub sql_instance: Option<String>,
    #[doc = "The SQL database."]
    #[serde(rename = "sqlDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub sql_database_name: Option<String>,
    #[doc = "The SQL database size."]
    #[serde(rename = "sqlDatabaseSize", default, skip_serializing_if = "Option::is_none")]
    pub sql_database_size: Option<i64>,
}
impl ServiceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The server properties for a given service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceMember {
    #[doc = "The id of the server."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
    #[doc = "The service id to whom this server belongs."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The tenant id to whom this server belongs."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The total number of alerts that are currently active for the server."]
    #[serde(rename = "activeAlerts", default, skip_serializing_if = "Option::is_none")]
    pub active_alerts: Option<i64>,
    #[doc = "The additional information, if any, for the server."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    #[doc = "The date time , in UTC, when the server was onboarded to Azure Active Directory Connect Health."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The server specific configuration related dimensions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<serde_json::Value>,
    #[doc = "Indicates if the server is disabled or not. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[doc = "The reason for disabling the server."]
    #[serde(rename = "disabledReason", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<i64>,
    #[doc = "The list of installed QFEs for the server."]
    #[serde(rename = "installedQfes", default, skip_serializing_if = "Option::is_none")]
    pub installed_qfes: Option<serde_json::Value>,
    #[doc = "The date and time , in UTC, when the server was last disabled."]
    #[serde(rename = "lastDisabled", default, with = "azure_core::date::rfc3339::option")]
    pub last_disabled: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the server was last rebooted."]
    #[serde(rename = "lastReboot", default, with = "azure_core::date::rfc3339::option")]
    pub last_reboot: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the server's data monitoring configuration was last changed."]
    #[serde(
        rename = "lastServerReportedMonitoringLevelChange",
        default,
        with = "azure_core::date::rfc3339::option"
    )]
    pub last_server_reported_monitoring_level_change: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the server properties were last updated."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The id of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "The name of the server."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The monitoring configuration of the server which determines what activities are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoringConfigurationsComputed", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configurations_computed: Option<serde_json::Value>,
    #[doc = "The customized monitoring configuration of the server which determines what activities are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoringConfigurationsCustomized", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configurations_customized: Option<serde_json::Value>,
    #[doc = "The name of the operating system installed in the machine."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of the operating system installed in the machine."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Server specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The list of recommended hotfixes for the server."]
    #[serde(rename = "recommendedQfes", default, skip_serializing_if = "Option::is_none")]
    pub recommended_qfes: Option<serde_json::Value>,
    #[doc = "The total count of alerts that are resolved for this server."]
    #[serde(rename = "resolvedAlerts", default, skip_serializing_if = "Option::is_none")]
    pub resolved_alerts: Option<i64>,
    #[doc = "The service role that is being monitored in the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The monitoring level reported by the server."]
    #[serde(rename = "serverReportedMonitoringLevel", default, skip_serializing_if = "Option::is_none")]
    pub server_reported_monitoring_level: Option<service_member::ServerReportedMonitoringLevel>,
    #[doc = "The health status of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ServiceMember {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_member {
    use super::*;
    #[doc = "The monitoring level reported by the server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerReportedMonitoringLevel {
        Partial,
        Full,
        Off,
    }
}
#[doc = "The list of servers that are onboarded for a given service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceMembers {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceMember>,
    #[doc = "The total count of service elements."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for ServiceMembers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceMembers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service properties for a given service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProperties {
    #[doc = "The id of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The count of alerts that are currently active for the service."]
    #[serde(rename = "activeAlerts", default, skip_serializing_if = "Option::is_none")]
    pub active_alerts: Option<i64>,
    #[doc = "The additional information related to the service."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    #[doc = "The date and time, in UTC, when the service was onboarded to Azure Active Directory Connect Health."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The list of additional emails that are configured to receive notifications about the service."]
    #[serde(rename = "customNotificationEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_notification_emails: Vec<String>,
    #[doc = "Indicates if the service is disabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[doc = "The display name of the service."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The health of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[doc = "The date and time, in UTC, when the service was last disabled."]
    #[serde(rename = "lastDisabled", default, with = "azure_core::date::rfc3339::option")]
    pub last_disabled: Option<time::OffsetDateTime>,
    #[doc = "The date or time , in UTC, when the service properties were last updated."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The monitoring configuration of the service which determines what activities are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoringConfigurationsComputed", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configurations_computed: Option<serde_json::Value>,
    #[doc = "The customized monitoring configuration of the service which determines what activities are monitored by Azure Active Directory Connect Health."]
    #[serde(rename = "monitoringConfigurationsCustomized", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configurations_customized: Option<serde_json::Value>,
    #[doc = "Indicates if email notification is enabled or not."]
    #[serde(rename = "notificationEmailEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notification_email_enabled: Option<bool>,
    #[doc = "Indicates if email notification is enabled for global administrators of the tenant."]
    #[serde(
        rename = "notificationEmailEnabledForGlobalAdmins",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_email_enabled_for_global_admins: Option<bool>,
    #[doc = "Indicates if email notification is enabled for global administrators of the tenant."]
    #[serde(
        rename = "notificationEmailsEnabledForGlobalAdmins",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_emails_enabled_for_global_admins: Option<bool>,
    #[doc = "The list of emails to whom service notifications will be sent."]
    #[serde(rename = "notificationEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_emails: Vec<String>,
    #[doc = "Gets the original disable state."]
    #[serde(rename = "originalDisabledState", default, skip_serializing_if = "Option::is_none")]
    pub original_disabled_state: Option<bool>,
    #[doc = "The total count of alerts that has been resolved for the service."]
    #[serde(rename = "resolvedAlerts", default, skip_serializing_if = "Option::is_none")]
    pub resolved_alerts: Option<i64>,
    #[doc = "The id of the service."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The name of the service."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The signature of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "List of service specific configuration properties."]
    #[serde(rename = "simpleProperties", default, skip_serializing_if = "Option::is_none")]
    pub simple_properties: Option<serde_json::Value>,
    #[doc = "The id of the tenant to which the service is registered to."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The service type for the services onboarded to Azure Active Directory Connect Health. Depending on whether the service is monitoring, ADFS, Sync or ADDS roles, the service type can either be AdFederationService or AadSyncService or AdDomainService."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of services for a given onboarded tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Services {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The value returned by the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceProperties>,
    #[doc = "The total count of service elements."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The continuation token for paginated calls."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl azure_core::Continuable for Services {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Services {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details for export error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TabularExportError {
    #[doc = "The service Id."]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "The server Id."]
    #[serde(rename = "serviceMemberId", default, skip_serializing_if = "Option::is_none")]
    pub service_member_id: Option<String>,
    #[doc = "The merged entity Id."]
    #[serde(rename = "mergedEntityId", default, skip_serializing_if = "Option::is_none")]
    pub merged_entity_id: Option<String>,
    #[doc = "The export error data."]
    #[serde(rename = "tabularExportErrorData", default, skip_serializing_if = "Option::is_none")]
    pub tabular_export_error_data: Option<String>,
}
impl TabularExportError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the onboarded tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tenant {
    #[doc = "The Id of the tenant."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The Azure Active Directory license of the tenant."]
    #[serde(rename = "aadLicense", default, skip_serializing_if = "Option::is_none")]
    pub aad_license: Option<String>,
    #[doc = "Indicate if the tenant has Azure Active Directory Premium license or not."]
    #[serde(rename = "aadPremium", default, skip_serializing_if = "Option::is_none")]
    pub aad_premium: Option<bool>,
    #[doc = "Indicates if the tenant is configured to automatically receive updates for Azure Active Directory Connect Health client side features."]
    #[serde(rename = "agentAutoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_update: Option<bool>,
    #[doc = "The time in minutes after which an alert will be auto-suppressed."]
    #[serde(rename = "alertSuppressionTimeInMins", default, skip_serializing_if = "Option::is_none")]
    pub alert_suppression_time_in_mins: Option<i64>,
    #[doc = "Indicates if the tenant data can be seen by Microsoft through Azure portal."]
    #[serde(rename = "consentedToMicrosoftDevOps", default, skip_serializing_if = "Option::is_none")]
    pub consented_to_microsoft_dev_ops: Option<bool>,
    #[doc = "The country letter code of the tenant."]
    #[serde(rename = "countryLetterCode", default, skip_serializing_if = "Option::is_none")]
    pub country_letter_code: Option<String>,
    #[doc = "The date, in UTC, when the tenant was onboarded to Azure Active Directory Connect Health."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, till when the tenant data can be seen by Microsoft through Azure portal."]
    #[serde(rename = "devOpsTtl", default, with = "azure_core::date::rfc3339::option")]
    pub dev_ops_ttl: Option<time::OffsetDateTime>,
    #[doc = "Indicates if the tenant is disabled in Azure Active Directory Connect Health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[doc = "The reason due to which the tenant was disabled in Azure Active Directory Connect Health."]
    #[serde(rename = "disabledReason", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<i64>,
    #[doc = "The list of global administrators for the tenant."]
    #[serde(rename = "globalAdminsEmail", default, skip_serializing_if = "Vec::is_empty")]
    pub global_admins_email: Vec<String>,
    #[doc = "The initial domain of the tenant."]
    #[serde(rename = "initialDomain", default, skip_serializing_if = "Option::is_none")]
    pub initial_domain: Option<String>,
    #[doc = "The date and time, in UTC, when the tenant was last disabled in Azure Active Directory Connect Health."]
    #[serde(rename = "lastDisabled", default, with = "azure_core::date::rfc3339::option")]
    pub last_disabled: Option<time::OffsetDateTime>,
    #[doc = "The date and time, in UTC, when the tenant onboarding status in Azure Active Directory Connect Health was last verified."]
    #[serde(rename = "lastVerified", default, with = "azure_core::date::rfc3339::option")]
    pub last_verified: Option<time::OffsetDateTime>,
    #[doc = "Indicates if the tenant is allowed to  onboard to Azure Active Directory Connect Health."]
    #[serde(rename = "onboardingAllowed", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_allowed: Option<bool>,
    #[doc = "Indicates if the tenant is already onboarded to Azure Active Directory Connect Health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarded: Option<bool>,
    #[doc = "The certificate associated with the tenant to onboard data to Azure Active Directory Connect Health."]
    #[serde(rename = "pksCertificate", default, skip_serializing_if = "Option::is_none")]
    pub pks_certificate: Option<serde_json::Value>,
    #[doc = "Indicates if the tenant has signed up for private preview of Azure Active Directory Connect Health features."]
    #[serde(rename = "privatePreviewTenant", default, skip_serializing_if = "Option::is_none")]
    pub private_preview_tenant: Option<bool>,
    #[doc = "Indicates if data collection for this tenant is disabled or not."]
    #[serde(rename = "tenantInQuarantine", default, skip_serializing_if = "Option::is_none")]
    pub tenant_in_quarantine: Option<bool>,
    #[doc = "The name of the tenant."]
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}
impl Tenant {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The tenant onboarding details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantOnboardingDetails {
    #[doc = "Indicates if the tenant is onboarded to Azure Active Directory Connect Health or not."]
    #[serde(rename = "tenantOnboarded", default, skip_serializing_if = "Option::is_none")]
    pub tenant_onboarded: Option<bool>,
    #[doc = "The display url, to help tenant navigate or onboard to Azure Active Directory Connect Health blade, based on tenant onboarding status."]
    #[serde(rename = "onboardingDisplayUrl", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_display_url: Option<String>,
}
impl TenantOnboardingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = " The user preference for a given feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserPreference {
    #[doc = "The name of the metric."]
    #[serde(rename = "metricNames", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_names: Vec<String>,
}
impl UserPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value of the delta."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValueDelta {
    #[doc = "The operation type."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<value_delta::OperationType>,
    #[doc = "The value of the delta."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ValueDelta {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod value_delta {
    use super::*;
    #[doc = "The operation type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        Undefined,
        Add,
        Update,
        Delete,
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
                Self::Undefined => serializer.serialize_unit_variant("OperationType", 0u32, "Undefined"),
                Self::Add => serializer.serialize_unit_variant("OperationType", 1u32, "Add"),
                Self::Update => serializer.serialize_unit_variant("OperationType", 2u32, "Update"),
                Self::Delete => serializer.serialize_unit_variant("OperationType", 3u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
