use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// This is the root document object for an example.
/// https://github.com/Azure/azure-rest-api-specs/blob/master/documentation/x-ms-examples.md
/// https://github.com/Azure/autorest/blob/main/packages/libs/autorest-schemas/example-schema.json
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Example {
    pub parameters: IndexMap<String, serde_json::Value>,
    pub responses: IndexMap<String, Response>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}
