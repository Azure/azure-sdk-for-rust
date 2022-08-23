#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Terms properties for Marketplace and Confluent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentAgreementProperties {
    #[doc = "Publisher identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Product identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Plan identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[doc = "Link to HTML with Microsoft and Publisher terms."]
    #[serde(rename = "licenseTextLink", default, skip_serializing_if = "Option::is_none")]
    pub license_text_link: Option<String>,
    #[doc = "Link to the privacy policy of the publisher."]
    #[serde(rename = "privacyPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_link: Option<String>,
    #[doc = "Date and time in UTC of when the terms were accepted. This is empty if Accepted is false."]
    #[serde(rename = "retrieveDatetime", default, with = "azure_core::date::rfc3339::option")]
    pub retrieve_datetime: Option<time::OffsetDateTime>,
    #[doc = "Terms signature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "If any version of the terms have been accepted, otherwise false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}
impl ConfluentAgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Agreement Terms definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentAgreementResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the agreement."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Terms properties for Marketplace and Confluent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfluentAgreementProperties>,
}
impl ConfluentAgreementResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentAgreementResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfluentAgreementResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfluentAgreementResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConfluentAgreementResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response body of Error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error detail"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Confluent Offer detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferDetail {
    #[doc = "Publisher Id"]
    #[serde(rename = "publisherId")]
    pub publisher_id: String,
    #[doc = "Offer Id"]
    pub id: String,
    #[doc = "Offer Plan Id"]
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[doc = "Offer Plan Name"]
    #[serde(rename = "planName")]
    pub plan_name: String,
    #[doc = "Offer Plan Term unit"]
    #[serde(rename = "termUnit")]
    pub term_unit: String,
    #[doc = "SaaS Offer Status for confluent RP"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SaaSOfferStatus>,
}
impl OfferDetail {
    pub fn new(publisher_id: String, id: String, plan_id: String, plan_name: String, term_unit: String) -> Self {
        Self {
            publisher_id,
            id,
            plan_id,
            plan_name,
            term_unit,
            status: None,
        }
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft.Confluent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Type on which the operation is performed, e.g., 'clusters'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Write confluent'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of GET request to list Confluent operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Confluent operations supported by the Microsoft.Confluent provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
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
#[doc = "An Confluent REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Organization resource property"]
    pub properties: OrganizationResourceProperties,
    #[doc = "Organization resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Location of Organization resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl OrganizationResource {
    pub fn new(properties: OrganizationResourceProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            system_data: None,
            properties,
            tags: None,
            location: None,
        }
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationResourceListResult {
    #[doc = "Result of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OrganizationResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrganizationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OrganizationResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Organization resource property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResourceProperties {
    #[doc = "The creation time of the resource."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Provision states for confluent RP"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Id of the Confluent organization."]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "SSO url for the Confluent organization."]
    #[serde(rename = "ssoUrl", default, skip_serializing_if = "Option::is_none")]
    pub sso_url: Option<String>,
    #[doc = "Confluent Offer detail"]
    #[serde(rename = "offerDetail")]
    pub offer_detail: OfferDetail,
    #[doc = "Subscriber detail"]
    #[serde(rename = "userDetail")]
    pub user_detail: UserDetail,
}
impl OrganizationResourceProperties {
    pub fn new(offer_detail: OfferDetail, user_detail: UserDetail) -> Self {
        Self {
            created_time: None,
            provisioning_state: None,
            organization_id: None,
            sso_url: None,
            offer_detail,
            user_detail,
        }
    }
}
#[doc = "Organization Resource update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationResourceUpdate {
    #[doc = "ARM resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OrganizationResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provision states for confluent RP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Default error response for resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDefaultErrorResponse {
    #[doc = "Response body of Error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
impl azure_core::Continuable for ResourceProviderDefaultErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ResourceProviderDefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SaaS Offer Status for confluent RP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SaaSOfferStatus")]
pub enum SaaSOfferStatus {
    Started,
    PendingFulfillmentStart,
    InProgress,
    Subscribed,
    Suspended,
    Reinstated,
    Succeeded,
    Failed,
    Unsubscribed,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SaaSOfferStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SaaSOfferStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SaaSOfferStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Started => serializer.serialize_unit_variant("SaaSOfferStatus", 0u32, "Started"),
            Self::PendingFulfillmentStart => serializer.serialize_unit_variant("SaaSOfferStatus", 1u32, "PendingFulfillmentStart"),
            Self::InProgress => serializer.serialize_unit_variant("SaaSOfferStatus", 2u32, "InProgress"),
            Self::Subscribed => serializer.serialize_unit_variant("SaaSOfferStatus", 3u32, "Subscribed"),
            Self::Suspended => serializer.serialize_unit_variant("SaaSOfferStatus", 4u32, "Suspended"),
            Self::Reinstated => serializer.serialize_unit_variant("SaaSOfferStatus", 5u32, "Reinstated"),
            Self::Succeeded => serializer.serialize_unit_variant("SaaSOfferStatus", 6u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("SaaSOfferStatus", 7u32, "Failed"),
            Self::Unsubscribed => serializer.serialize_unit_variant("SaaSOfferStatus", 8u32, "Unsubscribed"),
            Self::Updating => serializer.serialize_unit_variant("SaaSOfferStatus", 9u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Subscriber detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDetail {
    #[doc = "First name"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Email address"]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl UserDetail {
    pub fn new(email_address: String) -> Self {
        Self {
            first_name: None,
            last_name: None,
            email_address,
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
