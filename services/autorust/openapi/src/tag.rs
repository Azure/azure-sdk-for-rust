use crate::ExternalDocumentation;
use serde::{Deserialize, Serialize};

// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#tagObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}
