use serde::{Deserialize, Serialize};

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#contactObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // TODO: Make sure the url is a valid URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // TODO: Make sure the email is a valid email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
