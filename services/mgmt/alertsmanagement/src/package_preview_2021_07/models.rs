#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusRule {
    #[doc = "the name of the recording rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    #[doc = "the name of the alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    #[doc = "the flag that indicates whether the Prometheus rule is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "the expression to run for the rule."]
    pub expression: String,
    #[doc = "the severity of the alerts fired by the rule. Only relevant for alerts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[doc = "the amount of time alert must be active before firing. Only relevant for alerts."]
    #[serde(rename = "for", default, skip_serializing_if = "Option::is_none")]
    pub for_: Option<String>,
    #[doc = "labels for rule group. Only relevant for alerts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[doc = "annotations for rule group. Only relevant for alerts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    #[doc = "The array of actions that are performed when the alert rule becomes active, and when an alert condition is resolved. Only relevant for alerts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<PrometheusRuleGroupAction>,
    #[doc = "Specifies the Prometheus alert rule configuration."]
    #[serde(rename = "resolveConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub resolve_configuration: Option<PrometheusRuleResolveConfiguration>,
}
impl PrometheusRule {
    pub fn new(expression: String) -> Self {
        Self {
            record: None,
            alert: None,
            enabled: None,
            expression,
            severity: None,
            for_: None,
            labels: None,
            annotations: None,
            actions: Vec::new(),
            resolve_configuration: None,
        }
    }
}
#[doc = "An alert action. Only relevant for alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrometheusRuleGroupAction {
    #[doc = "The resource id of the action group to use."]
    #[serde(rename = "actionGroupId", default, skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[doc = "The properties of an action group object."]
    #[serde(rename = "actionProperties", default, skip_serializing_if = "Option::is_none")]
    pub action_properties: Option<serde_json::Value>,
}
impl PrometheusRuleGroupAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusRuleGroupProperties {
    #[doc = "the description of the Prometheus rule group that will be included in the alert email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the flag that indicates whether the Prometheus rule group is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "the cluster name of the rule group evaluation."]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "the list of resource id's that this rule group is scoped to."]
    pub scopes: Vec<String>,
    #[doc = "the interval in which to run the Prometheus rule group represented in ISO 8601 duration format. Should be between 1 and 15 minutes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "defines the rules in the Prometheus rule group."]
    pub rules: Vec<PrometheusRule>,
}
impl PrometheusRuleGroupProperties {
    pub fn new(scopes: Vec<String>, rules: Vec<PrometheusRule>) -> Self {
        Self {
            description: None,
            enabled: None,
            cluster_name: None,
            scopes,
            interval: None,
            rules,
        }
    }
}
#[doc = "The Prometheus rule group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrometheusRuleGroupResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "An alert rule."]
    pub properties: PrometheusRuleGroupProperties,
}
impl PrometheusRuleGroupResource {
    pub fn new(tracked_resource: TrackedResource, properties: PrometheusRuleGroupProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrometheusRuleGroupResourceCollection {
    #[doc = "the values for the alert rule resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrometheusRuleGroupResource>,
}
impl azure_core::Continuable for PrometheusRuleGroupResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrometheusRuleGroupResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Prometheus rule group resource for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrometheusRuleGroupResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<prometheus_rule_group_resource_patch::Properties>,
}
impl PrometheusRuleGroupResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod prometheus_rule_group_resource_patch {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "the flag that indicates whether the Prometheus rule group is enabled."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Specifies the Prometheus alert rule configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrometheusRuleResolveConfiguration {
    #[doc = "the flag that indicates whether or not to auto resolve a fired alert."]
    #[serde(rename = "autoResolved", default, skip_serializing_if = "Option::is_none")]
    pub auto_resolved: Option<bool>,
    #[doc = "the duration a rule must evaluate as healthy before the fired alert is automatically resolved represented in ISO 8601 duration format. Should be between 1 and 15 minutes"]
    #[serde(rename = "timeToResolve", default, skip_serializing_if = "Option::is_none")]
    pub time_to_resolve: Option<String>,
}
impl PrometheusRuleResolveConfiguration {
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
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
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(
        rename = "createdByType",
        default,
        skip_serializing_if = "Option::is_none",
        with = "azure_core::xml::text_content"
    )]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(
        rename = "lastModifiedByType",
        default,
        skip_serializing_if = "Option::is_none",
        with = "azure_core::xml::text_content"
    )]
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
