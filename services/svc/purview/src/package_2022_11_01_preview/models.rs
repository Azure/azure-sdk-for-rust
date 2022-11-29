#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevopsPolicy {
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Array of decisionRules for the policy"]
    #[serde(rename = "decisionRules")]
    pub decision_rules: Vec<DevopsPolicyDecisionRule>,
    #[doc = "Purview resource name"]
    #[serde(rename = "purviewResourceName")]
    pub purview_resource_name: String,
}
impl DevopsPolicy {
    pub fn new(decision_rules: Vec<DevopsPolicyDecisionRule>, purview_resource_name: String) -> Self {
        Self {
            system_data: None,
            decision_rules,
            purview_resource_name,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevopsPolicyDecisionRule {
    #[doc = "The effect for rule"]
    pub effect: devops_policy_decision_rule::Effect,
    #[doc = "This is field will indicate the role of a devops policy."]
    #[serde(rename = "purviewRoleName", default, skip_serializing_if = "Option::is_none")]
    pub purview_role_name: Option<String>,
    #[doc = "Azure Active Directory Ids for users"]
    pub principals: Vec<String>,
    #[doc = "Azure Active Directory Ids for groups"]
    #[serde(rename = "principalGroups")]
    pub principal_groups: Vec<String>,
    #[doc = "Optional resource path of the azure resource"]
    #[serde(rename = "resourceAzurePath", default, skip_serializing_if = "Option::is_none")]
    pub resource_azure_path: Option<String>,
}
impl DevopsPolicyDecisionRule {
    pub fn new(effect: devops_policy_decision_rule::Effect, principals: Vec<String>, principal_groups: Vec<String>) -> Self {
        Self {
            effect,
            purview_role_name: None,
            principals,
            principal_groups,
            resource_azure_path: None,
        }
    }
}
pub mod devops_policy_decision_rule {
    use super::*;
    #[doc = "The effect for rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Effect")]
    pub enum Effect {
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
                Self::Permit => serializer.serialize_unit_variant("Effect", 0u32, "Permit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevopsPolicyList {
    #[doc = "List of policies"]
    pub value: Vec<DevopsPolicy>,
    #[doc = "Pagination link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevopsPolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DevopsPolicyList {
    pub fn new(value: Vec<DevopsPolicy>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The error model for policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorModel {
    #[doc = "The error code"]
    pub code: String,
    #[doc = "The error message"]
    pub message: String,
    #[doc = "The error details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorModel>,
}
impl ErrorModel {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
#[doc = "The error response model for policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseModel {
    #[doc = "The error model for policy"]
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
pub struct SystemData {
    #[doc = "Created by"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Created by type"]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[doc = "Created at"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "last modified by"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "last modified by type"]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[doc = "last modified at"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
