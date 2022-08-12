use crate::{MsParameterGrouping, ReferenceOr, Schema, SchemaCommon};
use serde::{Deserialize, Serialize};

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    #[serde(flatten)]
    pub common: SchemaCommon,

    /// The name of the parameter.
    pub name: String,

    /// may be `header`, `query`, 'path`, `formData`
    #[serde(rename = "in")]
    pub in_: ParameterType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<CollectionFormat>,

    /// provides a mechanism to specify that the global parameter is actually a parameter on the operation and not a client property
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-location
    #[serde(rename = "x-ms-parameter-location", skip_serializing_if = "Option::is_none")]
    pub x_ms_parameter_location: Option<String>,

    /// skips URL encoding for path and query parameters
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-skip-url-encoding
    #[serde(rename = "x-ms-skip-url-encoding", skip_serializing_if = "Option::is_none")]
    pub x_ms_skip_url_encoding: Option<bool>,

    /// groups method parameters in generated clients
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-grouping
    #[serde(rename = "x-ms-parameter-grouping", skip_serializing_if = "Option::is_none")]
    pub x_ms_parameter_grouping: Option<MsParameterGrouping>,

    #[serde(rename = "x-ms-client-request-id", skip_serializing_if = "Option::is_none")]
    pub x_ms_client_request_id: Option<bool>,

    /// https://github.com/Azure/autorest/blob/main/docs/extensions/readme.md#x-ms-header-collection-prefix
    #[serde(rename = "x-ms-header-collection-prefix", skip_serializing_if = "Option::is_none")]
    pub x_ms_header_collection_prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    Path,
    Query,
    Header,
    Body,
    /// https://swagger.io/docs/specification/2-0/describing-parameters/#form-parameters
    FormData,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CollectionFormat {
    Csv,
    Ssv,
    Tsv,
    Pipes,
    Multi,
}
