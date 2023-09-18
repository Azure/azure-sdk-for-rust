#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The request object containing targets groups and a connection filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddToGroupsRequest {
    #[doc = "A list of groups which target connections will be added into"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<String>,
    #[doc = "An OData filter which target connections satisfy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl AddToGroupsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response object containing the token for the client"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientTokenResponse {
    #[doc = "The token value for the WebSocket client to connect to the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl ClientTokenResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner: Option<InnerError>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "A more specific error code than was provided by the containing error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner: Option<Box<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request object containing targets groups and a connection filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveFromGroupsRequest {
    #[doc = "A list of groups which target connections will be removed from"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<String>,
    #[doc = "An OData filter which target connections satisfy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl RemoveFromGroupsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
