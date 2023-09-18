#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Describes the format of Alert Rule Recommendations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleRecommendationProperties {
    #[doc = "The recommendation alert rule type."]
    #[serde(rename = "alertRuleType")]
    pub alert_rule_type: String,
    #[doc = "A dictionary that provides the display information for an alert rule recommendation."]
    #[serde(rename = "displayInformation")]
    pub display_information: serde_json::Value,
    #[doc = "A complete ARM template to deploy the alert rules."]
    #[serde(rename = "ruleArmTemplate")]
    pub rule_arm_template: RuleArmTemplate,
}
impl AlertRuleRecommendationProperties {
    pub fn new(alert_rule_type: String, display_information: serde_json::Value, rule_arm_template: RuleArmTemplate) -> Self {
        Self {
            alert_rule_type,
            display_information,
            rule_arm_template,
        }
    }
}
#[doc = "A single alert rule recommendation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleRecommendationResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the format of Alert Rule Recommendations response."]
    pub properties: AlertRuleRecommendationProperties,
}
impl AlertRuleRecommendationResource {
    pub fn new(properties: AlertRuleRecommendationProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "List of alert rule recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleRecommendationsListResponse {
    #[doc = "the values for the alert rule recommendations."]
    pub value: Vec<AlertRuleRecommendationResource>,
    #[doc = "URL to fetch the next set of recommendations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertRuleRecommendationsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertRuleRecommendationsListResponse {
    pub fn new(value: Vec<AlertRuleRecommendationResource>) -> Self {
        Self { value, next_link: None }
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
#[doc = "The unit to display for a metric alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricAlertsDisplayUnit")]
pub enum MetricAlertsDisplayUnit {
    None,
    Percentage,
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
    Petabytes,
    BytesPerDay,
    BytesPerHour,
    BytesPerMinute,
    BytesPerSecond,
    KilobytesPerSecond,
    MegabytesPerSecond,
    GigabytesPerSecond,
    TerabytesPerSecond,
    PetabytesPerSecond,
    Count,
    Thousand,
    Million,
    Billion,
    Trillion,
    MicroSeconds,
    MilliSeconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    CountPerDay,
    CountPerHour,
    CountPerMinute,
    CountPerSecond,
    ThousandPerSecond,
    MillionPerSecond,
    BillionPerSecond,
    TrillionPerSecond,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricAlertsDisplayUnit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricAlertsDisplayUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricAlertsDisplayUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 0u32, "None"),
            Self::Percentage => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 1u32, "Percentage"),
            Self::Bytes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 2u32, "Bytes"),
            Self::Kilobytes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 3u32, "Kilobytes"),
            Self::Megabytes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 4u32, "Megabytes"),
            Self::Gigabytes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 5u32, "Gigabytes"),
            Self::Terabytes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 6u32, "Terabytes"),
            Self::Petabytes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 7u32, "Petabytes"),
            Self::BytesPerDay => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 8u32, "BytesPerDay"),
            Self::BytesPerHour => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 9u32, "BytesPerHour"),
            Self::BytesPerMinute => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 10u32, "BytesPerMinute"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 11u32, "BytesPerSecond"),
            Self::KilobytesPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 12u32, "KilobytesPerSecond"),
            Self::MegabytesPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 13u32, "MegabytesPerSecond"),
            Self::GigabytesPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 14u32, "GigabytesPerSecond"),
            Self::TerabytesPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 15u32, "TerabytesPerSecond"),
            Self::PetabytesPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 16u32, "PetabytesPerSecond"),
            Self::Count => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 17u32, "Count"),
            Self::Thousand => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 18u32, "Thousand"),
            Self::Million => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 19u32, "Million"),
            Self::Billion => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 20u32, "Billion"),
            Self::Trillion => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 21u32, "Trillion"),
            Self::MicroSeconds => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 22u32, "MicroSeconds"),
            Self::MilliSeconds => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 23u32, "MilliSeconds"),
            Self::Seconds => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 24u32, "Seconds"),
            Self::Minutes => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 25u32, "Minutes"),
            Self::Hours => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 26u32, "Hours"),
            Self::Days => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 27u32, "Days"),
            Self::CountPerDay => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 28u32, "CountPerDay"),
            Self::CountPerHour => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 29u32, "CountPerHour"),
            Self::CountPerMinute => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 30u32, "CountPerMinute"),
            Self::CountPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 31u32, "CountPerSecond"),
            Self::ThousandPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 32u32, "ThousandPerSecond"),
            Self::MillionPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 33u32, "MillionPerSecond"),
            Self::BillionPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 34u32, "BillionPerSecond"),
            Self::TrillionPerSecond => serializer.serialize_unit_variant("MetricAlertsDisplayUnit", 35u32, "TrillionPerSecond"),
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
#[doc = "A complete ARM template to deploy the alert rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleArmTemplate {
    #[doc = "JSON schema reference"]
    #[serde(rename = "$schema")]
    pub schema: String,
    #[doc = "A 4 number format for the version number of this template file. For example, 1.0.0.0"]
    #[serde(rename = "contentVersion")]
    pub content_version: String,
    #[doc = "Variable definitions"]
    pub variables: serde_json::Value,
    #[doc = "Input parameter definitions"]
    pub parameters: serde_json::Value,
    #[doc = "Alert rule resource definitions"]
    pub resources: Vec<serde_json::Value>,
}
impl RuleArmTemplate {
    pub fn new(
        schema: String,
        content_version: String,
        variables: serde_json::Value,
        parameters: serde_json::Value,
        resources: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            schema,
            content_version,
            variables,
            parameters,
            resources,
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
