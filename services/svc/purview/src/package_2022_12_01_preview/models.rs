#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "List of Self-service policies"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServicePoliciesList {
    #[doc = "List of policies"]
    pub value: Vec<SelfServicePolicy>,
    #[doc = "Pagination link"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SelfServicePoliciesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SelfServicePoliciesList {
    pub fn new(value: Vec<SelfServicePolicy>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The Self-service policy model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServicePolicy {
    #[doc = "The policyId of the self-service policy"]
    #[serde(rename = "policyId")]
    pub policy_id: String,
    #[doc = "The requestor of the self-service policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requestor: Option<String>,
    #[doc = "Expiry Date of the policy"]
    #[serde(rename = "expiresAt", default, with = "azure_core::date::rfc3339::option")]
    pub expires_at: Option<time::OffsetDateTime>,
    #[doc = "The decision rule of a SelfServicePolicy"]
    #[serde(rename = "decisionRule")]
    pub decision_rule: SelfServicePolicyDecisionRule,
    #[doc = "The system data"]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SelfServicePolicy {
    pub fn new(policy_id: String, decision_rule: SelfServicePolicyDecisionRule) -> Self {
        Self {
            policy_id,
            requestor: None,
            expires_at: None,
            decision_rule,
            system_data: None,
        }
    }
}
#[doc = "The decision rule of a SelfServicePolicy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServicePolicyDecisionRule {
    #[doc = "The effect for rule"]
    pub effect: self_service_policy_decision_rule::Effect,
    #[doc = "This is field will indicate the role of a self service policy."]
    #[serde(rename = "purviewRoleName")]
    pub purview_role_name: String,
    #[doc = "Azure Active Directory Ids for users"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub principals: Vec<String>,
    #[doc = "Azure Active Directory Ids for groups"]
    #[serde(
        rename = "principalGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub principal_groups: Vec<String>,
    #[doc = "The resource path of the azure resource"]
    #[serde(rename = "resourceAzurePath")]
    pub resource_azure_path: String,
}
impl SelfServicePolicyDecisionRule {
    pub fn new(effect: self_service_policy_decision_rule::Effect, purview_role_name: String, resource_azure_path: String) -> Self {
        Self {
            effect,
            purview_role_name,
            principals: Vec::new(),
            principal_groups: Vec::new(),
            resource_azure_path,
        }
    }
}
pub mod self_service_policy_decision_rule {
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
#[doc = "The system data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "Created by"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Created at"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "last modified by"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "last modified at"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
