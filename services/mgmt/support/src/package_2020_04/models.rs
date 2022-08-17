#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Input of CheckNameAvailability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    #[doc = "The resource name to validate."]
    pub name: String,
    #[doc = "The type of resource."]
    #[serde(rename = "type")]
    pub type_: check_name_availability_input::Type,
}
impl CheckNameAvailabilityInput {
    pub fn new(name: String, type_: check_name_availability_input::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_input {
    use super::*;
    #[doc = "The type of resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Support/supportTickets")]
        MicrosoftSupportSupportTickets,
        #[serde(rename = "Microsoft.Support/communications")]
        MicrosoftSupportCommunications,
    }
}
#[doc = "Output of check name availability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityOutput {
    #[doc = "Indicates whether the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed error message describing why the name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that represents a Communication resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationDetails {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource 'Microsoft.Support/communications'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes the properties of a communication resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommunicationDetailsProperties>,
}
impl CommunicationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a communication resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationDetailsProperties {
    #[doc = "Communication type."]
    #[serde(rename = "communicationType", default, skip_serializing_if = "Option::is_none")]
    pub communication_type: Option<communication_details_properties::CommunicationType>,
    #[doc = "Direction of communication."]
    #[serde(rename = "communicationDirection", default, skip_serializing_if = "Option::is_none")]
    pub communication_direction: Option<communication_details_properties::CommunicationDirection>,
    #[doc = "Email address of the sender. This property is required if called by a service principal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[doc = "Subject of the communication."]
    pub subject: String,
    #[doc = "Body of the communication."]
    pub body: String,
    #[doc = "Time in UTC (ISO 8601 format) when the communication was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
}
impl CommunicationDetailsProperties {
    pub fn new(subject: String, body: String) -> Self {
        Self {
            communication_type: None,
            communication_direction: None,
            sender: None,
            subject,
            body,
            created_date: None,
        }
    }
}
pub mod communication_details_properties {
    use super::*;
    #[doc = "Communication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CommunicationType")]
    pub enum CommunicationType {
        #[serde(rename = "web")]
        Web,
        #[serde(rename = "phone")]
        Phone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CommunicationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CommunicationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CommunicationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Web => serializer.serialize_unit_variant("CommunicationType", 0u32, "web"),
                Self::Phone => serializer.serialize_unit_variant("CommunicationType", 1u32, "phone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Direction of communication."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CommunicationDirection")]
    pub enum CommunicationDirection {
        #[serde(rename = "inbound")]
        Inbound,
        #[serde(rename = "outbound")]
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CommunicationDirection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CommunicationDirection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CommunicationDirection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("CommunicationDirection", 0u32, "inbound"),
                Self::Outbound => serializer.serialize_unit_variant("CommunicationDirection", 1u32, "outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of Communication resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationsListResult {
    #[doc = "List of Communication resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommunicationDetails>,
    #[doc = "The URI to fetch the next page of Communication resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CommunicationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommunicationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contact information associated with the support ticket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfile {
    #[doc = "First name."]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[doc = "Last name."]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[doc = "Preferred contact method."]
    #[serde(rename = "preferredContactMethod")]
    pub preferred_contact_method: contact_profile::PreferredContactMethod,
    #[doc = "Primary email address."]
    #[serde(rename = "primaryEmailAddress")]
    pub primary_email_address: String,
    #[doc = "Additional email addresses listed will be copied on any correspondence about the support ticket."]
    #[serde(rename = "additionalEmailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_email_addresses: Vec<String>,
    #[doc = "Phone number. This is required if preferred contact method is phone."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "Time zone of the user. This is the name of the time zone from [Microsoft Time Zone Index Values](https://support.microsoft.com/help/973627/microsoft-time-zone-index-values)."]
    #[serde(rename = "preferredTimeZone")]
    pub preferred_time_zone: String,
    #[doc = "Country of the user. This is the ISO 3166-1 alpha-3 code."]
    pub country: String,
    #[doc = "Preferred language of support from Azure. Support languages vary based on the severity you choose for your support ticket. Learn more at [Azure Severity and responsiveness](https://azure.microsoft.com/support/plans/response). Use the standard language-country code. Valid values are 'en-us' for English, 'zh-hans' for Chinese, 'es-es' for Spanish, 'fr-fr' for French, 'ja-jp' for Japanese, 'ko-kr' for Korean, 'ru-ru' for Russian, 'pt-br' for Portuguese, 'it-it' for Italian, 'zh-tw' for Chinese and 'de-de' for German."]
    #[serde(rename = "preferredSupportLanguage")]
    pub preferred_support_language: String,
}
impl ContactProfile {
    pub fn new(
        first_name: String,
        last_name: String,
        preferred_contact_method: contact_profile::PreferredContactMethod,
        primary_email_address: String,
        preferred_time_zone: String,
        country: String,
        preferred_support_language: String,
    ) -> Self {
        Self {
            first_name,
            last_name,
            preferred_contact_method,
            primary_email_address,
            additional_email_addresses: Vec::new(),
            phone_number: None,
            preferred_time_zone,
            country,
            preferred_support_language,
        }
    }
}
pub mod contact_profile {
    use super::*;
    #[doc = "Preferred contact method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredContactMethod")]
    pub enum PreferredContactMethod {
        #[serde(rename = "email")]
        Email,
        #[serde(rename = "phone")]
        Phone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredContactMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredContactMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredContactMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Email => serializer.serialize_unit_variant("PreferredContactMethod", 0u32, "email"),
                Self::Phone => serializer.serialize_unit_variant("PreferredContactMethod", 1u32, "phone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The API error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExceptionResponse {
    #[doc = "The API error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}
impl azure_core::Continuable for ExceptionResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ExceptionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Microsoft Support resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that describes the operation."]
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
    #[doc = "The object that describes the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "The action that users can perform, based on their permission level."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Service provider: Microsoft Support."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The list of operations supported by Microsoft Support resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "The list of operations supported by Microsoft Support resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ProblemClassification resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProblemClassification {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource 'Microsoft.Support/problemClassification'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Details about a problem classification available for an Azure service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProblemClassificationProperties>,
}
impl ProblemClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about a problem classification available for an Azure service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProblemClassificationProperties {
    #[doc = "Localized name of problem classification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ProblemClassificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of ProblemClassification resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProblemClassificationsListResult {
    #[doc = "List of ProblemClassification resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProblemClassification>,
}
impl azure_core::Continuable for ProblemClassificationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ProblemClassificationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This property is required for providing the region and new quota limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaChangeRequest {
    #[doc = "Region for which the quota increase request is being made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Payload of the quota increase request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}
impl QuotaChangeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional set of information required for quota increase support ticket for certain quota types, e.g.: Virtual machine cores. Get complete details about Quota payload support request along with examples at [Support quota request](https://aka.ms/supportrpquotarequestpayload)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaTicketDetails {
    #[doc = "Required for certain quota types when there is a sub type, such as Batch, for which you are requesting a quota increase."]
    #[serde(rename = "quotaChangeRequestSubType", default, skip_serializing_if = "Option::is_none")]
    pub quota_change_request_sub_type: Option<String>,
    #[doc = "Quota change request version."]
    #[serde(rename = "quotaChangeRequestVersion", default, skip_serializing_if = "Option::is_none")]
    pub quota_change_request_version: Option<String>,
    #[doc = "This property is required for providing the region and new quota limits."]
    #[serde(rename = "quotaChangeRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub quota_change_requests: Vec<QuotaChangeRequest>,
}
impl QuotaTicketDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that represents a Service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Service {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource 'Microsoft.Support/services'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Details about an Azure service available for support ticket creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceProperties>,
}
impl Service {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The list of error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ServiceErrorDetail>,
}
impl ServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ServiceErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Level Agreement details for a support ticket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceLevelAgreement {
    #[doc = "Time in UTC (ISO 8601 format) when the service level agreement starts."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Time in UTC (ISO 8601 format) when the service level agreement expires."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
    #[doc = "Service Level Agreement in minutes."]
    #[serde(rename = "slaMinutes", default, skip_serializing_if = "Option::is_none")]
    pub sla_minutes: Option<i32>,
}
impl ServiceLevelAgreement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about an Azure service available for support ticket creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProperties {
    #[doc = "Localized name of the Azure service."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "ARM Resource types."]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<String>,
}
impl ServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Service resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesListResult {
    #[doc = "List of Service resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Service>,
}
impl azure_core::Continuable for ServicesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServicesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Support engineer information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportEngineer {
    #[doc = "Email address of the Azure Support engineer assigned to the support ticket."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
impl SupportEngineer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that represents SupportTicketDetails resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportTicketDetails {
    #[doc = "Id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource 'Microsoft.Support/supportTickets'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes the properties of a support ticket."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SupportTicketDetailsProperties>,
}
impl SupportTicketDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a support ticket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportTicketDetailsProperties {
    #[doc = "System generated support ticket Id that is unique."]
    #[serde(rename = "supportTicketId", default, skip_serializing_if = "Option::is_none")]
    pub support_ticket_id: Option<String>,
    #[doc = "Detailed description of the question or issue."]
    pub description: String,
    #[doc = "Each Azure service has its own set of issue categories, also known as problem classification. This parameter is the unique Id for the type of problem you are experiencing."]
    #[serde(rename = "problemClassificationId")]
    pub problem_classification_id: String,
    #[doc = "Localized name of problem classification."]
    #[serde(rename = "problemClassificationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub problem_classification_display_name: Option<String>,
    #[doc = "A value that indicates the urgency of the case, which in turn determines the response time according to the service level agreement of the technical support plan you have with Azure. Note: 'Highest critical impact', also known as the 'Emergency - Severe impact' level in the Azure portal is reserved only for our Premium customers."]
    pub severity: support_ticket_details_properties::Severity,
    #[doc = "Enrollment Id associated with the support ticket."]
    #[serde(rename = "enrollmentId", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_id: Option<String>,
    #[doc = "Indicates if this requires a 24x7 response from Azure."]
    #[serde(rename = "require24X7Response", default, skip_serializing_if = "Option::is_none")]
    pub require24_x7_response: Option<bool>,
    #[doc = "Contact information associated with the support ticket."]
    #[serde(rename = "contactDetails")]
    pub contact_details: ContactProfile,
    #[doc = "Service Level Agreement details for a support ticket."]
    #[serde(rename = "serviceLevelAgreement", default, skip_serializing_if = "Option::is_none")]
    pub service_level_agreement: Option<ServiceLevelAgreement>,
    #[doc = "Support engineer information."]
    #[serde(rename = "supportEngineer", default, skip_serializing_if = "Option::is_none")]
    pub support_engineer: Option<SupportEngineer>,
    #[doc = "Support plan type associated with the support ticket."]
    #[serde(rename = "supportPlanType", default, skip_serializing_if = "Option::is_none")]
    pub support_plan_type: Option<String>,
    #[doc = "Title of the support ticket."]
    pub title: String,
    #[doc = "Time in UTC (ISO 8601 format) when the problem started."]
    #[serde(rename = "problemStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub problem_start_time: Option<time::OffsetDateTime>,
    #[doc = "This is the resource Id of the Azure service resource associated with the support ticket."]
    #[serde(rename = "serviceId")]
    pub service_id: String,
    #[doc = "Localized name of the Azure service."]
    #[serde(rename = "serviceDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub service_display_name: Option<String>,
    #[doc = "Status of the support ticket."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Time in UTC (ISO 8601 format) when the support ticket was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Time in UTC (ISO 8601 format) when the support ticket was last modified."]
    #[serde(rename = "modifiedDate", default, with = "azure_core::date::rfc3339::option")]
    pub modified_date: Option<time::OffsetDateTime>,
    #[doc = "Additional information for technical support ticket."]
    #[serde(rename = "technicalTicketDetails", default, skip_serializing_if = "Option::is_none")]
    pub technical_ticket_details: Option<TechnicalTicketDetails>,
    #[doc = "Additional set of information required for quota increase support ticket for certain quota types, e.g.: Virtual machine cores. Get complete details about Quota payload support request along with examples at [Support quota request](https://aka.ms/supportrpquotarequestpayload)."]
    #[serde(rename = "quotaTicketDetails", default, skip_serializing_if = "Option::is_none")]
    pub quota_ticket_details: Option<QuotaTicketDetails>,
}
impl SupportTicketDetailsProperties {
    pub fn new(
        description: String,
        problem_classification_id: String,
        severity: support_ticket_details_properties::Severity,
        contact_details: ContactProfile,
        title: String,
        service_id: String,
    ) -> Self {
        Self {
            support_ticket_id: None,
            description,
            problem_classification_id,
            problem_classification_display_name: None,
            severity,
            enrollment_id: None,
            require24_x7_response: None,
            contact_details,
            service_level_agreement: None,
            support_engineer: None,
            support_plan_type: None,
            title,
            problem_start_time: None,
            service_id,
            service_display_name: None,
            status: None,
            created_date: None,
            modified_date: None,
            technical_ticket_details: None,
            quota_ticket_details: None,
        }
    }
}
pub mod support_ticket_details_properties {
    use super::*;
    #[doc = "A value that indicates the urgency of the case, which in turn determines the response time according to the service level agreement of the technical support plan you have with Azure. Note: 'Highest critical impact', also known as the 'Emergency - Severe impact' level in the Azure portal is reserved only for our Premium customers."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        #[serde(rename = "minimal")]
        Minimal,
        #[serde(rename = "moderate")]
        Moderate,
        #[serde(rename = "critical")]
        Critical,
        #[serde(rename = "highestcriticalimpact")]
        Highestcriticalimpact,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Minimal => serializer.serialize_unit_variant("Severity", 0u32, "minimal"),
                Self::Moderate => serializer.serialize_unit_variant("Severity", 1u32, "moderate"),
                Self::Critical => serializer.serialize_unit_variant("Severity", 2u32, "critical"),
                Self::Highestcriticalimpact => serializer.serialize_unit_variant("Severity", 3u32, "highestcriticalimpact"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Object that represents a collection of SupportTicket resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportTicketsListResult {
    #[doc = "List of SupportTicket resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SupportTicketDetails>,
    #[doc = "The URI to fetch the next page of SupportTicket resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SupportTicketsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SupportTicketsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional information for technical support ticket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TechnicalTicketDetails {
    #[doc = "This is the resource Id of the Azure service resource (For example: A virtual machine resource or an HDInsight resource) for which the support ticket is created."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl TechnicalTicketDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contact information associated with the support ticket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateContactProfile {
    #[doc = "First name."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Preferred contact method."]
    #[serde(rename = "preferredContactMethod", default, skip_serializing_if = "Option::is_none")]
    pub preferred_contact_method: Option<update_contact_profile::PreferredContactMethod>,
    #[doc = "Primary email address."]
    #[serde(rename = "primaryEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub primary_email_address: Option<String>,
    #[doc = "Email addresses listed will be copied on any correspondence about the support ticket."]
    #[serde(rename = "additionalEmailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_email_addresses: Vec<String>,
    #[doc = "Phone number. This is required if preferred contact method is phone."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "Time zone of the user. This is the name of the time zone from [Microsoft Time Zone Index Values](https://support.microsoft.com/help/973627/microsoft-time-zone-index-values)."]
    #[serde(rename = "preferredTimeZone", default, skip_serializing_if = "Option::is_none")]
    pub preferred_time_zone: Option<String>,
    #[doc = "Country of the user. This is the ISO 3166-1 alpha-3 code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Preferred language of support from Azure. Support languages vary based on the severity you choose for your support ticket. Learn more at [Azure Severity and responsiveness](https://azure.microsoft.com/support/plans/response/). Use the standard language-country code. Valid values are 'en-us' for English, 'zh-hans' for Chinese, 'es-es' for Spanish, 'fr-fr' for French, 'ja-jp' for Japanese, 'ko-kr' for Korean, 'ru-ru' for Russian, 'pt-br' for Portuguese, 'it-it' for Italian, 'zh-tw' for Chinese and 'de-de' for German."]
    #[serde(rename = "preferredSupportLanguage", default, skip_serializing_if = "Option::is_none")]
    pub preferred_support_language: Option<String>,
}
impl UpdateContactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_contact_profile {
    use super::*;
    #[doc = "Preferred contact method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredContactMethod")]
    pub enum PreferredContactMethod {
        #[serde(rename = "email")]
        Email,
        #[serde(rename = "phone")]
        Phone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredContactMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredContactMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredContactMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Email => serializer.serialize_unit_variant("PreferredContactMethod", 0u32, "email"),
                Self::Phone => serializer.serialize_unit_variant("PreferredContactMethod", 1u32, "phone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Updates severity, ticket status, and contact details in the support ticket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSupportTicket {
    #[doc = "Severity level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<update_support_ticket::Severity>,
    #[doc = "Status to be updated on the ticket."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update_support_ticket::Status>,
    #[doc = "Contact information associated with the support ticket."]
    #[serde(rename = "contactDetails", default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<UpdateContactProfile>,
}
impl UpdateSupportTicket {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_support_ticket {
    use super::*;
    #[doc = "Severity level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        #[serde(rename = "minimal")]
        Minimal,
        #[serde(rename = "moderate")]
        Moderate,
        #[serde(rename = "critical")]
        Critical,
        #[serde(rename = "highestcriticalimpact")]
        Highestcriticalimpact,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Minimal => serializer.serialize_unit_variant("Severity", 0u32, "minimal"),
                Self::Moderate => serializer.serialize_unit_variant("Severity", 1u32, "moderate"),
                Self::Critical => serializer.serialize_unit_variant("Severity", 2u32, "critical"),
                Self::Highestcriticalimpact => serializer.serialize_unit_variant("Severity", 3u32, "highestcriticalimpact"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status to be updated on the ticket."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "open")]
        Open,
        #[serde(rename = "closed")]
        Closed,
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
                Self::Open => serializer.serialize_unit_variant("Status", 0u32, "open"),
                Self::Closed => serializer.serialize_unit_variant("Status", 1u32, "closed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
