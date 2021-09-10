use serde::{Deserialize, Serialize};

/// see Response Headers https://swagger.io/docs/specification/2-0/describing-responses/
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Header {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
