#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Details of the actor identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewActorIdentity {
    #[doc = "The identity id"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The identity type : user/servicePrincipal"]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<access_review_actor_identity::PrincipalType>,
    #[doc = "The identity display name"]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "The user principal name(if valid)"]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
impl AccessReviewActorIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_actor_identity {
    use super::*;
    #[doc = "The identity type : user/servicePrincipal"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "servicePrincipal")]
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "user"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 1u32, "servicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Access Review Contacted Reviewer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewContactedReviewer {
    #[doc = "The access review reviewer id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review reviewer id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of access review contacted reviewer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewContactedReviewerProperties>,
}
impl AccessReviewContactedReviewer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of access review contacted reviewers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewContactedReviewerListResult {
    #[doc = "Access Review Contacted Reviewer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessReviewContactedReviewer>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewContactedReviewerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessReviewContactedReviewerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of access review contacted reviewer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewContactedReviewerProperties {
    #[doc = "The display name of the reviewer"]
    #[serde(rename = "userDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[doc = "The user principal name of the reviewer"]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "Date Time when the reviewer was contacted."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
}
impl AccessReviewContactedReviewerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecision {
    #[doc = "The access review decision id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review decision name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Approval Step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewDecisionProperties>,
}
impl AccessReviewDecision {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target of the decision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionIdentity {
    #[doc = "The id of principal whose access was reviewed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the user whose access was reviewed."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AccessReviewDecisionIdentity {
    pub fn new() -> Self {
        Self {
            id: None,
            display_name: None,
        }
    }
}
#[doc = "The type of decision target : User/ServicePrincipal"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AccessReviewDecisionIdentityUnion {
    #[serde(rename = "servicePrincipal")]
    ServicePrincipal(AccessReviewDecisionServicePrincipalIdentity),
    #[serde(rename = "user")]
    User(AccessReviewDecisionUserIdentity),
}
#[doc = "Access Review Decision Insight."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecisionInsight {
    #[doc = "The access review insight id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review insight name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Details of the Insight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewDecisionInsightPropertiesUnion>,
}
impl AccessReviewDecisionInsight {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the Insight."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionInsightProperties {
    #[doc = "Date Time when the insight was created."]
    #[serde(rename = "insightCreatedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub insight_created_date_time: Option<serde_json::Value>,
}
impl AccessReviewDecisionInsightProperties {
    pub fn new() -> Self {
        Self {
            insight_created_date_time: None,
        }
    }
}
#[doc = "The type of insight"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AccessReviewDecisionInsightPropertiesUnion {
    #[serde(rename = "userSignInInsight")]
    UserSignInInsight(AccessReviewDecisionUserSignInInsightProperties),
}
#[doc = "List of access review decisions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecisionListResult {
    #[doc = "Access Review Decision list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessReviewDecision>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewDecisionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessReviewDecisionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target of the decision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecisionPrincipalResourceMembership {
    #[doc = "Every decision item in an access review represents a principal's membership to a resource. This property represents details of the membership. Examples of this detail might be whether the principal has direct access or indirect access"]
    #[serde(
        rename = "membershipTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub membership_types: Vec<String>,
}
impl AccessReviewDecisionPrincipalResourceMembership {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Approval Step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecisionProperties {
    #[doc = "Target of the decision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<AccessReviewDecisionIdentityUnion>,
    #[doc = "Target of the decision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<AccessReviewDecisionResource>,
    #[doc = "The feature- generated recommendation shown to the reviewer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<access_review_decision_properties::Recommendation>,
    #[doc = "The decision on the approval step. This value is initially set to NotReviewed. Approvers can take action of Approve/Deny"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<access_review_decision_properties::Decision>,
    #[doc = "Justification provided by approvers for their action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Date Time when a decision was taken."]
    #[serde(rename = "reviewedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub reviewed_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "reviewedBy", default, skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<AccessReviewActorIdentity>,
    #[doc = "The outcome of applying the decision."]
    #[serde(rename = "applyResult", default, skip_serializing_if = "Option::is_none")]
    pub apply_result: Option<access_review_decision_properties::ApplyResult>,
    #[doc = "The date and time when the review decision was applied."]
    #[serde(rename = "appliedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub applied_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "appliedBy", default, skip_serializing_if = "Option::is_none")]
    pub applied_by: Option<AccessReviewActorIdentity>,
    #[doc = "This is the collection of insights for this decision item."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub insights: Vec<AccessReviewDecisionInsight>,
    #[doc = "Target of the decision."]
    #[serde(rename = "principalResourceMembership", default, skip_serializing_if = "Option::is_none")]
    pub principal_resource_membership: Option<AccessReviewDecisionPrincipalResourceMembership>,
}
impl AccessReviewDecisionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_decision_properties {
    use super::*;
    #[doc = "The feature- generated recommendation shown to the reviewer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Recommendation")]
    pub enum Recommendation {
        Approve,
        Deny,
        NoInfoAvailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Recommendation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Recommendation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Recommendation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Approve => serializer.serialize_unit_variant("Recommendation", 0u32, "Approve"),
                Self::Deny => serializer.serialize_unit_variant("Recommendation", 1u32, "Deny"),
                Self::NoInfoAvailable => serializer.serialize_unit_variant("Recommendation", 2u32, "NoInfoAvailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The decision on the approval step. This value is initially set to NotReviewed. Approvers can take action of Approve/Deny"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Decision")]
    pub enum Decision {
        Approve,
        Deny,
        NotReviewed,
        DontKnow,
        NotNotified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Decision {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Decision {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Decision {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Approve => serializer.serialize_unit_variant("Decision", 0u32, "Approve"),
                Self::Deny => serializer.serialize_unit_variant("Decision", 1u32, "Deny"),
                Self::NotReviewed => serializer.serialize_unit_variant("Decision", 2u32, "NotReviewed"),
                Self::DontKnow => serializer.serialize_unit_variant("Decision", 3u32, "DontKnow"),
                Self::NotNotified => serializer.serialize_unit_variant("Decision", 4u32, "NotNotified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The outcome of applying the decision."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplyResult")]
    pub enum ApplyResult {
        New,
        Applying,
        AppliedSuccessfully,
        AppliedWithUnknownFailure,
        AppliedSuccessfullyButObjectNotFound,
        ApplyNotSupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplyResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplyResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplyResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::New => serializer.serialize_unit_variant("ApplyResult", 0u32, "New"),
                Self::Applying => serializer.serialize_unit_variant("ApplyResult", 1u32, "Applying"),
                Self::AppliedSuccessfully => serializer.serialize_unit_variant("ApplyResult", 2u32, "AppliedSuccessfully"),
                Self::AppliedWithUnknownFailure => serializer.serialize_unit_variant("ApplyResult", 3u32, "AppliedWithUnknownFailure"),
                Self::AppliedSuccessfullyButObjectNotFound => {
                    serializer.serialize_unit_variant("ApplyResult", 4u32, "AppliedSuccessfullyButObjectNotFound")
                }
                Self::ApplyNotSupported => serializer.serialize_unit_variant("ApplyResult", 5u32, "ApplyNotSupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Target of the decision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionResource {
    #[doc = "The type of resource"]
    #[serde(rename = "type")]
    pub type_: access_review_decision_resource::Type,
    #[doc = "The id of resource associated with a decision record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of resource associated with a decision record."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AccessReviewDecisionResource {
    pub fn new(type_: access_review_decision_resource::Type) -> Self {
        Self {
            type_,
            id: None,
            display_name: None,
        }
    }
}
pub mod access_review_decision_resource {
    use super::*;
    #[doc = "The type of resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "azureRole")]
        AzureRole,
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
                Self::AzureRole => serializer.serialize_unit_variant("Type", 0u32, "azureRole"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Service Principal Decision Target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionServicePrincipalIdentity {
    #[serde(flatten)]
    pub access_review_decision_identity: AccessReviewDecisionIdentity,
    #[doc = "The appId for the service principal entity being reviewed"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}
impl AccessReviewDecisionServicePrincipalIdentity {
    pub fn new(access_review_decision_identity: AccessReviewDecisionIdentity) -> Self {
        Self {
            access_review_decision_identity,
            app_id: None,
        }
    }
}
#[doc = "User Decision Target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionUserIdentity {
    #[serde(flatten)]
    pub access_review_decision_identity: AccessReviewDecisionIdentity,
    #[doc = "The user principal name of the user whose access was reviewed."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
impl AccessReviewDecisionUserIdentity {
    pub fn new(access_review_decision_identity: AccessReviewDecisionIdentity) -> Self {
        Self {
            access_review_decision_identity,
            user_principal_name: None,
        }
    }
}
#[doc = "User Decision Target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionUserSignInInsightProperties {
    #[serde(flatten)]
    pub access_review_decision_insight_properties: AccessReviewDecisionInsightProperties,
    #[doc = "Date Time when the user signed into the tenant."]
    #[serde(rename = "lastSignInDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_sign_in_date_time: Option<serde_json::Value>,
}
impl AccessReviewDecisionUserSignInInsightProperties {
    pub fn new(access_review_decision_insight_properties: AccessReviewDecisionInsightProperties) -> Self {
        Self {
            access_review_decision_insight_properties,
            last_sign_in_date_time: None,
        }
    }
}
#[doc = "Access Review Default Settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDefaultSettings {
    #[doc = "The access review default settings id. This is only going to be default"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review default settings name. This is always going to be Access Review Default Settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Settings of an Access Review."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewScheduleSettings>,
}
impl AccessReviewDefaultSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review History Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryDefinition {
    #[doc = "The access review history definition id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review history definition unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Access Review History Instances."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewHistoryDefinitionProperties>,
}
impl AccessReviewHistoryDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Access Review History Instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryDefinitionInstanceListResult {
    #[doc = "Access Review History Definition's Instance list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessReviewHistoryInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewHistoryDefinitionInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessReviewHistoryDefinitionInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Access Review History Definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryDefinitionListResult {
    #[doc = "Access Review History Definition list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessReviewHistoryDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewHistoryDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessReviewHistoryDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review History Instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryDefinitionProperties {
    #[doc = "The display name for the history definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Date time used when selecting review data, all reviews included in data start on or after this date. For use only with one-time/non-recurring reports."]
    #[serde(rename = "reviewHistoryPeriodStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_history_period_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date time used when selecting review data, all reviews included in data end on or before this date. For use only with one-time/non-recurring reports."]
    #[serde(rename = "reviewHistoryPeriodEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_history_period_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Collection of review decisions which the history data should be filtered on. For example if Approve and Deny are supplied the data will only contain review results in which the decision maker approved or denied a review request."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub decisions: Vec<String>,
    #[doc = "This read-only field specifies the of the requested review history data. This is either requested, in-progress, done or error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_review_history_definition_properties::Status>,
    #[doc = "Date time when history definition was created"]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccessReviewActorIdentity>,
    #[doc = "A collection of scopes used when selecting review history data"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<AccessReviewScope>,
    #[doc = "Recurrence settings of an Access Review History Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccessReviewHistoryScheduleSettings>,
    #[doc = "Set of access review history instances for this history definition."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub instances: Vec<AccessReviewHistoryInstance>,
}
impl AccessReviewHistoryDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_history_definition_properties {
    use super::*;
    #[doc = "This read-only field specifies the of the requested review history data. This is either requested, in-progress, done or error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Requested,
        InProgress,
        Done,
        Error,
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
                Self::Requested => serializer.serialize_unit_variant("Status", 0u32, "Requested"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Done => serializer.serialize_unit_variant("Status", 2u32, "Done"),
                Self::Error => serializer.serialize_unit_variant("Status", 3u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Access Review History Definition Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryInstance {
    #[doc = "The access review history definition instance id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review history definition instance unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Access Review History Definition Instance properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewHistoryInstanceProperties>,
}
impl AccessReviewHistoryInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review History Definition Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryInstanceProperties {
    #[doc = "Date time used when selecting review data, all reviews included in data start on or after this date. For use only with one-time/non-recurring reports."]
    #[serde(rename = "reviewHistoryPeriodStartDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_history_period_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date time used when selecting review data, all reviews included in data end on or before this date. For use only with one-time/non-recurring reports."]
    #[serde(rename = "reviewHistoryPeriodEndDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_history_period_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The display name for the parent history definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Status of the requested review history instance data. This is either requested, in-progress, done or error. The state transitions are as follows - Requested -> InProgress -> Done -> Expired"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_review_history_instance_properties::Status>,
    #[doc = "Date time when the history data report is scheduled to be generated."]
    #[serde(rename = "runDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub run_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date time when the history data report is scheduled to be generated."]
    #[serde(rename = "fulfilledDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub fulfilled_date_time: Option<time::OffsetDateTime>,
    #[doc = "Uri which can be used to retrieve review history data. To generate this Uri, generateDownloadUri() must be called for a specific accessReviewHistoryDefinitionInstance. The link expires after a 24 hour period. Callers can see the expiration date time by looking at the 'se' parameter in the generated uri."]
    #[serde(rename = "downloadUri", default, skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    #[doc = "Date time when history data report expires and the associated data is deleted."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
}
impl AccessReviewHistoryInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_history_instance_properties {
    use super::*;
    #[doc = "Status of the requested review history instance data. This is either requested, in-progress, done or error. The state transitions are as follows - Requested -> InProgress -> Done -> Expired"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Requested,
        InProgress,
        Done,
        Error,
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
                Self::Requested => serializer.serialize_unit_variant("Status", 0u32, "Requested"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Done => serializer.serialize_unit_variant("Status", 2u32, "Done"),
                Self::Error => serializer.serialize_unit_variant("Status", 3u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recurrence settings of an Access Review History Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewHistoryScheduleSettings {
    #[doc = "Recurrence Pattern of an Access Review Schedule Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<AccessReviewRecurrencePattern>,
    #[doc = "Recurrence Range of an Access Review Schedule Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<AccessReviewRecurrenceRange>,
}
impl AccessReviewHistoryScheduleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewInstance {
    #[doc = "The access review instance id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Access Review Instance properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewInstanceProperties>,
}
impl AccessReviewInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Access Review Instances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewInstanceListResult {
    #[doc = "Access Review Instance list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessReviewInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessReviewInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewInstanceProperties {
    #[doc = "This read-only field specifies the status of an access review instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_review_instance_properties::Status>,
    #[doc = "The DateTime when the review instance is scheduled to be start."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The DateTime when the review instance is scheduled to end."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the collection of reviewers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This is the collection of backup reviewers."]
    #[serde(
        rename = "backupReviewers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This field specifies the type of reviewers for a review. Usually for a review, reviewers are explicitly assigned. However, in some cases, the reviewers may not be assigned and instead be chosen dynamically. For example managers review or self review."]
    #[serde(rename = "reviewersType", default, skip_serializing_if = "Option::is_none")]
    pub reviewers_type: Option<access_review_instance_properties::ReviewersType>,
}
impl AccessReviewInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_instance_properties {
    use super::*;
    #[doc = "This read-only field specifies the status of an access review instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        Applied,
        Initializing,
        Applying,
        Completing,
        Scheduled,
        AutoReviewing,
        AutoReviewed,
        Starting,
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
                Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::Applied => serializer.serialize_unit_variant("Status", 3u32, "Applied"),
                Self::Initializing => serializer.serialize_unit_variant("Status", 4u32, "Initializing"),
                Self::Applying => serializer.serialize_unit_variant("Status", 5u32, "Applying"),
                Self::Completing => serializer.serialize_unit_variant("Status", 6u32, "Completing"),
                Self::Scheduled => serializer.serialize_unit_variant("Status", 7u32, "Scheduled"),
                Self::AutoReviewing => serializer.serialize_unit_variant("Status", 8u32, "AutoReviewing"),
                Self::AutoReviewed => serializer.serialize_unit_variant("Status", 9u32, "AutoReviewed"),
                Self::Starting => serializer.serialize_unit_variant("Status", 10u32, "Starting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This field specifies the type of reviewers for a review. Usually for a review, reviewers are explicitly assigned. However, in some cases, the reviewers may not be assigned and instead be chosen dynamically. For example managers review or self review."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReviewersType")]
    pub enum ReviewersType {
        Assigned,
        #[serde(rename = "Self")]
        Self_,
        Managers,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReviewersType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReviewersType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReviewersType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Assigned => serializer.serialize_unit_variant("ReviewersType", 0u32, "Assigned"),
                Self::Self_ => serializer.serialize_unit_variant("ReviewersType", 1u32, "Self"),
                Self::Managers => serializer.serialize_unit_variant("ReviewersType", 2u32, "Managers"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recurrence Pattern of an Access Review Schedule Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewRecurrencePattern {
    #[doc = "The recurrence type : weekly, monthly, etc."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<access_review_recurrence_pattern::Type>,
    #[doc = "The interval for recurrence. For a quarterly review, the interval is 3 for type : absoluteMonthly."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
}
impl AccessReviewRecurrencePattern {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_recurrence_pattern {
    use super::*;
    #[doc = "The recurrence type : weekly, monthly, etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "absoluteMonthly")]
        AbsoluteMonthly,
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
                Self::Weekly => serializer.serialize_unit_variant("Type", 0u32, "weekly"),
                Self::AbsoluteMonthly => serializer.serialize_unit_variant("Type", 1u32, "absoluteMonthly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recurrence Range of an Access Review Schedule Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewRecurrenceRange {
    #[doc = "The recurrence range type. The possible values are: endDate, noEnd, numbered."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<access_review_recurrence_range::Type>,
    #[doc = "The number of times to repeat the access review. Required and must be positive if type is numbered."]
    #[serde(rename = "numberOfOccurrences", default, skip_serializing_if = "Option::is_none")]
    pub number_of_occurrences: Option<i32>,
    #[doc = "The DateTime when the review is scheduled to be start. This could be a date in the future. Required on create."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The DateTime when the review is scheduled to end. Required if type is endDate"]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
}
impl AccessReviewRecurrenceRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_recurrence_range {
    use super::*;
    #[doc = "The recurrence range type. The possible values are: endDate, noEnd, numbered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "noEnd")]
        NoEnd,
        #[serde(rename = "numbered")]
        Numbered,
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
                Self::EndDate => serializer.serialize_unit_variant("Type", 0u32, "endDate"),
                Self::NoEnd => serializer.serialize_unit_variant("Type", 1u32, "noEnd"),
                Self::Numbered => serializer.serialize_unit_variant("Type", 2u32, "numbered"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recurrence Settings of an Access Review Schedule Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewRecurrenceSettings {
    #[doc = "Recurrence Pattern of an Access Review Schedule Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<AccessReviewRecurrencePattern>,
    #[doc = "Recurrence Range of an Access Review Schedule Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<AccessReviewRecurrenceRange>,
}
impl AccessReviewRecurrenceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Descriptor for what needs to be reviewed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewReviewer {
    #[doc = "The id of the reviewer(user/servicePrincipal)"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The identity type : user/servicePrincipal"]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<access_review_reviewer::PrincipalType>,
}
impl AccessReviewReviewer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_reviewer {
    use super::*;
    #[doc = "The identity type : user/servicePrincipal"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "servicePrincipal")]
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "user"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 1u32, "servicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Access Review Schedule Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewScheduleDefinition {
    #[doc = "The access review schedule definition id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The access review schedule definition unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Access Review."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessReviewScheduleDefinitionProperties>,
}
impl AccessReviewScheduleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Access Review Schedule Definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewScheduleDefinitionListResult {
    #[doc = "Access Review Schedule Definition list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessReviewScheduleDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewScheduleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessReviewScheduleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Access Review."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewScheduleDefinitionProperties {
    #[doc = "The display name for the schedule definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "This read-only field specifies the status of an accessReview."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_review_schedule_definition_properties::Status>,
    #[doc = "The description provided by the access review creator and visible to admins."]
    #[serde(rename = "descriptionForAdmins", default, skip_serializing_if = "Option::is_none")]
    pub description_for_admins: Option<String>,
    #[doc = "The description provided by the access review creator to be shown to reviewers."]
    #[serde(rename = "descriptionForReviewers", default, skip_serializing_if = "Option::is_none")]
    pub description_for_reviewers: Option<String>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccessReviewActorIdentity>,
    #[doc = "Settings of an Access Review."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccessReviewScheduleSettings>,
    #[doc = "Descriptor for what needs to be reviewed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<AccessReviewScope>,
    #[doc = "This is the collection of reviewers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This is the collection of backup reviewers."]
    #[serde(
        rename = "backupReviewers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub backup_reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This field specifies the type of reviewers for a review. Usually for a review, reviewers are explicitly assigned. However, in some cases, the reviewers may not be assigned and instead be chosen dynamically. For example managers review or self review."]
    #[serde(rename = "reviewersType", default, skip_serializing_if = "Option::is_none")]
    pub reviewers_type: Option<access_review_schedule_definition_properties::ReviewersType>,
    #[doc = "This is the collection of instances returned when one does an expand on it."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub instances: Vec<AccessReviewInstance>,
}
impl AccessReviewScheduleDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_schedule_definition_properties {
    use super::*;
    #[doc = "This read-only field specifies the status of an accessReview."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        Applied,
        Initializing,
        Applying,
        Completing,
        Scheduled,
        AutoReviewing,
        AutoReviewed,
        Starting,
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
                Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::Applied => serializer.serialize_unit_variant("Status", 3u32, "Applied"),
                Self::Initializing => serializer.serialize_unit_variant("Status", 4u32, "Initializing"),
                Self::Applying => serializer.serialize_unit_variant("Status", 5u32, "Applying"),
                Self::Completing => serializer.serialize_unit_variant("Status", 6u32, "Completing"),
                Self::Scheduled => serializer.serialize_unit_variant("Status", 7u32, "Scheduled"),
                Self::AutoReviewing => serializer.serialize_unit_variant("Status", 8u32, "AutoReviewing"),
                Self::AutoReviewed => serializer.serialize_unit_variant("Status", 9u32, "AutoReviewed"),
                Self::Starting => serializer.serialize_unit_variant("Status", 10u32, "Starting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This field specifies the type of reviewers for a review. Usually for a review, reviewers are explicitly assigned. However, in some cases, the reviewers may not be assigned and instead be chosen dynamically. For example managers review or self review."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReviewersType")]
    pub enum ReviewersType {
        Assigned,
        #[serde(rename = "Self")]
        Self_,
        Managers,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReviewersType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReviewersType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReviewersType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Assigned => serializer.serialize_unit_variant("ReviewersType", 0u32, "Assigned"),
                Self::Self_ => serializer.serialize_unit_variant("ReviewersType", 1u32, "Self"),
                Self::Managers => serializer.serialize_unit_variant("ReviewersType", 2u32, "Managers"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Settings of an Access Review."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewScheduleSettings {
    #[doc = "Flag to indicate whether sending mails to reviewers and the review creator is enabled."]
    #[serde(rename = "mailNotificationsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub mail_notifications_enabled: Option<bool>,
    #[doc = "Flag to indicate whether sending reminder emails to reviewers are enabled."]
    #[serde(rename = "reminderNotificationsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub reminder_notifications_enabled: Option<bool>,
    #[doc = "Flag to indicate whether reviewers are required to provide a justification when reviewing access."]
    #[serde(rename = "defaultDecisionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub default_decision_enabled: Option<bool>,
    #[doc = "Flag to indicate whether the reviewer is required to pass justification when recording a decision."]
    #[serde(rename = "justificationRequiredOnApproval", default, skip_serializing_if = "Option::is_none")]
    pub justification_required_on_approval: Option<bool>,
    #[doc = "This specifies the behavior for the autoReview feature when an access review completes."]
    #[serde(rename = "defaultDecision", default, skip_serializing_if = "Option::is_none")]
    pub default_decision: Option<access_review_schedule_settings::DefaultDecision>,
    #[doc = "Flag to indicate whether auto-apply capability, to automatically change the target object access resource, is enabled. If not enabled, a user must, after the review completes, apply the access review."]
    #[serde(rename = "autoApplyDecisionsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_apply_decisions_enabled: Option<bool>,
    #[doc = "Flag to indicate whether showing recommendations to reviewers is enabled."]
    #[serde(rename = "recommendationsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub recommendations_enabled: Option<bool>,
    #[doc = "Recommendations for access reviews are calculated by looking back at 30 days of data(w.r.t the start date of the review) by default. However, in some scenarios, customers want to change how far back to look at and want to configure 60 days, 90 days, etc. instead. This setting allows customers to configure this duration. The value should be in ISO  8601 format (http://en.wikipedia.org/wiki/ISO_8601#Durations).This code can be used to convert TimeSpan to a valid interval string: XmlConvert.ToString(new TimeSpan(hours, minutes, seconds))"]
    #[serde(rename = "recommendationLookBackDuration", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_look_back_duration: Option<String>,
    #[doc = "The duration in days for an instance."]
    #[serde(rename = "instanceDurationInDays", default, skip_serializing_if = "Option::is_none")]
    pub instance_duration_in_days: Option<i32>,
    #[doc = "Recurrence Settings of an Access Review Schedule Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<AccessReviewRecurrenceSettings>,
}
impl AccessReviewScheduleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_schedule_settings {
    use super::*;
    #[doc = "This specifies the behavior for the autoReview feature when an access review completes."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultDecision")]
    pub enum DefaultDecision {
        Approve,
        Deny,
        Recommendation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultDecision {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultDecision {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultDecision {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Approve => serializer.serialize_unit_variant("DefaultDecision", 0u32, "Approve"),
                Self::Deny => serializer.serialize_unit_variant("DefaultDecision", 1u32, "Deny"),
                Self::Recommendation => serializer.serialize_unit_variant("DefaultDecision", 2u32, "Recommendation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Descriptor for what needs to be reviewed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewScope {
    #[doc = "ResourceId in which this review is getting created"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "This is used to indicate the role being reviewed"]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The identity type user/servicePrincipal to review"]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<access_review_scope::PrincipalType>,
    #[doc = "The role assignment state eligible/active to review"]
    #[serde(rename = "assignmentState", default, skip_serializing_if = "Option::is_none")]
    pub assignment_state: Option<access_review_scope::AssignmentState>,
    #[doc = "Duration users are inactive for. The value should be in ISO  8601 format (http://en.wikipedia.org/wiki/ISO_8601#Durations).This code can be used to convert TimeSpan to a valid interval string: XmlConvert.ToString(new TimeSpan(hours, minutes, seconds))"]
    #[serde(rename = "inactiveDuration", default, skip_serializing_if = "Option::is_none")]
    pub inactive_duration: Option<String>,
    #[doc = "Flag to indicate whether to expand nested memberships or not."]
    #[serde(rename = "expandNestedMemberships", default, skip_serializing_if = "Option::is_none")]
    pub expand_nested_memberships: Option<bool>,
    #[doc = "Flag to indicate whether to expand nested memberships or not."]
    #[serde(rename = "includeInheritedAccess", default, skip_serializing_if = "Option::is_none")]
    pub include_inherited_access: Option<bool>,
    #[doc = "Flag to indicate whether to expand nested memberships or not."]
    #[serde(rename = "includeAccessBelowResource", default, skip_serializing_if = "Option::is_none")]
    pub include_access_below_resource: Option<bool>,
    #[doc = "This is used to indicate the resource id(s) to exclude"]
    #[serde(rename = "excludeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub exclude_resource_id: Option<String>,
    #[doc = "This is used to indicate the role definition id(s) to exclude"]
    #[serde(rename = "excludeRoleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub exclude_role_definition_id: Option<String>,
}
impl AccessReviewScope {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_review_scope {
    use super::*;
    #[doc = "The identity type user/servicePrincipal to review"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "guestUser")]
        GuestUser,
        #[serde(rename = "servicePrincipal")]
        ServicePrincipal,
        #[serde(rename = "user,group")]
        UserGroup,
        #[serde(rename = "redeemedGuestUser")]
        RedeemedGuestUser,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "user"),
                Self::GuestUser => serializer.serialize_unit_variant("PrincipalType", 1u32, "guestUser"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "servicePrincipal"),
                Self::UserGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "user,group"),
                Self::RedeemedGuestUser => serializer.serialize_unit_variant("PrincipalType", 4u32, "redeemedGuestUser"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The role assignment state eligible/active to review"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssignmentState")]
    pub enum AssignmentState {
        #[serde(rename = "eligible")]
        Eligible,
        #[serde(rename = "active")]
        Active,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssignmentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssignmentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssignmentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Eligible => serializer.serialize_unit_variant("AssignmentState", 0u32, "eligible"),
                Self::Active => serializer.serialize_unit_variant("AssignmentState", 1u32, "active"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[doc = "The alert ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The alert name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The alert type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Alert properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertConfiguration {
    #[doc = "The alert configuration ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The alert configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The alert configuration type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Alert configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertConfigurationPropertiesUnion>,
}
impl AlertConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert configuration list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertConfigurationListResult {
    #[doc = "Alert configuration list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AlertConfiguration>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertConfigurationProperties {
    #[doc = "The alert definition ID."]
    #[serde(rename = "alertDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub alert_definition_id: Option<String>,
    #[doc = "The alert scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "True if the alert is enabled, false will disable the scanning for the specific alert."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Alert definition"]
    #[serde(rename = "alertDefinition", default, skip_serializing_if = "Option::is_none")]
    pub alert_definition: Option<AlertDefinition>,
}
impl AlertConfigurationProperties {
    pub fn new() -> Self {
        Self {
            alert_definition_id: None,
            scope: None,
            is_enabled: None,
            alert_definition: None,
        }
    }
}
#[doc = "The alert configuration type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "alertConfigurationType")]
pub enum AlertConfigurationPropertiesUnion {
    AzureRolesAssignedOutsidePimAlertConfiguration(AzureRolesAssignedOutsidePimAlertConfigurationProperties),
    DuplicateRoleCreatedAlertConfiguration(DuplicateRoleCreatedAlertConfigurationProperties),
    TooManyOwnersAssignedToResourceAlertConfiguration(TooManyOwnersAssignedToResourceAlertConfigurationProperties),
    TooManyPermanentOwnersAssignedToResourceAlertConfiguration(TooManyPermanentOwnersAssignedToResourceAlertConfigurationProperties),
}
#[doc = "Alert definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertDefinition {
    #[doc = "The alert definition ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The alert definition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The alert definition type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Alert definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertDefinitionProperties>,
}
impl AlertDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert definition list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertDefinitionListResult {
    #[doc = "Alert definition list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AlertDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertDefinitionProperties {
    #[doc = "The alert display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The alert scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The alert description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Severity level of the alert."]
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<alert_definition_properties::SeverityLevel>,
    #[doc = "Security impact of the alert."]
    #[serde(rename = "securityImpact", default, skip_serializing_if = "Option::is_none")]
    pub security_impact: Option<String>,
    #[doc = "The methods to mitigate the alert."]
    #[serde(rename = "mitigationSteps", default, skip_serializing_if = "Option::is_none")]
    pub mitigation_steps: Option<String>,
    #[doc = "The ways to prevent the alert."]
    #[serde(rename = "howToPrevent", default, skip_serializing_if = "Option::is_none")]
    pub how_to_prevent: Option<String>,
    #[doc = "True if the alert can be remediated; false, otherwise."]
    #[serde(rename = "isRemediatable", default, skip_serializing_if = "Option::is_none")]
    pub is_remediatable: Option<bool>,
    #[doc = "True if the alert configuration can be configured; false, otherwise."]
    #[serde(rename = "isConfigurable", default, skip_serializing_if = "Option::is_none")]
    pub is_configurable: Option<bool>,
}
impl AlertDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_definition_properties {
    use super::*;
    #[doc = "Severity level of the alert."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SeverityLevel")]
    pub enum SeverityLevel {
        Low,
        Medium,
        High,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SeverityLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SeverityLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SeverityLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Low => serializer.serialize_unit_variant("SeverityLevel", 0u32, "Low"),
                Self::Medium => serializer.serialize_unit_variant("SeverityLevel", 1u32, "Medium"),
                Self::High => serializer.serialize_unit_variant("SeverityLevel", 2u32, "High"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Alert incident"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertIncident {
    #[doc = "The alert incident ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The alert incident name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The alert incident type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Alert incident properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertIncidentPropertiesUnion>,
}
impl AlertIncident {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert incident list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertIncidentListResult {
    #[doc = "Alert incident list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AlertIncident>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertIncidentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertIncidentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The alert incident type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "alertIncidentType")]
pub enum AlertIncidentPropertiesUnion {
    AzureRolesAssignedOutsidePimAlertIncident(AzureRolesAssignedOutsidePimAlertIncidentProperties),
    DuplicateRoleCreatedAlertIncident(DuplicateRoleCreatedAlertIncidentProperties),
    TooManyOwnersAssignedToResourceAlertIncident(TooManyOwnersAssignedToResourceAlertIncidentProperties),
    TooManyPermanentOwnersAssignedToResourceAlertIncident(TooManyPermanentOwnersAssignedToResourceAlertIncidentProperties),
}
#[doc = "Alert list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertListResult {
    #[doc = "Alert list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Alert>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AlertListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert operation result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertOperationResult {
    #[doc = "The id of the alert operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The status of the alert operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The status detail of the alert operation."]
    #[serde(rename = "statusDetail", default, skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    #[doc = "The created date of the alert operation."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The last action date of the alert operation."]
    #[serde(rename = "lastActionDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "The location of the alert associated with the operation."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
}
impl AlertOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "The alert scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "False by default; true if the alert is active."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The number of generated incidents of the alert."]
    #[serde(rename = "incidentCount", default, skip_serializing_if = "Option::is_none")]
    pub incident_count: Option<i32>,
    #[doc = "The date time when the alert configuration was updated or new incidents were generated."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The date time when the alert was last scanned."]
    #[serde(rename = "lastScannedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_scanned_date_time: Option<time::OffsetDateTime>,
    #[doc = "Alert definition"]
    #[serde(rename = "alertDefinition", default, skip_serializing_if = "Option::is_none")]
    pub alert_definition: Option<AlertDefinition>,
    #[doc = "The alert incidents."]
    #[serde(
        rename = "alertIncidents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub alert_incidents: Vec<AlertIncident>,
    #[doc = "Alert configuration."]
    #[serde(rename = "alertConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub alert_configuration: Option<AlertConfiguration>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The approval settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalSettings {
    #[doc = "Determines whether approval is required or not."]
    #[serde(rename = "isApprovalRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_approval_required: Option<bool>,
    #[doc = "Determines whether approval is required for assignment extension."]
    #[serde(rename = "isApprovalRequiredForExtension", default, skip_serializing_if = "Option::is_none")]
    pub is_approval_required_for_extension: Option<bool>,
    #[doc = "Determine whether requestor justification is required."]
    #[serde(rename = "isRequestorJustificationRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_requestor_justification_required: Option<bool>,
    #[doc = "The type of rule"]
    #[serde(rename = "approvalMode", default, skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<approval_settings::ApprovalMode>,
    #[doc = "The approval stages of the request."]
    #[serde(
        rename = "approvalStages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub approval_stages: Vec<ApprovalStage>,
}
impl ApprovalSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval_settings {
    use super::*;
    #[doc = "The type of rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApprovalMode")]
    pub enum ApprovalMode {
        SingleStage,
        Serial,
        Parallel,
        NoApproval,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApprovalMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApprovalMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApprovalMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SingleStage => serializer.serialize_unit_variant("ApprovalMode", 0u32, "SingleStage"),
                Self::Serial => serializer.serialize_unit_variant("ApprovalMode", 1u32, "Serial"),
                Self::Parallel => serializer.serialize_unit_variant("ApprovalMode", 2u32, "Parallel"),
                Self::NoApproval => serializer.serialize_unit_variant("ApprovalMode", 3u32, "NoApproval"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The approval stage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalStage {
    #[doc = "The time in days when approval request would be timed out"]
    #[serde(rename = "approvalStageTimeOutInDays", default, skip_serializing_if = "Option::is_none")]
    pub approval_stage_time_out_in_days: Option<i32>,
    #[doc = "Determines whether approver need to provide justification for his decision."]
    #[serde(rename = "isApproverJustificationRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_approver_justification_required: Option<bool>,
    #[doc = "The time in minutes when the approval request would be escalated if the primary approver does not approve"]
    #[serde(rename = "escalationTimeInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub escalation_time_in_minutes: Option<i32>,
    #[doc = "The primary approver of the request."]
    #[serde(
        rename = "primaryApprovers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub primary_approvers: Vec<UserSet>,
    #[doc = "The value determine whether escalation feature is enabled."]
    #[serde(rename = "isEscalationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_escalation_enabled: Option<bool>,
    #[doc = "The escalation approver of the request."]
    #[serde(
        rename = "escalationApprovers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub escalation_approvers: Vec<UserSet>,
}
impl ApprovalStage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure roles assigned outside PIM alert configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureRolesAssignedOutsidePimAlertConfigurationProperties {
    #[serde(flatten)]
    pub alert_configuration_properties: AlertConfigurationProperties,
}
impl AzureRolesAssignedOutsidePimAlertConfigurationProperties {
    pub fn new(alert_configuration_properties: AlertConfigurationProperties) -> Self {
        Self {
            alert_configuration_properties,
        }
    }
}
#[doc = "Azure roles assigned outside PIM alert incident properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureRolesAssignedOutsidePimAlertIncidentProperties {
    #[doc = "The assignee display name."]
    #[serde(rename = "assigneeDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub assignee_display_name: Option<String>,
    #[doc = "The assignee user principal name."]
    #[serde(rename = "assigneeUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub assignee_user_principal_name: Option<String>,
    #[doc = "The assignee ID."]
    #[serde(rename = "assigneeId", default, skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[doc = "The role display name."]
    #[serde(rename = "roleDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub role_display_name: Option<String>,
    #[doc = "The role template ID."]
    #[serde(rename = "roleTemplateId", default, skip_serializing_if = "Option::is_none")]
    pub role_template_id: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The date the assignment was activated."]
    #[serde(rename = "assignmentActivatedDate", default, with = "azure_core::date::rfc3339::option")]
    pub assignment_activated_date: Option<time::OffsetDateTime>,
    #[doc = "The requestor ID."]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "The requestor display name."]
    #[serde(rename = "requestorDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub requestor_display_name: Option<String>,
    #[doc = "The requestor user principal name."]
    #[serde(rename = "requestorUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub requestor_user_principal_name: Option<String>,
}
impl AzureRolesAssignedOutsidePimAlertIncidentProperties {
    pub fn new() -> Self {
        Self {
            assignee_display_name: None,
            assignee_user_principal_name: None,
            assignee_id: None,
            role_display_name: None,
            role_template_id: None,
            role_definition_id: None,
            assignment_activated_date: None,
            requestor_id: None,
            requestor_display_name: None,
            requestor_user_principal_name: None,
        }
    }
}
#[doc = "Classic Administrators"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministrator {
    #[doc = "The ID of the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the administrator."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Classic Administrator properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClassicAdministratorProperties>,
}
impl ClassicAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ClassicAdministrator list result information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministratorListResult {
    #[doc = "An array of administrators."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ClassicAdministrator>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClassicAdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClassicAdministratorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Classic Administrator properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministratorProperties {
    #[doc = "The email address of the administrator."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "The role of the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl ClassicAdministratorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
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
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny Assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignment {
    #[doc = "The deny assignment ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The deny assignment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The deny assignment type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Deny assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DenyAssignmentProperties>,
}
impl DenyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny Assignments filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentFilter {
    #[doc = "Return deny assignment with specified name."]
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[doc = "Return all deny assignments where the specified principal is listed in the principals list of deny assignments."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Return all deny assignments where the specified principal is listed either in the principals list or exclude principals list of deny assignments."]
    #[serde(rename = "gdprExportPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub gdpr_export_principal_id: Option<String>,
}
impl DenyAssignmentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny assignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentListResult {
    #[doc = "Deny assignment list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DenyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DenyAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DenyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny assignment permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentPermission {
    #[doc = "Actions to which the deny assignment does not grant access."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
    #[doc = "Actions to exclude from that the deny assignment does not grant access."]
    #[serde(
        rename = "notActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_actions: Vec<String>,
    #[doc = "Data actions to which the deny assignment does not grant access."]
    #[serde(
        rename = "dataActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_actions: Vec<String>,
    #[doc = "Data actions to exclude from that the deny assignment does not grant access."]
    #[serde(
        rename = "notDataActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_data_actions: Vec<String>,
    #[doc = "The conditions on the Deny assignment permission. This limits the resources it applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition."]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
}
impl DenyAssignmentPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentProperties {
    #[doc = "The display name of the deny assignment."]
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[doc = "The description of the deny assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An array of permissions that are denied by the deny assignment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permissions: Vec<DenyAssignmentPermission>,
    #[doc = "The deny assignment scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Determines if the deny assignment applies to child scopes. Default value is false."]
    #[serde(rename = "doNotApplyToChildScopes", default, skip_serializing_if = "Option::is_none")]
    pub do_not_apply_to_child_scopes: Option<bool>,
    #[doc = "Array of principals to which the deny assignment applies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub principals: Vec<Principal>,
    #[doc = "Array of principals to which the deny assignment does not apply."]
    #[serde(
        rename = "excludePrincipals",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exclude_principals: Vec<Principal>,
    #[doc = "Specifies whether this deny assignment was created by Azure and cannot be edited or deleted."]
    #[serde(rename = "isSystemProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_system_protected: Option<bool>,
    #[doc = "The conditions on the deny assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition."]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "Time it was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time it was updated"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created the assignment"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Id of the user who updated the assignment"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}
impl DenyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The duplicate role created alert configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DuplicateRoleCreatedAlertConfigurationProperties {
    #[serde(flatten)]
    pub alert_configuration_properties: AlertConfigurationProperties,
}
impl DuplicateRoleCreatedAlertConfigurationProperties {
    pub fn new(alert_configuration_properties: AlertConfigurationProperties) -> Self {
        Self {
            alert_configuration_properties,
        }
    }
}
#[doc = "Duplicate role created alert incident properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DuplicateRoleCreatedAlertIncidentProperties {
    #[doc = "The role name."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "The duplicate roles."]
    #[serde(rename = "duplicateRoles", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_roles: Option<String>,
    #[doc = "The reason for the incident."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl DuplicateRoleCreatedAlertIncidentProperties {
    pub fn new() -> Self {
        Self {
            role_name: None,
            duplicate_roles: None,
            reason: None,
        }
    }
}
#[doc = "Eligible child resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EligibleChildResource {
    #[doc = "The resource scope Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl EligibleChildResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Eligible child resources list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EligibleChildResourcesListResult {
    #[doc = "Eligible child resource list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EligibleChildResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EligibleChildResourcesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EligibleChildResourcesListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Error description and code explaining why an operation failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Error description and code explaining why an operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinitionProperties>,
}
impl azure_core::Continuable for ErrorDefinition {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error description and code explaining why an operation failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinitionProperties {
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error code of list gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl ErrorDefinitionProperties {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandedProperties {
    #[doc = "Details of the resource scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<expanded_properties::Scope>,
    #[doc = "Details of role definition"]
    #[serde(rename = "roleDefinition", default, skip_serializing_if = "Option::is_none")]
    pub role_definition: Option<expanded_properties::RoleDefinition>,
    #[doc = "Details of the principal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<expanded_properties::Principal>,
}
impl ExpandedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod expanded_properties {
    use super::*;
    #[doc = "Details of the resource scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Scope {
        #[doc = "Scope id of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the resource"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the resource"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Scope {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of role definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RoleDefinition {
        #[doc = "Id of the role definition"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the role definition"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the role definition"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl RoleDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of the principal"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Principal {
        #[doc = "Id of the principal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the principal"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Email id of the principal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Type of the principal"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Principal {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The definition of a Microsoft.Authorization operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The display information for a Microsoft.Authorization operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The display information for a Microsoft.Authorization operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The resource provider name: Microsoft.Authorization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation that users can perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a request to list Microsoft.Authorization operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The collection value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role definition permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permission {
    #[doc = "Allowed actions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
    #[doc = "Denied actions."]
    #[serde(
        rename = "notActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_actions: Vec<String>,
    #[doc = "Allowed Data actions."]
    #[serde(
        rename = "dataActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_actions: Vec<String>,
    #[doc = "Denied Data actions."]
    #[serde(
        rename = "notDataActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_data_actions: Vec<String>,
    #[doc = "The conditions on the role definition. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently the only accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
}
impl Permission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permissions information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionGetResult {
    #[doc = "An array of permissions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Permission>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PermissionGetResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PermissionGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentProperties {
    #[doc = "Details of the resource scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<policy_assignment_properties::Scope>,
    #[doc = "Details of role definition"]
    #[serde(rename = "roleDefinition", default, skip_serializing_if = "Option::is_none")]
    pub role_definition: Option<policy_assignment_properties::RoleDefinition>,
    #[doc = "Details of the policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<policy_assignment_properties::Policy>,
}
impl PolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_assignment_properties {
    use super::*;
    #[doc = "Details of the resource scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Scope {
        #[doc = "Scope id of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the resource"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the resource"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Scope {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of role definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RoleDefinition {
        #[doc = "Id of the role definition"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the role definition"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the role definition"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl RoleDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of the policy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Policy {
        #[doc = "Id of the policy"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "The name of the entity last modified it"]
        #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
        pub last_modified_by: Option<Principal>,
        #[doc = "The last modified date time."]
        #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_modified_date_time: Option<time::OffsetDateTime>,
    }
    impl Policy {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyProperties {
    #[doc = "Details of the resource scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<policy_properties::Scope>,
}
impl PolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_properties {
    use super::*;
    #[doc = "Details of the resource scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Scope {
        #[doc = "Scope id of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the resource"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the resource"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Scope {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The name of the entity last modified it"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Principal {
    #[doc = "The id of the principal made changes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the principal made changes"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Type of principal such as user , group etc"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Email of principal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl Principal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperation {
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The operation origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The dataAction flag to specify the operation type."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl ProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider Operations metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationsMetadata {
    #[doc = "The provider id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The provider type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The provider display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The provider resource types"]
    #[serde(
        rename = "resourceTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_types: Vec<ResourceType>,
    #[doc = "The provider operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<ProviderOperation>,
}
impl ProviderOperationsMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider operations metadata list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationsMetadataListResult {
    #[doc = "The list of providers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProviderOperationsMetadata>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderOperationsMetadataListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProviderOperationsMetadataListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Record All Decisions payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordAllDecisionsProperties {
    #[doc = "The id of principal which needs to be approved/denied."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The id of resource which needs to be approved/denied."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The decision to make. Approvers can take action of Approve/Deny"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<record_all_decisions_properties::Decision>,
    #[doc = "Justification provided by approvers for their action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
}
impl RecordAllDecisionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod record_all_decisions_properties {
    use super::*;
    #[doc = "The decision to make. Approvers can take action of Approve/Deny"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Decision")]
    pub enum Decision {
        Approve,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Decision {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Decision {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Decision {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Approve => serializer.serialize_unit_variant("Decision", 0u32, "Approve"),
                Self::Deny => serializer.serialize_unit_variant("Decision", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceType {
    #[doc = "The resource type name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The resource type operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<ProviderOperation>,
}
impl ResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role Assignments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignment {
    #[doc = "The role assignment ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentProperties>,
}
impl RoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentCreateParameters {
    #[doc = "Role assignment properties."]
    pub properties: RoleAssignmentProperties,
}
impl RoleAssignmentCreateParameters {
    pub fn new(properties: RoleAssignmentProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Role Assignments filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentFilter {
    #[doc = "Returns role assignment of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl RoleAssignmentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentListResult {
    #[doc = "Role assignment list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleAssignment>,
    #[doc = "The skipToken to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentProperties {
    #[doc = "The role assignment scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_properties::PrincipalType>,
    #[doc = "Description of role assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently the only accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "Time it was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time it was updated"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created the assignment"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Id of the user who updated the assignment"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "Id of the delegated managed identity resource"]
    #[serde(rename = "delegatedManagedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_managed_identity_resource_id: Option<String>,
}
impl RoleAssignmentProperties {
    pub fn new(role_definition_id: String, principal_id: String) -> Self {
        Self {
            scope: None,
            role_definition_id,
            principal_id,
            principal_type: None,
            description: None,
            condition: None,
            condition_version: None,
            created_on: None,
            updated_on: None,
            created_by: None,
            updated_by: None,
            delegated_managed_identity_resource_id: None,
        }
    }
}
pub mod role_assignment_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PrincipalType {
        fn default() -> Self {
            Self::User
        }
    }
}
#[doc = "Role Assignment schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentSchedule {
    #[doc = "The role assignment schedule Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment schedule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment schedule type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentScheduleProperties>,
}
impl RoleAssignmentSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleFilter {
    #[doc = "Returns role assignment schedule of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role assignment schedule of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role assignment schedule instances of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleAssignmentScheduleFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about current or upcoming role assignment schedule instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstance {
    #[doc = "The role assignment schedule instance ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment schedule instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment schedule instance type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentScheduleInstanceProperties>,
}
impl RoleAssignmentScheduleInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule instance filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstanceFilter {
    #[doc = "Returns role assignment schedule instances of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role assignment schedule instances of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role assignment schedule instances of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Returns role assignment schedule instances belonging to a specific role assignment schedule."]
    #[serde(rename = "roleAssignmentScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_schedule_id: Option<String>,
}
impl RoleAssignmentScheduleInstanceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule instance list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstanceListResult {
    #[doc = "Role assignment schedule instance list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleAssignmentScheduleInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentScheduleInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleAssignmentScheduleInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstanceProperties {
    #[doc = "The role assignment schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_schedule_instance_properties::PrincipalType>,
    #[doc = "Id of the master role assignment schedule"]
    #[serde(rename = "roleAssignmentScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_schedule_id: Option<String>,
    #[doc = "Role Assignment Id in external system"]
    #[serde(rename = "originRoleAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub origin_role_assignment_id: Option<String>,
    #[doc = "The status of the role assignment schedule instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_assignment_schedule_instance_properties::Status>,
    #[doc = "The startDateTime of the role assignment schedule instance"]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The endDateTime of the role assignment schedule instance"]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "roleEligibilityScheduleId used to activate"]
    #[serde(rename = "linkedRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub linked_role_eligibility_schedule_id: Option<String>,
    #[doc = "roleEligibilityScheduleInstanceId linked to this roleAssignmentScheduleInstance"]
    #[serde(
        rename = "linkedRoleEligibilityScheduleInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linked_role_eligibility_schedule_instance_id: Option<String>,
    #[doc = "Assignment type of the role assignment schedule"]
    #[serde(rename = "assignmentType", default, skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<role_assignment_schedule_instance_properties::AssignmentType>,
    #[doc = "Membership type of the role assignment schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_assignment_schedule_instance_properties::MemberType>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role assignment schedule was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleAssignmentScheduleInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_assignment_schedule_instance_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role assignment schedule instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Assignment type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssignmentType")]
    pub enum AssignmentType {
        Activated,
        Assigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Activated => serializer.serialize_unit_variant("AssignmentType", 0u32, "Activated"),
                Self::Assigned => serializer.serialize_unit_variant("AssignmentType", 1u32, "Assigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role assignment schedule list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleListResult {
    #[doc = "Role assignment schedule list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleAssignmentSchedule>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleAssignmentScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleProperties {
    #[doc = "The role assignment schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_schedule_properties::PrincipalType>,
    #[doc = "The id of roleAssignmentScheduleRequest used to create this roleAssignmentSchedule"]
    #[serde(rename = "roleAssignmentScheduleRequestId", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_schedule_request_id: Option<String>,
    #[doc = "The id of roleEligibilitySchedule used to activated this roleAssignmentSchedule"]
    #[serde(rename = "linkedRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub linked_role_eligibility_schedule_id: Option<String>,
    #[doc = "Assignment type of the role assignment schedule"]
    #[serde(rename = "assignmentType", default, skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<role_assignment_schedule_properties::AssignmentType>,
    #[doc = "Membership type of the role assignment schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_assignment_schedule_properties::MemberType>,
    #[doc = "The status of the role assignment schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_assignment_schedule_properties::Status>,
    #[doc = "Start DateTime when role assignment schedule"]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End DateTime when role assignment schedule"]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role assignment schedule was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "DateTime when role assignment schedule was modified"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleAssignmentScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_assignment_schedule_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Assignment type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssignmentType")]
    pub enum AssignmentType {
        Activated,
        Assigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Activated => serializer.serialize_unit_variant("AssignmentType", 0u32, "Activated"),
                Self::Assigned => serializer.serialize_unit_variant("AssignmentType", 1u32, "Assigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role assignment schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role Assignment schedule request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleRequest {
    #[doc = "The role assignment schedule request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment schedule request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment schedule request type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment schedule request properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentScheduleRequestProperties>,
}
impl RoleAssignmentScheduleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule request filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleRequestFilter {
    #[doc = "Returns role assignment requests of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role assignment requests of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role assignment requests created by specific principal."]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Returns role assignment requests of specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleAssignmentScheduleRequestFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule request list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleRequestListResult {
    #[doc = "Role assignment schedule request list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleAssignmentScheduleRequest>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentScheduleRequestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleAssignmentScheduleRequestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule request properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentScheduleRequestProperties {
    #[doc = "The role assignment schedule request scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_schedule_request_properties::PrincipalType>,
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[serde(rename = "requestType")]
    pub request_type: role_assignment_schedule_request_properties::RequestType,
    #[doc = "The status of the role assignment schedule request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_assignment_schedule_request_properties::Status>,
    #[doc = "The approvalId of the role assignment schedule request."]
    #[serde(rename = "approvalId", default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[doc = "The resultant role assignment schedule id or the role assignment schedule id being updated"]
    #[serde(rename = "targetRoleAssignmentScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub target_role_assignment_schedule_id: Option<String>,
    #[doc = "The role assignment schedule instance id being updated"]
    #[serde(
        rename = "targetRoleAssignmentScheduleInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_role_assignment_schedule_instance_id: Option<String>,
    #[doc = "Schedule info of the role assignment schedule"]
    #[serde(rename = "scheduleInfo", default, skip_serializing_if = "Option::is_none")]
    pub schedule_info: Option<role_assignment_schedule_request_properties::ScheduleInfo>,
    #[doc = "The linked role eligibility schedule id - to activate an eligibility."]
    #[serde(rename = "linkedRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub linked_role_eligibility_schedule_id: Option<String>,
    #[doc = "Justification for the role assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Ticket Info of the role assignment"]
    #[serde(rename = "ticketInfo", default, skip_serializing_if = "Option::is_none")]
    pub ticket_info: Option<role_assignment_schedule_request_properties::TicketInfo>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role assignment schedule request was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created this request"]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Expanded info of resource, role and principal"]
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleAssignmentScheduleRequestProperties {
    pub fn new(
        role_definition_id: String,
        principal_id: String,
        request_type: role_assignment_schedule_request_properties::RequestType,
    ) -> Self {
        Self {
            scope: None,
            role_definition_id,
            principal_id,
            principal_type: None,
            request_type,
            status: None,
            approval_id: None,
            target_role_assignment_schedule_id: None,
            target_role_assignment_schedule_instance_id: None,
            schedule_info: None,
            linked_role_eligibility_schedule_id: None,
            justification: None,
            ticket_info: None,
            condition: None,
            condition_version: None,
            created_on: None,
            requestor_id: None,
            expanded_properties: None,
        }
    }
}
pub mod role_assignment_schedule_request_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestType")]
    pub enum RequestType {
        AdminAssign,
        AdminRemove,
        AdminUpdate,
        AdminExtend,
        AdminRenew,
        SelfActivate,
        SelfDeactivate,
        SelfExtend,
        SelfRenew,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AdminAssign => serializer.serialize_unit_variant("RequestType", 0u32, "AdminAssign"),
                Self::AdminRemove => serializer.serialize_unit_variant("RequestType", 1u32, "AdminRemove"),
                Self::AdminUpdate => serializer.serialize_unit_variant("RequestType", 2u32, "AdminUpdate"),
                Self::AdminExtend => serializer.serialize_unit_variant("RequestType", 3u32, "AdminExtend"),
                Self::AdminRenew => serializer.serialize_unit_variant("RequestType", 4u32, "AdminRenew"),
                Self::SelfActivate => serializer.serialize_unit_variant("RequestType", 5u32, "SelfActivate"),
                Self::SelfDeactivate => serializer.serialize_unit_variant("RequestType", 6u32, "SelfDeactivate"),
                Self::SelfExtend => serializer.serialize_unit_variant("RequestType", 7u32, "SelfExtend"),
                Self::SelfRenew => serializer.serialize_unit_variant("RequestType", 8u32, "SelfRenew"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role assignment schedule request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Schedule info of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ScheduleInfo {
        #[doc = "Start DateTime of the role assignment schedule."]
        #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_date_time: Option<time::OffsetDateTime>,
        #[doc = "Expiration of the role assignment schedule"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<schedule_info::Expiration>,
    }
    impl ScheduleInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod schedule_info {
        use super::*;
        #[doc = "Expiration of the role assignment schedule"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Expiration {
            #[doc = "Type of the role assignment schedule expiration"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<expiration::Type>,
            #[doc = "End DateTime of the role assignment schedule."]
            #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
            pub end_date_time: Option<time::OffsetDateTime>,
            #[doc = "Duration of the role assignment schedule in TimeSpan."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub duration: Option<String>,
        }
        impl Expiration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod expiration {
            use super::*;
            #[doc = "Type of the role assignment schedule expiration"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                AfterDuration,
                AfterDateTime,
                NoExpiration,
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
                        Self::AfterDuration => serializer.serialize_unit_variant("Type", 0u32, "AfterDuration"),
                        Self::AfterDateTime => serializer.serialize_unit_variant("Type", 1u32, "AfterDateTime"),
                        Self::NoExpiration => serializer.serialize_unit_variant("Type", 2u32, "NoExpiration"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "Ticket Info of the role assignment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TicketInfo {
        #[doc = "Ticket number for the role assignment"]
        #[serde(rename = "ticketNumber", default, skip_serializing_if = "Option::is_none")]
        pub ticket_number: Option<String>,
        #[doc = "Ticket system name for the role assignment"]
        #[serde(rename = "ticketSystem", default, skip_serializing_if = "Option::is_none")]
        pub ticket_system: Option<String>,
    }
    impl TicketInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinition {
    #[doc = "The role definition ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role definition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role definition type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleDefinitionProperties>,
}
impl RoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role Definitions filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionFilter {
    #[doc = "Returns role definition with the specific name."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Returns role definition with the specific type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RoleDefinitionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role definition list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionListResult {
    #[doc = "Role definition list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionProperties {
    #[doc = "The role name."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "The role definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The role type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role definition permissions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permissions: Vec<Permission>,
    #[doc = "Role definition assignable scopes."]
    #[serde(
        rename = "assignableScopes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assignable_scopes: Vec<String>,
    #[doc = "Time it was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time it was updated"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created the assignment"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Id of the user who updated the assignment"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}
impl RoleDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilitySchedule {
    #[doc = "The role eligibility schedule Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role eligibility schedule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role eligibility schedule type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role eligibility schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleEligibilityScheduleProperties>,
}
impl RoleEligibilitySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleFilter {
    #[doc = "Returns role eligibility schedule of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role eligibility schedule of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role eligibility schedule of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleEligibilityScheduleFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about current or upcoming role eligibility schedule instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstance {
    #[doc = "The role eligibility schedule instance ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role eligibility schedule instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role eligibility schedule instance type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role eligibility schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleEligibilityScheduleInstanceProperties>,
}
impl RoleEligibilityScheduleInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule instance filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstanceFilter {
    #[doc = "Returns role eligibility schedule instances of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role eligibility schedule instances of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role eligibility schedule instances of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Returns role eligibility schedule instances belonging to a specific role eligibility schedule."]
    #[serde(rename = "roleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_eligibility_schedule_id: Option<String>,
}
impl RoleEligibilityScheduleInstanceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule instance list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstanceListResult {
    #[doc = "Role eligibility schedule instance list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleEligibilityScheduleInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleEligibilityScheduleInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleEligibilityScheduleInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstanceProperties {
    #[doc = "The role eligibility schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_eligibility_schedule_instance_properties::PrincipalType>,
    #[doc = "Id of the master role eligibility schedule"]
    #[serde(rename = "roleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_eligibility_schedule_id: Option<String>,
    #[doc = "The status of the role eligibility schedule instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_eligibility_schedule_instance_properties::Status>,
    #[doc = "The startDateTime of the role eligibility schedule instance"]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The endDateTime of the role eligibility schedule instance"]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Membership type of the role eligibility schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_eligibility_schedule_instance_properties::MemberType>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role eligibility schedule was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleEligibilityScheduleInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_eligibility_schedule_instance_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role eligibility schedule instance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role eligibility schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "role eligibility schedule list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleListResult {
    #[doc = "role eligibility schedule list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleEligibilitySchedule>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleEligibilityScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleEligibilityScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleProperties {
    #[doc = "The role eligibility schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_eligibility_schedule_properties::PrincipalType>,
    #[doc = "The id of roleEligibilityScheduleRequest used to create this roleAssignmentSchedule"]
    #[serde(rename = "roleEligibilityScheduleRequestId", default, skip_serializing_if = "Option::is_none")]
    pub role_eligibility_schedule_request_id: Option<String>,
    #[doc = "Membership type of the role eligibility schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_eligibility_schedule_properties::MemberType>,
    #[doc = "The status of the role eligibility schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_eligibility_schedule_properties::Status>,
    #[doc = "Start DateTime when role eligibility schedule"]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End DateTime when role eligibility schedule"]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role eligibility schedule was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "DateTime when role eligibility schedule was modified"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleEligibilityScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_eligibility_schedule_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role eligibility schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role eligibility schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role Eligibility schedule request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleRequest {
    #[doc = "The role eligibility schedule request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role eligibility schedule request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role eligibility schedule request type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role eligibility schedule request properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleEligibilityScheduleRequestProperties>,
}
impl RoleEligibilityScheduleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule request filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleRequestFilter {
    #[doc = "Returns role eligibility requests of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role eligibility requests of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role eligibility requests created by specific principal."]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Returns role eligibility requests of specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleEligibilityScheduleRequestFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule request list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleRequestListResult {
    #[doc = "Role eligibility schedule request list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleEligibilityScheduleRequest>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleEligibilityScheduleRequestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleEligibilityScheduleRequestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule request properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleEligibilityScheduleRequestProperties {
    #[doc = "The role eligibility schedule request scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_eligibility_schedule_request_properties::PrincipalType>,
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[serde(rename = "requestType")]
    pub request_type: role_eligibility_schedule_request_properties::RequestType,
    #[doc = "The status of the role eligibility schedule request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_eligibility_schedule_request_properties::Status>,
    #[doc = "The approvalId of the role eligibility schedule request."]
    #[serde(rename = "approvalId", default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[doc = "Schedule info of the role eligibility schedule"]
    #[serde(rename = "scheduleInfo", default, skip_serializing_if = "Option::is_none")]
    pub schedule_info: Option<role_eligibility_schedule_request_properties::ScheduleInfo>,
    #[doc = "The resultant role eligibility schedule id or the role eligibility schedule id being updated"]
    #[serde(rename = "targetRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub target_role_eligibility_schedule_id: Option<String>,
    #[doc = "The role eligibility schedule instance id being updated"]
    #[serde(
        rename = "targetRoleEligibilityScheduleInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_role_eligibility_schedule_instance_id: Option<String>,
    #[doc = "Justification for the role eligibility"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Ticket Info of the role eligibility"]
    #[serde(rename = "ticketInfo", default, skip_serializing_if = "Option::is_none")]
    pub ticket_info: Option<role_eligibility_schedule_request_properties::TicketInfo>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role eligibility schedule request was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created this request"]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Expanded info of resource, role and principal"]
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleEligibilityScheduleRequestProperties {
    pub fn new(
        role_definition_id: String,
        principal_id: String,
        request_type: role_eligibility_schedule_request_properties::RequestType,
    ) -> Self {
        Self {
            scope: None,
            role_definition_id,
            principal_id,
            principal_type: None,
            request_type,
            status: None,
            approval_id: None,
            schedule_info: None,
            target_role_eligibility_schedule_id: None,
            target_role_eligibility_schedule_instance_id: None,
            justification: None,
            ticket_info: None,
            condition: None,
            condition_version: None,
            created_on: None,
            requestor_id: None,
            expanded_properties: None,
        }
    }
}
pub mod role_eligibility_schedule_request_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestType")]
    pub enum RequestType {
        AdminAssign,
        AdminRemove,
        AdminUpdate,
        AdminExtend,
        AdminRenew,
        SelfActivate,
        SelfDeactivate,
        SelfExtend,
        SelfRenew,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AdminAssign => serializer.serialize_unit_variant("RequestType", 0u32, "AdminAssign"),
                Self::AdminRemove => serializer.serialize_unit_variant("RequestType", 1u32, "AdminRemove"),
                Self::AdminUpdate => serializer.serialize_unit_variant("RequestType", 2u32, "AdminUpdate"),
                Self::AdminExtend => serializer.serialize_unit_variant("RequestType", 3u32, "AdminExtend"),
                Self::AdminRenew => serializer.serialize_unit_variant("RequestType", 4u32, "AdminRenew"),
                Self::SelfActivate => serializer.serialize_unit_variant("RequestType", 5u32, "SelfActivate"),
                Self::SelfDeactivate => serializer.serialize_unit_variant("RequestType", 6u32, "SelfDeactivate"),
                Self::SelfExtend => serializer.serialize_unit_variant("RequestType", 7u32, "SelfExtend"),
                Self::SelfRenew => serializer.serialize_unit_variant("RequestType", 8u32, "SelfRenew"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role eligibility schedule request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Schedule info of the role eligibility schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ScheduleInfo {
        #[doc = "Start DateTime of the role eligibility schedule."]
        #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
        pub start_date_time: Option<time::OffsetDateTime>,
        #[doc = "Expiration of the role eligibility schedule"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<schedule_info::Expiration>,
    }
    impl ScheduleInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod schedule_info {
        use super::*;
        #[doc = "Expiration of the role eligibility schedule"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Expiration {
            #[doc = "Type of the role eligibility schedule expiration"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<expiration::Type>,
            #[doc = "End DateTime of the role eligibility schedule."]
            #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
            pub end_date_time: Option<time::OffsetDateTime>,
            #[doc = "Duration of the role eligibility schedule in TimeSpan."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub duration: Option<String>,
        }
        impl Expiration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod expiration {
            use super::*;
            #[doc = "Type of the role eligibility schedule expiration"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                AfterDuration,
                AfterDateTime,
                NoExpiration,
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
                        Self::AfterDuration => serializer.serialize_unit_variant("Type", 0u32, "AfterDuration"),
                        Self::AfterDateTime => serializer.serialize_unit_variant("Type", 1u32, "AfterDateTime"),
                        Self::NoExpiration => serializer.serialize_unit_variant("Type", 2u32, "NoExpiration"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "Ticket Info of the role eligibility"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TicketInfo {
        #[doc = "Ticket number for the role eligibility"]
        #[serde(rename = "ticketNumber", default, skip_serializing_if = "Option::is_none")]
        pub ticket_number: Option<String>,
        #[doc = "Ticket system name for the role eligibility"]
        #[serde(rename = "ticketSystem", default, skip_serializing_if = "Option::is_none")]
        pub ticket_system: Option<String>,
    }
    impl TicketInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Role management policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicy {
    #[doc = "The role management policy Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role management policy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role management policy type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role management policy properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleManagementPolicyProperties>,
}
impl RoleManagementPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy approval rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyApprovalRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The approval settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting: Option<ApprovalSettings>,
}
impl RoleManagementPolicyApprovalRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            setting: None,
        }
    }
}
#[doc = "Role management policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyAssignment {
    #[doc = "The role management policy Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role management policy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role management policy type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role management policy assignment properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleManagementPolicyAssignmentProperties>,
}
impl RoleManagementPolicyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role management policy assignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyAssignmentListResult {
    #[doc = "Role management policy assignment list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleManagementPolicyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleManagementPolicyAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleManagementPolicyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role management policy assignment properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyAssignmentProperties {
    #[doc = "The role management policy scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition of management policy assignment."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The policy id role management policy assignment."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "policyAssignmentProperties", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_properties: Option<PolicyAssignmentProperties>,
}
impl RoleManagementPolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy authentication context rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyAuthenticationContextRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The value indicating if rule is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "The claim value."]
    #[serde(rename = "claimValue", default, skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
}
impl RoleManagementPolicyAuthenticationContextRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            is_enabled: None,
            claim_value: None,
        }
    }
}
#[doc = "The role management policy enablement rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyEnablementRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The list of enabled rules."]
    #[serde(
        rename = "enabledRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_rules: Vec<String>,
}
impl RoleManagementPolicyEnablementRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            enabled_rules: Vec::new(),
        }
    }
}
#[doc = "The role management policy expiration rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyExpirationRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The value indicating whether expiration is required."]
    #[serde(rename = "isExpirationRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_expiration_required: Option<bool>,
    #[doc = "The maximum duration of expiration in timespan."]
    #[serde(rename = "maximumDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<String>,
}
impl RoleManagementPolicyExpirationRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            is_expiration_required: None,
            maximum_duration: None,
        }
    }
}
#[doc = "Role management policy list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyListResult {
    #[doc = "Role management policy list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RoleManagementPolicy>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleManagementPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RoleManagementPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy notification rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyNotificationRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The type of notification."]
    #[serde(rename = "notificationType", default, skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<role_management_policy_notification_rule::NotificationType>,
    #[doc = "The notification level."]
    #[serde(rename = "notificationLevel", default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<role_management_policy_notification_rule::NotificationLevel>,
    #[doc = "The recipient type."]
    #[serde(rename = "recipientType", default, skip_serializing_if = "Option::is_none")]
    pub recipient_type: Option<role_management_policy_notification_rule::RecipientType>,
    #[doc = "The list of notification recipients."]
    #[serde(
        rename = "notificationRecipients",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notification_recipients: Vec<String>,
    #[doc = "Determines if the notification will be sent to the recipient type specified in the policy rule."]
    #[serde(rename = "isDefaultRecipientsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_default_recipients_enabled: Option<bool>,
}
impl RoleManagementPolicyNotificationRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            notification_type: None,
            notification_level: None,
            recipient_type: None,
            notification_recipients: Vec::new(),
            is_default_recipients_enabled: None,
        }
    }
}
pub mod role_management_policy_notification_rule {
    use super::*;
    #[doc = "The type of notification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotificationType")]
    pub enum NotificationType {
        Email,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotificationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotificationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotificationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Email => serializer.serialize_unit_variant("NotificationType", 0u32, "Email"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The notification level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotificationLevel")]
    pub enum NotificationLevel {
        None,
        Critical,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotificationLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotificationLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotificationLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("NotificationLevel", 0u32, "None"),
                Self::Critical => serializer.serialize_unit_variant("NotificationLevel", 1u32, "Critical"),
                Self::All => serializer.serialize_unit_variant("NotificationLevel", 2u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The recipient type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecipientType")]
    pub enum RecipientType {
        Requestor,
        Approver,
        Admin,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecipientType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecipientType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecipientType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Requestor => serializer.serialize_unit_variant("RecipientType", 0u32, "Requestor"),
                Self::Approver => serializer.serialize_unit_variant("RecipientType", 1u32, "Approver"),
                Self::Admin => serializer.serialize_unit_variant("RecipientType", 2u32, "Admin"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role management policy properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyProperties {
    #[doc = "The role management policy scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role management policy display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The role management policy description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The role management policy is default policy."]
    #[serde(rename = "isOrganizationDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_organization_default: Option<bool>,
    #[doc = "The name of the entity last modified it"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<Principal>,
    #[doc = "The last modified date time."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The rule applied to the policy."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<RoleManagementPolicyRuleUnion>,
    #[doc = "The readonly computed rule applied to the policy."]
    #[serde(
        rename = "effectiveRules",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub effective_rules: Vec<RoleManagementPolicyRuleUnion>,
    #[serde(rename = "policyProperties", default, skip_serializing_if = "Option::is_none")]
    pub policy_properties: Option<PolicyProperties>,
}
impl RoleManagementPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyRule {
    #[doc = "The id of the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role management policy rule target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RoleManagementPolicyRuleTarget>,
}
impl RoleManagementPolicyRule {
    pub fn new() -> Self {
        Self { id: None, target: None }
    }
}
#[doc = "The type of rule"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ruleType")]
pub enum RoleManagementPolicyRuleUnion {}
#[doc = "The role management policy rule target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyRuleTarget {
    #[doc = "The caller of the setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,
    #[doc = "The type of operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<String>,
    #[doc = "The assignment level to which rule is applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The list of target objects."]
    #[serde(
        rename = "targetObjects",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_objects: Vec<String>,
    #[doc = "The list of inheritable settings."]
    #[serde(
        rename = "inheritableSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inheritable_settings: Vec<String>,
    #[doc = "The list of enforced settings."]
    #[serde(
        rename = "enforcedSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enforced_settings: Vec<String>,
}
impl RoleManagementPolicyRuleTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RoleManagementPolicyRuleType")]
pub enum RoleManagementPolicyRuleType {
    RoleManagementPolicyApprovalRule,
    RoleManagementPolicyAuthenticationContextRule,
    RoleManagementPolicyEnablementRule,
    RoleManagementPolicyExpirationRule,
    RoleManagementPolicyNotificationRule,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RoleManagementPolicyRuleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RoleManagementPolicyRuleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RoleManagementPolicyRuleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RoleManagementPolicyApprovalRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 0u32, "RoleManagementPolicyApprovalRule")
            }
            Self::RoleManagementPolicyAuthenticationContextRule => serializer.serialize_unit_variant(
                "RoleManagementPolicyRuleType",
                1u32,
                "RoleManagementPolicyAuthenticationContextRule",
            ),
            Self::RoleManagementPolicyEnablementRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 2u32, "RoleManagementPolicyEnablementRule")
            }
            Self::RoleManagementPolicyExpirationRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 3u32, "RoleManagementPolicyExpirationRule")
            }
            Self::RoleManagementPolicyNotificationRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 4u32, "RoleManagementPolicyNotificationRule")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Too many owners assigned to resource alert configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TooManyOwnersAssignedToResourceAlertConfigurationProperties {
    #[serde(flatten)]
    pub alert_configuration_properties: AlertConfigurationProperties,
    #[doc = "The threshold number of owners."]
    #[serde(rename = "thresholdNumberOfOwners", default, skip_serializing_if = "Option::is_none")]
    pub threshold_number_of_owners: Option<i32>,
    #[doc = "The threshold percentage of owners out of all role members."]
    #[serde(
        rename = "thresholdPercentageOfOwnersOutOfAllRoleMembers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub threshold_percentage_of_owners_out_of_all_role_members: Option<i32>,
}
impl TooManyOwnersAssignedToResourceAlertConfigurationProperties {
    pub fn new(alert_configuration_properties: AlertConfigurationProperties) -> Self {
        Self {
            alert_configuration_properties,
            threshold_number_of_owners: None,
            threshold_percentage_of_owners_out_of_all_role_members: None,
        }
    }
}
#[doc = "Too many owners assigned to resource alert incident properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TooManyOwnersAssignedToResourceAlertIncidentProperties {
    #[doc = "The assignee name."]
    #[serde(rename = "assigneeName", default, skip_serializing_if = "Option::is_none")]
    pub assignee_name: Option<String>,
    #[doc = "The assignee type."]
    #[serde(rename = "assigneeType", default, skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<String>,
}
impl TooManyOwnersAssignedToResourceAlertIncidentProperties {
    pub fn new() -> Self {
        Self {
            assignee_name: None,
            assignee_type: None,
        }
    }
}
#[doc = "Too many permanent owners assigned to resource alert configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TooManyPermanentOwnersAssignedToResourceAlertConfigurationProperties {
    #[serde(flatten)]
    pub alert_configuration_properties: AlertConfigurationProperties,
    #[doc = "The threshold number of permanent owners."]
    #[serde(rename = "thresholdNumberOfPermanentOwners", default, skip_serializing_if = "Option::is_none")]
    pub threshold_number_of_permanent_owners: Option<i32>,
    #[doc = "The threshold percentage of permanent owners out of all owners."]
    #[serde(
        rename = "thresholdPercentageOfPermanentOwnersOutOfAllOwners",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub threshold_percentage_of_permanent_owners_out_of_all_owners: Option<i32>,
}
impl TooManyPermanentOwnersAssignedToResourceAlertConfigurationProperties {
    pub fn new(alert_configuration_properties: AlertConfigurationProperties) -> Self {
        Self {
            alert_configuration_properties,
            threshold_number_of_permanent_owners: None,
            threshold_percentage_of_permanent_owners_out_of_all_owners: None,
        }
    }
}
#[doc = "Too many permanent owners assigned to resource alert incident properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TooManyPermanentOwnersAssignedToResourceAlertIncidentProperties {
    #[doc = "The assignee name."]
    #[serde(rename = "assigneeName", default, skip_serializing_if = "Option::is_none")]
    pub assignee_name: Option<String>,
    #[doc = "The assignee type."]
    #[serde(rename = "assigneeType", default, skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<String>,
}
impl TooManyPermanentOwnersAssignedToResourceAlertIncidentProperties {
    pub fn new() -> Self {
        Self {
            assignee_name: None,
            assignee_type: None,
        }
    }
}
#[doc = "The detail of a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSet {
    #[doc = "The type of user."]
    #[serde(rename = "userType", default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<user_set::UserType>,
    #[doc = "The value indicating whether the user is a backup fallback approver"]
    #[serde(rename = "isBackup", default, skip_serializing_if = "Option::is_none")]
    pub is_backup: Option<bool>,
    #[doc = "The object id of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The description of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl UserSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_set {
    use super::*;
    #[doc = "The type of user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserType")]
    pub enum UserType {
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("UserType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("UserType", 1u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Validation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponse {
    #[doc = "Whether or not validation succeeded"]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Failed validation result details"]
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ValidationResponseErrorInfo>,
}
impl ValidationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failed validation result details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponseErrorInfo {
    #[doc = "Error code indicating why validation failed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Message indicating why validation failed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidationResponseErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
