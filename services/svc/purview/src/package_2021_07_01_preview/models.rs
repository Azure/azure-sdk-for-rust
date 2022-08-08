#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Attribute matcher for a rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeMatcher {
    #[doc = "AttributeName"]
    #[serde(rename = "attributeName", default, skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc = "Value for attribute"]
    #[serde(rename = "attributeValueIncludes", default, skip_serializing_if = "Option::is_none")]
    pub attribute_value_includes: Option<String>,
    #[doc = "List of values for attribute"]
    #[serde(rename = "attributeValueIncludedIn", default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_value_included_in: Vec<String>,
    #[doc = "Value excluded for attribute"]
    #[serde(rename = "attributeValueExcludes", default, skip_serializing_if = "Option::is_none")]
    pub attribute_value_excludes: Option<String>,
    #[doc = "List of values excluded for attribute"]
    #[serde(rename = "attributeValueExcludedIn", default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_value_excluded_in: Vec<String>,
}
impl AttributeMatcher {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The attribute rule for a policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeRule {
    #[doc = "The kind of rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<attribute_rule::Kind>,
    #[doc = "The id for rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name for rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The dnf Condition for a rule"]
    #[serde(rename = "dnfCondition", default, skip_serializing_if = "Option::is_none")]
    pub dnf_condition: Option<DnfCondition>,
}
impl AttributeRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod attribute_rule {
    use super::*;
    #[doc = "The kind of rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "attributerule")]
        Attributerule,
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
                Self::Attributerule => serializer.serialize_unit_variant("Kind", 0u32, "attributerule"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type CnfCondition = Vec<Vec<AttributeMatcher>>;
#[doc = "The collection reference for a policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionReference {
    #[doc = "The type of reference"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of reference"]
    #[serde(rename = "referenceName", default, skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<String>,
}
impl CollectionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The decision rule for a policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DecisionRule {
    #[doc = "The kind of rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<decision_rule::Kind>,
    #[doc = "The effect for rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<decision_rule::Effect>,
    #[doc = "The dnf Condition for a rule"]
    #[serde(rename = "dnfCondition", default, skip_serializing_if = "Option::is_none")]
    pub dnf_condition: Option<DnfCondition>,
}
impl DecisionRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod decision_rule {
    use super::*;
    #[doc = "The kind of rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "decisionrule")]
        Decisionrule,
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
                Self::Decisionrule => serializer.serialize_unit_variant("Kind", 0u32, "decisionrule"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The effect for rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Effect")]
    pub enum Effect {
        Deny,
        Permit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Effect {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Effect {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Effect {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Deny => serializer.serialize_unit_variant("Effect", 0u32, "Deny"),
                Self::Permit => serializer.serialize_unit_variant("Effect", 1u32, "Permit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type DnfCondition = Vec<Vec<AttributeMatcher>>;
#[doc = "The error model for metadata policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorModel {
    #[doc = "The error code"]
    pub code: String,
    #[doc = "The error message"]
    pub message: String,
    #[doc = "The error target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorModel>,
}
impl ErrorModel {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "The error response model for metadata policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseModel {
    #[doc = "The error model for metadata policy"]
    pub error: ErrorModel,
}
impl azure_core::Continuable for ErrorResponseModel {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseModel {
    pub fn new(error: ErrorModel) -> Self {
        Self { error }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataPolicy {
    #[doc = "The name of policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The id of policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The version of policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataPolicyProperties>,
}
impl MetadataPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Metadata Policies"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataPolicyList {
    pub values: Vec<MetadataPolicy>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MetadataPolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MetadataPolicyList {
    pub fn new(values: Vec<MetadataPolicy>) -> Self {
        Self { values, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataPolicyProperties {
    #[doc = "The description of policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The DecisionRules of policy"]
    #[serde(rename = "decisionRules", default, skip_serializing_if = "Vec::is_empty")]
    pub decision_rules: Vec<DecisionRule>,
    #[doc = "The AttributeRules of policy"]
    #[serde(rename = "attributeRules", default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_rules: Vec<AttributeRule>,
    #[doc = "The collection reference for a policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionReference>,
    #[doc = "The parent collection of the policy"]
    #[serde(rename = "parentCollectionName", default, skip_serializing_if = "Option::is_none")]
    pub parent_collection_name: Option<String>,
}
impl MetadataPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataRole {
    #[doc = "The Id of role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of role"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataRoleProperties>,
}
impl MetadataRole {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Metadata roles"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataRoleList {
    pub values: Vec<MetadataRole>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MetadataRoleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MetadataRoleList {
    pub fn new(values: Vec<MetadataRole>) -> Self {
        Self { values, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataRoleProperties {
    #[doc = "The provisioningState of role"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The type of role"]
    #[serde(rename = "roleType", default, skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    #[doc = "The friendly name of role"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The description of role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The cnf Condition for a rule"]
    #[serde(rename = "cnfCondition", default, skip_serializing_if = "Option::is_none")]
    pub cnf_condition: Option<CnfCondition>,
    #[doc = "The dnf Condition for a rule"]
    #[serde(rename = "dnfCondition", default, skip_serializing_if = "Option::is_none")]
    pub dnf_condition: Option<DnfCondition>,
    #[doc = "The version of role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl MetadataRoleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
