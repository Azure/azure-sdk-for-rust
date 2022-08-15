#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "The look up resource Id request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LookUpResourceIdRequest {
    #[doc = "The System generated Id that is unique. Use supportTicketId property for Microsoft.Support/supportTickets resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The type of resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<look_up_resource_id_request::Type>,
}
impl LookUpResourceIdRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod look_up_resource_id_request {
    use super::*;
    #[doc = "The type of resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Support/supportTickets")]
        MicrosoftSupportSupportTickets,
    }
}
#[doc = "The look up resource id response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LookUpResourceIdResponse {
    #[doc = "The resource Id of support resource type."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl LookUpResourceIdResponse {
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
