use serde::{Deserialize, Serialize};

use crate::MsEnum;

/// see Response Headers https://swagger.io/docs/specification/2-0/describing-responses/
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Header {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    // https://github.com/Azure/autorest/blob/main/docs/extensions/readme.md#x-ms-client-name
    #[serde(rename = "x-ms-client-name", skip_serializing_if = "Option::is_none")]
    pub x_ms_client_name: Option<String>,

    #[serde(rename = "x-ms-enum", skip_serializing_if = "Option::is_none")]
    pub x_ms_enum: Option<MsEnum>,

    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,

    /// https://github.com/Azure/autorest/blob/main/docs/extensions/readme.md#x-ms-header-collection-prefix
    #[serde(rename = "x-ms-header-collection-prefix", skip_serializing_if = "Option::is_none")]
    pub x_ms_header_collection_prefix: Option<String>,
}
