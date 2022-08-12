//! AutoRest Extensions for OpenAPI 2.0
//! https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md

use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-enum
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MsEnum {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_as_string: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<MsEnumValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde()]
pub struct MsEnumValue {
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// provides insight to Autorest on how to generate code. It doesn't alter the modeling of what is actually sent on the wire
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-mutability
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MsMutability {
    Create,
    Read,
    Update,
}

/// allows paging through lists of data
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-pageable
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MsPageable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    // nextLinkName is required, null is valid
    pub next_link_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

/// describes the format for specifying examples for request and response of an operation
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-examples
pub type MsExamples = IndexMap<String, ReferenceOr<Operation>>;

/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-long-running-operation-options
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MsLongRunningOperationOptions {
    #[serde(rename = "final-state-via")]
    pub final_state_via: MsLongRunningOperationOptionsFinalStateVia,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MsLongRunningOperationOptionsFinalStateVia {
    AzureAsyncOperation,
    Location,
    OriginalUri,
}

impl Default for MsLongRunningOperationOptionsFinalStateVia {
    fn default() -> Self {
        MsLongRunningOperationOptionsFinalStateVia::AzureAsyncOperation
    }
}

/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-location
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MsParameterLocation {
    Client,
    Method,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MsCodeGenerationSetting {
    String(String),
    Bool(bool),
}

/// replaces the fixed host with a host template that can be replaced with variable parameters
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameterized-host
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MsParameterizedHost {
    pub host_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_scheme_prefix: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_operation: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
}

/// groups method parameters in generated clients
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-grouping
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MsParameterGrouping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postfix: Option<String>,
}

// specify xml serialization
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MsXml {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,
    #[serde(rename = "x-ms-text", skip_serializing_if = "Option::is_none")]
    pub x_ms_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::MsPageable;

    #[test]
    fn pageable_nextlinkname_may_be_null() {
        let json = r#"{"x-ms-pageable":{"nextLinkName":null}}"#;
        serde_json::from_str::<MsPageable>(json).unwrap();
    }
}
