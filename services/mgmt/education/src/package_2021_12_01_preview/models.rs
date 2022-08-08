#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The amount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Amount {
    #[doc = "The type of currency being used for the value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Amount value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl Amount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Describes the format of Error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for ErrorResponseBody {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Grant detail properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GrantDetailProperties {
    #[doc = "The amount."]
    #[serde(rename = "offerCap", default, skip_serializing_if = "Option::is_none")]
    pub offer_cap: Option<Amount>,
    #[doc = "Grant Effective Date"]
    #[serde(rename = "effectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
    #[doc = "Grant Offer Type"]
    #[serde(rename = "offerType", default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<grant_detail_properties::OfferType>,
    #[doc = "Expiration Date"]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Grant status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<grant_detail_properties::Status>,
    #[doc = "The amount."]
    #[serde(rename = "allocatedBudget", default, skip_serializing_if = "Option::is_none")]
    pub allocated_budget: Option<Amount>,
}
impl GrantDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod grant_detail_properties {
    use super::*;
    #[doc = "Grant Offer Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OfferType")]
    pub enum OfferType {
        Student,
        Academic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OfferType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OfferType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OfferType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Student => serializer.serialize_unit_variant("OfferType", 0u32, "Student"),
                Self::Academic => serializer.serialize_unit_variant("OfferType", 1u32, "Academic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Grant status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Inactive,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 1u32, "Inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Grant details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GrantDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Grant detail properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GrantDetailProperties>,
}
impl GrantDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Grants info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GrantListResponse {
    #[doc = "The list of labs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GrantDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GrantListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GrantListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "invite code generate request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InviteCodeGenerateRequest {
    #[doc = "the total number of students that can be accepted to the lab."]
    #[serde(rename = "maxStudentCount", default, skip_serializing_if = "Option::is_none")]
    pub max_student_count: Option<f64>,
}
impl InviteCodeGenerateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "join requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JoinRequestDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Join request properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JoinRequestProperties>,
}
impl JoinRequestDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "list of join requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JoinRequestList {
    #[doc = "The list of requests."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JoinRequestDetails>,
    #[doc = "the link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JoinRequestList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JoinRequestList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Join request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JoinRequestProperties {
    #[doc = "First Name"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last Name"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "join request email"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Join request status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<join_request_properties::Status>,
}
impl JoinRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod join_request_properties {
    use super::*;
    #[doc = "Join request status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Denied,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Denied => serializer.serialize_unit_variant("Status", 1u32, "Denied"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Lab details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Lab detail result properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabProperties>,
}
impl LabDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of labs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabListResult {
    #[doc = "The list of labs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LabListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LabListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab detail result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabProperties {
    #[doc = "Lab Display Name"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The amount."]
    #[serde(rename = "budgetPerStudent")]
    pub budget_per_student: Amount,
    #[doc = "Detail description of this lab"]
    pub description: String,
    #[doc = "Default expiration date for each student in this lab"]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339")]
    pub expiration_date: time::OffsetDateTime,
    #[doc = "Lab creation date"]
    #[serde(rename = "effectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
    #[doc = "The status of this lab"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<lab_properties::Status>,
    #[doc = "the total number of students that can be accepted to the lab."]
    #[serde(rename = "maxStudentCount", default, skip_serializing_if = "Option::is_none")]
    pub max_student_count: Option<f64>,
    #[doc = "invitation code for redeemable lab"]
    #[serde(rename = "invitationCode", default, skip_serializing_if = "Option::is_none")]
    pub invitation_code: Option<String>,
    #[doc = "The amount."]
    #[serde(rename = "totalBudget", default, skip_serializing_if = "Option::is_none")]
    pub total_budget: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "totalAllocatedBudget", default, skip_serializing_if = "Option::is_none")]
    pub total_allocated_budget: Option<Amount>,
}
impl LabProperties {
    pub fn new(display_name: String, budget_per_student: Amount, description: String, expiration_date: time::OffsetDateTime) -> Self {
        Self {
            display_name,
            budget_per_student,
            description,
            expiration_date,
            effective_date: None,
            status: None,
            max_student_count: None,
            invitation_code: None,
            total_budget: None,
            total_allocated_budget: None,
        }
    }
}
pub mod lab_properties {
    use super::*;
    #[doc = "The status of this lab"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Deleted,
        Pending,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 1u32, "Deleted"),
                Self::Pending => serializer.serialize_unit_variant("Status", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "redeem request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedeemRequest {
    #[doc = "redeem code"]
    #[serde(rename = "redeemCode")]
    pub redeem_code: String,
    #[doc = "first name of requester"]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[doc = "last name of requester"]
    #[serde(rename = "lastName")]
    pub last_name: String,
}
impl RedeemRequest {
    pub fn new(redeem_code: String, first_name: String, last_name: String) -> Self {
        Self {
            redeem_code,
            first_name,
            last_name,
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Student details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StudentDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Student detail properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StudentProperties>,
}
impl StudentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Student lab details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StudentLabDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Student lab detail properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StudentLabProperties>,
}
impl StudentLabDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of labs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StudentLabListResult {
    #[doc = "The list of labs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StudentLabDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StudentLabListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StudentLabListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Student lab detail properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StudentLabProperties {
    #[doc = "Student lab Display Name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detail description of this lab"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Date the lab will expire and by default will be the expiration date for each student in this lab"]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Student Role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<student_lab_properties::Role>,
    #[doc = "The amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<Amount>,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Student Lab Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<student_lab_properties::Status>,
    #[doc = "User Added Date"]
    #[serde(rename = "effectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
    #[doc = "Lab Scope. /providers/Microsoft.Billing/billingAccounts/{billingAccountName}/billingProfiles/{billingProfileName}/invoiceSections/{invoiceSectionName}/providers/Microsoft.Education/labs/default"]
    #[serde(rename = "labScope", default, skip_serializing_if = "Option::is_none")]
    pub lab_scope: Option<String>,
}
impl StudentLabProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod student_lab_properties {
    use super::*;
    #[doc = "Student Role"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        Student,
        Admin,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Student => serializer.serialize_unit_variant("Role", 0u32, "Student"),
                Self::Admin => serializer.serialize_unit_variant("Role", 1u32, "Admin"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Student Lab Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Disabled,
        Expired,
        Pending,
        Deleted,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::Expired => serializer.serialize_unit_variant("Status", 2u32, "Expired"),
                Self::Pending => serializer.serialize_unit_variant("Status", 3u32, "Pending"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 4u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of students."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StudentListResult {
    #[doc = "The list of students."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StudentDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StudentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StudentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Student detail properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudentProperties {
    #[doc = "First Name"]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[doc = "Last Name"]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[doc = "Student Email"]
    pub email: String,
    #[doc = "Student Role"]
    pub role: student_properties::Role,
    #[doc = "The amount."]
    pub budget: Amount,
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Date this student is set to expire from the lab."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339")]
    pub expiration_date: time::OffsetDateTime,
    #[doc = "Student Lab Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<student_properties::Status>,
    #[doc = "Date student was added to the lab"]
    #[serde(rename = "effectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription alias"]
    #[serde(rename = "subscriptionAlias", default, skip_serializing_if = "Option::is_none")]
    pub subscription_alias: Option<String>,
    #[doc = "subscription invite last sent date"]
    #[serde(rename = "subscriptionInviteLastSentDate", with = "azure_core::date::rfc3339::option")]
    pub subscription_invite_last_sent_date: Option<time::OffsetDateTime>,
}
impl StudentProperties {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        role: student_properties::Role,
        budget: Amount,
        expiration_date: time::OffsetDateTime,
    ) -> Self {
        Self {
            first_name,
            last_name,
            email,
            role,
            budget,
            subscription_id: None,
            expiration_date,
            status: None,
            effective_date: None,
            subscription_alias: None,
            subscription_invite_last_sent_date: None,
        }
    }
}
pub mod student_properties {
    use super::*;
    #[doc = "Student Role"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        Student,
        Admin,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Student => serializer.serialize_unit_variant("Role", 0u32, "Student"),
                Self::Admin => serializer.serialize_unit_variant("Role", 1u32, "Admin"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Student Lab Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Disabled,
        Expired,
        Pending,
        Deleted,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::Expired => serializer.serialize_unit_variant("Status", 2u32, "Expired"),
                Self::Pending => serializer.serialize_unit_variant("Status", 3u32, "Pending"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 4u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
