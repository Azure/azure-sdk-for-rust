#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl azure_core::Continuable for AzureCoreFoundationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The schema, including its metadata and content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[doc = "String representation (UTF-8) of the schema."]
    #[serde(rename = "schemaContent")]
    pub schema_content: String,
}
impl Schema {
    pub fn new(schema_content: String) -> Self {
        Self { schema_content }
    }
}
#[doc = "Describes closed list of schema content type values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SchemaContentTypeValues {
    #[serde(rename = "application/json; serialization=Avro")]
    ApplicationJsonSerializationAvro,
    #[serde(rename = "application/json; serialization=Json")]
    ApplicationJsonSerializationJson,
    #[serde(rename = "text/plain; charset=utf-8")]
    TextPlainCharsetUtf8,
    #[serde(rename = "text/vnd.ms.protobuf")]
    TextVndMsProtobuf,
}
#[doc = "Schema Group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaGroup {
    #[doc = "Name of schema group."]
    #[serde(rename = "groupName")]
    pub group_name: String,
}
impl SchemaGroup {
    pub fn new(group_name: String) -> Self {
        Self { group_name }
    }
}
#[doc = "The list of schema group names with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaGroups {
    #[doc = "The collection of pageable schema group name items."]
    #[serde(rename = "Value")]
    pub value: Vec<String>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "NextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaGroups {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SchemaGroups {
    pub fn new(value: Vec<String>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Metadata of a schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaProperties {}
impl SchemaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of schema versions with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaVersions {
    #[doc = "The collection of schema version pageable items."]
    #[serde(rename = "Value")]
    pub value: Vec<i32>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "NextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaVersions {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SchemaVersions {
    pub fn new(value: Vec<i32>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Schemas resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemasName {
    #[doc = "Name of schema."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
}
impl SchemasName {
    pub fn new(schema_name: String) -> Self {
        Self { schema_name }
    }
}
