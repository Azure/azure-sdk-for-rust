use serde::{Deserialize, Serialize};

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#licenseObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct License {
    /// The name of the license type. It's encouraged to use an OSI
    /// compatible license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the license.
    // TODO: Make sure the url is a valid URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
