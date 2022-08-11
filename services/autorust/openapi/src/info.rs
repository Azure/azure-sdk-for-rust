use crate::{Contact, License, MsCodeGenerationSetting};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// General information about the API.
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#info-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    /// A unique and precise title of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A semantic version number of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// enables passing code generation settings via OpenAPI definition
    /// (deprecated! Please use configuration files instead.)
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-code-generation-settings
    #[serde(rename = "x-ms-code-generation-settings", default, skip_serializing_if = "IndexMap::is_empty")]
    pub x_ms_code_generation_settings: IndexMap<String, MsCodeGenerationSetting>,
}
