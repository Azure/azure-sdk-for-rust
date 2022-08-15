#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Authorization Profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationProfile {
    #[doc = "The requested time"]
    #[serde(rename = "requestedTime", with = "azure_core::date::rfc3339::option")]
    pub requested_time: Option<time::OffsetDateTime>,
    #[doc = "The requester"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<String>,
    #[doc = "The requester object id"]
    #[serde(rename = "requesterObjectId", default, skip_serializing_if = "Option::is_none")]
    pub requester_object_id: Option<String>,
    #[doc = "The approved time"]
    #[serde(rename = "approvedTime", with = "azure_core::date::rfc3339::option")]
    pub approved_time: Option<time::OffsetDateTime>,
    #[doc = "The approver"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<String>,
}
impl AuthorizationProfile {
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
#[doc = "Error response indicates that the service is not able to process the incoming request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "List of previewed features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureOperationsListResult {
    #[doc = "The array of features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FeatureResult>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FeatureOperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FeatureOperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureProperties {
    #[doc = "The registration state of the feature for the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl FeatureProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Previewed feature information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureResult {
    #[doc = "The name of the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Information about feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FeatureProperties>,
    #[doc = "The resource ID of the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource type of the feature."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl FeatureResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.Features operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Features"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Microsoft.Features operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.Features operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "An Azure proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Azure resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription feature registration details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionFeatureRegistration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<subscription_feature_registration::Properties>,
}
impl SubscriptionFeatureRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_feature_registration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The tenantId."]
        #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
        #[doc = "The subscriptionId."]
        #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
        pub subscription_id: Option<String>,
        #[doc = "The featureName."]
        #[serde(rename = "featureName", default, skip_serializing_if = "Option::is_none")]
        pub feature_name: Option<String>,
        #[doc = "The featureDisplayName."]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "The providerNamespace."]
        #[serde(rename = "providerNamespace", default, skip_serializing_if = "Option::is_none")]
        pub provider_namespace: Option<String>,
        #[doc = "The state."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<properties::State>,
        #[doc = "Authorization Profile"]
        #[serde(rename = "authorizationProfile", default, skip_serializing_if = "Option::is_none")]
        pub authorization_profile: Option<AuthorizationProfile>,
        #[doc = "Key-value pairs for meta data."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
        #[doc = "The feature release date."]
        #[serde(rename = "releaseDate", with = "azure_core::date::rfc3339::option")]
        pub release_date: Option<time::OffsetDateTime>,
        #[doc = "The feature registration date."]
        #[serde(rename = "registrationDate", with = "azure_core::date::rfc3339::option")]
        pub registration_date: Option<time::OffsetDateTime>,
        #[doc = "The feature documentation link."]
        #[serde(rename = "documentationLink", default, skip_serializing_if = "Option::is_none")]
        pub documentation_link: Option<String>,
        #[doc = "The feature approval type."]
        #[serde(rename = "approvalType", default, skip_serializing_if = "Option::is_none")]
        pub approval_type: Option<properties::ApprovalType>,
        #[doc = "Indicates whether feature should be displayed in Portal."]
        #[serde(rename = "shouldFeatureDisplayInPortal", default, skip_serializing_if = "Option::is_none")]
        pub should_feature_display_in_portal: Option<bool>,
        #[doc = "The feature description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The state."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "State")]
        pub enum State {
            NotSpecified,
            NotRegistered,
            Pending,
            Registering,
            Registered,
            Unregistering,
            Unregistered,
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
                    Self::NotSpecified => serializer.serialize_unit_variant("State", 0u32, "NotSpecified"),
                    Self::NotRegistered => serializer.serialize_unit_variant("State", 1u32, "NotRegistered"),
                    Self::Pending => serializer.serialize_unit_variant("State", 2u32, "Pending"),
                    Self::Registering => serializer.serialize_unit_variant("State", 3u32, "Registering"),
                    Self::Registered => serializer.serialize_unit_variant("State", 4u32, "Registered"),
                    Self::Unregistering => serializer.serialize_unit_variant("State", 5u32, "Unregistering"),
                    Self::Unregistered => serializer.serialize_unit_variant("State", 6u32, "Unregistered"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "The feature approval type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ApprovalType")]
        pub enum ApprovalType {
            NotSpecified,
            ApprovalRequired,
            AutoApproval,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for ApprovalType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for ApprovalType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for ApprovalType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::NotSpecified => serializer.serialize_unit_variant("ApprovalType", 0u32, "NotSpecified"),
                    Self::ApprovalRequired => serializer.serialize_unit_variant("ApprovalType", 1u32, "ApprovalRequired"),
                    Self::AutoApproval => serializer.serialize_unit_variant("ApprovalType", 2u32, "AutoApproval"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "The list of subscription feature registrations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionFeatureRegistrationList {
    #[doc = "The link used to get the next page of subscription feature registrations list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of subscription feature registrations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionFeatureRegistration>,
}
impl azure_core::Continuable for SubscriptionFeatureRegistrationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SubscriptionFeatureRegistrationList {
    pub fn new() -> Self {
        Self::default()
    }
}
