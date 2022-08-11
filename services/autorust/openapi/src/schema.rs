use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// http://json.schemastore.org/swagger-2.0

/// The transfer protocol of the API. Values MUST be from the list: "http", "https", "ws", "wss".
/// If the schemes is not included, the default scheme to be used is the one used to access the Swagger definition itself.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::Http
    }
}

/// https://swagger.io/docs/specification/data-models/data-types/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
    File,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#securityRequirementObject
pub type SecurityRequirement = IndexMap<String, Vec<String>>;

/// https://swagger.io/docs/specification/2-0/describing-responses/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#responseObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, ReferenceOr<Header>>,

    #[serde(rename = "x-ms-error-response", skip_serializing_if = "Option::is_none")]
    pub x_ms_error_response: Option<bool>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Boolean(bool),
    Schema(ReferenceOr<Schema>),
}

/// common fields in both Schema Object & Parameter Object
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaCommon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#items-object
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<DataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Box<Option<ReferenceOr<Schema>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,

    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,

    #[serde(rename = "x-ms-enum", skip_serializing_if = "Option::is_none")]
    pub x_ms_enum: Option<MsEnum>,

    #[serde(rename = "x-ms-client-name", skip_serializing_if = "Option::is_none")]
    pub x_ms_client_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<MsXml>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(flatten)]
    pub common: SchemaCommon,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, ReferenceOr<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Box<Option<AdditionalProperties>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<ReferenceOr<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    #[serde(rename = "x-ms-secret", skip_serializing_if = "Option::is_none")]
    pub x_ms_secret: Option<bool>,

    /// indicates that the Definition Schema Object is a resource as defined by the Resource Manager API
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-azure-resource
    #[serde(rename = "x-ms-azure-resource", skip_serializing_if = "Option::is_none")]
    pub x_ms_azure_resource: Option<bool>,

    /// provides insight to Autorest on how to generate code. It doesn't alter the modeling of what is actually sent on the wire
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-mutability
    #[serde(rename = "x-ms-mutability", default, skip_serializing_if = "Vec::is_empty")]
    pub x_ms_mutability: Vec<MsMutability>,

    /// allows specific Definition Objects to be excluded from code generation
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-external
    #[serde(rename = "x-ms-external", skip_serializing_if = "Option::is_none")]
    pub x_ms_external: Option<bool>,

    #[serde(rename = "x-nullable", skip_serializing_if = "Option::is_none")]
    pub x_nullable: Option<bool>,

    #[serde(rename = "x-ms-discriminator-value", skip_serializing_if = "Option::is_none")]
    pub x_ms_discriminator_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
}
