#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationActionMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<String>,
}
impl AuthorizationActionMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanaryTrafficRegionRolloutConfiguration {
    #[serde(rename = "skipRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub skip_regions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
}
impl CanaryTrafficRegionRolloutConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilitySpecifications {
    #[serde(rename = "enableDefaultValidation", default, skip_serializing_if = "Option::is_none")]
    pub enable_default_validation: Option<bool>,
    #[serde(rename = "resourceTypesWithCustomValidation", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types_with_custom_validation: Vec<String>,
}
impl CheckNameAvailabilitySpecifications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckinManifestInfo {
    #[serde(rename = "isCheckedIn")]
    pub is_checked_in: bool,
    #[serde(rename = "statusMessage")]
    pub status_message: String,
    #[serde(rename = "pullRequest", default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<String>,
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
}
impl CheckinManifestInfo {
    pub fn new(is_checked_in: bool, status_message: String) -> Self {
        Self {
            is_checked_in,
            status_message,
            pull_request: None,
            commit_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckinManifestParams {
    #[doc = "The environment supplied to the checkin manifest operation."]
    pub environment: String,
    #[doc = "The baseline ARM manifest location supplied to the checkin manifest operation."]
    #[serde(rename = "baselineArmManifestLocation")]
    pub baseline_arm_manifest_location: String,
}
impl CheckinManifestParams {
    pub fn new(environment: String, baseline_arm_manifest_location: String) -> Self {
        Self {
            environment,
            baseline_arm_manifest_location,
        }
    }
}
#[doc = "Rollout details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRollout {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the rollout."]
    pub properties: serde_json::Value,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl CustomRollout {
    pub fn new(properties: serde_json::Value) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRolloutArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomRollout>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomRolloutArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomRolloutArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRolloutProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    pub specification: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
}
impl CustomRolloutProperties {
    pub fn new(specification: serde_json::Value) -> Self {
        Self {
            provisioning_state: None,
            specification,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRolloutSpecification {
    pub canary: serde_json::Value,
    #[serde(rename = "providerRegistration", default, skip_serializing_if = "Option::is_none")]
    pub provider_registration: Option<serde_json::Value>,
    #[serde(rename = "resourceTypeRegistrations", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_type_registrations: Vec<ResourceTypeRegistration>,
}
impl CustomRolloutSpecification {
    pub fn new(canary: serde_json::Value) -> Self {
        Self {
            canary,
            provider_registration: None,
            resource_type_registrations: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRolloutStatus {
    #[serde(rename = "completedRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub completed_regions: Vec<String>,
    #[serde(rename = "failedOrSkippedRegions", default, skip_serializing_if = "Option::is_none")]
    pub failed_or_skipped_regions: Option<serde_json::Value>,
}
impl CustomRolloutStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Default rollout definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultRollout {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the rollout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DefaultRollout {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultRolloutArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DefaultRollout>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DefaultRolloutArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DefaultRolloutArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultRolloutProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
}
impl DefaultRolloutProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultRolloutSpecification {
    #[serde(rename = "expeditedRollout", default, skip_serializing_if = "Option::is_none")]
    pub expedited_rollout: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canary: Option<serde_json::Value>,
    #[serde(rename = "lowTraffic", default, skip_serializing_if = "Option::is_none")]
    pub low_traffic: Option<serde_json::Value>,
    #[serde(rename = "mediumTraffic", default, skip_serializing_if = "Option::is_none")]
    pub medium_traffic: Option<serde_json::Value>,
    #[serde(rename = "highTraffic", default, skip_serializing_if = "Option::is_none")]
    pub high_traffic: Option<serde_json::Value>,
    #[serde(rename = "restOfTheWorldGroupOne", default, skip_serializing_if = "Option::is_none")]
    pub rest_of_the_world_group_one: Option<serde_json::Value>,
    #[serde(rename = "restOfTheWorldGroupTwo", default, skip_serializing_if = "Option::is_none")]
    pub rest_of_the_world_group_two: Option<serde_json::Value>,
    #[serde(rename = "providerRegistration", default, skip_serializing_if = "Option::is_none")]
    pub provider_registration: Option<serde_json::Value>,
    #[serde(rename = "resourceTypeRegistrations", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_type_registrations: Vec<ResourceTypeRegistration>,
}
impl DefaultRolloutSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultRolloutStatus {
    #[serde(flatten)]
    pub rollout_status_base: RolloutStatusBase,
    #[serde(rename = "nextTrafficRegion", default, skip_serializing_if = "Option::is_none")]
    pub next_traffic_region: Option<default_rollout_status::NextTrafficRegion>,
    #[serde(rename = "nextTrafficRegionScheduledTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_traffic_region_scheduled_time: Option<time::OffsetDateTime>,
    #[serde(rename = "subscriptionReregistrationResult", default, skip_serializing_if = "Option::is_none")]
    pub subscription_reregistration_result: Option<default_rollout_status::SubscriptionReregistrationResult>,
}
impl DefaultRolloutStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod default_rollout_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NextTrafficRegion")]
    pub enum NextTrafficRegion {
        NotSpecified,
        Canary,
        LowTraffic,
        MediumTraffic,
        HighTraffic,
        None,
        RestOfTheWorldGroupOne,
        RestOfTheWorldGroupTwo,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NextTrafficRegion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NextTrafficRegion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NextTrafficRegion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("NextTrafficRegion", 0u32, "NotSpecified"),
                Self::Canary => serializer.serialize_unit_variant("NextTrafficRegion", 1u32, "Canary"),
                Self::LowTraffic => serializer.serialize_unit_variant("NextTrafficRegion", 2u32, "LowTraffic"),
                Self::MediumTraffic => serializer.serialize_unit_variant("NextTrafficRegion", 3u32, "MediumTraffic"),
                Self::HighTraffic => serializer.serialize_unit_variant("NextTrafficRegion", 4u32, "HighTraffic"),
                Self::None => serializer.serialize_unit_variant("NextTrafficRegion", 5u32, "None"),
                Self::RestOfTheWorldGroupOne => serializer.serialize_unit_variant("NextTrafficRegion", 6u32, "RestOfTheWorldGroupOne"),
                Self::RestOfTheWorldGroupTwo => serializer.serialize_unit_variant("NextTrafficRegion", 7u32, "RestOfTheWorldGroupTwo"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionReregistrationResult")]
    pub enum SubscriptionReregistrationResult {
        NotApplicable,
        ConditionalUpdate,
        ForcedUpdate,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SubscriptionReregistrationResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SubscriptionReregistrationResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SubscriptionReregistrationResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("SubscriptionReregistrationResult", 0u32, "NotApplicable"),
                Self::ConditionalUpdate => serializer.serialize_unit_variant("SubscriptionReregistrationResult", 1u32, "ConditionalUpdate"),
                Self::ForcedUpdate => serializer.serialize_unit_variant("SubscriptionReregistrationResult", 2u32, "ForcedUpdate"),
                Self::Failed => serializer.serialize_unit_variant("SubscriptionReregistrationResult", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Standard error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Array of details about specific errors that led to this reported error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[doc = "Object containing more specific information than  the current object about the error."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<serde_json::Value>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Standard error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Standard error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
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
#[doc = "Expedited rollout configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpeditedRolloutDefinition {
    #[doc = "Indicates whether expedited rollout is enabled/disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ExpeditedRolloutDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ExtendedErrorInfo>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<TypedErrorInfo>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocationOptions {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "supportedPolicy", default, skip_serializing_if = "Option::is_none")]
    pub supported_policy: Option<String>,
}
impl ExtendedLocationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionOptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub response: Vec<String>,
}
impl ExtensionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeaturesRule {
    #[serde(rename = "requiredFeaturesPolicy")]
    pub required_features_policy: features_rule::RequiredFeaturesPolicy,
}
impl FeaturesRule {
    pub fn new(required_features_policy: features_rule::RequiredFeaturesPolicy) -> Self {
        Self { required_features_policy }
    }
}
pub mod features_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequiredFeaturesPolicy")]
    pub enum RequiredFeaturesPolicy {
        Any,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequiredFeaturesPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequiredFeaturesPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequiredFeaturesPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Any => serializer.serialize_unit_variant("RequiredFeaturesPolicy", 0u32, "Any"),
                Self::All => serializer.serialize_unit_variant("RequiredFeaturesPolicy", 1u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityManagement {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_management::Type>,
}
impl IdentityManagement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_management {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        NotSpecified,
        SystemAssigned,
        UserAssigned,
        Actor,
        DelegatedResourceIdentity,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Type", 0u32, "NotSpecified"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::Actor => serializer.serialize_unit_variant("Type", 3u32, "Actor"),
                Self::DelegatedResourceIdentity => serializer.serialize_unit_variant("Type", 4u32, "DelegatedResourceIdentity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityManagementProperties {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_management_properties::Type>,
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
impl IdentityManagementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_management_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        NotSpecified,
        SystemAssigned,
        UserAssigned,
        Actor,
        DelegatedResourceIdentity,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Type", 0u32, "NotSpecified"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::Actor => serializer.serialize_unit_variant("Type", 3u32, "Actor"),
                Self::DelegatedResourceIdentity => serializer.serialize_unit_variant("Type", 4u32, "DelegatedResourceIdentity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Inner error containing list of errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "Specific error code than was provided by the containing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Object containing more specific information than the current object about the error."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<serde_json::Value>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightHouseAuthorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
impl LightHouseAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            role_definition_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedAccessCheck {
    #[serde(rename = "actionName", default, skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "linkedProperty", default, skip_serializing_if = "Option::is_none")]
    pub linked_property: Option<String>,
    #[serde(rename = "linkedAction", default, skip_serializing_if = "Option::is_none")]
    pub linked_action: Option<String>,
    #[serde(rename = "linkedActionVerb", default, skip_serializing_if = "Option::is_none")]
    pub linked_action_verb: Option<String>,
    #[serde(rename = "linkedType", default, skip_serializing_if = "Option::is_none")]
    pub linked_type: Option<String>,
}
impl LinkedAccessCheck {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedOperationRule {
    #[serde(rename = "linkedOperation")]
    pub linked_operation: linked_operation_rule::LinkedOperation,
    #[serde(rename = "linkedAction")]
    pub linked_action: linked_operation_rule::LinkedAction,
}
impl LinkedOperationRule {
    pub fn new(linked_operation: linked_operation_rule::LinkedOperation, linked_action: linked_operation_rule::LinkedAction) -> Self {
        Self {
            linked_operation,
            linked_action,
        }
    }
}
pub mod linked_operation_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkedOperation")]
    pub enum LinkedOperation {
        None,
        CrossResourceGroupResourceMove,
        CrossSubscriptionResourceMove,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkedOperation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkedOperation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkedOperation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("LinkedOperation", 0u32, "None"),
                Self::CrossResourceGroupResourceMove => {
                    serializer.serialize_unit_variant("LinkedOperation", 1u32, "CrossResourceGroupResourceMove")
                }
                Self::CrossSubscriptionResourceMove => {
                    serializer.serialize_unit_variant("LinkedOperation", 2u32, "CrossSubscriptionResourceMove")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkedAction")]
    pub enum LinkedAction {
        NotSpecified,
        Blocked,
        Validate,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkedAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkedAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkedAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LinkedAction", 0u32, "NotSpecified"),
                Self::Blocked => serializer.serialize_unit_variant("LinkedAction", 1u32, "Blocked"),
                Self::Validate => serializer.serialize_unit_variant("LinkedAction", 2u32, "Validate"),
                Self::Enabled => serializer.serialize_unit_variant("LinkedAction", 3u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizedOperationDefinition {
    #[doc = "Name of the operation."]
    pub name: String,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<localized_operation_definition::Origin>,
    #[doc = "Display information of the operation."]
    pub display: serde_json::Value,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<localized_operation_definition::ActionType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl LocalizedOperationDefinition {
    pub fn new(name: String, display: serde_json::Value) -> Self {
        Self {
            name,
            is_data_action: None,
            origin: None,
            display,
            action_type: None,
            properties: None,
        }
    }
}
pub mod localized_operation_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        NotSpecified,
        User,
        System,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        NotSpecified,
        Internal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizedOperationDisplayDefinition {
    #[doc = "Display information of the operation."]
    pub default: serde_json::Value,
    #[doc = "Display information of the operation for en locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<serde_json::Value>,
    #[doc = "Display information of the operation for cs locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cs: Option<serde_json::Value>,
    #[doc = "Display information of the operation for de locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub de: Option<serde_json::Value>,
    #[doc = "Display information of the operation for es locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub es: Option<serde_json::Value>,
    #[doc = "Display information of the operation for fr locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr: Option<serde_json::Value>,
    #[doc = "Display information of the operation for hu locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hu: Option<serde_json::Value>,
    #[doc = "Display information of the operation for it locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub it: Option<serde_json::Value>,
    #[doc = "Display information of the operation for ja locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja: Option<serde_json::Value>,
    #[doc = "Display information of the operation for ko locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ko: Option<serde_json::Value>,
    #[doc = "Display information of the operation for nl locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nl: Option<serde_json::Value>,
    #[doc = "Display information of the operation for pl locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl: Option<serde_json::Value>,
    #[doc = "Display information of the operation for pt-BR locale."]
    #[serde(rename = "ptBR", default, skip_serializing_if = "Option::is_none")]
    pub pt_br: Option<serde_json::Value>,
    #[doc = "Display information of the operation for pt-PT locale."]
    #[serde(rename = "ptPT", default, skip_serializing_if = "Option::is_none")]
    pub pt_pt: Option<serde_json::Value>,
    #[doc = "Display information of the operation for ru locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ru: Option<serde_json::Value>,
    #[doc = "Display information of the operation for sv locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sv: Option<serde_json::Value>,
    #[doc = "Display information of the operation for zh-Hans locale."]
    #[serde(rename = "zhHans", default, skip_serializing_if = "Option::is_none")]
    pub zh_hans: Option<serde_json::Value>,
    #[doc = "Display information of the operation for zh-Hant locale."]
    #[serde(rename = "zhHant", default, skip_serializing_if = "Option::is_none")]
    pub zh_hant: Option<serde_json::Value>,
}
impl LocalizedOperationDisplayDefinition {
    pub fn new(default: serde_json::Value) -> Self {
        Self {
            default,
            en: None,
            cs: None,
            de: None,
            es: None,
            fr: None,
            hu: None,
            it: None,
            ja: None,
            ko: None,
            nl: None,
            pl: None,
            pt_br: None,
            pt_pt: None,
            ru: None,
            sv: None,
            zh_hans: None,
            zh_hant: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggingHiddenPropertyPath {
    #[serde(rename = "hiddenPathsOnRequest", default, skip_serializing_if = "Vec::is_empty")]
    pub hidden_paths_on_request: Vec<String>,
    #[serde(rename = "hiddenPathsOnResponse", default, skip_serializing_if = "Vec::is_empty")]
    pub hidden_paths_on_response: Vec<String>,
}
impl LoggingHiddenPropertyPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggingRule {
    pub action: String,
    pub direction: logging_rule::Direction,
    #[serde(rename = "detailLevel")]
    pub detail_level: logging_rule::DetailLevel,
    #[serde(rename = "hiddenPropertyPaths", default, skip_serializing_if = "Option::is_none")]
    pub hidden_property_paths: Option<serde_json::Value>,
}
impl LoggingRule {
    pub fn new(action: String, direction: logging_rule::Direction, detail_level: logging_rule::DetailLevel) -> Self {
        Self {
            action,
            direction,
            detail_level,
            hidden_property_paths: None,
        }
    }
}
pub mod logging_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        None,
        Request,
        Response,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Direction", 0u32, "None"),
                Self::Request => serializer.serialize_unit_variant("Direction", 1u32, "Request"),
                Self::Response => serializer.serialize_unit_variant("Direction", 2u32, "Response"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailLevel")]
    pub enum DetailLevel {
        None,
        Body,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DetailLevel", 0u32, "None"),
                Self::Body => serializer.serialize_unit_variant("DetailLevel", 1u32, "Body"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationEndpoint {
    #[serde(rename = "notificationDestination", default, skip_serializing_if = "Option::is_none")]
    pub notification_destination: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
}
impl NotificationEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The notification registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationRegistration {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl NotificationRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationRegistrationArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationRegistration>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationRegistrationArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NotificationRegistrationArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationRegistrationProperties {
    #[serde(rename = "notificationMode", default, skip_serializing_if = "Option::is_none")]
    pub notification_mode: Option<notification_registration_properties::NotificationMode>,
    #[serde(rename = "messageScope", default, skip_serializing_if = "Option::is_none")]
    pub message_scope: Option<notification_registration_properties::MessageScope>,
    #[serde(rename = "includedEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub included_events: Vec<String>,
    #[serde(rename = "notificationEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_endpoints: Vec<NotificationEndpoint>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NotificationRegistrationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notification_registration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotificationMode")]
    pub enum NotificationMode {
        NotSpecified,
        EventHub,
        WebHook,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotificationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotificationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotificationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("NotificationMode", 0u32, "NotSpecified"),
                Self::EventHub => serializer.serialize_unit_variant("NotificationMode", 1u32, "EventHub"),
                Self::WebHook => serializer.serialize_unit_variant("NotificationMode", 2u32, "WebHook"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MessageScope")]
    pub enum MessageScope {
        NotSpecified,
        RegisteredSubscriptions,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MessageScope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MessageScope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MessageScope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("MessageScope", 0u32, "NotSpecified"),
                Self::RegisteredSubscriptions => serializer.serialize_unit_variant("MessageScope", 1u32, "RegisteredSubscriptions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenApiConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<OpenApiValidation>,
}
impl OpenApiConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenApiValidation {
    #[doc = "Indicates whether a non compliance response is allowed for a LIST call"]
    #[serde(rename = "allowNoncompliantCollectionResponse", default, skip_serializing_if = "Option::is_none")]
    pub allow_noncompliant_collection_response: Option<bool>,
}
impl OpenApiValidation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsContent {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationsDefinition>,
}
impl OperationsContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsContentProperties {
    #[doc = "Operations content."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<LocalizedOperationDefinition>,
}
impl OperationsContentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDefinition {
    #[doc = "Name of the operation."]
    pub name: String,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operations_definition::Origin>,
    #[doc = "Display information of the operation."]
    pub display: serde_json::Value,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operations_definition::ActionType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationsDefinition {
    pub fn new(name: String, display: serde_json::Value) -> Self {
        Self {
            name,
            is_data_action: None,
            origin: None,
            display,
            action_type: None,
            properties: None,
        }
    }
}
pub mod operations_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        NotSpecified,
        User,
        System,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        NotSpecified,
        Internal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDefinitionArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDefinition>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationsDefinitionArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsDefinitionArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsDisplayDefinition {
    pub provider: String,
    pub resource: String,
    pub operation: String,
    pub description: String,
}
impl OperationsDisplayDefinition {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsPutContent {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationsPutContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderHubMetadata {
    #[serde(rename = "providerAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_authorizations: Vec<ResourceProviderAuthorization>,
    #[serde(rename = "providerAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub provider_authentication: Option<serde_json::Value>,
    #[serde(rename = "thirdPartyProviderAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub third_party_provider_authorization: Option<serde_json::Value>,
}
impl ProviderHubMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderRegistration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ProviderRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderRegistrationArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderRegistration>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderRegistrationArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderRegistrationArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderRegistrationProperties {
    #[serde(flatten)]
    pub resource_provider_manifest_properties: ResourceProviderManifestProperties,
    #[serde(rename = "providerHubMetadata", default, skip_serializing_if = "Option::is_none")]
    pub provider_hub_metadata: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(
        rename = "subscriptionLifecycleNotificationSpecifications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_lifecycle_notification_specifications: Option<serde_json::Value>,
}
impl ProviderRegistrationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    MovingResources,
    TransientFailure,
    RolloutInProgress,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
            Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Created"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Succeeded"),
            Self::MovingResources => serializer.serialize_unit_variant("ProvisioningState", 10u32, "MovingResources"),
            Self::TransientFailure => serializer.serialize_unit_variant("ProvisioningState", 11u32, "TransientFailure"),
            Self::RolloutInProgress => serializer.serialize_unit_variant("ProvisioningState", 12u32, "RolloutInProgress"),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReRegisterSubscriptionMetadata {
    pub enabled: bool,
    #[serde(rename = "concurrencyLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrency_limit: Option<i32>,
}
impl ReRegisterSubscriptionMetadata {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            concurrency_limit: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestHeaderOptions {
    #[serde(rename = "optInHeaders", default, skip_serializing_if = "Option::is_none")]
    pub opt_in_headers: Option<request_header_options::OptInHeaders>,
}
impl RequestHeaderOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod request_header_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OptInHeaders")]
    pub enum OptInHeaders {
        NotSpecified,
        SignedUserToken,
        ClientGroupMembership,
        SignedAuxiliaryTokens,
        UnboundedClientGroupMembership,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OptInHeaders {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OptInHeaders {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OptInHeaders {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("OptInHeaders", 0u32, "NotSpecified"),
                Self::SignedUserToken => serializer.serialize_unit_variant("OptInHeaders", 1u32, "SignedUserToken"),
                Self::ClientGroupMembership => serializer.serialize_unit_variant("OptInHeaders", 2u32, "ClientGroupMembership"),
                Self::SignedAuxiliaryTokens => serializer.serialize_unit_variant("OptInHeaders", 3u32, "SignedAuxiliaryTokens"),
                Self::UnboundedClientGroupMembership => {
                    serializer.serialize_unit_variant("OptInHeaders", 4u32, "UnboundedClientGroupMembership")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceConcurrencyControlOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<resource_concurrency_control_option::Policy>,
}
impl ResourceConcurrencyControlOption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_concurrency_control_option {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Policy")]
    pub enum Policy {
        NotSpecified,
        SynchronizeBeginExtension,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Policy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Policy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Policy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Policy", 0u32, "NotSpecified"),
                Self::SynchronizeBeginExtension => serializer.serialize_unit_variant("Policy", 1u32, "SynchronizeBeginExtension"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGraphConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl ResourceGraphConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceManagementAction {
    #[doc = "resource management action content."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceManagementEntity>,
}
impl ResourceManagementAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceManagementEntity {
    #[doc = "The resource id."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The home tenant id."]
    #[serde(rename = "homeTenantId", default, skip_serializing_if = "Option::is_none")]
    pub home_tenant_id: Option<String>,
    #[doc = "The location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ResourceManagementEntity {
    pub fn new(resource_id: String) -> Self {
        Self {
            resource_id,
            home_tenant_id: None,
            location: None,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceMovePolicy {
    #[serde(rename = "validationRequired", default, skip_serializing_if = "Option::is_none")]
    pub validation_required: Option<bool>,
    #[serde(rename = "crossResourceGroupMoveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cross_resource_group_move_enabled: Option<bool>,
    #[serde(rename = "crossSubscriptionMoveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub cross_subscription_move_enabled: Option<bool>,
}
impl ResourceMovePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderAuthentication {
    #[serde(rename = "allowedAudiences")]
    pub allowed_audiences: Vec<String>,
}
impl ResourceProviderAuthentication {
    pub fn new(allowed_audiences: Vec<String>) -> Self {
        Self { allowed_audiences }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderAuthorization {
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[serde(rename = "managedByRoleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_role_definition_id: Option<String>,
}
impl ResourceProviderAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderCapabilities {
    #[serde(rename = "quotaId")]
    pub quota_id: String,
    pub effect: resource_provider_capabilities::Effect,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
}
impl ResourceProviderCapabilities {
    pub fn new(quota_id: String, effect: resource_provider_capabilities::Effect) -> Self {
        Self {
            quota_id,
            effect,
            required_features: Vec::new(),
        }
    }
}
pub mod resource_provider_capabilities {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Effect")]
    pub enum Effect {
        NotSpecified,
        Allow,
        Disallow,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Effect", 0u32, "NotSpecified"),
                Self::Allow => serializer.serialize_unit_variant("Effect", 1u32, "Allow"),
                Self::Disallow => serializer.serialize_unit_variant("Effect", 2u32, "Disallow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(rename = "featuresRule", default, skip_serializing_if = "Option::is_none")]
    pub features_rule: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<resource_provider_endpoint::EndpointType>,
}
impl ResourceProviderEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_endpoint {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        NotSpecified,
        Canary,
        Production,
        TestInProduction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("EndpointType", 0u32, "NotSpecified"),
                Self::Canary => serializer.serialize_unit_variant("EndpointType", 1u32, "Canary"),
                Self::Production => serializer.serialize_unit_variant("EndpointType", 2u32, "Production"),
                Self::TestInProduction => serializer.serialize_unit_variant("EndpointType", 3u32, "TestInProduction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderManagement {
    #[serde(rename = "schemaOwners", default, skip_serializing_if = "Vec::is_empty")]
    pub schema_owners: Vec<String>,
    #[serde(rename = "manifestOwners", default, skip_serializing_if = "Vec::is_empty")]
    pub manifest_owners: Vec<String>,
    #[serde(rename = "incidentRoutingService", default, skip_serializing_if = "Option::is_none")]
    pub incident_routing_service: Option<String>,
    #[serde(rename = "incidentRoutingTeam", default, skip_serializing_if = "Option::is_none")]
    pub incident_routing_team: Option<String>,
    #[serde(rename = "incidentContactEmail", default, skip_serializing_if = "Option::is_none")]
    pub incident_contact_email: Option<String>,
    #[serde(rename = "serviceTreeInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub service_tree_infos: Vec<ServiceTreeInfo>,
    #[serde(rename = "resourceAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub resource_access_policy: Option<resource_provider_management::ResourceAccessPolicy>,
    #[serde(rename = "resourceAccessRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_access_roles: Vec<serde_json::Value>,
}
impl ResourceProviderManagement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_management {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceAccessPolicy {
        NotSpecified,
        AcisReadAllowed,
        AcisActionAllowed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderManifest {
    #[serde(rename = "providerAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub provider_authentication: Option<serde_json::Value>,
    #[serde(rename = "providerAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_authorizations: Vec<ResourceProviderAuthorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "providerVersion", default, skip_serializing_if = "Option::is_none")]
    pub provider_version: Option<String>,
    #[serde(rename = "providerType", default, skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<resource_provider_manifest::ProviderType>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(rename = "featuresRule", default, skip_serializing_if = "Option::is_none")]
    pub features_rule: Option<serde_json::Value>,
    #[serde(rename = "requestHeaderOptions", default, skip_serializing_if = "Option::is_none")]
    pub request_header_options: Option<serde_json::Value>,
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceProviderCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "globalNotificationEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub global_notification_endpoints: Vec<ResourceProviderEndpoint>,
    #[serde(rename = "reRegisterSubscriptionMetadata", default, skip_serializing_if = "Option::is_none")]
    pub re_register_subscription_metadata: Option<serde_json::Value>,
}
impl ResourceProviderManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_manifest {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProviderType")]
    pub enum ProviderType {
        NotSpecified,
        Internal,
        External,
        Hidden,
        RegistrationFree,
        LegacyRegistrationRequired,
        TenantOnly,
        AuthorizationFree,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProviderType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProviderType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProviderType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ProviderType", 0u32, "NotSpecified"),
                Self::Internal => serializer.serialize_unit_variant("ProviderType", 1u32, "Internal"),
                Self::External => serializer.serialize_unit_variant("ProviderType", 2u32, "External"),
                Self::Hidden => serializer.serialize_unit_variant("ProviderType", 3u32, "Hidden"),
                Self::RegistrationFree => serializer.serialize_unit_variant("ProviderType", 4u32, "RegistrationFree"),
                Self::LegacyRegistrationRequired => serializer.serialize_unit_variant("ProviderType", 5u32, "LegacyRegistrationRequired"),
                Self::TenantOnly => serializer.serialize_unit_variant("ProviderType", 6u32, "TenantOnly"),
                Self::AuthorizationFree => serializer.serialize_unit_variant("ProviderType", 7u32, "AuthorizationFree"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderManifestProperties {
    #[serde(rename = "providerAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub provider_authentication: Option<serde_json::Value>,
    #[serde(rename = "providerAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_authorizations: Vec<ResourceProviderAuthorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "providerVersion", default, skip_serializing_if = "Option::is_none")]
    pub provider_version: Option<String>,
    #[serde(rename = "providerType", default, skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<resource_provider_manifest_properties::ProviderType>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(rename = "featuresRule", default, skip_serializing_if = "Option::is_none")]
    pub features_rule: Option<serde_json::Value>,
    #[serde(rename = "requestHeaderOptions", default, skip_serializing_if = "Option::is_none")]
    pub request_header_options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceProviderCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "templateDeploymentOptions", default, skip_serializing_if = "Option::is_none")]
    pub template_deployment_options: Option<serde_json::Value>,
}
impl ResourceProviderManifestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_manifest_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProviderType")]
    pub enum ProviderType {
        NotSpecified,
        Internal,
        External,
        Hidden,
        RegistrationFree,
        LegacyRegistrationRequired,
        TenantOnly,
        AuthorizationFree,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProviderType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProviderType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProviderType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ProviderType", 0u32, "NotSpecified"),
                Self::Internal => serializer.serialize_unit_variant("ProviderType", 1u32, "Internal"),
                Self::External => serializer.serialize_unit_variant("ProviderType", 2u32, "External"),
                Self::Hidden => serializer.serialize_unit_variant("ProviderType", 3u32, "Hidden"),
                Self::RegistrationFree => serializer.serialize_unit_variant("ProviderType", 4u32, "RegistrationFree"),
                Self::LegacyRegistrationRequired => serializer.serialize_unit_variant("ProviderType", 5u32, "LegacyRegistrationRequired"),
                Self::TenantOnly => serializer.serialize_unit_variant("ProviderType", 6u32, "TenantOnly"),
                Self::AuthorizationFree => serializer.serialize_unit_variant("ProviderType", 7u32, "AuthorizationFree"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingType", default, skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<resource_type::RoutingType>,
    #[serde(rename = "resourceValidation", default, skip_serializing_if = "Option::is_none")]
    pub resource_validation: Option<resource_type::ResourceValidation>,
    #[serde(rename = "allowedUnauthorizedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_unauthorized_actions: Vec<String>,
    #[serde(rename = "authorizationActionMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_action_mappings: Vec<AuthorizationActionMapping>,
    #[serde(rename = "linkedAccessChecks", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_access_checks: Vec<LinkedAccessCheck>,
    #[serde(rename = "defaultApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_api_version: Option<String>,
    #[serde(rename = "loggingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub logging_rules: Vec<LoggingRule>,
    #[serde(rename = "throttlingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub throttling_rules: Vec<ThrottlingRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<ResourceProviderEndpoint>,
    #[serde(rename = "marketplaceType", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_type: Option<resource_type::MarketplaceType>,
    #[serde(rename = "identityManagement", default, skip_serializing_if = "Option::is_none")]
    pub identity_management: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(rename = "featuresRule", default, skip_serializing_if = "Option::is_none")]
    pub features_rule: Option<serde_json::Value>,
    #[serde(rename = "subscriptionStateRules", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_state_rules: Vec<SubscriptionStateRule>,
    #[serde(rename = "serviceTreeInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub service_tree_infos: Vec<ServiceTreeInfo>,
    #[serde(rename = "requestHeaderOptions", default, skip_serializing_if = "Option::is_none")]
    pub request_header_options: Option<serde_json::Value>,
    #[serde(rename = "skuLink", default, skip_serializing_if = "Option::is_none")]
    pub sku_link: Option<String>,
    #[serde(rename = "disallowedActionVerbs", default, skip_serializing_if = "Vec::is_empty")]
    pub disallowed_action_verbs: Vec<String>,
    #[serde(rename = "templateDeploymentPolicy", default, skip_serializing_if = "Option::is_none")]
    pub template_deployment_policy: Option<serde_json::Value>,
    #[serde(rename = "extendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub extended_locations: Vec<ExtendedLocationOptions>,
    #[serde(rename = "linkedOperationRules", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_operation_rules: Vec<LinkedOperationRule>,
    #[serde(rename = "resourceDeletionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub resource_deletion_policy: Option<resource_type::ResourceDeletionPolicy>,
}
impl ResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_type {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingType")]
    pub enum RoutingType {
        Default,
        ProxyOnly,
        HostBased,
        Extension,
        Tenant,
        Fanout,
        LocationBased,
        Failover,
        CascadeExtension,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("RoutingType", 0u32, "Default"),
                Self::ProxyOnly => serializer.serialize_unit_variant("RoutingType", 1u32, "ProxyOnly"),
                Self::HostBased => serializer.serialize_unit_variant("RoutingType", 2u32, "HostBased"),
                Self::Extension => serializer.serialize_unit_variant("RoutingType", 3u32, "Extension"),
                Self::Tenant => serializer.serialize_unit_variant("RoutingType", 4u32, "Tenant"),
                Self::Fanout => serializer.serialize_unit_variant("RoutingType", 5u32, "Fanout"),
                Self::LocationBased => serializer.serialize_unit_variant("RoutingType", 6u32, "LocationBased"),
                Self::Failover => serializer.serialize_unit_variant("RoutingType", 7u32, "Failover"),
                Self::CascadeExtension => serializer.serialize_unit_variant("RoutingType", 8u32, "CascadeExtension"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceValidation")]
    pub enum ResourceValidation {
        NotSpecified,
        ReservedWords,
        ProfaneWords,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceValidation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceValidation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceValidation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ResourceValidation", 0u32, "NotSpecified"),
                Self::ReservedWords => serializer.serialize_unit_variant("ResourceValidation", 1u32, "ReservedWords"),
                Self::ProfaneWords => serializer.serialize_unit_variant("ResourceValidation", 2u32, "ProfaneWords"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MarketplaceType {
        NotSpecified,
        AddOn,
        Bypass,
        Store,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceDeletionPolicy")]
    pub enum ResourceDeletionPolicy {
        NotSpecified,
        Cascade,
        Force,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceDeletionPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceDeletionPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceDeletionPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ResourceDeletionPolicy", 0u32, "NotSpecified"),
                Self::Cascade => serializer.serialize_unit_variant("ResourceDeletionPolicy", 1u32, "Cascade"),
                Self::Force => serializer.serialize_unit_variant("ResourceDeletionPolicy", 2u32, "Force"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(rename = "featuresRule", default, skip_serializing_if = "Option::is_none")]
    pub features_rule: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<ResourceTypeExtension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<resource_type_endpoint::EndpointType>,
}
impl ResourceTypeEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_type_endpoint {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        NotSpecified,
        Canary,
        Production,
        TestInProduction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("EndpointType", 0u32, "NotSpecified"),
                Self::Canary => serializer.serialize_unit_variant("EndpointType", 1u32, "Canary"),
                Self::Production => serializer.serialize_unit_variant("EndpointType", 2u32, "Production"),
                Self::TestInProduction => serializer.serialize_unit_variant("EndpointType", 3u32, "TestInProduction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeExtension {
    #[serde(rename = "endpointUri", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_uri: Option<String>,
    #[serde(rename = "extensionCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub extension_categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl ResourceTypeExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeExtensionOptions {
    #[serde(rename = "resourceCreationBegin", default, skip_serializing_if = "Option::is_none")]
    pub resource_creation_begin: Option<serde_json::Value>,
}
impl ResourceTypeExtensionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeRegistration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ResourceTypeRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeRegistrationArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceTypeRegistration>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceTypeRegistrationArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceTypeRegistrationArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTypeRegistrationProperties {
    #[serde(rename = "routingType", default, skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<resource_type_registration_properties::RoutingType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regionality: Option<resource_type_registration_properties::Regionality>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<ResourceTypeEndpoint>,
    #[serde(rename = "extensionOptions", default, skip_serializing_if = "Option::is_none")]
    pub extension_options: Option<serde_json::Value>,
    #[serde(rename = "marketplaceType", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_type: Option<resource_type_registration_properties::MarketplaceType>,
    #[serde(rename = "swaggerSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub swagger_specifications: Vec<SwaggerSpecification>,
    #[serde(rename = "allowedUnauthorizedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_unauthorized_actions: Vec<String>,
    #[serde(rename = "authorizationActionMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_action_mappings: Vec<AuthorizationActionMapping>,
    #[serde(rename = "linkedAccessChecks", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_access_checks: Vec<LinkedAccessCheck>,
    #[serde(rename = "defaultApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_api_version: Option<String>,
    #[serde(rename = "loggingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub logging_rules: Vec<LoggingRule>,
    #[serde(rename = "throttlingRules", default, skip_serializing_if = "Vec::is_empty")]
    pub throttling_rules: Vec<ThrottlingRule>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(rename = "featuresRule", default, skip_serializing_if = "Option::is_none")]
    pub features_rule: Option<serde_json::Value>,
    #[serde(rename = "enableAsyncOperation", default, skip_serializing_if = "Option::is_none")]
    pub enable_async_operation: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "enableThirdPartyS2S", default, skip_serializing_if = "Option::is_none")]
    pub enable_third_party_s2s: Option<bool>,
    #[serde(
        rename = "subscriptionLifecycleNotificationSpecifications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_lifecycle_notification_specifications: Option<serde_json::Value>,
    #[serde(rename = "isPureProxy", default, skip_serializing_if = "Option::is_none")]
    pub is_pure_proxy: Option<bool>,
    #[serde(rename = "identityManagement", default, skip_serializing_if = "Option::is_none")]
    pub identity_management: Option<serde_json::Value>,
    #[serde(rename = "checkNameAvailabilitySpecifications", default, skip_serializing_if = "Option::is_none")]
    pub check_name_availability_specifications: Option<serde_json::Value>,
    #[serde(rename = "disallowedActionVerbs", default, skip_serializing_if = "Vec::is_empty")]
    pub disallowed_action_verbs: Vec<String>,
    #[serde(rename = "serviceTreeInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub service_tree_infos: Vec<ServiceTreeInfo>,
    #[serde(rename = "requestHeaderOptions", default, skip_serializing_if = "Option::is_none")]
    pub request_header_options: Option<serde_json::Value>,
    #[serde(rename = "subscriptionStateRules", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_state_rules: Vec<SubscriptionStateRule>,
    #[serde(rename = "templateDeploymentOptions", default, skip_serializing_if = "Option::is_none")]
    pub template_deployment_options: Option<serde_json::Value>,
    #[serde(rename = "extendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub extended_locations: Vec<ExtendedLocationOptions>,
    #[serde(rename = "resourceMovePolicy", default, skip_serializing_if = "Option::is_none")]
    pub resource_move_policy: Option<serde_json::Value>,
    #[serde(rename = "resourceDeletionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub resource_deletion_policy: Option<resource_type_registration_properties::ResourceDeletionPolicy>,
    #[serde(rename = "resourceConcurrencyControlOptions", default, skip_serializing_if = "Option::is_none")]
    pub resource_concurrency_control_options: Option<serde_json::Value>,
    #[serde(rename = "resourceGraphConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub resource_graph_configuration: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management: Option<serde_json::Value>,
    #[serde(rename = "openApiConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub open_api_configuration: Option<OpenApiConfiguration>,
}
impl ResourceTypeRegistrationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_type_registration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingType")]
    pub enum RoutingType {
        Default,
        ProxyOnly,
        HostBased,
        Extension,
        Tenant,
        Fanout,
        LocationBased,
        Failover,
        CascadeExtension,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("RoutingType", 0u32, "Default"),
                Self::ProxyOnly => serializer.serialize_unit_variant("RoutingType", 1u32, "ProxyOnly"),
                Self::HostBased => serializer.serialize_unit_variant("RoutingType", 2u32, "HostBased"),
                Self::Extension => serializer.serialize_unit_variant("RoutingType", 3u32, "Extension"),
                Self::Tenant => serializer.serialize_unit_variant("RoutingType", 4u32, "Tenant"),
                Self::Fanout => serializer.serialize_unit_variant("RoutingType", 5u32, "Fanout"),
                Self::LocationBased => serializer.serialize_unit_variant("RoutingType", 6u32, "LocationBased"),
                Self::Failover => serializer.serialize_unit_variant("RoutingType", 7u32, "Failover"),
                Self::CascadeExtension => serializer.serialize_unit_variant("RoutingType", 8u32, "CascadeExtension"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Regionality")]
    pub enum Regionality {
        NotSpecified,
        Global,
        Regional,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Regionality {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Regionality {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Regionality {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Regionality", 0u32, "NotSpecified"),
                Self::Global => serializer.serialize_unit_variant("Regionality", 1u32, "Global"),
                Self::Regional => serializer.serialize_unit_variant("Regionality", 2u32, "Regional"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MarketplaceType {
        NotSpecified,
        AddOn,
        Bypass,
        Store,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceDeletionPolicy")]
    pub enum ResourceDeletionPolicy {
        NotSpecified,
        CascadeDeleteAll,
        CascadeDeleteProxyOnlyChildren,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceDeletionPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceDeletionPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceDeletionPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ResourceDeletionPolicy", 0u32, "NotSpecified"),
                Self::CascadeDeleteAll => serializer.serialize_unit_variant("ResourceDeletionPolicy", 1u32, "CascadeDeleteAll"),
                Self::CascadeDeleteProxyOnlyChildren => {
                    serializer.serialize_unit_variant("ResourceDeletionPolicy", 2u32, "CascadeDeleteProxyOnlyChildren")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTypeSku {
    #[serde(rename = "skuSettings")]
    pub sku_settings: Vec<SkuSetting>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ResourceTypeSku {
    pub fn new(sku_settings: Vec<SkuSetting>) -> Self {
        Self {
            sku_settings,
            provisioning_state: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RolloutStatusBase {
    #[serde(rename = "completedRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub completed_regions: Vec<String>,
    #[serde(rename = "failedOrSkippedRegions", default, skip_serializing_if = "Option::is_none")]
    pub failed_or_skipped_regions: Option<serde_json::Value>,
}
impl RolloutStatusBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTreeInfo {
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness: Option<service_tree_info::Readiness>,
}
impl ServiceTreeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_tree_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Readiness")]
    pub enum Readiness {
        ClosingDown,
        Deprecated,
        #[serde(rename = "GA")]
        Ga,
        InDevelopment,
        InternalOnly,
        PrivatePreview,
        PublicPreview,
        #[serde(rename = "RemovedFromARM")]
        RemovedFromArm,
        Retired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Readiness {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Readiness {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Readiness {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ClosingDown => serializer.serialize_unit_variant("Readiness", 0u32, "ClosingDown"),
                Self::Deprecated => serializer.serialize_unit_variant("Readiness", 1u32, "Deprecated"),
                Self::Ga => serializer.serialize_unit_variant("Readiness", 2u32, "GA"),
                Self::InDevelopment => serializer.serialize_unit_variant("Readiness", 3u32, "InDevelopment"),
                Self::InternalOnly => serializer.serialize_unit_variant("Readiness", 4u32, "InternalOnly"),
                Self::PrivatePreview => serializer.serialize_unit_variant("Readiness", 5u32, "PrivatePreview"),
                Self::PublicPreview => serializer.serialize_unit_variant("Readiness", 6u32, "PublicPreview"),
                Self::RemovedFromArm => serializer.serialize_unit_variant("Readiness", 7u32, "RemovedFromARM"),
                Self::Retired => serializer.serialize_unit_variant("Readiness", 8u32, "Retired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapability {
    pub name: String,
    pub value: String,
}
impl SkuCapability {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapacity {
    pub minimum: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sku_capacity::ScaleType>,
}
impl SkuCapacity {
    pub fn new(minimum: i32) -> Self {
        Self {
            minimum,
            maximum: None,
            default: None,
            scale_type: None,
        }
    }
}
pub mod sku_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        None,
        Manual,
        Automatic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ScaleType", 0u32, "None"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 2u32, "Automatic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCost {
    #[serde(rename = "meterId")]
    pub meter_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl SkuCost {
    pub fn new(meter_id: String) -> Self {
        Self {
            meter_id,
            quantity: None,
            extended_unit: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuLocationInfo {
    pub location: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[serde(rename = "zoneDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_details: Vec<SkuZoneDetail>,
    #[serde(rename = "extendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub extended_locations: Vec<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sku_location_info::Type>,
}
impl SkuLocationInfo {
    pub fn new(location: String) -> Self {
        Self {
            location,
            zones: Vec::new(),
            zone_details: Vec::new(),
            extended_locations: Vec::new(),
            type_: None,
        }
    }
}
pub mod sku_location_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        EdgeZone,
        ArcZone,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SkuResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuResourceArrayResponseWithContinuation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuResource>,
    #[doc = "The URL to get to the next set of results, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SkuResourceArrayResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SkuResourceArrayResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuSetting {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfo>,
    #[serde(rename = "requiredQuotaIds", default, skip_serializing_if = "Vec::is_empty")]
    pub required_quota_ids: Vec<String>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
}
impl SkuSetting {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            kind: None,
            locations: Vec::new(),
            location_info: Vec::new(),
            required_quota_ids: Vec::new(),
            required_features: Vec::new(),
            capacity: None,
            costs: Vec::new(),
            capabilities: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuZoneDetail {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
}
impl SkuZoneDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionLifecycleNotificationSpecifications {
    #[serde(rename = "subscriptionStateOverrideActions", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_state_override_actions: Vec<SubscriptionStateOverrideAction>,
    #[serde(rename = "softDeleteTTL", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_ttl: Option<String>,
}
impl SubscriptionLifecycleNotificationSpecifications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionStateOverrideAction {
    pub state: subscription_state_override_action::State,
    pub action: subscription_state_override_action::Action,
}
impl SubscriptionStateOverrideAction {
    pub fn new(state: subscription_state_override_action::State, action: subscription_state_override_action::Action) -> Self {
        Self { state, action }
    }
}
pub mod subscription_state_override_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
        WarnedToRegistered,
        WarnedToSuspended,
        WarnedToDeleted,
        WarnedToUnregistered,
        SuspendedToRegistered,
        SuspendedToWarned,
        SuspendedToDeleted,
        SuspendedToUnregistered,
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
                Self::Registered => serializer.serialize_unit_variant("State", 0u32, "Registered"),
                Self::Unregistered => serializer.serialize_unit_variant("State", 1u32, "Unregistered"),
                Self::Warned => serializer.serialize_unit_variant("State", 2u32, "Warned"),
                Self::Suspended => serializer.serialize_unit_variant("State", 3u32, "Suspended"),
                Self::Deleted => serializer.serialize_unit_variant("State", 4u32, "Deleted"),
                Self::WarnedToRegistered => serializer.serialize_unit_variant("State", 5u32, "WarnedToRegistered"),
                Self::WarnedToSuspended => serializer.serialize_unit_variant("State", 6u32, "WarnedToSuspended"),
                Self::WarnedToDeleted => serializer.serialize_unit_variant("State", 7u32, "WarnedToDeleted"),
                Self::WarnedToUnregistered => serializer.serialize_unit_variant("State", 8u32, "WarnedToUnregistered"),
                Self::SuspendedToRegistered => serializer.serialize_unit_variant("State", 9u32, "SuspendedToRegistered"),
                Self::SuspendedToWarned => serializer.serialize_unit_variant("State", 10u32, "SuspendedToWarned"),
                Self::SuspendedToDeleted => serializer.serialize_unit_variant("State", 11u32, "SuspendedToDeleted"),
                Self::SuspendedToUnregistered => serializer.serialize_unit_variant("State", 12u32, "SuspendedToUnregistered"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        NotDefined,
        DeleteAllResources,
        SoftDeleteAllResources,
        NoOp,
        BillingCancellation,
        UndoSoftDelete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotDefined => serializer.serialize_unit_variant("Action", 0u32, "NotDefined"),
                Self::DeleteAllResources => serializer.serialize_unit_variant("Action", 1u32, "DeleteAllResources"),
                Self::SoftDeleteAllResources => serializer.serialize_unit_variant("Action", 2u32, "SoftDeleteAllResources"),
                Self::NoOp => serializer.serialize_unit_variant("Action", 3u32, "NoOp"),
                Self::BillingCancellation => serializer.serialize_unit_variant("Action", 4u32, "BillingCancellation"),
                Self::UndoSoftDelete => serializer.serialize_unit_variant("Action", 5u32, "UndoSoftDelete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionStateRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_state_rule::State>,
    #[serde(rename = "allowedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_actions: Vec<String>,
}
impl SubscriptionStateRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_state_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        NotDefined,
        Enabled,
        Warned,
        PastDue,
        Disabled,
        Deleted,
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
                Self::NotDefined => serializer.serialize_unit_variant("State", 0u32, "NotDefined"),
                Self::Enabled => serializer.serialize_unit_variant("State", 1u32, "Enabled"),
                Self::Warned => serializer.serialize_unit_variant("State", 2u32, "Warned"),
                Self::PastDue => serializer.serialize_unit_variant("State", 3u32, "PastDue"),
                Self::Disabled => serializer.serialize_unit_variant("State", 4u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("State", 5u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerSpecification {
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[serde(rename = "swaggerSpecFolderUri", default, skip_serializing_if = "Option::is_none")]
    pub swagger_spec_folder_uri: Option<String>,
}
impl SwaggerSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemplateDeploymentOptions {
    #[serde(rename = "preflightSupported", default, skip_serializing_if = "Option::is_none")]
    pub preflight_supported: Option<bool>,
    #[serde(rename = "preflightOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub preflight_options: Vec<String>,
}
impl TemplateDeploymentOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateDeploymentPolicy {
    pub capabilities: template_deployment_policy::Capabilities,
    #[serde(rename = "preflightOptions")]
    pub preflight_options: template_deployment_policy::PreflightOptions,
}
impl TemplateDeploymentPolicy {
    pub fn new(
        capabilities: template_deployment_policy::Capabilities,
        preflight_options: template_deployment_policy::PreflightOptions,
    ) -> Self {
        Self {
            capabilities,
            preflight_options,
        }
    }
}
pub mod template_deployment_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Capabilities")]
    pub enum Capabilities {
        Default,
        Preflight,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Capabilities {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Capabilities {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Capabilities {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("Capabilities", 0u32, "Default"),
                Self::Preflight => serializer.serialize_unit_variant("Capabilities", 1u32, "Preflight"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreflightOptions")]
    pub enum PreflightOptions {
        None,
        ValidationRequests,
        DeploymentRequests,
        TestOnly,
        RegisteredOnly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreflightOptions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreflightOptions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreflightOptions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PreflightOptions", 0u32, "None"),
                Self::ValidationRequests => serializer.serialize_unit_variant("PreflightOptions", 1u32, "ValidationRequests"),
                Self::DeploymentRequests => serializer.serialize_unit_variant("PreflightOptions", 2u32, "DeploymentRequests"),
                Self::TestOnly => serializer.serialize_unit_variant("PreflightOptions", 3u32, "TestOnly"),
                Self::RegisteredOnly => serializer.serialize_unit_variant("PreflightOptions", 4u32, "RegisteredOnly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThirdPartyProviderAuthorization {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<LightHouseAuthorization>,
    #[serde(rename = "managedByTenantId", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_tenant_id: Option<String>,
}
impl ThirdPartyProviderAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThrottlingMetric {
    #[serde(rename = "type")]
    pub type_: throttling_metric::Type,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}
impl ThrottlingMetric {
    pub fn new(type_: throttling_metric::Type, limit: i64) -> Self {
        Self {
            type_,
            limit,
            interval: None,
        }
    }
}
pub mod throttling_metric {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        NotSpecified,
        NumberOfRequests,
        NumberOfResources,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Type", 0u32, "NotSpecified"),
                Self::NumberOfRequests => serializer.serialize_unit_variant("Type", 1u32, "NumberOfRequests"),
                Self::NumberOfResources => serializer.serialize_unit_variant("Type", 2u32, "NumberOfResources"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThrottlingRule {
    pub action: String,
    pub metrics: Vec<ThrottlingMetric>,
    #[serde(rename = "requiredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
}
impl ThrottlingRule {
    pub fn new(action: String, metrics: Vec<ThrottlingMetric>) -> Self {
        Self {
            action,
            metrics,
            required_features: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficRegionRolloutConfiguration {
    #[serde(flatten)]
    pub traffic_regions: TrafficRegions,
    #[serde(rename = "waitDuration", default, skip_serializing_if = "Option::is_none")]
    pub wait_duration: Option<String>,
}
impl TrafficRegionRolloutConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficRegions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
}
impl TrafficRegions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypedErrorInfo {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl TypedErrorInfo {
    pub fn new(type_: String) -> Self {
        Self { type_, info: None }
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
