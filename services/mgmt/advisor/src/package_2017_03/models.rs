#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The operation supported by Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Microsoft Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Advisor operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationProperties {
    #[doc = "The category of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<recommendation_properties::Category>,
    #[doc = "The business impact of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<recommendation_properties::Impact>,
    #[doc = "The resource type identified by Advisor."]
    #[serde(rename = "impactedField", default, skip_serializing_if = "Option::is_none")]
    pub impacted_field: Option<String>,
    #[doc = "The resource identified by Advisor."]
    #[serde(rename = "impactedValue", default, skip_serializing_if = "Option::is_none")]
    pub impacted_value: Option<String>,
    #[doc = "The most recent time that Advisor checked the validity of the recommendation."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The recommendation metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The recommendation-type GUID."]
    #[serde(rename = "recommendationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type_id: Option<String>,
    #[doc = "The potential risk of not implementing the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<recommendation_properties::Risk>,
    #[doc = "A summary of the recommendation."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<ShortDescription>,
    #[doc = "The list of snoozed and dismissed rules for the recommendation."]
    #[serde(rename = "suppressionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub suppression_ids: Vec<String>,
}
impl RecommendationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommendation_properties {
    use super::*;
    #[doc = "The category of the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        HighAvailability,
        Security,
        Performance,
        Cost,
        OperationalExcellence,
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
                Self::HighAvailability => serializer.serialize_unit_variant("Category", 0u32, "HighAvailability"),
                Self::Security => serializer.serialize_unit_variant("Category", 1u32, "Security"),
                Self::Performance => serializer.serialize_unit_variant("Category", 2u32, "Performance"),
                Self::Cost => serializer.serialize_unit_variant("Category", 3u32, "Cost"),
                Self::OperationalExcellence => serializer.serialize_unit_variant("Category", 4u32, "OperationalExcellence"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The business impact of the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Impact")]
    pub enum Impact {
        High,
        Medium,
        Low,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Impact {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Impact {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Impact {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::High => serializer.serialize_unit_variant("Impact", 0u32, "High"),
                Self::Medium => serializer.serialize_unit_variant("Impact", 1u32, "Medium"),
                Self::Low => serializer.serialize_unit_variant("Impact", 2u32, "Low"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The potential risk of not implementing the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Risk")]
    pub enum Risk {
        Error,
        Warning,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Risk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Risk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Risk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("Risk", 0u32, "Error"),
                Self::Warning => serializer.serialize_unit_variant("Risk", 1u32, "Warning"),
                Self::None => serializer.serialize_unit_variant("Risk", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Advisor Recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRecommendationBase {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendationProperties>,
}
impl ResourceRecommendationBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Advisor recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRecommendationBaseListResult {
    #[doc = "The link used to get the next page of recommendations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of recommendations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceRecommendationBase>,
}
impl azure_core::Continuable for ResourceRecommendationBaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceRecommendationBaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A summary of the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShortDescription {
    #[doc = "The issue or opportunity identified by the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
    #[doc = "The remediation action suggested by the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
}
impl ShortDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the snoozed or dismissed rule; for example, the duration, name, and GUID associated with the rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuppressionContract {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the suppression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SuppressionProperties>,
}
impl SuppressionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the suppression."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuppressionProperties {
    #[doc = "The GUID of the suppression."]
    #[serde(rename = "suppressionId", default, skip_serializing_if = "Option::is_none")]
    pub suppression_id: Option<String>,
    #[doc = "The duration for which the suppression is valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}
impl SuppressionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
