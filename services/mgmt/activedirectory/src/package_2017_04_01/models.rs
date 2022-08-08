#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettings {
    #[doc = "The resource ID of the storage account to which you would like to send Diagnostic Logs."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The service bus rule Id of the diagnostic setting. This is here to maintain backwards compatibility."]
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[doc = "The workspace ID (resource ID of a Log Analytics workspace) for a Log Analytics workspace to which you would like to send Diagnostic Logs. Example: /subscriptions/4b9e8510-67ab-4e9a-95a9-e2f1e570ea9c/resourceGroups/insights-integration/providers/Microsoft.OperationalInsights/workspaces/viruela2"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The resource Id for the event hub authorization rule."]
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[doc = "The name of the event hub. If none is specified, the default event hub will be selected."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[doc = "The list of logs settings."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<LogSettings>,
}
impl DiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic settings Category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategory {
    #[doc = "The type of the diagnostic settings category."]
    #[serde(rename = "categoryType", default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<diagnostic_settings_category::CategoryType>,
}
impl DiagnosticSettingsCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_settings_category {
    use super::*;
    #[doc = "The type of the diagnostic settings category."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CategoryType")]
    pub enum CategoryType {
        Logs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CategoryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CategoryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CategoryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Logs => serializer.serialize_unit_variant("CategoryType", 0u32, "Logs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The diagnostic settings category resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategoryResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "The diagnostic settings Category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettingsCategory>,
}
impl DiagnosticSettingsCategoryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of diagnostic setting category resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategoryResourceCollection {
    #[doc = "The collection of diagnostic settings category resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticSettingsCategoryResource>,
}
impl DiagnosticSettingsCategoryResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "The diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettings>,
}
impl DiagnosticSettingsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsResourceCollection {
    #[doc = "The collection of diagnostic settings resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticSettingsResource>,
}
impl DiagnosticSettingsResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the localized display information for this particular operation / action. These value will be used by several clients for (1) custom role definitions for RBAC; (2) complex query filters for the event service; and (3) audit history / records for management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = "The publisher. The localized friendly form of the resource publisher name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The provider. The localized friendly form of the resource provider name – it is expected to also include the publisher/company responsible. It should use Title Casing and begin with \"Microsoft\" for 1st party services. e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource. The localized friendly form of the resource related to this action/operation – it should match the public documentation for the resource provider. It should use Title Casing. This value should be unique for a particular URL type (e.g. nested types should *not* reuse their parent’s display.resource field). e.g. \"Virtual Machines\" or \"Scheduler Job Collections\", or \"Virtual Machine VM Sizes\" or \"Scheduler Jobs\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation. The localized friendly name for the operation, as it should be shown to the user. It should be concise (to fit in drop downs) but clear (i.e. self-documenting). It should use Title Casing. Prescriptive guidance: Read Create or Update Delete 'ActionName'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description. The localized friendly description for the operation, as it should be shown to the user. It should be thorough, yet concise – it will be used in tool tips and detailed views. Prescriptive guidance for namespaces: Read any 'display.provider' resource Create or Update any 'display.provider' resource Delete any 'display.provider' resource Perform any other action on any 'display.provider' resource Prescriptive guidance for namespaces: Read any 'display.resource' Create or Update any 'display.resource' Delete any 'display.resource' 'ActionName' any 'display.resources'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Part of MultiTenantDiagnosticSettings. Specifies the settings for a particular log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSettings {
    #[doc = "Name of a Diagnostic Log category for a resource type this setting is applied to. To obtain the list of Diagnostic Log categories for a resource, first perform a GET diagnostic settings operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<log_settings::Category>,
    #[doc = "A value indicating whether this log is enabled."]
    pub enabled: bool,
    #[doc = "Specifies the retention policy for the log."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl LogSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            category: None,
            enabled,
            retention_policy: None,
        }
    }
}
pub mod log_settings {
    use super::*;
    #[doc = "Name of a Diagnostic Log category for a resource type this setting is applied to. To obtain the list of Diagnostic Log categories for a resource, first perform a GET diagnostic settings operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        AuditLogs,
        SignInLogs,
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
                Self::AuditLogs => serializer.serialize_unit_variant("Category", 0u32, "AuditLogs"),
                Self::SignInLogs => serializer.serialize_unit_variant("Category", 1u32, "SignInLogs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Operations discovery class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscovery {
    #[doc = "Name of the API. The name of the operation being performed on this particular object. It should match the action name that appears in RBAC / the event service. Examples of operations include: * Microsoft.Compute/virtualMachine/capture/action * Microsoft.Compute/virtualMachine/restart/action * Microsoft.Compute/virtualMachine/write * Microsoft.Compute/virtualMachine/read * Microsoft.Compute/virtualMachine/delete Each action should include, in order: (1) Resource Provider Namespace (2) Type hierarchy for which the action applies (e.g. server/databases for a SQL Azure database) (3) Read, Write, Action or Delete indicating which type applies. If it is a PUT/PATCH on a collection or named value, Write should be used. If it is a GET, Read should be used. If it is a DELETE, Delete should be used. If it is a POST, Action should be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action. These value will be used by several clients for (1) custom role definitions for RBAC; (2) complex query filters for the event service; and (3) audit history / records for management operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[doc = "Origin. The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "ClientDiscovery properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationsDiscoveryProperties>,
}
impl OperationsDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of ClientDiscovery details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscoveryCollection {
    #[doc = "The ClientDiscovery details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDiscovery>,
}
impl OperationsDiscoveryCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ClientDiscovery properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscoveryProperties {}
impl OperationsDiscoveryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A proxy only azure resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the retention policy for the log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "A value indicating whether the retention policy is enabled."]
    pub enabled: bool,
    #[doc = "The number of days for the retention in days. A value of 0 will retain the events indefinitely."]
    pub days: i32,
}
impl RetentionPolicy {
    pub fn new(enabled: bool, days: i32) -> Self {
        Self { enabled, days }
    }
}
