#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Request content object, in the use of Approve or Deny a Lockbox request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Approval {
    #[doc = "Approval decision to the Lockbox request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<approval::Status>,
    #[doc = "Reason of the decision"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl Approval {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval {
    use super::*;
    #[doc = "Approval decision to the Lockbox request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Approve,
        Deny,
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
                Self::Approve => serializer.serialize_unit_variant("Status", 0u32, "Approve"),
                Self::Deny => serializer.serialize_unit_variant("Status", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An error additional info for the Lockbox service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The type of error info."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Additional information about the request that is in error state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<error_additional_info::Info>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_additional_info {
    use super::*;
    #[doc = "Additional information about the request that is in error state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Info {
        #[doc = "The status of the request."]
        #[serde(rename = "currentStatus", default, skip_serializing_if = "Option::is_none")]
        pub current_status: Option<LockboxRequestStatus>,
    }
    impl Info {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An error response body from the Lockbox service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of error details about the error."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Lockbox service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "An error response body from the Lockbox service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
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
#[doc = "A Lockbox request response object, containing all information associated with the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LockboxRequestResponse {
    #[doc = "The Arm resource id of the Lockbox request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Lockbox request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the Lockbox request."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties that are associated with a lockbox request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LockboxRequestResponseProperties>,
}
impl LockboxRequestResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with a lockbox request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LockboxRequestResponseProperties {
    #[doc = "The Lockbox request ID."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The justification of the requestor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "The status of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LockboxRequestStatus>,
    #[doc = "The creation time of the request."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The expiration time of the request."]
    #[serde(rename = "expirationDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date_time: Option<time::OffsetDateTime>,
    #[doc = "The duration of the request in hours."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "A list of resource IDs associated with the Lockbox request separated by ','."]
    #[serde(rename = "resourceIds", default, skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<String>,
    #[doc = "The resource type of the requested resources."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The id of the support request associated."]
    #[serde(rename = "supportRequest", default, skip_serializing_if = "Option::is_none")]
    pub support_request: Option<String>,
    #[doc = "The url of the support case."]
    #[serde(rename = "supportCaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub support_case_url: Option<String>,
    #[doc = "The support case system that was used to initiate the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workitemsource: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Access level for requested resource"]
    #[serde(rename = "accessLevel", default, skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
}
impl LockboxRequestResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LockboxRequestStatus")]
pub enum LockboxRequestStatus {
    Initializing,
    Pending,
    Approving,
    Denying,
    Approved,
    Denied,
    Expired,
    Revoking,
    Revoked,
    Error,
    Unknown,
    Completed,
    Completing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LockboxRequestStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LockboxRequestStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LockboxRequestStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Initializing => serializer.serialize_unit_variant("LockboxRequestStatus", 0u32, "Initializing"),
            Self::Pending => serializer.serialize_unit_variant("LockboxRequestStatus", 1u32, "Pending"),
            Self::Approving => serializer.serialize_unit_variant("LockboxRequestStatus", 2u32, "Approving"),
            Self::Denying => serializer.serialize_unit_variant("LockboxRequestStatus", 3u32, "Denying"),
            Self::Approved => serializer.serialize_unit_variant("LockboxRequestStatus", 4u32, "Approved"),
            Self::Denied => serializer.serialize_unit_variant("LockboxRequestStatus", 5u32, "Denied"),
            Self::Expired => serializer.serialize_unit_variant("LockboxRequestStatus", 6u32, "Expired"),
            Self::Revoking => serializer.serialize_unit_variant("LockboxRequestStatus", 7u32, "Revoking"),
            Self::Revoked => serializer.serialize_unit_variant("LockboxRequestStatus", 8u32, "Revoked"),
            Self::Error => serializer.serialize_unit_variant("LockboxRequestStatus", 9u32, "Error"),
            Self::Unknown => serializer.serialize_unit_variant("LockboxRequestStatus", 10u32, "Unknown"),
            Self::Completed => serializer.serialize_unit_variant("LockboxRequestStatus", 11u32, "Completed"),
            Self::Completing => serializer.serialize_unit_variant("LockboxRequestStatus", 12u32, "Completing"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Operation result model for ARM RP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Gets or sets action name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets a value indicating whether it is a data plane action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Gets or sets properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[doc = "Gets or sets origin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Contains the localized display information for this particular operation / action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly form of the resource type related to this action/operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The localized friendly name for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The localized friendly description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Customer Lockbox operations. It contains a list of operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Customer Lockbox operations supported by the Microsoft.StreamAnalytics resource provider."]
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
#[doc = "Object containing a list of streaming jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestListResult {
    #[doc = "A list of Lockbox requests. Populated by a 'List' operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LockboxRequestResponse>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RequestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RequestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "TenantOptIn Response object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantOptInResponse {
    #[doc = "True if tenant is opted in, false otherwise "]
    #[serde(rename = "isOptedIn", default, skip_serializing_if = "Option::is_none")]
    pub is_opted_in: Option<bool>,
}
impl TenantOptInResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
