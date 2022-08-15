#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Agreement Terms definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementProperties {
    #[doc = "Publisher identifier string of image being deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Offer identifier string of image being deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Plan identifier string of image being deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[doc = "Link to HTML with Microsoft and Publisher terms."]
    #[serde(rename = "licenseTextLink", default, skip_serializing_if = "Option::is_none")]
    pub license_text_link: Option<String>,
    #[doc = "Link to the privacy policy of the publisher."]
    #[serde(rename = "privacyPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_link: Option<String>,
    #[doc = "Date and time in UTC of when the terms were accepted. This is empty if Accepted is false."]
    #[serde(rename = "retrieveDatetime", with = "azure_core::date::rfc3339::option")]
    pub retrieve_datetime: Option<time::OffsetDateTime>,
    #[doc = "Terms signature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "If any version of the terms have been accepted, otherwise false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}
impl AgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Terms properties for provided Publisher/Offer/Plan tuple"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementTerms {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Agreement Terms definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgreementProperties>,
}
impl AgreementTerms {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the list of AgreementTerms objects"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementTermsList {
    #[doc = "The value of the array."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OldAgreementTerms>,
}
impl AgreementTermsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Microsoft.MarketplaceOrdering service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
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
pub mod error_response {
    use super::*;
    #[doc = "The details of the error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Old Agreement Terms definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OldAgreementProperties {
    #[doc = "A unique identifier of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Publisher identifier string of image being deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Offer identifier string of image being deployed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "Date and time in UTC of when the terms were accepted. This is empty if state is cancelled."]
    #[serde(rename = "signDate", with = "azure_core::date::rfc3339::option")]
    pub sign_date: Option<time::OffsetDateTime>,
    #[doc = "Date and time in UTC of when the terms were cancelled. This is empty if state is active."]
    #[serde(rename = "cancelDate", with = "azure_core::date::rfc3339::option")]
    pub cancel_date: Option<time::OffsetDateTime>,
    #[doc = "Whether the agreement is active or cancelled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<old_agreement_properties::State>,
}
impl OldAgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod old_agreement_properties {
    use super::*;
    #[doc = "Whether the agreement is active or cancelled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Active,
        Canceled,
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
                Self::Active => serializer.serialize_unit_variant("State", 0u32, "Active"),
                Self::Canceled => serializer.serialize_unit_variant("State", 1u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Terms properties for provided Publisher/Offer/Plan tuple"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OldAgreementTerms {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Old Agreement Terms definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OldAgreementProperties>,
}
impl OldAgreementTerms {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.MarketplaceOrdering REST API operation"]
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
        #[doc = "Service provider: Microsoft.MarketplaceOrdering"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Agreement, virtualmachine, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Get Agreement, Sign Agreement, Cancel Agreement etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list MarketplaceOrdering operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.MarketplaceOrdering operations supported by the Microsoft.MarketplaceOrdering resource provider."]
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
#[doc = "ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response that indicates the media type in the request is unsupported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnsupportedMediaTypeErrorResponse {
    #[doc = "The details of the error."]
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for UnsupportedMediaTypeErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl UnsupportedMediaTypeErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
