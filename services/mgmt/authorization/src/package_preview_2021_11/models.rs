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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessReviewContactedReviewer>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewContactedReviewerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339::option")]
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
    #[doc = "The type of decision target : User/ServicePrincipal"]
    #[serde(rename = "type")]
    pub type_: access_review_decision_identity::Type,
    #[doc = "The id of principal whose access was reviewed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the user whose access was reviewed."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AccessReviewDecisionIdentity {
    pub fn new(type_: access_review_decision_identity::Type) -> Self {
        Self {
            type_,
            id: None,
            display_name: None,
        }
    }
}
pub mod access_review_decision_identity {
    use super::*;
    #[doc = "The type of decision target : User/ServicePrincipal"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "servicePrincipal")]
        ServicePrincipal,
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
                Self::User => serializer.serialize_unit_variant("Type", 0u32, "user"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("Type", 1u32, "servicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of access review decisions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecisionListResult {
    #[doc = "Access Review Decision list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessReviewDecision>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewDecisionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccessReviewDecisionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Approval Step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessReviewDecisionProperties {
    #[doc = "Target of the decision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<AccessReviewDecisionIdentity>,
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
    #[serde(rename = "reviewedDateTime", with = "azure_core::date::rfc3339::option")]
    pub reviewed_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "reviewedBy", default, skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<AccessReviewActorIdentity>,
    #[doc = "The outcome of applying the decision."]
    #[serde(rename = "applyResult", default, skip_serializing_if = "Option::is_none")]
    pub apply_result: Option<access_review_decision_properties::ApplyResult>,
    #[doc = "The date and time when the review decision was applied."]
    #[serde(rename = "appliedDateTime", with = "azure_core::date::rfc3339::option")]
    pub applied_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "appliedBy", default, skip_serializing_if = "Option::is_none")]
    pub applied_by: Option<AccessReviewActorIdentity>,
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
    #[doc = "The type of resource: azureRole"]
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
    #[doc = "The type of resource: azureRole"]
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
#[doc = "Target of the decision."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessReviewDecisionResourceAzureRole {
    #[serde(flatten)]
    pub access_review_decision_resource: AccessReviewDecisionResource,
    #[doc = "The display name of resource associated with a decision record."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AccessReviewDecisionResourceAzureRole {
    pub fn new(access_review_decision_resource: AccessReviewDecisionResource) -> Self {
        Self {
            access_review_decision_resource,
            display_name: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessReviewHistoryInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewHistoryDefinitionInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessReviewHistoryDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewHistoryDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "reviewHistoryPeriodStartDateTime", with = "azure_core::date::rfc3339::option")]
    pub review_history_period_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date time used when selecting review data, all reviews included in data end on or before this date. For use only with one-time/non-recurring reports."]
    #[serde(rename = "reviewHistoryPeriodEndDateTime", with = "azure_core::date::rfc3339::option")]
    pub review_history_period_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Collection of review decisions which the history data should be filtered on. For example if Approve and Deny are supplied the data will only contain review results in which the decision maker approved or denied a review request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decisions: Vec<String>,
    #[doc = "This read-only field specifies the of the requested review history data. This is either requested, in-progress, done or error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_review_history_definition_properties::Status>,
    #[doc = "Date time when history definition was created"]
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the actor identity"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AccessReviewActorIdentity>,
    #[doc = "A collection of scopes used when selecting review history data"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<AccessReviewScope>,
    #[doc = "Recurrence settings of an Access Review History Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccessReviewHistoryScheduleSettings>,
    #[doc = "Set of access review history instances for this history definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "reviewHistoryPeriodStartDateTime", with = "azure_core::date::rfc3339::option")]
    pub review_history_period_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date time used when selecting review data, all reviews included in data end on or before this date. For use only with one-time/non-recurring reports."]
    #[serde(rename = "reviewHistoryPeriodEndDateTime", with = "azure_core::date::rfc3339::option")]
    pub review_history_period_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The display name for the parent history definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Status of the requested review history instance data. This is either requested, in-progress, done or error. The state transitions are as follows - Requested -> InProgress -> Done -> Expired"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<access_review_history_instance_properties::Status>,
    #[doc = "Date time when the history data report is scheduled to be generated."]
    #[serde(rename = "runDateTime", with = "azure_core::date::rfc3339::option")]
    pub run_date_time: Option<time::OffsetDateTime>,
    #[doc = "Date time when the history data report is scheduled to be generated."]
    #[serde(rename = "fulfilledDateTime", with = "azure_core::date::rfc3339::option")]
    pub fulfilled_date_time: Option<time::OffsetDateTime>,
    #[doc = "Uri which can be used to retrieve review history data. To generate this Uri, generateDownloadUri() must be called for a specific accessReviewHistoryDefinitionInstance. The link expires after a 24 hour period. Callers can see the expiration date time by looking at the 'se' parameter in the generated uri."]
    #[serde(rename = "downloadUri", default, skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    #[doc = "Date time when history data report expires and the associated data is deleted."]
    #[serde(with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessReviewInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The DateTime when the review instance is scheduled to end."]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the collection of reviewers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This is the collection of backup reviewers."]
    #[serde(rename = "backupReviewers", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The DateTime when the review is scheduled to end. Required if type is endDate"]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessReviewScheduleDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessReviewScheduleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This is the collection of backup reviewers."]
    #[serde(rename = "backupReviewers", default, skip_serializing_if = "Vec::is_empty")]
    pub backup_reviewers: Vec<AccessReviewReviewer>,
    #[doc = "This field specifies the type of reviewers for a review. Usually for a review, reviewers are explicitly assigned. However, in some cases, the reviewers may not be assigned and instead be chosen dynamically. For example managers review or self review."]
    #[serde(rename = "reviewersType", default, skip_serializing_if = "Option::is_none")]
    pub reviewers_type: Option<access_review_schedule_definition_properties::ReviewersType>,
    #[doc = "This is the collection of instances returned when one does an expand on it."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
